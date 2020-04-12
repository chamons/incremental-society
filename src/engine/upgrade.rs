use std::collections::{HashMap, HashSet};

use super::{process, EngineError};
use crate::engine::data::{get_building, get_conversion, get_edict, get_research, get_upgrade};
use crate::engine::data::{get_building_names, get_conversion_names, get_edict_names, get_research_names, get_upgrade_names};
use crate::state::{Building, Conversion, DelayedAction, Edict, GameState, Research, ResourceAmount, Upgrade, UpgradeActions, Waiter};
use crate::state::{MAX_UPGRADES, UPGRADE_LENGTH};

pub fn can_apply_upgrades(state: &GameState, upgrades: &[Upgrade]) -> Result<(), EngineError> {
    let cost = get_upgrade_cost(state, &upgrades);

    if upgrades.len() > MAX_UPGRADES {
        return Err(EngineError::init("Insufficient upgrade slots for upgrade plan."));
    }

    if state.actions.iter().any(|x| x.action.is_upgrade()) {
        return Err(EngineError::init("Unable to upgrade due to another upgrade already in progress."));
    }

    if !state.resources.has_range(&cost) {
        return Err(EngineError::init("Insufficient resources for upgrade cost."));
    }

    Ok(())
}

pub fn upgrade(state: &mut GameState, upgrades: Vec<Upgrade>) -> Result<(), EngineError> {
    can_apply_upgrades(state, &upgrades)?;

    let cost = get_upgrade_cost(state, &upgrades);
    state.resources.remove_range(&cost);

    let action = Waiter::init_one_shot(
        "Implementing Upgrades",
        UPGRADE_LENGTH,
        DelayedAction::Upgrade(upgrades.iter().map(|x| x.name.to_owned()).collect()),
    );
    state.actions.push(action);
    process::recalculate(state);

    Ok(())
}

pub fn apply_upgrade(state: &mut GameState, upgrades: Vec<Upgrade>) {
    state.upgrades = upgrades.iter().map(|x| x.name.to_string()).collect();

    // We must first recalculate to take into account the new upgraded buildings
    process::recalculate(state);

    // Since we can toggle between upgrades (for a price) it is easier to check "update" redo every thing that can be upgraded (building/edict)
    for r in &mut state.regions {
        for i in 0..r.buildings.len() {
            r.buildings[i] = state.derived_state.find_building(&r.buildings[i].name).clone();
        }
    }

    // We must recalculate again to take into account those upgraded buildings
    process::recalculate(state);
}

pub fn get_upgrade_cost(state: &GameState, upgrades: &[Upgrade]) -> Vec<ResourceAmount> {
    let current: HashSet<&String> = state.upgrades.iter().collect();
    let desired: HashSet<&String> = upgrades.iter().map(|x| &x.name).collect();

    let difference: HashSet<_> = desired.symmetric_difference(&current).collect();

    let mut cost: Vec<ResourceAmount> = vec![];
    for diff in difference.iter().map(|x| state.derived_state.find_upgrade(x)) {
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

#[allow(clippy::single_match)]
fn apply_edict_upgrade(edict: &mut Edict, upgrade: &UpgradeActions) {
    match upgrade {
        UpgradeActions::ChangeEdictLength(length) => edict.conversion.length = *length,
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
    use crate::engine::add_job;
    use crate::engine::tests::*;
    use crate::state::{ConversionLength, Region, ResourceKind};

    fn give_test_update_resources(state: &mut GameState, count: i64) {
        state.resources.add(ResourceKind::Knowledge, 25 * count);
    }

    #[test]
    fn available_to_research_has_upgrades_applied() {
        let mut state = init_empty_game_state();

        let available = available_to_build(&state);
        let before = available.iter().filter(|x| x.name == "Test Building").nth(0).unwrap();
        assert_eq!(2, before.jobs.len());

        state.upgrades.insert("TestUpgrade".to_owned());

        let available = available_to_build(&state);
        let after = available.iter().filter(|x| x.name == "Test Building").nth(0).unwrap();
        assert_eq!(3, after.jobs.len());
    }

    #[test]
    fn available_to_invoke_has_upgrades_applied() {
        let mut state = init_empty_game_state();

        let available = available_to_invoke(&state);
        let before = available.iter().filter(|x| x.name == "TestEdict").nth(0).unwrap();
        assert_eq!(ConversionLength::Short, before.conversion.length);

        state.upgrades.insert("TestEdictUpgrade".to_owned());

        let available = available_to_invoke(&state);
        let after = available.iter().filter(|x| x.name == "TestEdict").nth(0).unwrap();
        assert_eq!(ConversionLength::Long, after.conversion.length);
    }

    #[test]
    fn available_to_upgrade_unlocks_by_research() {
        let mut state = init_empty_game_state();
        let available = available_to_upgrade(&state);
        let initial_count = available.len();

        state.research.insert("UpgradeTech".to_owned());

        let available = available_to_upgrade(&state);
        assert_eq!(initial_count + 1, available.len());
    }
    #[test]
    fn available_to_upgrade_shows_all_unlocked() {
        let mut state = init_empty_game_state();
        let available = available_to_upgrade(&state);
        let inital_count = available.len();

        state.upgrades.insert(available.get(0).unwrap().name.to_string());
        let available = available_to_upgrade(&state);
        assert_eq!(inital_count, available.len());
    }

    #[test]
    fn research_multiple_in_flight() {
        let mut state = init_empty_game_state();
        give_test_update_resources(&mut state, 2);

        upgrade(&mut state, vec![get_test_upgrade("TestUpgrade")]).unwrap();
        assert!(upgrade(&mut state, vec![get_test_upgrade("TestUpgrade")]).is_err());
    }

    #[test]
    fn upgrade_costs_resources() {
        let mut state = init_empty_game_state();
        assert!(upgrade(&mut state, vec![get_test_upgrade("TestUpgrade")]).is_err());

        give_test_update_resources(&mut state, 1);

        assert!(upgrade(&mut state, vec![get_test_upgrade("TestUpgrade")]).is_ok());
        assert_eq!(0, state.resources[ResourceKind::Knowledge]);
    }

    #[test]
    fn has_building_upgrade_applied() {
        let mut state = init_empty_game_state();
        state.regions = vec![Region::init_with_buildings("First Region", vec![get_test_building("Test Building").clone()])];
        recalculate(&mut state);

        give_test_update_resources(&mut state, 1);

        upgrade(&mut state, vec![get_test_upgrade("TestUpgrade")]).unwrap();

        for _ in 0..UPGRADE_LENGTH {
            assert_eq!(0, state.upgrades.len());
            assert_eq!(2, state.buildings()[0].jobs.len());
            process::process_tick(&mut state);
        }

        assert_eq!(1, state.upgrades.len());
        assert_eq!(0, state.resources[ResourceKind::Knowledge]);
        assert_eq!(3, state.buildings()[0].jobs.len());
    }

    #[test]
    fn has_edict_applied() {
        let mut state = init_empty_game_state();

        give_test_update_resources(&mut state, 1);
        upgrade(&mut state, vec![get_test_upgrade("TestEdictUpgrade")]).unwrap();

        for _ in 0..UPGRADE_LENGTH {
            assert_eq!(0, state.upgrades.len());
            assert_eq!(ConversionLength::Short, state.derived_state.find_edict("TestEdict").conversion.length);
            process::process_tick(&mut state);
        }

        assert_eq!(1, state.upgrades.len());
        assert_eq!(ConversionLength::Long, state.derived_state.find_edict("TestEdict").conversion.length);
        assert_eq!(0, state.resources[ResourceKind::Knowledge]);
    }

    #[test]
    fn has_conversion_applied() {
        let mut state = init_empty_game_state();
        state.regions = vec![Region::init_with_buildings(
            "First Region",
            vec![get_test_building("Test Building").clone(), get_test_building("Stability Building").clone()],
        )];
        state.resources[ResourceKind::Food] = 300;
        state.pops = 2;
        recalculate(&mut state);

        add_job(&mut state, "TestChop").unwrap();
        add_job(&mut state, "TestChop").unwrap();

        give_test_update_resources(&mut state, 1);

        upgrade(&mut state, vec![get_test_upgrade("TestConversionUpgrade")]).unwrap();

        for _ in 0..UPGRADE_LENGTH {
            assert_eq!(0, state.upgrades.len());
            assert_eq!(1, state.derived_state.find_conversion("TestChop").output.len());
            process::process_tick(&mut state);
        }

        assert_eq!(1, state.upgrades.len());
        assert_eq!(2, state.derived_state.find_conversion("TestChop").output.len());
        assert_eq!(0, state.resources[ResourceKind::Knowledge]);

        let conversion_length = state.derived_state.find_conversion("TestChop").tick_length();
        for _ in 0..conversion_length {
            process::process_tick(&mut state);
        }
        assert_eq!(2, state.resources[ResourceKind::Knowledge]);
    }

    #[test]
    fn has_upgrades_removed() {
        let mut state = init_empty_game_state();
        state.regions = vec![Region::init_with_buildings("First Region", vec![get_test_building("Test Building").clone()])];
        recalculate(&mut state);
        give_test_update_resources(&mut state, 1);

        apply_upgrade(&mut state, vec![get_test_upgrade("TestUpgrade")]);

        upgrade(&mut state, vec![]).unwrap();

        for _ in 0..UPGRADE_LENGTH {
            assert_eq!(1, state.upgrades.len());
            assert_eq!(3, state.buildings()[0].jobs.len());
            process::process_tick(&mut state);
        }

        assert_eq!(0, state.upgrades.len());
        assert_eq!(2, state.buildings()[0].jobs.len());
        assert_eq!(0, state.resources[ResourceKind::Knowledge]);
    }

    #[test]
    fn upgrade_affects_multiple_items_differently() {
        let mut state = init_empty_game_state();
        state.regions = vec![Region::init_with_buildings("First Region", vec![get_test_building("Test Building").clone()])];
        state.upgrades.insert("TestMultiUpgrade".to_string());
        recalculate(&mut state);

        assert_eq!(2, state.buildings()[0].jobs.len());
        assert_eq!(2, state.derived_state.find_conversion("TestChop").output.len());
        assert_eq!(ConversionLength::Long, state.derived_state.find_edict("TestEdict").conversion.length);
    }

    #[test]
    fn apply_research_allow_up_to_cap_selections() {
        // If changes, test need changes
        assert_eq!(2, MAX_UPGRADES);

        let mut state = init_empty_game_state();
        give_test_update_resources(&mut state, 2);

        assert!(can_apply_upgrades(&state, &vec![get_test_upgrade("TestUpgrade"), get_test_upgrade("TestEdictUpgrade")]).is_ok());

        let err = can_apply_upgrades(
            &state,
            &vec![
                get_test_upgrade("TestUpgrade"),
                get_test_upgrade("TestEdictUpgrade"),
                get_test_upgrade("TestOtherUpgrade"),
            ],
        )
        .unwrap_err();

        assert_eq!("Insufficient upgrade slots for upgrade plan.", err.to_string());
    }

    #[test]
    fn apply_research_costs_per_added() {
        let state = init_empty_game_state();

        let total_cost = get_upgrade_cost(&state, &vec![get_test_upgrade("TestUpgrade"), get_test_upgrade("TestEdictUpgrade")]);

        assert_eq!(total_cost[0].amount, 50);
    }

    #[test]
    fn apply_research_costs_per_removed() {
        let mut state = init_empty_game_state();
        state.upgrades.insert("TestUpgrade".to_owned());
        state.upgrades.insert("TestEdictUpgrade".to_owned());

        let total_cost = get_upgrade_cost(&state, &vec![get_test_upgrade("TestUpgrade")]);

        assert_eq!(total_cost[0].amount, 25);
    }

    #[test]
    fn apply_research_costs_per_toggle() {
        let mut state = init_empty_game_state();
        state.upgrades.insert("TestUpgrade".to_owned());

        let total_cost = get_upgrade_cost(&state, &vec![get_test_upgrade("TestEdictUpgrade")]);

        assert_eq!(total_cost[0].amount, 50);
    }

    #[test]
    fn available_to_research_dependencies() {
        let mut state = init_empty_game_state();
        let mut base_research = available_to_research(&state);
        assert_eq!(4, base_research.len());

        state.research.insert("TestNoDeps".to_owned());
        base_research = available_to_research(&state);
        assert_eq!(3, base_research.len());

        state.research.insert("Dep".to_owned());
        base_research = available_to_research(&state);
        assert_eq!(3, base_research.len());

        state.research.insert("TestWithDep".to_owned());
        base_research = available_to_research(&state);
        assert_eq!(2, base_research.len());
    }

    #[test]
    fn available_to_build_changes_with_unlocked_tech() {
        let mut state = init_empty_game_state();
        let base_build = available_to_build(&state);
        state.research.insert("TestNoDeps".to_owned());

        assert_eq!(base_build.len() + 1, available_to_build(&state).len());
    }

    #[test]
    fn available_to_invoke_changes_with_unlocked_tech() {
        let mut state = init_empty_game_state();
        let base_invoke = available_to_invoke(&state);
        state.research.insert("TestNoDeps".to_owned());

        assert_eq!(base_invoke.len() + 1, available_to_invoke(&state).len());
    }
}
