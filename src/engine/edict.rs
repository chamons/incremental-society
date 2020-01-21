use super::process;
use super::EngineError;
use crate::data::get_edict;
use crate::state::{DelayedAction, GameState, Waiter};

pub fn can_invoke_edict(state: &GameState, edict: &str) -> Result<(), EngineError> {
    if state.actions.iter().any(|x| x.action.is_edict()) {
        return Err(EngineError::init("Edict already in progress"));
    }

    let edict = get_edict(edict);
    for cost in &edict.conversion.input {
        if !state.resources.has_amount(&cost) {
            return Err(EngineError::init("Insufficient resources for edict"));
        }
    }

    Ok(())
}

pub fn edict(state: &mut GameState, edict_name: &str) -> Result<(), EngineError> {
    can_invoke_edict(&state, edict_name)?;
    let edict = get_edict(edict_name);

    state.resources.remove_range(&edict.conversion.input);

    let action = Waiter::init_one_shot(edict_name, edict.conversion.tick_length(), DelayedAction::Edict(edict_name.to_string()));
    state.actions.push(action);
    process::recalculate(state);

    Ok(())
}

pub fn apply_edict(state: &mut GameState, name: &str) {
    // We've already paid the cost on queue, so just get the output
    state.resources.add_range(&get_edict(name).conversion.output);
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::{super::process, *};
    use crate::data;
    use crate::state::{Region, ResourceKind};

    #[test]
    fn invoke_valid() {
        let mut state = process::init_empty_game_state();
        state
            .regions
            .push(Region::init_with_buildings("Region", vec![data::get_building("Stability Building")]));
        state.resources[ResourceKind::Fuel] = 1;

        edict(&mut state, "TestEdict").unwrap();
        state.action_with_name("TestEdict").unwrap();
        assert_eq!(0, state.resources[ResourceKind::Fuel]);

        for _ in 0..get_edict("TestEdict").conversion.tick_length() {
            assert_eq!(0, state.resources[ResourceKind::Fuel]);
            assert_eq!(0, state.resources[ResourceKind::Knowledge]);

            process::process_tick(&mut state);
        }

        assert_eq!(0, state.resources[ResourceKind::Fuel]);
        assert_eq!(1, state.resources[ResourceKind::Knowledge]);
    }

    #[test]
    fn invoke_no_resources() {
        let mut state = process::init_test_game_state();
        assert_eq!("Insufficient resources for edict", edict(&mut state, "TestEdict").unwrap_err().description());
    }

    #[test]
    fn invoke_can_not_while_any_edict_in_flight() {
        let mut state = process::init_test_game_state();
        state.resources[ResourceKind::Fuel] = 1;
        edict(&mut state, "TestEdict").unwrap();

        assert_eq!("Edict already in progress", edict(&mut state, "OtherTestEdict").unwrap_err().description());
    }

    #[test]
    fn invoke_twice() {
        let mut state = process::init_test_game_state();
        state.resources[ResourceKind::Fuel] = 2;

        edict(&mut state, "TestEdict").unwrap();
        assert_eq!("Edict already in progress", edict(&mut state, "TestEdict").unwrap_err().description());
    }
}
