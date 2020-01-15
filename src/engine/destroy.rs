use crate::engine::EngineError;
use crate::state::GameState;

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

    Ok(())
}

pub fn destroy(state: &mut GameState, region_index: usize, building_index: usize) -> Result<(), EngineError> {
    can_destroy_building(state, region_index, building_index)?;
    let region = state.regions.get_mut(region_index).unwrap();
    let building = region.buildings.get(building_index).unwrap();

    if building.pops > 0 && state.derived_state.used_pops > state.derived_state.pops - building.pops {
        return Err(EngineError::init("Insufficient pops for remaining buildings after destruction"));
    }

    if building.immortal {
        return Err(EngineError::init(format!("Unable to destroy {}", building.name)));
    }

    region.remove_building(building_index);
    state.recalculate();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    use crate::data::get_building;
    use crate::state::ResourceKind;

    #[test]
    fn destroy_invalid_region() {
        let mut state = GameState::init_test_game_state();
        assert!(destroy(&mut state, 2, 0).is_err());
    }

    #[test]
    fn destroy_invalid_building() {
        let mut state = GameState::init_test_game_state();
        assert!(destroy(&mut state, 0, 2).is_err());
    }

    #[test]
    fn destroy_valid_building() {
        let mut state = GameState::init_test_game_state();
        let old_storage = state.derived_state.storage[ResourceKind::Food];
        assert!(destroy(&mut state, 1, 0).is_ok());

        assert_eq!(2, state.buildings().len());
        assert_ne!(old_storage, state.derived_state.storage[ResourceKind::Food]);
    }

    #[test]
    fn destroy_drops_pops_too_low_fails() {
        let mut state = GameState::init_test_game_state();

        assert_eq!(
            "Insufficient pops for remaining buildings after destruction",
            destroy(&mut state, 0, 0).unwrap_err().description()
        );
    }

    #[test]
    fn destroy_immortal_building() {
        let mut state = GameState::init_test_game_state();
        state.regions[1].add_building(get_building("Test Immortal"));
        assert_eq!("Unable to destroy Test Immortal", destroy(&mut state, 1, 1).unwrap_err().description());
    }
}
