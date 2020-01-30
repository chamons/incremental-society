use std::collections::HashSet;

use crate::state::{DelayedAction, GameState, Waiter, SUSTAIN_POP_DURATION};

pub fn apply_convert(state: &mut GameState, name: &str) {
    state.derived_state.find_conversion(name).convert(&mut state.resources);
}

// There is a modeling problem the engine conversion code needs to handle:
// - The game state actions system (actions.rs) handles all conversions
// - That assumes that none of them are canceled in flight.
// - That makes sense for most things, but the source of conversions are buildings, which can appear/disappear at a whim
// - Imagine:
//     Region 1 has Library which gives Conversion Research
//     Build it on tick 100, Destroy on tick 110
//     There will be a Research conversion in flight with zero buildings "powering" it.
//     You really don't want to get research when it finishes, and you don't want to see it in your UI
// - You also need to "kick" a new delayed action on the first building, or anything adding a brand new conversion
//
// Ideally you'd be able to put this in DerivedState, as it gets blapped every major state change.
// However, you need to remember the ticks. If you have library 1 built and are 10 tick into a research
// and build a second, you want to still be ten ticks in.
// DerivedState also is not serialized on save, so we can't depend on it existing always.
//
// So after recalculating the derived state, we then "synchronize" the derived state conversion list with the actions list
// Make a list of every conversion provided, and every conversion in flight.
// - If there are new ones on the building side kick them.
// - If there any in the action list not in the conversion list, kill them
pub fn sync_building_to_conversions(state: &mut GameState) {
    let in_flight = get_in_flight(state);
    let active_conversions = &state.derived_state.conversions;
    for orphan in in_flight.iter().filter(|x| !active_conversions.contains_key(*x)) {
        let position = state.actions.iter().position(|x| matches_conversion_name(x, orphan)).unwrap();
        state.actions.remove(position);
    }

    for not_started in active_conversions.keys().filter(|x| !in_flight.contains(*x)) {
        let conversion = state.derived_state.find_conversion(not_started);
        let action = Waiter::init_repeating(not_started, conversion.tick_length(), DelayedAction::Conversion(not_started.to_string()));
        state.actions.push(action);
    }

    if state.action_with_name("Sustain Population").is_none() {
        let action = Waiter::init_repeating("Sustain Population", SUSTAIN_POP_DURATION, DelayedAction::SustainPops());
        state.actions.push(action);
    }
}

fn get_in_flight(state: &GameState) -> HashSet<String> {
    state.actions.iter().filter_map(filter_map_conversion_name).collect()
}

fn filter_map_conversion_name(waiter: &Waiter) -> Option<String> {
    if let DelayedAction::Conversion(name) = &waiter.action {
        Some(name.to_string())
    } else {
        None
    }
}

fn matches_conversion_name(waiter: &Waiter, name: &str) -> bool {
    if let DelayedAction::Conversion(conversion_name) = &waiter.action {
        conversion_name == name
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::tests::*;
    use crate::state::Region;

    #[test]
    fn existing_conversions_untouched_on_sync() {
        let mut state = init_test_game_state();
        assert_eq!(3, state.actions.len());

        recalculate(&mut state);
        assert_eq!(3, state.actions.len());
    }

    #[test]
    fn removed_buildings_remove_conversion_on_sync() {
        let mut state = init_empty_game_state();
        state
            .regions
            .push(Region::init_with_buildings("Region", vec![get_test_building("Test Gather Hut")]));
        recalculate(&mut state);
        assert_eq!(2, state.actions.len());

        state.regions.get_mut(0).unwrap().buildings.remove(0);
        recalculate(&mut state);

        assert_eq!(1, state.actions.len());
    }

    #[test]
    fn added_buildings_add_conversions_on_sync() {
        let mut state = init_empty_game_state();
        recalculate(&mut state);
        assert_eq!(1, state.actions.len());

        state
            .regions
            .push(Region::init_with_buildings("Region", vec![get_test_building("Test Gather Hut")]));
        recalculate(&mut state);

        assert_eq!(2, state.actions.len());
    }

    #[test]
    fn add_and_remove_multiple_on_sync() {
        let mut state = init_empty_game_state();
        state.regions.push(Region::init_with_buildings(
            "Region",
            vec![get_test_building("Test Building"), get_test_building("Test Gather Hut")],
        ));
        recalculate(&mut state);
        assert_eq!(3, state.actions.len());

        let region = state.regions.get_mut(0).unwrap();
        region.buildings.remove(0);
        region.buildings.push(get_test_building("Test Hunt Cabin"));

        recalculate(&mut state);

        assert_eq!(3, state.actions.len());
    }

    #[test]
    fn removed_then_readded_starts_at_zero_on_sync() {
        let mut state = init_empty_game_state();
        state
            .regions
            .push(Region::init_with_buildings("Region", vec![get_test_building("Test Gather Hut")]));
        recalculate(&mut state);

        state.action_with_name_mut("TestGather").unwrap().current_tick = 10;
        recalculate(&mut state);

        state.regions.get_mut(0).unwrap().buildings.remove(0);
        recalculate(&mut state);

        state.regions.get_mut(0).unwrap().buildings.push(get_test_building("Test Gather Hut"));
        recalculate(&mut state);

        assert_eq!(100, state.action_with_name("TestGather").unwrap().current_tick);
    }
}
