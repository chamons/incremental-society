use crate::actions::{DelayedAction, Waiter};
use crate::data::get_edict;
use crate::engine::EngineError;
use crate::state::GameState;

pub fn can_invoke_edict(state: &GameState, edict: &str) -> Result<(), EngineError> {
    if state.action_with_name(edict).is_some() {
        return Err(EngineError::init("Edict already in progress"));
    }

    let edict = get_edict(edict);
    for cost in &edict.input {
        if !state.resources.has_amount(&cost) {
            return Err(EngineError::init("Insufficient resources for edict"));
        }
    }

    Ok(())
}

pub fn edict(state: &mut GameState, edict_name: &str) -> Result<(), EngineError> {
    can_invoke_edict(&state, edict_name)?;
    let edict = get_edict(edict_name);

    let action = Waiter::init_one_shot(edict_name, edict.tick_length(), DelayedAction::Edict(edict_name.to_string()));
    state.actions.push(action);
    state.recalculate();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::edict;
    use crate::resources::*;
    use std::error::Error;

    #[test]
    fn invoke_valid() {
        let mut state = GameState::init();
        state.resources[ResourceKind::Fuel] = 1;

        edict(&mut state, "TestEdict").unwrap();
        state.action_with_name("TestEdict").unwrap();
    }

    #[test]
    fn invoke_no_resources() {
        let mut state = GameState::init_test_game_state();
        assert_eq!("Insufficient resources for edict", edict(&mut state, "TestEdict").unwrap_err().description());
    }

    #[test]
    fn invoke_can_not_while_itself_in_flight() {
        let mut state = GameState::init_test_game_state();
        state.resources[ResourceKind::Fuel] = 1;
        edict(&mut state, "TestEdict").unwrap();

        assert_eq!("Edict already in progress", edict(&mut state, "TestEdict").unwrap_err().description());
    }

    #[test]
    fn invoke_twice() {
        let mut state = GameState::init_test_game_state();
        state.resources[ResourceKind::Fuel] = 2;

        edict(&mut state, "TestEdict").unwrap();
        assert_eq!("Edict already in progress", edict(&mut state, "TestEdict").unwrap_err().description());
    }
}
