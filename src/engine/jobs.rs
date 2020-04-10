use super::{conversions::reset_conversion_status, EngineError};
use crate::state::GameState;

pub fn add_job(state: &mut GameState, name: &str) -> Result<(), EngineError> {
    match state.derived_state.current_building_jobs.get(name) {
        Some(available_count) => {
            let current_count = state.jobs.entry(name.to_string()).or_insert(0);
            if *current_count < *available_count {
                *current_count += 1;
                if let Some(_) = state.action_with_name(name) {
                    reset_conversion_status(state, name);
                }
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
                    if let Some(_) = state.action_with_name(name) {
                        reset_conversion_status(state, name);
                    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::tests::*;

    #[test]
    pub fn add_with_open_spot() {
        let mut state = init_test_game_state();
        add_job(&mut state, "TestGather").unwrap();
        assert_eq!(1, state.jobs["TestGather"]);
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
        state.jobs.insert("TestGather".to_string(), 1);
        let error = add_job(&mut state, "TestGather").unwrap_err();
        assert_eq!("TestGather does not have another available slot", error.to_string());
    }

    #[test]
    pub fn add_with_conversion_in_flight_resets() {
        let mut state = init_test_game_state();
        super::super::process_tick(&mut state);
        let tick_before_assign = state.action_with_name("TestGather").unwrap().current_tick;
        add_job(&mut state, "TestGather").unwrap();
        assert_ne!(tick_before_assign, state.action_with_name("TestGather").unwrap().current_tick);
    }

    #[test]
    pub fn remove_with_active_job() {
        let mut state = init_test_game_state();
        add_job(&mut state, "TestGather").unwrap();
        remove_job(&mut state, "TestGather").unwrap();
        assert_eq!(0, state.jobs["TestGather"]);
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
        super::super::process_tick(&mut state);
        let tick_before_remove = state.action_with_name("TestGather").unwrap().current_tick;
        remove_job(&mut state, "TestGather").unwrap();
        assert_ne!(tick_before_remove, state.action_with_name("TestGather").unwrap().current_tick);
    }
}
