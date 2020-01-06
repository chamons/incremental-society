use crate::building::Building;
use crate::data::get_conversion;
use crate::derived_state::DerivedState;
use crate::engine_error::EngineError;
use crate::region::Region;
use crate::resources::NUM_RESOURCES;
use crate::state::GameState;

use std::cmp;

pub fn build(state: &mut GameState, building: Building, region_index: usize) -> Result<(), EngineError> {
    let region = state.regions.get_mut(region_index);
    if region.is_none() {
        return Err(EngineError::init(format!("Could not find index {}", region_index)));
    }
    let region: &mut Region = region.unwrap();

    for cost in &building.build_cost {
        if !state.resources.has_amount(&cost) {
            return Err(EngineError::init("Insufficient resources for build cost".to_string()));
        }
    }

    if region.buildings.len() >= region.max_building_count() {
        return Err(EngineError::init("Insufficient room for building".to_string()));
    }

    region.add_building(building);
    state.derived_state = crate::derived_state::DerivedState::calculate(&state);
    Ok(())
}

pub fn destroy(state: &mut GameState, region_index: usize, building_index: usize) -> Result<(), EngineError> {
    let region = state.regions.get_mut(region_index);
    if region.is_none() {
        return Err(EngineError::init(format!("Could not find index {}", region_index)));
    }
    let region: &mut Region = region.unwrap();

    let building = region.buildings.get_mut(building_index);
    if building.is_none() {
        return Err(EngineError::init(format!("Could not find building at {}", building_index)));
    }

    region.remove_building(building_index);
    state.derived_state = DerivedState::calculate(&state);
    Ok(())
}

pub const CONVERSION_TICK_START: u32 = 100;

pub fn process_tick(mut state: &mut GameState) {
    process_conversion(&mut state);
    honor_storage_limits(&mut state);
}

fn process_conversion(state: &mut GameState) {
    for c in &state.derived_state.conversion_counts {
        let entry = state.ticks.entry(c.name.to_string()).or_insert(CONVERSION_TICK_START);
        if *entry == 0 {
            *entry = CONVERSION_TICK_START;
            let conversion = get_conversion(&c.name);
            for _ in 0..c.count {
                conversion.convert(&mut state.resources);
            }
        } else {
            *entry -= 1;
        }
    }
}

fn honor_storage_limits(state: &mut GameState) {
    for i in 0..NUM_RESOURCES {
        state.resources[i] = cmp::min(state.resources[i], state.derived_state.storage[i]);
    }
}

pub fn get_conversion_current_tick(state: &GameState, conversion_name: &str) -> Option<u32> {
    match state.ticks.get(conversion_name) {
        Some(x) => Some(CONVERSION_TICK_START - *x),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::get_building;
    use crate::resources::*;
    use std::error::Error;

    #[test]
    fn simple_conversion_tick() {
        let mut state = GameState::init_test_game_state();
        process_tick(&mut state);

        assert_eq!(1, get_conversion_current_tick(&state, "TestChop").unwrap());
        assert_eq!(1, get_conversion_current_tick(&state, "TestGather").unwrap());
    }

    #[test]
    fn get_conversion_tick_with_no_ticks() {
        let state = GameState::init_test_game_state();
        assert!(get_conversion_current_tick(&state, "TestChop").is_none());
        assert!(get_conversion_current_tick(&state, "TestGather").is_none());
    }

    #[test]
    fn get_non_existent_conversion_tick() {
        let mut state = GameState::init_test_game_state();
        process_tick(&mut state);

        assert!(get_conversion_current_tick(&state, "NonExistentConvert").is_none());
    }

    #[test]
    fn process_tick_none_ready() {
        let mut state = GameState::init_test_game_state();
        process_tick(&mut state);
        assert_eq!(0, state.resources[ResourceKind::Food]);
        assert_eq!(0, state.resources[ResourceKind::Fuel]);
    }

    #[test]
    fn process_tick_none_one_ready() {
        let mut state = GameState::init_test_game_state();
        *state.ticks.entry("TestChop".to_string()).or_default() = 0;
        process_tick(&mut state);

        assert_eq!(0, state.resources[ResourceKind::Food]);
        assert_eq!(4, state.resources[ResourceKind::Fuel]);
    }

    #[test]
    fn process_tick_none_many_ready() {
        let mut state = GameState::init_test_game_state();
        *state.ticks.entry("TestChop".to_string()).or_default() = 0;
        *state.ticks.entry("TestGather".to_string()).or_default() = 0;
        process_tick(&mut state);

        assert_eq!(1, state.resources[ResourceKind::Food]);
        assert_eq!(4, state.resources[ResourceKind::Fuel]);
    }

    #[test]
    fn process_tick_storage_limits_honored() {
        let mut state = GameState::init_test_game_state();
        state.resources[ResourceKind::Food] = state.derived_state.storage[ResourceKind::Food] - 1;
        state.resources[ResourceKind::Fuel] = state.derived_state.storage[ResourceKind::Fuel] - 1;
        *state.ticks.entry("TestChop".to_string()).or_default() = 0;
        *state.ticks.entry("TestGather".to_string()).or_default() = 0;

        process_tick(&mut state);
        assert_eq!(state.resources[ResourceKind::Food], state.derived_state.storage[ResourceKind::Food]);
        assert_eq!(state.resources[ResourceKind::Fuel], state.derived_state.storage[ResourceKind::Fuel]);
    }

    #[test]
    fn build_invalid_region() {
        let mut state = GameState::init();
        state.regions = vec![];

        assert!(build(&mut state, get_building("Test Building"), 0).is_err());
    }

    #[test]
    fn build_valid_building() {
        let mut state = GameState::init();
        state.regions = vec![Region::init("First Region")];
        state.resources[ResourceKind::Fuel] = 20;
        let old_storage = state.derived_state.storage[ResourceKind::Fuel];

        build(&mut state, get_building("Test Building"), 0).unwrap();

        assert_eq!(1, state.buildings().len());
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
    fn destory_invalid_region() {
        let mut state = GameState::init_test_game_state();
        assert!(destroy(&mut state, 2, 0).is_err());
    }

    #[test]
    fn destory_invalid_building() {
        let mut state = GameState::init_test_game_state();
        assert!(destroy(&mut state, 0, 2).is_err());
    }

    #[test]
    fn destory_valid_building() {
        let mut state = GameState::init_test_game_state();
        let old_storage = state.derived_state.storage[ResourceKind::Fuel];
        assert!(destroy(&mut state, 0, 0).is_ok());

        assert_eq!(2, state.buildings().len());
        assert_ne!(old_storage, state.derived_state.storage[ResourceKind::Fuel]);
    }
}
