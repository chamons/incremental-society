use std::collections::HashMap;

use super::{EngineError, GameContext};
use crate::state::UPGRADE_LENGTH;
use crate::state::{Building, DelayedAction, Upgrade, Waiter};

pub fn can_apply_upgrades(context: &GameContext, upgrade: &Upgrade) -> Result<(), EngineError> {
    if context.state.actions.iter().any(|x| x.action.is_upgrade()) {
        return Err(EngineError::init("Upgrade already in progress."));
    }

    for cost in &upgrade.cost {
        if !context.state.resources.has_amount(&cost) {
            return Err(EngineError::init("Insufficient resources for upgrade."));
        }
    }

    Ok(())
}

pub fn upgrade(context: &mut GameContext, upgrade: &Upgrade) -> Result<(), EngineError> {
    can_apply_upgrades(context, upgrade)?;

    context.state.resources.remove_range(&upgrade.cost);

    let action = Waiter::init_one_shot(
        &format!("Researching {}", upgrade.name)[..],
        UPGRADE_LENGTH,
        DelayedAction::Upgrade(upgrade.name.to_string()),
    );
    context.state.actions.push(action);
    context.recalculate();

    Ok(())
}

pub fn apply_upgrade(context: &mut GameContext, upgrades: &str) {
    context.state.upgrades.insert(upgrades.to_owned());

    // We must first recalculate to take into account the new upgraded buildings
    context.recalculate();

    let mut new_buildings: HashMap<String, Building> = HashMap::new();
    for r in &context.state.regions {
        for b in &r.buildings {
            if !new_buildings.contains_key(&b.name) {
                new_buildings.insert(b.name.to_string(), context.find_building(&b.name));
            }
        }
    }

    for r in &mut context.state.regions {
        for i in 0..r.buildings.len() {
            r.buildings[i] = new_buildings[&r.buildings[i].name[..]].clone();
        }
    }

    // We must recalculate again to take into account those upgraded buildings
    context.recalculate();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::tests::*;
    use crate::engine::{add_job, process};
    use crate::state::{ConversionLength, GameState, Region, ResourceKind};

    fn give_test_update_resources(state: &mut GameState, count: i64) {
        state.resources.add(ResourceKind::Knowledge, 25 * count);
    }

    #[test]
    fn upgrade_multiple_in_flight() {
        let mut context = GameContext::init_empty_test_game_context();
        give_test_update_resources(&mut context.state, 2);

        upgrade(&mut context, &get_test_upgrade("TestUpgrade")).unwrap();
        assert!(upgrade(&mut context, &get_test_upgrade("TestUpgrade")).is_err());
    }

    #[test]
    fn upgrade_costs_resources() {
        let mut context = GameContext::init_empty_test_game_context();
        assert!(upgrade(&mut context, &get_test_upgrade("TestUpgrade")).is_err());

        give_test_update_resources(&mut context.state, 1);

        assert!(upgrade(&mut context, &get_test_upgrade("TestUpgrade")).is_ok());
        assert_eq!(0, context.state.resources[ResourceKind::Knowledge]);
    }

    #[test]
    fn has_building_upgrade_applied() {
        let mut context = GameContext::init_empty_test_game_context();
        context.state.regions = vec![Region::init_with_buildings("First Region", vec![get_test_building("Test Building").clone()])];
        context.recalculate();

        give_test_update_resources(&mut context.state, 1);

        upgrade(&mut context, &get_test_upgrade("TestUpgrade")).unwrap();

        for _ in 0..UPGRADE_LENGTH {
            assert_eq!(0, context.state.upgrades.len());
            assert_eq!(2, context.state.buildings()[0].jobs.len());
            process::process_tick(&mut context);
        }

        assert_eq!(1, context.state.upgrades.len());
        assert_eq!(0, context.state.resources[ResourceKind::Knowledge]);
        assert_eq!(3, context.state.buildings()[0].jobs.len());
    }

    fn upgrade_edict(context: &mut GameContext, name: &str) {
        give_test_update_resources(&mut context.state, 1);
        upgrade(context, &get_test_upgrade(name)).unwrap();

        for _ in 0..UPGRADE_LENGTH {
            assert_eq!(0, context.state.upgrades.len());
            assert_eq!(ConversionLength::Short, context.find_edict("TestEdict").conversion.length);
            process::process_tick(context);
        }

        assert_eq!(1, context.state.upgrades.len());
        assert_eq!(0, context.state.resources[ResourceKind::Knowledge]);
    }

    #[test]
    fn edict_length_applied() {
        let mut context = GameContext::init_empty_test_game_context();

        upgrade_edict(&mut context, "TestEdictUpgrade");

        assert_eq!(ConversionLength::Long, context.find_edict("TestEdict").conversion.length);
    }

    #[test]
    fn edict_bonus_applied() {
        let mut context = GameContext::init_empty_test_game_context();

        upgrade_edict(&mut context, "TestEdictUpgradeYield");

        assert_eq!(1.0, context.find_edict("TestEdict").effective_bonus);
    }

    #[test]
    fn has_conversion_applied() {
        let mut context = GameContext::init_empty_test_game_context();
        context.state.regions = vec![Region::init_with_buildings(
            "First Region",
            vec![get_test_building("Test Building").clone(), get_test_building("Stability Building").clone()],
        )];
        context.state.resources[ResourceKind::Food] = 300;
        context.state.pops = 2;
        context.recalculate();

        add_job(&mut context, "TestChop").unwrap();
        add_job(&mut context, "TestChop").unwrap();

        give_test_update_resources(&mut context.state, 1);

        upgrade(&mut context, &get_test_upgrade("TestConversionUpgrade")).unwrap();

        for _ in 0..UPGRADE_LENGTH {
            assert_eq!(0, context.state.upgrades.len());
            assert_eq!(1, context.find_conversion("TestChop").output.len());
            process::process_tick(&mut context);
        }

        assert_eq!(1, context.state.upgrades.len());
        assert_eq!(2, context.find_conversion("TestChop").output.len());
        assert_eq!(0, context.state.resources[ResourceKind::Knowledge]);

        let conversion_length = context.find_conversion("TestChop").tick_length();
        for _ in 0..conversion_length {
            process::process_tick(&mut context);
        }
        assert_eq!(2, context.state.resources[ResourceKind::Knowledge]);
    }

    #[test]
    fn upgrade_affects_multiple_items_differently() {
        let mut context = GameContext::init_empty_test_game_context();
        context.state.regions = vec![Region::init_with_buildings("First Region", vec![get_test_building("Test Building").clone()])];
        context.state.upgrades.insert("TestMultiUpgrade".to_string());
        context.recalculate();

        assert_eq!(2, context.state.buildings()[0].jobs.len());
        assert_eq!(2, context.find_conversion("TestChop").output.len());
        assert_eq!(ConversionLength::Long, context.find_edict("TestEdict").conversion.length);
    }

    #[test]
    fn current_stability_gain() {
        let mut context = GameContext::init_empty_test_game_context();
        context.state.upgrades.insert("StabilityUpgrade".to_owned());
        context.recalculate();

        assert_eq!(2, context.get_stability_gain());

        context.state.upgrades.insert("OtherStabilityUpgrade".to_owned());
        context.recalculate();

        assert_eq!(3, context.get_stability_gain());
    }
}
