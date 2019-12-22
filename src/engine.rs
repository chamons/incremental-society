use crate::buildings::*;
use crate::engine_error::*;
use crate::regions::*;
use crate::state::*;

pub fn build<'a>(state: &mut GameState<'a>, building: &Building<'a>, region_index: usize) -> Result<(), EngineError> {
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

    region.add_building(building.clone());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::*;

    #[test]
    fn build_valid_building() {
        let mut state = GameState::init();
        state.regions = vec![Region::init("First Region", vec![])];
        state.resources[ResourceKind::Fuel] = 20;

        let building = Building::init("Building", vec![], vec![ResourceAmount::init(ResourceKind::Fuel, 10)]);

        build(&mut state, &building, 0).unwrap();
        assert_eq!(1, state.buildings().len());
    }

    #[test]
    fn build_without_resources() {
        let mut state = GameState::init();
        state.regions = vec![Region::init("First Region", vec![])];
        let building = Building::init("Building", vec![], vec![ResourceAmount::init(ResourceKind::Fuel, 10)]);

        assert!(build(&mut state, &building, 0).is_err());
    }
}
