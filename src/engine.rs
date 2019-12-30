use crate::building::*;
use crate::engine_error::*;
use crate::region::*;
use crate::state::*;

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

        assert!(build(&mut state, &building, 0).is_err());
    }

    #[test]
    fn build_valid_building() {
        let mut state = GameState::init();
        state.regions = vec![Region::init("First Region")];
        state.resources[ResourceKind::Fuel] = 20;

        let building = Building::init("Building", vec![], vec![ResourceAmount::init(ResourceKind::Fuel, 10)]);

        build(&mut state, &building, 0).unwrap();
        assert_eq!(1, state.buildings().len());
    }

    #[test]
    fn build_without_resources() {
        let mut state = GameState::init();
        state.regions = vec![Region::init("First Region")];
        let building = Building::init("Building", vec![], vec![ResourceAmount::init(ResourceKind::Fuel, 10)]);

        let error = build(&mut state, &building, 0).unwrap_err();
        assert_eq!("Insufficient resources for build cost", error.description());
    }

    #[test]
    fn build_without_room() {
        let building = Building::init("Building", vec![], vec![ResourceAmount::init(ResourceKind::Fuel, 10)]);

        let mut state = GameState::init();
        state.resources[ResourceKind::Fuel] = 20;
        state.regions = vec![Region::init_with_buildings("First Region", vec![building.clone(), building.clone()])];

        let error = build(&mut state, &building, 0).unwrap_err();
        assert_eq!("Insufficient room for building", error.description());
    }
}
