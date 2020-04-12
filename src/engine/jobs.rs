use std::collections::HashMap;

use super::conversions::{clear_conversion, reset_conversion_status};
use super::{process, EngineError};
use crate::state::{Building, GameState};

pub fn add_job(state: &mut GameState, name: &str) -> Result<(), EngineError> {
    let current_job_usage: u32 = state.total_jobs_assigned();
    if current_job_usage + 1 > state.pops {
        return Err(EngineError::init(format!("No free population to assign to {} job", name)));
    }

    match state.derived_state.current_building_jobs.get(name) {
        Some(available_count) => {
            let current_count = state.jobs.entry(name.to_string()).or_insert(0);
            if *current_count < *available_count {
                *current_count += 1;

                if state.action_with_name(name).is_some() {
                    reset_conversion_status(state, name);
                }
                process::recalculate(state);
                Ok(())
            } else {
                Err(EngineError::init(format!("{} does not have another available slot", name)))
            }
        }
        None => Err(EngineError::init(format!("{} is not a valid current job", name))),
    }
}

pub fn remove_job(state: &mut GameState, name: &str) -> Result<(), EngineError> {
    match state.derived_state.current_building_jobs.get(name) {
        Some(_) => match state.jobs.get_mut(name) {
            Some(current_count) => {
                if *current_count > 0 {
                    *current_count -= 1;
                    if *current_count == 0 {
                        clear_conversion(state, name);
                    } else if state.action_with_name(name).is_some() {
                        reset_conversion_status(state, name);
                    }

                    process::recalculate(state);
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

pub fn reduce_active_jobs_by_loss(state: &mut GameState, building: &Building) {
    let mut building_job_count = HashMap::new();
    for job in building.jobs.iter() {
        building_job_count.entry(job).and_modify(|e| *e += 1).or_insert(1);
    }

    for (job_lost, count) in building_job_count.iter() {
        let new_building_max = state.derived_state.current_building_jobs[&(*job_lost).to_string()] - count;
        if new_building_max < state.job_count(job_lost) {
            state.jobs.insert((*job_lost).to_string(), new_building_max);

            if new_building_max == 0 {
                clear_conversion(state, job_lost);
            } else {
                reset_conversion_status(state, job_lost);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::tests::*;
    use crate::engine::{add_job, destroy, process_tick};
    use crate::state::DESTROY_LENGTH;

    #[test]
    pub fn add_with_open_spot() {
        let mut state = init_test_game_state();
        add_job(&mut state, "TestGather").unwrap();
        assert_eq!(1, state.jobs["TestGather"]);
    }

    #[test]
    pub fn add_with_no_pops() {
        let mut state = init_test_game_state();
        add_job(&mut state, "TestChop").unwrap();
        let error = add_job(&mut state, "TestChop").unwrap_err();
        assert_eq!("No free population to assign to TestChop job", error.to_string());
    }

    #[test]
    pub fn add_with_non_existent_job() {
        let mut state = init_test_game_state();
        let error = add_job(&mut state, "NotAJob").unwrap_err();
        assert_eq!("NotAJob is not a valid current job", error.to_string());
    }

    #[test]
    pub fn add_with_no_full_spot() {
        let mut state = init_test_game_state();
        state.pops = 2;
        add_job(&mut state, "TestGather").unwrap();
        let error = add_job(&mut state, "TestGather").unwrap_err();
        assert_eq!("TestGather does not have another available slot", error.to_string());
    }

    #[test]
    pub fn add_with_conversion_in_flight_resets() {
        let mut state = init_test_game_state();
        state.pops = 2;

        add_job(&mut state, "TestChop").unwrap();
        process_tick(&mut state);
        let tick_before_assign = state.action_with_name("TestChop").unwrap().current_tick;
        add_job(&mut state, "TestChop").unwrap();
        assert_ne!(tick_before_assign, state.action_with_name("TestChop").unwrap().current_tick);
    }

    #[test]
    pub fn remove_with_active_job() {
        let mut state = init_test_game_state();
        add_job(&mut state, "TestGather").unwrap();
        assert_is_some(state.action_with_name("TestGather"));

        remove_job(&mut state, "TestGather").unwrap();
        assert_eq!(0, state.jobs["TestGather"]);
        assert_is_none(state.action_with_name("TestGather"));
    }

    #[test]
    pub fn remove_with_non_existent_job() {
        let mut state = init_test_game_state();
        let error = remove_job(&mut state, "NotAJob").unwrap_err();
        assert_eq!("NotAJob is not a valid current job", error.to_string());
    }

    #[test]
    pub fn remove_with_no_active_job() {
        let mut state = init_test_game_state();
        let error = remove_job(&mut state, "TestGather").unwrap_err();
        assert_eq!("TestGather does not have an active slot", error.to_string());
    }

    #[test]
    pub fn remove_with_conversion_in_flight_resets() {
        let mut state = init_test_game_state();
        add_job(&mut state, "TestGather").unwrap();
        process_tick(&mut state);
        assert_is_some(state.action_with_name("TestGather"));
        remove_job(&mut state, "TestGather").unwrap();
        assert_is_none(state.action_with_name("TestGather"));
    }

    #[test]
    pub fn reduce_jobs_no_active_jobs_lost() {
        let mut state = init_test_game_state();
        add_job(&mut state, "TestChop").unwrap();
        process_tick(&mut state);
        assert_eq!(0, state.job_count("TestGather"));

        reduce_active_jobs_by_loss(&mut state, &get_test_building("Test Building"));
        assert_eq!(0, state.job_count("TestGather"));
    }

    #[test]
    pub fn reduce_jobs_one_active_job_lost() {
        let mut state = init_test_game_state();
        state.pops = 4;

        add_job(&mut state, "TestGather").unwrap();
        for _ in 0..3 {
            add_job(&mut state, "TestChop").unwrap();
        }
        process_tick(&mut state);
        assert_eq!(3, state.job_count("TestChop"));

        reduce_active_jobs_by_loss(&mut state, &get_test_building("Test Building"));
        assert_eq!(2, state.job_count("TestChop"));
        assert_eq!(1, state.job_count("TestGather"));
    }

    #[test]
    pub fn reduce_jobs_many_active_jobs_lost() {
        let mut state = init_test_game_state();
        state.pops = 4;

        for _ in 0..3 {
            add_job(&mut state, "TestChop").unwrap();
        }
        process_tick(&mut state);
        assert_eq!(3, state.job_count("TestChop"));

        reduce_active_jobs_by_loss(&mut state, &get_test_building("Test Building"));
        assert_eq!(2, state.job_count("TestChop"));
        destroy(&mut state, 0, 0).unwrap();
        for _ in 0..DESTROY_LENGTH {
            process::process_tick(&mut state);
        }

        reduce_active_jobs_by_loss(&mut state, &get_test_building("Test Building"));
        assert_eq!(0, state.job_count("TestChop"));
    }
}
