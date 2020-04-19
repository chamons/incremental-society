use super::GameContext;
use crate::state::{DelayedAction, Waiter, SUSTAIN_POP_LENGTH, SUSTAIN_POP_NAME};
use std::collections::HashSet;

pub fn apply_convert(context: &mut GameContext, name: &str) {
    context.find_conversion(name).convert(&mut context.state.resources);
}

pub fn start_missing_converts(context: &mut GameContext) {
    let current_converts: HashSet<String> = context.state.conversion_names();
    let missing_converts = context.current_building_jobs.keys().filter(|x| !current_converts.contains(*x));

    for not_started in missing_converts {
        if context.state.job_count(not_started) > 0 {
            let conversion = context.find_conversion(not_started);
            let action = Waiter::init_repeating(not_started, conversion.tick_length(), DelayedAction::Conversion(not_started.to_string()));
            context.state.actions.push(action);
        }
    }

    if context.state.action_with_name(SUSTAIN_POP_NAME).is_none() {
        let action = Waiter::init_repeating(SUSTAIN_POP_NAME, SUSTAIN_POP_LENGTH, DelayedAction::SustainPops());
        context.state.actions.push(action);
    }
}

pub fn reset_conversion_status(context: &mut GameContext, name: &str) {
    context.state.action_with_name_mut(name).unwrap().reset();
}

pub fn clear_conversion(context: &mut GameContext, name: &str) -> Option<Waiter> {
    let to_remove = context.state.action_with_name(name)?;
    let pos_to_remove = context.state.actions.iter().position(|x| x.name == to_remove.name)?;
    Some(context.state.actions.remove(pos_to_remove))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::tests::*;
    use crate::engine::{build, jobs::add_job, process};
    use crate::state::{Region, ResourceKind, BUILD_LENGTH};

    #[test]
    fn valid_apply_convert() {
        let mut context = GameContext::init_test_game_context();
        assert_eq!(0, context.state.resources[ResourceKind::Food]);
        apply_convert(&mut context, "TestGather");
        assert_ne!(0, context.state.resources[ResourceKind::Food]);
    }

    #[test]
    fn start_missing_converts_sustain_only() {
        let mut context = GameContext::init_empty_test_game_context();
        // Ensure no actions are running
        context.state.actions.clear();

        start_missing_converts(&mut context);

        assert_eq!(1, context.state.actions.len());
        assert_eq!(SUSTAIN_POP_NAME, context.state.actions[0].name);
    }

    #[test]
    fn start_missing_both() {
        let mut context = GameContext::init_empty_test_game_context();
        context.state.pops = 2;

        // Ensure no actions are running
        context.state.actions.clear();

        let region = Region::init_with_buildings("TestRegion", vec![get_test_building("Test Building")]);
        context.state.regions.insert(0, region);
        context.recalculate();

        add_job(&mut context, "TestChop").unwrap();
        add_job(&mut context, "TestChop").unwrap();

        start_missing_converts(&mut context);

        assert_eq!(2, context.state.actions.len());
    }

    #[test]
    fn start_only_new() {
        let mut context = GameContext::init_test_game_context();
        context.state.pops = 5;

        add_job(&mut context, "TestChop").unwrap();
        add_job(&mut context, "TestChop").unwrap();
        add_job(&mut context, "TestGather").unwrap();
        assert_eq!(3, context.state.actions.len());

        build(&mut context, get_test_building("Test Hunt Cabin"), 0).unwrap();
        for _ in 0..BUILD_LENGTH {
            process::process_tick(&mut context);
        }
        add_job(&mut context, "TestHunt").unwrap();
        add_job(&mut context, "TestHunt").unwrap();

        start_missing_converts(&mut context);

        assert_eq!(4, context.state.actions.len());
    }

    #[test]
    fn state_none_if_no_jobs_set() {
        let mut context = GameContext::init_test_game_context();

        build(&mut context, get_test_building("Test Hunt Cabin"), 0).unwrap();
        for _ in 0..BUILD_LENGTH {
            process::process_tick(&mut context);
        }
        add_job(&mut context, "TestHunt").unwrap();

        start_missing_converts(&mut context);
        assert_eq!(1, context.state.actions.iter().filter(|x| x.action.is_conversion()).count());
    }

    #[test]
    fn reset_conversion() {
        let mut context = GameContext::init_test_game_context();
        add_job(&mut context, "TestChop").unwrap();

        let starting_tick = context.state.action_with_name("TestChop").unwrap().current_tick;
        process::process_tick(&mut context);
        assert_eq!(1, starting_tick - context.state.action_with_name("TestChop").unwrap().current_tick);

        reset_conversion_status(&mut context, "TestChop");

        assert_eq!(starting_tick, context.state.action_with_name("TestChop").unwrap().current_tick);
    }

    #[test]
    fn clear_conversion_removes_if_exists() {
        let mut context = GameContext::init_test_game_context();
        add_job(&mut context, "TestChop").unwrap();
        process::process_tick(&mut context);
        assert_is_some(context.state.action_with_name("TestChop"));

        clear_conversion(&mut context, "TestChop").unwrap();
        assert_is_none(context.state.action_with_name("TestChop"));
    }

    #[test]
    fn clear_conversion_none_if_not_found() {
        let mut context = GameContext::init_test_game_context();
        add_job(&mut context, "TestChop").unwrap();

        process::process_tick(&mut context);
        assert_is_none(clear_conversion(&mut context, "TestGather"));
    }
}
