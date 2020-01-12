use crate::state::GameState;

pub fn process_tick(state: &mut GameState) -> Option<&'static str> {
    super::conversions::process_conversions(state);
    super::delayed::process_delayed_tick(state);
    super::limits::honor_storage_and_floors(state);
    super::disaster::invoke_disaster_if_needed(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data;
    use crate::engine::edict;
    use crate::resources::*;

    #[test]
    fn process_tick_storage_limits_honored() {
        let mut state = GameState::init_test_game_state();
        state.resources[ResourceKind::Food] = state.derived_state.storage[ResourceKind::Food] - 1;
        state.resources[ResourceKind::Fuel] = state.derived_state.storage[ResourceKind::Fuel] - 1;
        *state.ticks.entry("TestChop".to_string()).or_default() = 0;
        *state.ticks.entry("TestGather".to_string()).or_default() = 0;

        process_tick(&mut state);
        assert_eq!(state.resources[ResourceKind::Food], state.derived_state.storage[ResourceKind::Food]);
        assert_eq!(state.resources[ResourceKind::Fuel], state.derived_state.storage[ResourceKind::Fuel]);
    }

    #[test]
    fn invoke_takes_times_to_complete() {
        let mut state = GameState::init_test_game_state();
        state.resources[ResourceKind::Fuel] = 2;

        edict(&mut state, "TestEdict").unwrap();
        let edict_length = data::get_edict("TestEdict").tick_length();
        for _ in 0..=edict_length {
            assert!(state.ticks.contains_key("TestEdict"));
            process_tick(&mut state);
        }
        assert!(!state.ticks.contains_key("TestEdict"));
    }
}
