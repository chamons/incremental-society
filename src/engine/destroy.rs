use super::{jobs, process, EngineError};
use crate::state::{DelayedAction, GameState, Waiter, DESTROY_LENGTH};

pub fn can_destroy_building(state: &GameState, region_index: usize, building_index: usize) -> Result<(), EngineError> {
    let region = state.regions.get(region_index);
    if region.is_none() {
        return Err(EngineError::init(format!("Could not find index {}", region_index)));
    }
    let region = region.unwrap();

    let building = region.buildings.get(building_index);
    if building.is_none() {
        return Err(EngineError::init(format!("Could not find building at {}", building_index)));
    }
    let building = building.unwrap();

    if building.immortal {
        return Err(EngineError::init(format!("Unable to destroy {}", building.name)));
    }

    if state.actions.iter().any(|x| x.action.is_destroy()) {
        return Err(EngineError::init("Unable to destroy due to another destruction taking place already."));
    }

    Ok(())
}

pub fn destroy(state: &mut GameState, region_index: usize, building_index: usize) -> Result<(), EngineError> {
    can_destroy_building(state, region_index, building_index)?;

    let region = state.regions.get(region_index).unwrap();
    let building = region.buildings.get(building_index).unwrap();

    let action = Waiter::init_one_shot(
        &format!("Destroy {}", building.name)[..],
        DESTROY_LENGTH,
        DelayedAction::Destroy(region_index, building_index),
    );
    state.actions.push(action);
    process::recalculate(state);

    Ok(())
}

fn get_building_name(state: &GameState, region_index: usize, building_index: usize) -> String {
    let region = state.regions.get(region_index).unwrap();
    region.buildings[building_index].name.to_string()
}

fn apply_job_loss(state: &mut GameState, region_index: usize, building_index: usize) {
    let building_name = get_building_name(state, region_index, building_index);
    let building = state.derived_state.find_building(&building_name).clone();

    jobs::reduce_active_jobs_by_loss(state, &building);
}

pub fn apply_destroy(state: &mut GameState, region_index: usize, building_index: usize) {
    apply_job_loss(state, region_index, building_index);

    let region = state.regions.get_mut(region_index).unwrap();
    region.remove_building(building_index);
    process::recalculate(state);
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::engine::add_job;
    use crate::engine::tests::*;
    use crate::state::{Region, ResourceKind, DESTROY_LENGTH};

    #[test]
    fn destroy_invalid_region() {
        let mut state = init_test_game_state();
        assert!(destroy(&mut state, 2, 0).is_err());
    }

    #[test]
    fn destroy_invalid_building() {
        let mut state = init_test_game_state();
        assert!(destroy(&mut state, 0, 2).is_err());
    }

    #[test]
    fn destroy_immortal_building() {
        let mut state = init_test_game_state();
        state.regions[1].add_building(get_test_building("Test Immortal"));
        assert_eq!("Unable to destroy Test Immortal", destroy(&mut state, 1, 1).unwrap_err().to_string());
    }

    #[test]
    fn destroy_building_already_being_destroyed() {
        let mut state = init_empty_game_state();
        state.regions.push(Region::init_with_buildings(
            "Region",
            vec![get_test_building("Empty Building"), get_test_building("Empty Building")],
        ));
        assert!(destroy(&mut state, 0, 0).is_ok());
        assert!(destroy(&mut state, 0, 1).is_err());
    }

    fn test_destroy_building(state: &mut GameState, region: usize, index: usize) {
        assert!(destroy(state, region, index).is_ok());
        for _ in 0..DESTROY_LENGTH {
            process::process_tick(state);
        }
    }

    #[test]
    fn destroy_valid_building() {
        let mut state = init_test_game_state();
        let old_storage = state.derived_state.storage[ResourceKind::Food];
        assert_eq!(3, state.buildings().len());

        test_destroy_building(&mut state, 1, 0);

        assert_eq!(2, state.buildings().len());
        assert_ne!(old_storage, state.derived_state.storage[ResourceKind::Food]);
    }

    #[test]
    fn destroy_building_with_jobs_unassigns() {
        let mut state = init_test_game_state();
        for _ in 0..4 {
            add_job(&mut state, "TestChop").unwrap();
        }
        let starting_tick = state.action_with_name("TestChop").unwrap().current_tick;
        // So we are one tick into chopping
        super::super::process_tick(&mut state);

        test_destroy_building(&mut state, 0, 0);
        assert_eq!(2, state.jobs["TestChop"]);
        assert_eq!(0, starting_tick - state.action_with_name("TestChop").unwrap().current_tick);
    }
}
