use super::{process, EngineError};
use crate::state::{Building, DelayedAction, GameState, Waiter, BUILD_LENGTH};

pub fn can_build_in_region(state: &GameState, region_index: usize) -> Result<(), EngineError> {
    let region = state.regions.get(region_index);
    if region.is_none() {
        return Err(EngineError::init(format!("Could not find index {}", region_index)));
    }
    let region = region.unwrap();

    if region.buildings.len() >= region.max_building_count() {
        return Err(EngineError::init("Insufficient room for building"));
    }

    if state.actions.iter().any(|x| x.action.is_build()) {
        return Err(EngineError::init("Unable to build due to another building already in progress."));
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

    state.resources.remove_range(&building.build_cost);

    let action = Waiter::init_one_shot(
        &format!("Build {}", building.name)[..],
        BUILD_LENGTH,
        DelayedAction::Build(building.name.to_string(), region_index),
    );
    state.actions.push(action);
    process::recalculate(state);

    Ok(())
}

pub fn apply_build(state: &mut GameState, building: &str, region_index: usize) {
    let region = state.regions.get_mut(region_index).unwrap();
    let building = state.derived_state.find_building(building);
    region.add_building(building.clone());
    process::recalculate(state);
}

#[cfg(test)]
mod tests {
    use super::{super::process, *};

    use std::error::Error;

    use crate::engine::tests::*;
    use crate::state::{Region, ResourceKind, BUILD_LENGTH};

    #[test]
    fn build_invalid_region() {
        let mut state = process::init_empty_game_state();
        state.regions = vec![];

        assert!(build(&mut state, get_building("Test Building"), 0).is_err());
    }

    #[test]
    fn build_without_resources() {
        let mut state = process::init_empty_game_state();
        state.regions = vec![Region::init("First Region")];

        let error = build(&mut state, get_building("Test Building"), 0).unwrap_err();
        assert_eq!("Insufficient resources for build cost", error.description());
    }

    #[test]
    fn build_without_room() {
        let building = get_building("Test Building");

        let mut state = process::init_empty_game_state();
        state.resources[ResourceKind::Fuel] = 20;
        state.regions = vec![Region::init_with_buildings("First Region", vec![building.clone(), building.clone()])];

        let error = build(&mut state, building, 0).unwrap_err();
        assert_eq!("Insufficient room for building", error.description());
    }

    #[test]
    fn build_without_pops() {
        let mut state = process::init_empty_game_state();
        state.regions = vec![Region::init("Test Region")];
        process::recalculate(&mut state);

        let error = build(&mut state, get_building("Test Gather Hut"), 0).unwrap_err();
        assert_eq!("Insufficient pops for building", error.description());
    }

    #[test]
    fn build_immortal_building() {
        let mut state = process::init_test_game_state();
        assert_eq!(
            "Unable to build Test Immortal",
            build(&mut state, get_building("Test Immortal"), 1).unwrap_err().description()
        );
    }

    #[test]
    fn build_multiple_buildings_at_once() {
        let mut state = process::init_empty_game_state();
        state.regions.push(Region::init_with_buildings("Region", vec![get_building("Test Building")]));
        process::recalculate(&mut state);

        build(&mut state, get_building("Test Gather Hut"), 0).unwrap();
        assert!(build(&mut state, get_building("Test Gather Hut"), 0).is_err());
    }

    #[test]
    fn build_valid_building() {
        let mut state = process::init_empty_game_state();
        state.regions = vec![Region::init_with_buildings("First Region", vec![get_building("Test Building")])];
        state.resources[ResourceKind::Fuel] = 20;
        process::recalculate(&mut state);

        let old_storage = state.derived_state.storage[ResourceKind::Fuel];

        build(&mut state, get_building("Test Building"), 0).unwrap();
        assert_eq!(10, state.resources[ResourceKind::Fuel]);

        for _ in 0..BUILD_LENGTH {
            assert_eq!(1, state.buildings().len());
            process::process_tick(&mut state);
        }

        // Chops from Test Building
        assert!(10 < state.resources[ResourceKind::Fuel]);
        assert_eq!(2, state.buildings().len());
        assert_ne!(old_storage, state.derived_state.storage[ResourceKind::Fuel]);
    }
}
