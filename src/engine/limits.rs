use std::cmp;

use crate::state::{GameState, ResourceKind, NUM_RESOURCES};

pub fn honor_storage_and_floors(state: &mut GameState) {
    for i in 0..NUM_RESOURCES {
        if state.resources[i] < 0 {
            // Instability can go negative and that's fine (everyone is happy)
            if ResourceKind::Instability == ResourceKind::name_for_index(i) {
                state.resources[i] = 0;
            } else {
                panic!(
                    "Resource {} had invalid value {} at end of tick processing",
                    ResourceKind::name_for_index(i),
                    state.resources[i]
                );
            }
        }

        state.resources[i] = cmp::min(state.resources[i], state.derived_state.storage[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::super::process;
    use super::*;

    #[test]
    fn storage_limits_honored() {
        let mut state = process::init_test_game_state();
        state.resources[ResourceKind::Food] = state.derived_state.storage[ResourceKind::Food] + 1;
        state.resources[ResourceKind::Fuel] = state.derived_state.storage[ResourceKind::Fuel] + 1;

        honor_storage_and_floors(&mut state);
        assert_eq!(state.resources[ResourceKind::Food], state.derived_state.storage[ResourceKind::Food]);
        assert_eq!(state.resources[ResourceKind::Fuel], state.derived_state.storage[ResourceKind::Fuel]);
    }

    #[test]
    fn process_tick_instability_floor_negative() {
        let mut state = process::init_empty_game_state();
        state.resources[ResourceKind::Instability] = -10;
        honor_storage_and_floors(&mut state);
        assert_eq!(0, state.resources[ResourceKind::Instability]);
    }

    #[test]
    #[should_panic]
    fn process_tick_other_negative_die() {
        let mut state = process::init_empty_game_state();
        state.resources[ResourceKind::Food] = -10;
        honor_storage_and_floors(&mut state);
    }
}
