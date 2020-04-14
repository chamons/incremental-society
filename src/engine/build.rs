use super::{EngineError, GameContext};
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

    if building.immortal {
        return Err(EngineError::init(format!("Unable to build {}", building.name)));
    }

    Ok(())
}

pub fn build(context: &mut GameContext, building: Building, region_index: usize) -> Result<(), EngineError> {
    can_build_in_region(&context.state, region_index)?;
    can_build_building(&context.state, &building)?;

    context.state.resources.remove_range(&building.build_cost);

    let action = Waiter::init_one_shot(
        &format!("Build {}", building.name)[..],
        BUILD_LENGTH,
        DelayedAction::Build(building.name, region_index),
    );
    context.state.actions.push(action);
    context.recalculate();

    Ok(())
}

pub fn apply_build(context: &mut GameContext, building: &str, region_index: usize) {
    let building = context.find_building(building);
    let region = context.state.regions.get_mut(region_index).unwrap();
    region.add_building(building);
    context.recalculate();
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::process;
    use crate::data::tests::*;
    use crate::state::{Region, ResourceKind, BUILD_LENGTH};

    #[test]
    fn build_invalid_region() {
        let mut context = GameContext::init_empty_test_game_context();
        context.state.regions = vec![];

        assert!(build(&mut context, get_test_building("Test Building"), 0).is_err());
    }

    #[test]
    fn build_without_resources() {
        let mut context = GameContext::init_empty_test_game_context();
        context.state.regions = vec![Region::init("First Region")];

        let error = build(&mut context, get_test_building("Test Building"), 0).unwrap_err();
        assert_eq!("Insufficient resources for build cost", error.to_string());
    }

    #[test]
    fn build_without_room() {
        let building = get_test_building("Test Building");

        let mut context = GameContext::init_empty_test_game_context();
        context.state.resources[ResourceKind::Fuel] = 20;
        context.state.regions = vec![Region::init_with_buildings(
            "First Region",
            vec![building.clone(), building.clone(), building.clone(), building.clone()],
        )];

        let error = build(&mut context, building, 0).unwrap_err();
        assert_eq!("Insufficient room for building", error.to_string());
    }

    #[test]
    fn build_immortal_building() {
        let mut context = GameContext::init_test_game_context();
        assert_eq!(
            "Unable to build Test Immortal",
            build(&mut context, get_test_building("Test Immortal"), 1).unwrap_err().to_string()
        );
    }

    #[test]
    fn build_multiple_buildings_at_once() {
        let mut context = GameContext::init_empty_test_game_context();
        context
            .state
            .regions
            .push(Region::init_with_buildings("Region", vec![get_test_building("Test Building")]));
        context.recalculate();

        build(&mut context, get_test_building("Test Gather Hut"), 0).unwrap();
        assert!(build(&mut context, get_test_building("Test Gather Hut"), 0).is_err());
    }

    #[test]
    fn build_valid_building() {
        let mut context = GameContext::init_empty_test_game_context();
        context.state.regions = vec![Region::init_with_buildings("First Region", vec![get_test_building("Test Building")])];
        context.state.resources[ResourceKind::Fuel] = 20;
        context.recalculate();

        let old_storage = context.storage[ResourceKind::Fuel];

        build(&mut context, get_test_building("Test Building"), 0).unwrap();
        assert_eq!(10, context.state.resources[ResourceKind::Fuel]);

        for _ in 0..BUILD_LENGTH {
            assert_eq!(1, context.state.buildings().len());
            process::process_tick(&mut context);
        }

        assert_eq!(2, context.state.buildings().len());
        assert_ne!(old_storage, context.storage[ResourceKind::Fuel]);
    }
}
