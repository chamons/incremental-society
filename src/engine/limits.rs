use std::cmp;

use super::GameContext;
use crate::state::{ResourceKind, NUM_RESOURCES};

pub fn honor_storage_and_floors(context: &mut GameContext) {
    for i in 0..NUM_RESOURCES {
        if context.state.resources[i] < 0 {
            // Instability can go negative and that's fine (everyone is happy)
            if ResourceKind::Instability == ResourceKind::name_for_index(i) {
                context.state.resources[i] = 0;
            } else {
                panic!(
                    "Resource {} had invalid value {} at end of tick processing",
                    ResourceKind::name_for_index(i),
                    context.state.resources[i]
                );
            }
        }

        context.state.resources[i] = cmp::min(context.state.resources[i], context.storage[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_limits_honored() {
        let mut context = GameContext::init_test_game_context();
        context.state.resources[ResourceKind::Food] = context.storage[ResourceKind::Food] + 1;
        context.state.resources[ResourceKind::Fuel] = context.storage[ResourceKind::Fuel] + 1;

        honor_storage_and_floors(&mut context);
        assert_eq!(context.state.resources[ResourceKind::Food], context.storage[ResourceKind::Food]);
        assert_eq!(context.state.resources[ResourceKind::Fuel], context.storage[ResourceKind::Fuel]);
    }

    #[test]
    fn process_tick_instability_floor_negative() {
        let mut context = GameContext::init_empty_test_game_context();
        context.state.resources[ResourceKind::Instability] = -10;
        honor_storage_and_floors(&mut context);
        assert_eq!(0, context.state.resources[ResourceKind::Instability]);
    }

    #[test]
    #[should_panic]
    fn process_tick_other_negative_die() {
        let mut context = GameContext::init_empty_test_game_context();
        context.state.resources[ResourceKind::Food] = -10;
        honor_storage_and_floors(&mut context);
    }
}
