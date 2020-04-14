use super::{EngineError, GameContext};
use crate::state::{DelayedAction, Edict, GameState, Waiter};

pub fn can_invoke_edict(state: &GameState, edict: &Edict) -> Result<(), EngineError> {
    if state.actions.iter().any(|x| x.action.is_edict()) {
        return Err(EngineError::init("Edict already in progress"));
    }

    for cost in &edict.conversion.input {
        if !state.resources.has_amount(&cost) {
            return Err(EngineError::init("Insufficient resources for edict"));
        }
    }

    Ok(())
}

pub fn edict(context: &mut GameContext, edict: &Edict) -> Result<(), EngineError> {
    can_invoke_edict(&context.state, edict)?;

    context.state.resources.remove_range(&edict.conversion.input);

    let action = Waiter::init_one_shot(&edict.name, edict.conversion.tick_length(), DelayedAction::Edict(edict.name.to_owned()));
    context.state.actions.push(action);
    context.recalculate();

    Ok(())
}

pub fn apply_edict(context: &mut GameContext, name: &str) {
    // We've already paid the cost on queue, so just get the output
    let edict = context.find_edict(name);
    context.state.resources.add_range(&edict.conversion.output);
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::data::tests::*;
    use crate::engine::process;
    use crate::state::{Region, ResourceKind};

    #[test]
    fn invoke_valid() {
        let mut context = GameContext::init_empty_test_game_context();
        let region = Region::init_with_buildings("Region", vec![get_test_building("Stability Building")]);
        context.state.regions.push(region);
        context.state.resources[ResourceKind::Fuel] = 1;
        context.recalculate();

        let test_edict = get_test_edict("TestEdict");

        edict(&mut context, &test_edict).unwrap();
        context.state.action_with_name("TestEdict").unwrap();
        assert_eq!(0, context.state.resources[ResourceKind::Fuel]);

        for _ in 0..test_edict.conversion.tick_length() {
            assert_eq!(0, context.state.resources[ResourceKind::Fuel]);
            assert_eq!(0, context.state.resources[ResourceKind::Knowledge]);

            process::process_tick(&mut context);
        }

        assert_eq!(0, context.state.resources[ResourceKind::Fuel]);
        assert_eq!(1, context.state.resources[ResourceKind::Knowledge]);
    }

    #[test]
    fn invoke_no_resources() {
        let mut context = GameContext::init_test_game_context();
        let test_edict = get_test_edict("TestEdict");

        assert_eq!("Insufficient resources for edict", edict(&mut context, &test_edict).unwrap_err().to_string());
    }

    #[test]
    fn invoke_can_not_while_any_edict_in_flight() {
        let mut context = GameContext::init_test_game_context();
        context.state.resources[ResourceKind::Fuel] = 1;
        let test_edict = get_test_edict("TestEdict");

        edict(&mut context, &test_edict).unwrap();

        let other_test_edict = get_test_edict("OtherTestEdict");

        assert_eq!("Edict already in progress", edict(&mut context, &other_test_edict).unwrap_err().to_string());
    }

    #[test]
    fn invoke_twice() {
        let mut context = GameContext::init_test_game_context();
        context.state.resources[ResourceKind::Fuel] = 2;
        let test_edict = get_test_edict("TestEdict");

        edict(&mut context, &test_edict).unwrap();
        assert_eq!("Edict already in progress", edict(&mut context, &test_edict).unwrap_err().to_string());
    }
}
