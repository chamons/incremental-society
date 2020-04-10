use crate::state::{DelayedAction, GameState, Waiter, SUSTAIN_POP_DURATION, SUSTAIN_POP_NAME};
use std::collections::HashSet;

pub fn apply_convert(state: &mut GameState, name: &str) {
    state.derived_state.find_conversion(name).convert(&mut state.resources);
}

pub fn start_missing_converts(state: &mut GameState) {
    let current_converts: HashSet<String> = state.conversion_names();
    let missing_converts = state.derived_state.current_building_jobs.keys().filter(|x| !current_converts.contains(*x));

    for not_started in missing_converts {
        let conversion = state.derived_state.find_conversion(not_started);
        let action = Waiter::init_repeating(not_started, conversion.tick_length(), DelayedAction::Conversion(not_started.to_string()));
        state.actions.push(action);
    }

    if state.action_with_name(SUSTAIN_POP_NAME).is_none() {
        let action = Waiter::init_repeating(SUSTAIN_POP_NAME, SUSTAIN_POP_DURATION, DelayedAction::SustainPops());
        state.actions.push(action);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::tests::*;
    use crate::state::{Region, ResourceKind};

    #[test]
    pub fn valid_apply_convert() {
        let mut state = init_test_game_state();
        assert_eq!(0, state.resources[ResourceKind::Food]);
        apply_convert(&mut state, "TestGather");
        assert_ne!(0, state.resources[ResourceKind::Food]);
    }
    #[test]
    pub fn start_missing_converts_sustain_only() {
        let mut state = init_empty_game_state();
        // Ensure no actions are running
        state.actions.clear();

        start_missing_converts(&mut state);

        assert_eq!(1, state.actions.len());
        assert_eq!(SUSTAIN_POP_NAME, state.actions[0].name);
    }

    #[test]
    pub fn start_missing_both() {
        let mut state = init_empty_game_state();
        // Ensure no actions are running
        state.actions.clear();

        state
            .regions
            .insert(0, Region::init_with_buildings("TestRegion", vec![get_test_building("Test Building")]));
        recalculate(&mut state);
        start_missing_converts(&mut state);

        assert_eq!(2, state.actions.len());
    }

    #[test]
    pub fn start_only_new() {
        let mut state = init_test_game_state();
        assert_eq!(3, state.actions.len());

        super::super::build(&mut state, get_test_building("Test Hunt Cabin"), 0).unwrap();

        start_missing_converts(&mut state);

        assert_eq!(4, state.actions.len());
    }
}
