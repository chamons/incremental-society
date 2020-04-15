use std::collections::HashMap;

use super::conversions::{clear_conversion, reset_conversion_status, start_missing_converts};
use super::{EngineError, GameContext};
use crate::state::Building;

pub fn add_job(context: &mut GameContext, name: &str) -> Result<(), EngineError> {
    let current_job_usage: u32 = context.state.total_jobs_assigned();
    if current_job_usage + 1 > context.state.pops {
        return Err(EngineError::init(format!("No free population to assign to {} job", name)));
    }

    match context.current_building_jobs.get(name) {
        Some(available_count) => {
            let current_count = context.state.jobs.entry(name.to_string()).or_insert(0);
            if *current_count < *available_count {
                *current_count += 1;
                if *current_count == 1 {
                    start_missing_converts(context);
                }

                if context.state.action_with_name(name).is_some() {
                    reset_conversion_status(context, name);
                }

                context.recalculate();
                Ok(())
            } else {
                Err(EngineError::init(format!("{} does not have another available slot", name)))
            }
        }
        None => Err(EngineError::init(format!("{} is not a valid current job", name))),
    }
}

pub fn remove_job(context: &mut GameContext, name: &str) -> Result<(), EngineError> {
    match context.current_building_jobs.get(name) {
        Some(_) => match context.state.jobs.get_mut(name) {
            Some(current_count) => {
                if *current_count > 0 {
                    *current_count -= 1;
                    if *current_count == 0 {
                        clear_conversion(context, name);
                    } else if context.state.action_with_name(name).is_some() {
                        reset_conversion_status(context, name);
                    }

                    context.recalculate();
                    Ok(())
                } else {
                    Err(EngineError::init(format!("{} does not have an active slot", name)))
                }
            }
            None => Err(EngineError::init(format!("{} does not have an active slot", name))),
        },
        None => Err(EngineError::init(format!("{} is not a valid current job", name))),
    }
}

pub fn reduce_active_jobs_by_loss(context: &mut GameContext, building: &Building) {
    let mut building_job_count = HashMap::new();
    for job in building.jobs.iter() {
        building_job_count.entry(job).and_modify(|e| *e += 1).or_insert(1);
    }

    for (job_lost, count) in building_job_count.iter() {
        let new_building_max = context.current_building_jobs[&(*job_lost).to_string()] - count;
        if new_building_max < context.state.job_count(job_lost) {
            context.state.jobs.insert((*job_lost).to_string(), new_building_max);

            if new_building_max == 0 {
                clear_conversion(context, job_lost);
            } else {
                reset_conversion_status(context, job_lost);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::tests::*;
    use crate::engine::{add_job, destroy, process};
    use crate::state::DESTROY_LENGTH;

    #[test]
    fn add_with_open_spot() {
        let mut context = GameContext::init_test_game_context();
        add_job(&mut context, "TestGather").unwrap();
        assert_eq!(1, context.state.jobs["TestGather"]);
    }

    #[test]
    fn add_with_no_pops() {
        let mut context = GameContext::init_test_game_context();
        add_job(&mut context, "TestChop").unwrap();
        let error = add_job(&mut context, "TestChop").unwrap_err();
        assert_eq!("No free population to assign to TestChop job", error.to_string());
    }

    #[test]
    fn add_with_non_existent_job() {
        let mut context = GameContext::init_test_game_context();
        let error = add_job(&mut context, "NotAJob").unwrap_err();
        assert_eq!("NotAJob is not a valid current job", error.to_string());
    }

    #[test]
    fn add_with_no_full_spot() {
        let mut context = GameContext::init_test_game_context();
        context.state.pops = 2;
        add_job(&mut context, "TestGather").unwrap();
        let error = add_job(&mut context, "TestGather").unwrap_err();
        assert_eq!("TestGather does not have another available slot", error.to_string());
    }

    #[test]
    fn add_with_conversion_in_flight_resets() {
        let mut context = GameContext::init_test_game_context();
        context.state.pops = 2;

        add_job(&mut context, "TestChop").unwrap();
        process::process_tick(&mut context);
        let tick_before_assign = context.state.action_with_name("TestChop").unwrap().current_tick;
        add_job(&mut context, "TestChop").unwrap();
        assert_ne!(tick_before_assign, context.state.action_with_name("TestChop").unwrap().current_tick);
    }

    #[test]
    fn remove_with_active_job() {
        let mut context = GameContext::init_test_game_context();
        add_job(&mut context, "TestGather").unwrap();
        assert_is_some(context.state.action_with_name("TestGather"));

        remove_job(&mut context, "TestGather").unwrap();
        assert_eq!(0, context.state.jobs["TestGather"]);
        assert_is_none(context.state.action_with_name("TestGather"));
    }

    #[test]
    fn remove_with_non_existent_job() {
        let mut context = GameContext::init_test_game_context();
        let error = remove_job(&mut context, "NotAJob").unwrap_err();
        assert_eq!("NotAJob is not a valid current job", error.to_string());
    }

    #[test]
    fn remove_with_no_active_job() {
        let mut context = GameContext::init_test_game_context();
        let error = remove_job(&mut context, "TestGather").unwrap_err();
        assert_eq!("TestGather does not have an active slot", error.to_string());
    }

    #[test]
    fn remove_with_conversion_in_flight_resets() {
        let mut context = GameContext::init_test_game_context();
        add_job(&mut context, "TestGather").unwrap();
        process::process_tick(&mut context);
        assert_is_some(context.state.action_with_name("TestGather"));
        remove_job(&mut context, "TestGather").unwrap();
        assert_is_none(context.state.action_with_name("TestGather"));
    }

    #[test]
    fn reduce_jobs_no_active_jobs_lost() {
        let mut context = GameContext::init_test_game_context();
        add_job(&mut context, "TestChop").unwrap();
        process::process_tick(&mut context);
        assert_eq!(0, context.state.job_count("TestGather"));

        reduce_active_jobs_by_loss(&mut context, &get_test_building("Test Building"));
        assert_eq!(0, context.state.job_count("TestGather"));
    }

    #[test]
    fn reduce_jobs_one_active_job_lost() {
        let mut context = GameContext::init_test_game_context();
        context.state.pops = 4;

        add_job(&mut context, "TestGather").unwrap();
        for _ in 0..3 {
            add_job(&mut context, "TestChop").unwrap();
        }
        process::process_tick(&mut context);
        assert_eq!(3, context.state.job_count("TestChop"));

        reduce_active_jobs_by_loss(&mut context, &get_test_building("Test Building"));
        assert_eq!(2, context.state.job_count("TestChop"));
        assert_eq!(1, context.state.job_count("TestGather"));
    }

    #[test]
    fn reduce_jobs_many_active_jobs_lost() {
        let mut context = GameContext::init_test_game_context();
        context.state.pops = 4;

        for _ in 0..3 {
            add_job(&mut context, "TestChop").unwrap();
        }
        process::process_tick(&mut context);
        assert_eq!(3, context.state.job_count("TestChop"));

        reduce_active_jobs_by_loss(&mut context, &get_test_building("Test Building"));
        assert_eq!(2, context.state.job_count("TestChop"));
        destroy(&mut context, 0, 0).unwrap();
        for _ in 0..DESTROY_LENGTH {
            process::process_tick(&mut context);
        }

        reduce_active_jobs_by_loss(&mut context, &get_test_building("Test Building"));
        assert_eq!(0, context.state.job_count("TestChop"));
    }
}
