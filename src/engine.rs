use crate::building::Building;
use crate::data::get_conversion;
use crate::engine_error::EngineError;
use crate::region::Region;
use crate::state::GameState;

pub fn build<'a>(state: &mut GameState<'a>, building: Building<'a>, region_index: usize) -> Result<(), EngineError> {
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

    Ok(())
}

pub const CONVERSION_TICK_START: u32 = 100;

pub fn process_tick<'a>(state: &mut GameState<'a>) {
    for (conversion, count) in state.conversion_with_counts() {
        let entry = state.ticks.entry(&conversion).or_insert(CONVERSION_TICK_START);
        if *entry == 0 {
            *entry = CONVERSION_TICK_START;
            let conversion = get_conversion(&conversion);
            for _ in 0..count {
                conversion.convert(&mut state.resources);
            }
        } else {
            *entry -= 1;
        }
    }
}

pub fn get_conversion_tick<'a>(state: &GameState<'a>, conversion_name: &'a str) -> Option<u32> {
    match state.ticks.get(conversion_name) {
        Some(x) => Some(CONVERSION_TICK_START - *x),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::*;
    use std::error::Error;

    #[test]
    fn build_invalid_region() {
        let mut state = GameState::init();
        state.regions = vec![];
        let building = Building::init("Building", vec![], vec![ResourceAmount::init(ResourceKind::Fuel, 10)]);

        assert!(build(&mut state, building, 0).is_err());
    }

    #[test]
    fn build_valid_building() {
        let mut state = GameState::init();
        state.regions = vec![Region::init("First Region")];
        state.resources[ResourceKind::Fuel] = 20;

        let building = Building::init("Building", vec![], vec![ResourceAmount::init(ResourceKind::Fuel, 10)]);

        build(&mut state, building, 0).unwrap();
        assert_eq!(1, state.buildings().len());
    }

    #[test]
    fn build_without_resources() {
        let mut state = GameState::init();
        state.regions = vec![Region::init("First Region")];
        let building = Building::init("Building", vec![], vec![ResourceAmount::init(ResourceKind::Fuel, 10)]);

        let error = build(&mut state, building, 0).unwrap_err();
        assert_eq!("Insufficient resources for build cost", error.description());
    }

    #[test]
    fn build_without_room() {
        let building = Building::init("Building", vec![], vec![ResourceAmount::init(ResourceKind::Fuel, 10)]);

        let mut state = GameState::init();
        state.resources[ResourceKind::Fuel] = 20;
        state.regions = vec![Region::init_with_buildings("First Region", vec![building.clone(), building.clone()])];

        let error = build(&mut state, building, 0).unwrap_err();
        assert_eq!("Insufficient room for building", error.description());
    }
}
