use std::collections::{HashMap, HashSet};

use super::{EngineError, GameContext};
use crate::data::{get_building, get_conversion, get_edict, get_research, get_upgrade};
use crate::data::{get_building_names, get_conversion_names, get_edict_names, get_research_names, get_upgrade_names};
use crate::state::UPGRADE_LENGTH;
use crate::state::{Building, Conversion, DelayedAction, Edict, GameState, Research, ResourceAmount, Upgrade, UpgradeActions, Waiter};

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

pub fn get_upgrade_cost(context: &GameContext, upgrades: &[Upgrade]) -> Vec<ResourceAmount> {
    let current: HashSet<&String> = context.state.upgrades.iter().collect();
    let desired: HashSet<&String> = upgrades.iter().map(|x| &x.name).collect();

    let difference: HashSet<_> = desired.symmetric_difference(&current).collect();

    let mut cost: Vec<ResourceAmount> = vec![];
    for diff in difference.iter().map(|x| context.find_upgrade(x)) {
        for c in diff.cost.iter() {
            if let Some(i) = cost.iter().position(|x| x.kind == c.kind) {
                cost[i].amount += c.amount;
            } else {
                cost.push(c.clone());
            }
        }
    }
    cost
}

pub fn current_conversions(state: &GameState) -> Vec<Conversion> {
    let upgrades = get_upgrades_by_name(&state.upgrades);
    let mut conversions: Vec<Conversion> = get_conversion_names().iter().map(|x| get_conversion(x)).collect();

    for mut conversion in &mut conversions {
        if let Some(upgrades) = upgrades.get(&conversion.name) {
            for u in upgrades.iter().flat_map(|x| &x.upgrades) {
                apply_conversion_upgrade(&mut conversion, u);
            }
        }
    }

    conversions
}

pub fn get_current_stability_gain(state: &GameState) -> u32 {
    let upgrades: Vec<Upgrade> = state.upgrades.iter().map(|u| get_upgrade(u)).collect();

    upgrades
        .iter()
        .flat_map(|u| &u.upgrades)
        .map(|a| match a {
            UpgradeActions::ImproveStabilityGain(gain) => *gain,
            _ => 0,
        })
        .sum()
}

pub fn available_to_research(state: &GameState) -> Vec<Research> {
    get_research_by_research(&state)
}

pub fn available_to_build(state: &GameState) -> Vec<Building> {
    let upgrades = get_upgrades_by_name(&state.upgrades);
    let mut buildings: Vec<Building> = get_building_by_research(&state);

    for mut b in &mut buildings {
        if let Some(upgrades) = upgrades.get(&b.name) {
            for u in upgrades.iter().flat_map(|x| &x.upgrades) {
                apply_building_upgrade(&mut b, u);
            }
        }
    }

    buildings
}

pub fn available_to_invoke(state: &GameState) -> Vec<Edict> {
    let upgrades = get_upgrades_by_name(&state.upgrades);
    let mut edicts: Vec<Edict> = get_edict_by_research(&state);

    for mut e in &mut edicts {
        if let Some(upgrades) = upgrades.get(&e.name) {
            for u in upgrades.iter().flat_map(|x| &x.upgrades) {
                apply_edict_upgrade(&mut e, u);
            }
        }
    }

    edicts
}

pub fn available_to_upgrade(state: &GameState) -> Vec<Upgrade> {
    get_upgrade_names().iter().map(|x| get_upgrade(x)).filter(|x| x.is_available(state)).collect()
}

fn apply_building_upgrade(building: &mut Building, upgrade: &UpgradeActions) {
    match upgrade {
        UpgradeActions::AddBuildingHousing(housing) => building.housing += housing,
        UpgradeActions::AddBuildingJob(name) => building.jobs.push(name.to_string()),
        UpgradeActions::AddBuildingStorage(storage) => {
            if let Some(position) = building.storage.iter().position(|x| x.kind == storage.kind) {
                let current = &mut building.storage.get_mut(position).unwrap();
                current.amount += storage.amount;
            } else {
                building.storage.push(*storage);
            }
        }
        _ => {}
    }
}

fn apply_edict_upgrade(edict: &mut Edict, upgrade: &UpgradeActions) {
    match upgrade {
        UpgradeActions::ChangeEdictLength(length) => edict.conversion.length = *length,
        UpgradeActions::AddEdictBonus(amount) => edict.effective_bonus += amount,
        _ => {}
    }
}

fn apply_conversion_upgrade(conversion: &mut Conversion, upgrade: &UpgradeActions) {
    match upgrade {
        UpgradeActions::ChangeConversionLength(length) => conversion.length = *length,
        UpgradeActions::ChangeConversionInput(input) => conversion.input.push(*input),
        UpgradeActions::ChangeConversionOutput(output) => conversion.output.push(*output),
        _ => {}
    }
}

fn get_upgrades_by_name(upgrades: &HashSet<String>) -> HashMap<String, Vec<Upgrade>> {
    let mut sorted_list: HashMap<String, Vec<Upgrade>> = HashMap::new();
    for upgrade in upgrades {
        let upgrade = get_upgrade(upgrade);
        for item in &upgrade.items_upgraded {
            sorted_list.entry(item.to_string()).or_insert_with(|| vec![]).push(upgrade.clone());
        }
    }

    sorted_list
}

fn get_research_by_research(state: &GameState) -> Vec<Research> {
    get_research_names().iter().map(|x| get_research(x)).filter(|x| x.is_available(state)).collect()
}

fn get_building_by_research(state: &GameState) -> Vec<Building> {
    get_building_names().iter().map(|x| get_building(x)).filter(|x| x.is_available(state)).collect()
}

fn get_edict_by_research(state: &GameState) -> Vec<Edict> {
    get_edict_names().iter().map(|x| get_edict(x)).filter(|x| x.is_available(state)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::tests::*;
    use crate::engine::{add_job, process};
    use crate::state::{ConversionLength, Region, ResourceKind};

    fn give_test_update_resources(state: &mut GameState, count: i64) {
        state.resources.add(ResourceKind::Knowledge, 25 * count);
    }

    #[test]
    fn available_to_research_has_upgrades_applied() {
        let mut context = GameContext::init_empty_test_game_context();

        let available = available_to_build(&context.state);
        let before = available.iter().filter(|x| x.name == "Test Building").nth(0).unwrap();
        assert_eq!(2, before.jobs.len());

        context.state.upgrades.insert("TestUpgrade".to_owned());

        let available = available_to_build(&context.state);
        let after = available.iter().filter(|x| x.name == "Test Building").nth(0).unwrap();
        assert_eq!(3, after.jobs.len());
    }

    #[test]
    fn available_to_invoke_has_upgrades_applied() {
        let mut context = GameContext::init_empty_test_game_context();

        let available = available_to_invoke(&context.state);
        let before = available.iter().filter(|x| x.name == "TestEdict").nth(0).unwrap();
        assert_eq!(ConversionLength::Short, before.conversion.length);

        context.state.upgrades.insert("TestEdictUpgrade".to_owned());

        let available = available_to_invoke(&context.state);
        let after = available.iter().filter(|x| x.name == "TestEdict").nth(0).unwrap();
        assert_eq!(ConversionLength::Long, after.conversion.length);
    }

    #[test]
    fn available_to_upgrade_unlocks_by_research() {
        let mut context = GameContext::init_empty_test_game_context();
        let available = available_to_upgrade(&context.state);
        let initial_count = available.len();

        context.state.research.insert("UpgradeTech".to_owned());

        let available = available_to_upgrade(&context.state);
        assert_eq!(initial_count + 1, available.len());
    }

    #[test]
    fn available_to_upgrade_shows_unchosen() {
        let mut context = GameContext::init_empty_test_game_context();
        let available = available_to_upgrade(&context.state);
        let initial_count = available.len();

        context.state.upgrades.insert(available.get(0).unwrap().name.to_string());
        let available = available_to_upgrade(&context.state);
        assert_eq!(initial_count - 1, available.len());
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

    fn apply_edict_upgrade(context: &mut GameContext, name: &str) {
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

        apply_edict_upgrade(&mut context, "TestEdictUpgrade");

        assert_eq!(ConversionLength::Long, context.find_edict("TestEdict").conversion.length);
    }

    #[test]
    fn edict_bonus_applied() {
        let mut context = GameContext::init_empty_test_game_context();

        apply_edict_upgrade(&mut context, "TestEdictUpgradeYield");

        assert_eq!(1, context.find_edict("TestEdict").effective_bonus);
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
    fn apply_research_costs_per_added() {
        let context = GameContext::init_empty_test_game_context();
        let total_cost = get_upgrade_cost(&context, &vec![get_test_upgrade("TestUpgrade"), get_test_upgrade("TestEdictUpgrade")]);
        assert_eq!(total_cost[0].amount, 50);
    }

    #[test]
    fn apply_research_costs_per_removed() {
        let mut context = GameContext::init_empty_test_game_context();
        context.state.upgrades.insert("TestUpgrade".to_owned());
        context.state.upgrades.insert("TestEdictUpgrade".to_owned());

        let total_cost = get_upgrade_cost(&context, &vec![get_test_upgrade("TestUpgrade")]);

        assert_eq!(total_cost[0].amount, 25);
    }

    #[test]
    fn apply_research_costs_per_toggle() {
        let mut context = GameContext::init_empty_test_game_context();
        context.state.upgrades.insert("TestUpgrade".to_owned());

        let total_cost = get_upgrade_cost(&context, &vec![get_test_upgrade("TestEdictUpgrade")]);

        assert_eq!(total_cost[0].amount, 50);
    }

    #[test]
    fn available_to_research_dependencies() {
        let mut context = GameContext::init_empty_test_game_context();
        let mut base_research = available_to_research(&context.state);
        assert_eq!(4, base_research.len());

        context.state.research.insert("TestNoDeps".to_owned());
        base_research = available_to_research(&context.state);
        assert_eq!(3, base_research.len());

        context.state.research.insert("Dep".to_owned());
        base_research = available_to_research(&context.state);
        assert_eq!(3, base_research.len());

        context.state.research.insert("TestWithDep".to_owned());
        base_research = available_to_research(&context.state);
        assert_eq!(2, base_research.len());
    }

    #[test]
    fn available_to_build_changes_with_unlocked_tech() {
        let mut context = GameContext::init_empty_test_game_context();
        let base_build = available_to_build(&context.state);
        context.state.research.insert("TestNoDeps".to_owned());

        assert_eq!(base_build.len() + 1, available_to_build(&context.state).len());
    }

    #[test]
    fn available_to_invoke_changes_with_unlocked_tech() {
        let mut context = GameContext::init_empty_test_game_context();
        let base_invoke = available_to_invoke(&context.state);
        context.state.research.insert("TestNoDeps".to_owned());

        assert_eq!(base_invoke.len() + 1, available_to_invoke(&context.state).len());
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
