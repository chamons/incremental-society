use super::process;
use super::EngineError;
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

pub fn edict(state: &mut GameState, edict: &Edict) -> Result<(), EngineError> {
    can_invoke_edict(&state, edict)?;

    state.resources.remove_range(&edict.conversion.input);

    let action = Waiter::init_one_shot(&edict.name, edict.conversion.tick_length(), DelayedAction::Edict(edict.name.to_owned()));
    state.actions.push(action);
    process::recalculate(state);

    Ok(())
}

pub fn apply_edict(state: &mut GameState, name: &str) {
    // We've already paid the cost on queue, so just get the output
    let edict = state.derived_state.find_edict(name);
    state.resources.add_range(&edict.conversion.output);
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
        let region = Region::init_with_buildings("Region", vec![data::get_building("Stability Building")]);
        state.regions.push(region);
        state.resources[ResourceKind::Fuel] = 1;

        let test_edict = data::get_edict("TestEdict");

        edict(&mut state, &test_edict).unwrap();
        state.action_with_name("TestEdict").unwrap();
        assert_eq!(0, state.resources[ResourceKind::Fuel]);

        for _ in 0..test_edict.conversion.tick_length() {
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
        let test_edict = data::get_edict("TestEdict");

        assert_eq!("Insufficient resources for edict", edict(&mut state, &test_edict).unwrap_err().description());
    }

    #[test]
    fn invoke_can_not_while_any_edict_in_flight() {
        let mut state = process::init_test_game_state();
        state.resources[ResourceKind::Fuel] = 1;
        let test_edict = data::get_edict("TestEdict");

        edict(&mut state, &test_edict).unwrap();

        let other_test_edict = data::get_edict("OtherTestEdict");

        assert_eq!("Edict already in progress", edict(&mut state, &other_test_edict).unwrap_err().description());
    }

    #[test]
    fn invoke_twice() {
        let mut state = process::init_test_game_state();
        state.resources[ResourceKind::Fuel] = 2;
        let test_edict = data::get_edict("TestEdict");

        edict(&mut state, &test_edict).unwrap();
        assert_eq!("Edict already in progress", edict(&mut state, &test_edict).unwrap_err().description());
    }
}
