use crate::building::Building;
use crate::engine::EngineError;
use crate::state::GameState;

pub fn can_build_in_region(state: &GameState, region_index: usize) -> Result<(), EngineError> {
    let region = state.regions.get(region_index);
    if region.is_none() {
        return Err(EngineError::init(format!("Could not find index {}", region_index)));
    }
    let region = region.unwrap();

    if region.buildings.len() >= region.max_building_count() {
        return Err(EngineError::init("Insufficient room for building"));
    }
    Ok(())
}

pub fn can_build_building(state: &GameState, building: &Building) -> Result<(), EngineError> {
    for cost in &building.build_cost {
        if !state.resources.has_amount(&cost) {
            return Err(EngineError::init("Insufficient resources for build cost"));
        }
    }

    if state.derived_state.used_pops + 1 > state.derived_state.pops {
        return Err(EngineError::init("Insufficient pops for building"));
    }

    if building.immortal {
        return Err(EngineError::init(format!("Unable to build {}", building.name)));
    }

    Ok(())
}

pub fn build(state: &mut GameState, building: Building, region_index: usize) -> Result<(), EngineError> {
    can_build_in_region(state, region_index)?;
    can_build_building(state, &building)?;

    let region = state.regions.get_mut(region_index).unwrap();
    region.add_building(building);
    state.recalculate();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::get_building;
    use crate::region::Region;
    use crate::resources::*;
    use std::error::Error;

    #[test]
    fn build_invalid_region() {
        let mut state = GameState::init();
        state.regions = vec![];

        assert!(build(&mut state, get_building("Test Building"), 0).is_err());
    }

    #[test]
    fn build_valid_building() {
        let mut state = GameState::init();
        state.regions = vec![Region::init_with_buildings("First Region", vec![get_building("Test Building")])];
        state.resources[ResourceKind::Fuel] = 20;
        state.recalculate();

        let old_storage = state.derived_state.storage[ResourceKind::Fuel];

        build(&mut state, get_building("Test Building"), 0).unwrap();

        assert_eq!(2, state.buildings().len());
        assert_ne!(old_storage, state.derived_state.storage[ResourceKind::Fuel]);
    }

    #[test]
    fn build_without_resources() {
        let mut state = GameState::init();
        state.regions = vec![Region::init("First Region")];

        let error = build(&mut state, get_building("Test Building"), 0).unwrap_err();
        assert_eq!("Insufficient resources for build cost", error.description());
    }

    #[test]
    fn build_without_room() {
        let building = get_building("Test Building");

        let mut state = GameState::init();
        state.resources[ResourceKind::Fuel] = 20;
        state.regions = vec![Region::init_with_buildings("First Region", vec![building.clone(), building.clone()])];

        let error = build(&mut state, building, 0).unwrap_err();
        assert_eq!("Insufficient room for building", error.description());
    }

    #[test]
    fn build_without_pops() {
        let mut state = GameState::init();
        state.regions = vec![Region::init("Test Region")];
        state.recalculate();

        let error = build(&mut state, get_building("Test Gather Hut"), 0).unwrap_err();
        assert_eq!("Insufficient pops for building", error.description());
    }

    #[test]
    fn build_immortal_building() {
        let mut state = GameState::init_test_game_state();
        assert_eq!(
            "Unable to build Test Immortal",
            build(&mut state, get_building("Test Immortal"), 1).unwrap_err().description()
        );
    }
}