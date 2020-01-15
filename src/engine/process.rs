use super::conversions;
use super::DerivedState;
use crate::data;
use crate::state::{DelayedAction, GameState, Region, ResourceTotal};

pub fn process_tick(state: &mut GameState) -> Option<&'static str> {
    apply_actions(state);
    super::limits::honor_storage_and_floors(state);
    super::disaster::invoke_disaster_if_needed(state)
}

fn apply_actions(state: &mut GameState) {
    let fired_actions = super::actions::tick_actions(&mut state.actions);
    for action in fired_actions.iter() {
        match action {
            DelayedAction::Edict(name) => conversions::apply_convert(state, name),
            DelayedAction::Conversion(name) => {
                for _ in 0..*state.derived_state.conversions.get(name).unwrap() {
                    conversions::apply_convert(state, name);
                }
            }
            DelayedAction::SustainPops() => {
                for _ in 0..state.derived_state.pops {
                    conversions::apply_convert(state, "Sustain Population");
                }
            }
        }
    }
}

pub fn recalculate(state: &mut GameState) {
    state.derived_state = DerivedState::calculate(&state);
    // See sync_building_to_conversions for the story on why we're doing this :(
    crate::engine::sync_building_to_conversions(state);
}

pub fn init_new_game_state() -> GameState {
    let mut state = GameState {
        resources: ResourceTotal::init(),
        regions: vec![
            Region::init_with_buildings("Lusitania", vec![data::get_building("Settlement"), data::get_building("Hunting Grounds")]),
            Region::init("Illyricum"),
        ],
        actions: vec![],
        derived_state: DerivedState::init(),
    };
    recalculate(&mut state);
    state
}

#[cfg(test)]
pub fn init_empty_game_state() -> GameState {
    let mut state = GameState {
        resources: ResourceTotal::init(),
        regions: vec![],
        actions: vec![],
        derived_state: DerivedState::init(),
    };
    recalculate(&mut state);
    state
}

#[cfg(test)]
pub fn init_test_game_state() -> GameState {
    let mut state = GameState {
        resources: ResourceTotal::init(),
        regions: vec![
            Region::init_with_buildings("Lusitania", vec![data::get_building("Test Building"), data::get_building("Test Building")]),
            Region::init_with_buildings("Illyricum", vec![data::get_building("Test Gather Hut")]),
        ],
        actions: vec![],
        derived_state: DerivedState::init(),
    };
    recalculate(&mut state);

    state
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data;
    use crate::data::get_edict;
    use crate::engine::edict;
    use crate::state::{GameState, ResourceKind, ResourceTotal};

    #[test]
    fn serialization() {
        let state = init_test_game_state();
        let save = state.save();
        let state = GameState::init_from_json(save);
        assert_eq!(2, state.regions.len());
    }

    #[test]
    fn buildings() {
        let state = init_test_game_state();
        let buildings = state.buildings();
        assert_eq!(3, buildings.len());
        assert_eq!("Test Building", buildings.get(0).unwrap().name);
        assert_eq!("Test Building", buildings.get(1).unwrap().name);
        assert_eq!("Test Gather Hut", buildings.get(2).unwrap().name);
    }

    #[test]
    fn process_tick_storage_limits_honored() {
        let mut state = init_test_game_state();
        state.resources[ResourceKind::Food] = state.derived_state.storage[ResourceKind::Food] - 1;
        state.resources[ResourceKind::Fuel] = state.derived_state.storage[ResourceKind::Fuel] - 1;
        state.action_with_name_mut("TestGather").unwrap().current_tick = 1;
        state.action_with_name_mut("TestChop").unwrap().current_tick = 1;

        process_tick(&mut state);
        assert_eq!(state.resources[ResourceKind::Food], state.derived_state.storage[ResourceKind::Food]);
        assert_eq!(state.resources[ResourceKind::Fuel], state.derived_state.storage[ResourceKind::Fuel]);
    }

    #[test]
    fn invoke_takes_times_to_complete() {
        let mut state = init_empty_game_state();
        state.resources[ResourceKind::Fuel] = 2;

        edict(&mut state, "TestEdict").unwrap();
        let edict_length = get_edict("TestEdict").tick_length();
        for _ in 0..edict_length {
            assert_eq!(2, state.actions.len());
            process_tick(&mut state);
        }
        assert_eq!(1, state.actions.len());
    }

    #[test]
    fn process_conversions_none_ready() {
        let mut state = init_test_game_state();
        process_tick(&mut state);
        assert_eq!(0, state.resources[ResourceKind::Food]);
        assert_eq!(0, state.resources[ResourceKind::Fuel]);
    }

    #[test]
    fn process_conversions_one_ready() {
        let mut state = init_test_game_state();
        state.action_with_name_mut("TestChop").unwrap().current_tick = 1;
        process_tick(&mut state);

        assert_eq!(0, state.resources[ResourceKind::Food]);
        assert_eq!(4, state.resources[ResourceKind::Fuel]);
    }

    #[test]
    fn process_conversions_many_ready() {
        let mut state = init_test_game_state();
        state.action_with_name_mut("TestGather").unwrap().current_tick = 1;
        state.action_with_name_mut("TestChop").unwrap().current_tick = 1;
        process_tick(&mut state);

        assert_eq!(1, state.resources[ResourceKind::Food]);
        assert_eq!(4, state.resources[ResourceKind::Fuel]);
    }
}
