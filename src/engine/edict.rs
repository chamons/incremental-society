use crate::data::get_edict;
use crate::engine::EngineError;
use crate::state::GameState;

pub fn can_invoke_edict(state: &GameState, edict: &str) -> Result<(), EngineError> {
    let edict = get_edict(edict);
    for cost in &edict.input {
        if !state.resources.has_amount(&cost) {
            return Err(EngineError::init("Insufficient resources for edict"));
        }
    }

    Ok(())
}

pub fn edict(state: &mut GameState, edict: &str) -> Result<(), EngineError> {
    can_invoke_edict(&state, edict)?;
    let edict = get_edict(edict);

    edict.convert(&mut state.resources);
    state.recalculate();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::*;

    #[test]
    fn invoke_valid() {
        let mut state = GameState::init_test_game_state();
        state.resources[ResourceKind::Fuel] = 1;

        edict(&mut state, "TestEdict").unwrap();
        assert_eq!(1, state.resources[ResourceKind::Knowledge]);
    }

    #[test]
    fn invoke_no_resources() {
        let mut state = GameState::init_test_game_state();
        assert!(edict(&mut state, "TestEdict").is_err());
    }
}
