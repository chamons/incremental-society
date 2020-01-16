use super::process;
use super::EngineError;
use crate::state::{DelayedAction, GameState, Waiter};

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

    if building.pops > 0 && state.derived_state.used_pops > state.derived_state.pops - building.pops {
        return Err(EngineError::init("Insufficient pops for remaining buildings after destruction"));
    }

    if building.immortal {
        return Err(EngineError::init(format!("Unable to destroy {}", building.name)));
    }

    if state
        .actions
        .iter()
        .any(|x| if let DelayedAction::Destroy(_, _) = x.action { true } else { false })
    {
        return Err(EngineError::init("Unable to destroy due to another destruction taking place already."));
    }

    Ok(())
}

const DESTROY_LENGTH: u32 = 50;

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

pub fn apply_destroy(state: &mut GameState, region_index: usize, building_index: usize) {
    let region = state.regions.get_mut(region_index).unwrap();
    region.remove_building(building_index);
    process::recalculate(state);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    use crate::data::get_building;
    use crate::state::{Region, ResourceKind};

    #[test]
    fn destroy_invalid_region() {
        let mut state = process::init_test_game_state();
        assert!(destroy(&mut state, 2, 0).is_err());
    }

    #[test]
    fn destroy_invalid_building() {
        let mut state = process::init_test_game_state();
        assert!(destroy(&mut state, 0, 2).is_err());
    }

    #[test]
    fn destroy_drops_pops_too_low_fails() {
        let mut state = process::init_test_game_state();

        assert_eq!(
            "Insufficient pops for remaining buildings after destruction",
            destroy(&mut state, 0, 0).unwrap_err().description()
        );
    }

    #[test]
    fn destroy_immortal_building() {
        let mut state = process::init_test_game_state();
        state.regions[1].add_building(get_building("Test Immortal"));
        assert_eq!("Unable to destroy Test Immortal", destroy(&mut state, 1, 1).unwrap_err().description());
    }

    #[test]
    fn destroy_building_already_being_destroyed() {
        let mut state = process::init_empty_game_state();
        state.regions.push(Region::init_with_buildings(
            "Region",
            vec![get_building("Empty Building"), get_building("Empty Building")],
        ));
        assert!(destroy(&mut state, 0, 0).is_ok());
        assert!(destroy(&mut state, 0, 1).is_err());
    }

    #[test]
    fn destroy_valid_building() {
        let mut state = process::init_test_game_state();
        let old_storage = state.derived_state.storage[ResourceKind::Food];
        assert!(destroy(&mut state, 1, 0).is_ok());

        for _ in 0..DESTROY_LENGTH {
            assert_eq!(3, state.buildings().len());
            process::process_tick(&mut state);
        }

        assert_eq!(2, state.buildings().len());
        assert_ne!(old_storage, state.derived_state.storage[ResourceKind::Food]);
    }
}
