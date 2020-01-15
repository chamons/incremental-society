use crate::state::{DelayedAction, GameState};

use crate::engine::conversions::apply_convert;

pub fn process_tick(state: &mut GameState) -> Option<&'static str> {
    apply_actions(state);
    super::limits::honor_storage_and_floors(state);
    super::disaster::invoke_disaster_if_needed(state)
}

fn apply_actions(state: &mut GameState) {
    let fired_actions = super::actions::tick_actions(&mut state.actions);
    for action in fired_actions.iter() {
        match action {
            DelayedAction::Edict(name) => apply_convert(state, name),
            DelayedAction::Conversion(name) => {
                for _ in 0..*state.derived_state.conversions.get(name).unwrap() {
                    apply_convert(state, name);
                }
            }
            DelayedAction::SustainPops() => {
                for _ in 0..state.derived_state.pops {
                    apply_convert(state, "Sustain Population");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::get_edict;
    use crate::engine::edict;
    use crate::state::ResourceKind;

    #[test]
    fn process_tick_storage_limits_honored() {
        let mut state = GameState::init_test_game_state();
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
        let mut state = GameState::init();
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
        let mut state = GameState::init_test_game_state();
        process_tick(&mut state);
        assert_eq!(0, state.resources[ResourceKind::Food]);
        assert_eq!(0, state.resources[ResourceKind::Fuel]);
    }

    #[test]
    fn process_conversions_one_ready() {
        let mut state = GameState::init_test_game_state();
        state.action_with_name_mut("TestChop").unwrap().current_tick = 1;
        process_tick(&mut state);

        assert_eq!(0, state.resources[ResourceKind::Food]);
        assert_eq!(4, state.resources[ResourceKind::Fuel]);
    }

    #[test]
    fn process_conversions_many_ready() {
        let mut state = GameState::init_test_game_state();
        state.action_with_name_mut("TestGather").unwrap().current_tick = 1;
        state.action_with_name_mut("TestChop").unwrap().current_tick = 1;
        process_tick(&mut state);

        assert_eq!(1, state.resources[ResourceKind::Food]);
        assert_eq!(4, state.resources[ResourceKind::Fuel]);
    }
}
