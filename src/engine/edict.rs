use crate::actions::{DelayedAction, Waiter};
use crate::data::get_edict;
use crate::engine::EngineError;
use crate::state::GameState;

pub fn can_invoke_edict(state: &GameState, edict: &str) -> Result<(), EngineError> {
    if state.actions.iter().any(|x| {
        if let DelayedAction::Edict(name) = &x.action {
            if name == edict {
                return true;
            }
        }

        false
    }) {
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

    let action = Waiter::init_one_shot(edict.tick_length(), DelayedAction::Edict(edict_name.to_string()));
    state.actions.push(action);

    Ok(())
}
