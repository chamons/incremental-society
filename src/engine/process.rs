use crate::actions::DelayedAction;
use crate::engine::conversions::apply_convert;
use crate::state::GameState;

pub fn process_tick(state: &mut GameState) -> Option<&'static str> {
    super::conversions::sync_building_to_conversions(state);
    apply_actions(state);

    super::limits::honor_storage_and_floors(state);
    super::disaster::invoke_disaster_if_needed(state)
}

fn apply_actions(state: &mut GameState) {
    let fired_actions = super::actions::tick_actions(&mut state.actions);
    for a in fired_actions.iter() {
        match a {
            DelayedAction::Edict(name) => apply_convert(state, name),
            DelayedAction::Conversion(name) => {
                for _ in 0..*state.derived_state.conversions.get(name).unwrap() {
                    apply_convert(state, name);
                }
            }
            DelayedAction::SustainPops() => {}
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::data;
//     use crate::engine::edict;
//     use crate::resources::*;

//     #[test]
//     fn process_tick_storage_limits_honored() {
//         let mut state = GameState::init_test_game_state();
//         state.resources[ResourceKind::Food] = state.derived_state.storage[ResourceKind::Food] - 1;
//         state.resources[ResourceKind::Fuel] = state.derived_state.storage[ResourceKind::Fuel] - 1;
//         *state.ticks.entry("TestChop".to_string()).or_default() = 0;
//         *state.ticks.entry("TestGather".to_string()).or_default() = 0;

//         process_tick(&mut state);
//         assert_eq!(state.resources[ResourceKind::Food], state.derived_state.storage[ResourceKind::Food]);
//         assert_eq!(state.resources[ResourceKind::Fuel], state.derived_state.storage[ResourceKind::Fuel]);
//     }

//     #[test]
//     fn invoke_takes_times_to_complete() {
//         let mut state = GameState::init_test_game_state();
//         state.resources[ResourceKind::Fuel] = 2;

//         edict(&mut state, "TestEdict").unwrap();
//         let edict_length = data::get_edict("TestEdict").tick_length();
//         for _ in 0..=edict_length {
//             assert!(state.ticks.contains_key("TestEdict"));
//             process_tick(&mut state);
//         }
//         assert!(!state.ticks.contains_key("TestEdict"));
//     }
// }

// #[test]
// fn simple_process() {
//     let mut state = GameState::init_test_game_state();
//     process_conversions(&mut state);

//     assert_eq!(0.01, get_conversion_percentage(&state, "TestChop").unwrap());
//     assert_eq!(0.01, get_conversion_percentage(&state, "TestGather").unwrap());
// }

// #[test]
// fn process_conversions_none_ready() {
//     let mut state = GameState::init_test_game_state();
//     process_conversions(&mut state);
//     assert_eq!(0, state.resources[ResourceKind::Food]);
//     assert_eq!(0, state.resources[ResourceKind::Fuel]);
// }

// #[test]
// fn process_conversions_one_ready() {
//     let mut state = GameState::init_test_game_state();
//     *state.ticks.entry("TestChop".to_string()).or_default() = 0;
//     process_conversions(&mut state);

//     assert_eq!(0, state.resources[ResourceKind::Food]);
//     assert_eq!(4, state.resources[ResourceKind::Fuel]);
// }

// #[test]
// fn process_conversions_many_ready() {
//     let mut state = GameState::init_test_game_state();
//     *state.ticks.entry("TestChop".to_string()).or_default() = 0;
//     *state.ticks.entry("TestGather".to_string()).or_default() = 0;
//     process_conversions(&mut state);

//     assert_eq!(1, state.resources[ResourceKind::Food]);
//     assert_eq!(4, state.resources[ResourceKind::Fuel]);
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::resources::*;
//     use std::error::Error;

//     #[test]
//     fn invoke_valid() {
//         let mut state = GameState::init_test_game_state();
//         state.resources[ResourceKind::Fuel] = 1;

//         edict(&mut state, "TestEdict").unwrap();
//         assert!(state.actions.contains_key("TestEdict"));
//     }

//     #[test]
//     fn invoke_no_resources() {
//         let mut state = GameState::init_test_game_state();
//         assert_eq!("Insufficient resources for edict", edict(&mut state, "TestEdict").unwrap_err().description());
//     }

//     #[test]
//     fn invoke_can_not_while_itself_in_flight() {
//         let mut state = GameState::init_test_game_state();
//         state.resources[ResourceKind::Fuel] = 1;
//         *state.ticks.entry("TestEdict".to_string()).or_default() = 10;

//         assert_eq!("Edict already in progress", edict(&mut state, "TestEdict").unwrap_err().description());
//     }

//     #[test]
//     fn invoke_twice() {
//         let mut state = GameState::init_test_game_state();
//         state.resources[ResourceKind::Fuel] = 2;

//         edict(&mut state, "TestEdict").unwrap();
//         assert_eq!("Edict already in progress", edict(&mut state, "TestEdict").unwrap_err().description());
//     }
// }
