use crate::state::{DelayedAction, GameState, Waiter, SUSTAIN_POP_DURATION, SUSTAIN_POP_NAME};
use std::collections::HashSet;

pub fn apply_convert(state: &mut GameState, name: &str) {
    state.derived_state.find_conversion(name).convert(&mut state.resources);
}

pub fn start_missing_converts(state: &mut GameState) {
    let current_converts: HashSet<String> = state.conversion_names();
    let missing_converts = state.derived_state.current_building_jobs.keys().filter(|x| !current_converts.contains(*x));

    for not_started in missing_converts {
        if state.job_count(not_started) > 0 {
            let conversion = state.derived_state.find_conversion(not_started);
            let action = Waiter::init_repeating(not_started, conversion.tick_length(), DelayedAction::Conversion(not_started.to_string()));
            state.actions.push(action);
        }
    }

    if state.action_with_name(SUSTAIN_POP_NAME).is_none() {
        let action = Waiter::init_repeating(SUSTAIN_POP_NAME, SUSTAIN_POP_DURATION, DelayedAction::SustainPops());
        state.actions.push(action);
    }
}

pub fn reset_conversion_status(state: &mut GameState, name: &str) {
    state.action_with_name_mut(name).unwrap().reset();
}

pub fn clear_conversion(state: &mut GameState, name: &str) -> Option<Waiter> {
    let to_remove = state.action_with_name(name)?;
    let pos_to_remove = state.actions.iter().position(|x| x.name == to_remove.name)?;
    Some(state.actions.remove(pos_to_remove))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::tests::*;
    use crate::engine::{build, jobs::add_job, process};
    use crate::state::{Region, ResourceKind, BUILD_LENGTH};

    #[test]
    fn valid_apply_convert() {
        let mut state = init_test_game_state();
        assert_eq!(0, state.resources[ResourceKind::Food]);
        apply_convert(&mut state, "TestGather");
        assert_ne!(0, state.resources[ResourceKind::Food]);
    }

    #[test]
    fn start_missing_converts_sustain_only() {
        let mut state = init_empty_game_state();
        // Ensure no actions are running
        state.actions.clear();

        start_missing_converts(&mut state);

        assert_eq!(1, state.actions.len());
        assert_eq!(SUSTAIN_POP_NAME, state.actions[0].name);
    }

    #[test]
    fn start_missing_both() {
        let mut state = init_empty_game_state();
        state.pops = 2;

        // Ensure no actions are running
        state.actions.clear();

        let region = Region::init_with_buildings("TestRegion", vec![get_test_building("Test Building")]);
        state.regions.insert(0, region);
        recalculate(&mut state);

        add_job(&mut state, "TestChop").unwrap();
        add_job(&mut state, "TestChop").unwrap();

        start_missing_converts(&mut state);

        assert_eq!(2, state.actions.len());
    }

    #[test]
    fn start_only_new() {
        let mut state = init_test_game_state();
        state.pops = 5;

        add_job(&mut state, "TestChop").unwrap();
        add_job(&mut state, "TestChop").unwrap();
        add_job(&mut state, "TestGather").unwrap();
        assert_eq!(3, state.actions.len());

        build(&mut state, get_test_building("Test Hunt Cabin"), 0).unwrap();
        for _ in 0..BUILD_LENGTH {
            process::process_tick(&mut state);
        }
        add_job(&mut state, "TestHunt").unwrap();
        add_job(&mut state, "TestHunt").unwrap();

        start_missing_converts(&mut state);

        assert_eq!(4, state.actions.len());
    }

    #[test]
    fn state_none_if_no_jobs_set() {
        let mut state = init_test_game_state();
        assert_eq!(1, state.actions.len());

        build(&mut state, get_test_building("Test Hunt Cabin"), 0).unwrap();
        for _ in 0..BUILD_LENGTH {
            process::process_tick(&mut state);
        }

        start_missing_converts(&mut state);
        assert_eq!(1, state.actions.len());
    }

    #[test]
    fn reset_conversion() {
        let mut state = init_test_game_state();
        add_job(&mut state, "TestChop").unwrap();

        let starting_tick = state.action_with_name("TestChop").unwrap().current_tick;
        process::process_tick(&mut state);
        assert_eq!(1, starting_tick - state.action_with_name("TestChop").unwrap().current_tick);

        reset_conversion_status(&mut state, "TestChop");

        assert_eq!(starting_tick, state.action_with_name("TestChop").unwrap().current_tick);
    }

    #[test]
    fn clear_conversion_removes_if_exists() {
        let mut state = init_test_game_state();
        add_job(&mut state, "TestChop").unwrap();
        process::process_tick(&mut state);
        assert_is_some(state.action_with_name("TestChop"));

        clear_conversion(&mut state, "TestChop").unwrap();
        assert_is_none(state.action_with_name("TestChop"));
    }

    #[test]
    fn clear_conversion_none_if_not_found() {
        let mut state = init_test_game_state();
        add_job(&mut state, "TestChop").unwrap();

        process::process_tick(&mut state);
        assert_is_none(clear_conversion(&mut state, "TestGather"));
    }
}
