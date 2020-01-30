use std::collections::{HashMap, HashSet};

use super::{die, process, EngineError};
use crate::engine::data::{get_building, get_building_names, get_edict, get_edict_names, get_research, get_research_names, get_upgrade, get_upgrade_names};
use crate::state::{Building, DelayedAction, Edict, GameState, Research, ResourceAmount, Upgrade, UpgradeActions, Waiter};
use crate::state::{COST_PER_UPGRADE, MAX_UPGRADES, UPGRADE_LENGTH};

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
    for _ in 0..difference.len() {
        for c in COST_PER_UPGRADE.iter() {
            if let Some(i) = cost.iter().position(|x| x.kind == c.0) {
                cost[i].amount += c.1;
            } else {
                cost.push(ResourceAmount::init(c.0, c.1));
            }
        }
    }
    cost
}

pub fn available_to_research(state: &GameState) -> Vec<Research> {
    get_research_by_research(&state)
}

pub fn available_to_build(state: &GameState) -> Vec<Building> {
    let mut available = vec![];

    let upgrades = get_upgrades_by_name(&state.upgrades);
    for b in get_building_by_research(&state) {
        let mut b = b.clone();
        if let Some(upgrades) = upgrades.get(&b.name) {
            for u in upgrades.iter().flat_map(|x| &x.upgrades) {
                apply_building_upgrade(&mut b, u);
            }
        }
        available.push(b);
    }

    available
}

pub fn available_to_invoke(state: &GameState) -> Vec<Edict> {
    let mut available = vec![];

    let upgrades = get_upgrades_by_name(&state.upgrades);
    for e in get_edict_by_research(&state) {
        let mut e = e.clone();
        if let Some(upgrades) = upgrades.get(&e.name) {
            for u in upgrades.iter().flat_map(|x| &x.upgrades) {
                apply_edict_upgrade(&mut e, u);
            }
        }

        available.push(e);
    }

    available
}

pub fn available_to_upgrade(state: &GameState) -> Vec<Upgrade> {
    let mut available = vec![];
    for upgrade in get_upgrade_names().iter().map(|x| get_upgrade(x)) {
        if upgrade.is_available(state) {
            available.push(upgrade);
        }
    }

    available
}

fn apply_building_upgrade(building: &mut Building, upgrade: &UpgradeActions) {
    match upgrade {
        UpgradeActions::AddBuildingPops(pops) => building.pops += pops,
        UpgradeActions::AddBuildingConversion(name) => building.conversions.push(name.to_string()),
        UpgradeActions::AddBuildingStorage(storage) => {
            if let Some(position) = building.storage.iter().position(|x| x.kind == storage.kind) {
                let current = &mut building.storage.get_mut(position).unwrap();
                current.amount += storage.amount;
            } else {
                building.storage.push(*storage);
            }
        }
        UpgradeActions::ChangeEdictLength(_) => die(&"ChangeEdictLength upgrade on building"),
    }
}

fn apply_edict_upgrade(edict: &mut Edict, upgrade: &UpgradeActions) {
    match upgrade {
        UpgradeActions::ChangeEdictLength(length) => edict.conversion.length = *length,
        UpgradeActions::AddBuildingPops(_) => die(&"AddBuildingPops upgrade on edict"),
        UpgradeActions::AddBuildingConversion(_) => die(&"AddBuildingConversion upgrade on edict"),
        UpgradeActions::AddBuildingStorage(_) => die(&"AddBuildingStorage upgrade on edict"),
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
    let mut available = vec![];
    for research in get_research_names().iter().map(|x| get_research(x)) {
        if research.is_available(state) {
            available.push(research);
        }
    }

    available
}

fn get_building_by_research(state: &GameState) -> Vec<Building> {
    let mut available = vec![];

    for building in get_building_names().iter().map(|x| get_building(x)) {
        if building.is_available(&state) {
            available.push(building);
        }
    }

    available
}

fn get_edict_by_research(state: &GameState) -> Vec<Edict> {
    let mut available = vec![];

    for edict in get_edict_names().iter().map(|x| get_edict(x)) {
        if edict.is_available(&state) {
            available.push(edict);
        }
    }

    available
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;
    use crate::engine::tests::*;
    use crate::state::{ConversionLength, Region, ResourceKind};

    fn give_update_resources(state: &mut GameState, count: i64) {
        for c in COST_PER_UPGRADE.iter() {
            state.resources.add(c.0, c.1 * count);
        }
    }

    #[test]
    fn available_to_research_has_upgrades_applied() {
        let mut state = init_empty_game_state();

        let available = available_to_build(&state);
        let before = available.iter().filter(|x| x.name == "Test Building").nth(0).unwrap();
        assert_eq!(2, before.conversions.len());

        state.upgrades.insert("TestUpgrade".to_owned());

        let available = available_to_build(&state);
        let after = available.iter().filter(|x| x.name == "Test Building").nth(0).unwrap();
        assert_eq!(3, after.conversions.len());
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
        give_update_resources(&mut state, 2);

        upgrade(&mut state, vec![get_test_upgrade("TestUpgrade")]).unwrap();
        assert!(upgrade(&mut state, vec![get_test_upgrade("TestUpgrade")]).is_err());
    }

    #[test]
    fn research_has_upgrades_applied() {
        let mut state = init_empty_game_state();
        state.regions = vec![Region::init_with_buildings("First Region", vec![get_test_building("Test Building").clone()])];
        recalculate(&mut state);

        give_update_resources(&mut state, 2);

        upgrade(&mut state, vec![get_test_upgrade("TestUpgrade"), get_test_upgrade("TestEdictUpgrade")]).unwrap();

        for _ in 0..UPGRADE_LENGTH {
            assert_eq!(0, state.upgrades.len());
            assert_eq!(ConversionLength::Short, state.derived_state.find_edict("TestEdict").conversion.length);
            assert_eq!(2, state.buildings()[0].conversions.len());
            process::process_tick(&mut state);
        }

        assert_eq!(2, state.upgrades.len());
        assert_eq!(ConversionLength::Long, state.derived_state.find_edict("TestEdict").conversion.length);
        assert_eq!(0, state.resources[ResourceKind::Knowledge]);
        assert_eq!(3, state.buildings()[0].conversions.len());
    }

    #[test]
    fn research_has_upgrades_removed() {
        let mut state = init_empty_game_state();
        state.regions = vec![Region::init_with_buildings("First Region", vec![get_test_building("Test Building").clone()])];
        recalculate(&mut state);
        give_update_resources(&mut state, 1);

        apply_upgrade(&mut state, vec![get_test_upgrade("TestUpgrade")]);

        upgrade(&mut state, vec![]).unwrap();

        for _ in 0..UPGRADE_LENGTH {
            assert_eq!(1, state.upgrades.len());
            assert_eq!(3, state.buildings()[0].conversions.len());
            process::process_tick(&mut state);
        }

        assert_eq!(0, state.upgrades.len());
        assert_eq!(2, state.buildings()[0].conversions.len());
        assert_eq!(0, state.resources[ResourceKind::Knowledge]);
    }

    #[test]
    fn apply_research_allow_up_to_cap_selections() {
        // If changes, test need changes
        assert_eq!(2, MAX_UPGRADES);

        let mut state = init_empty_game_state();
        give_update_resources(&mut state, 2);

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

        assert_eq!("Insufficient upgrade slots for upgrade plan.", err.description());
    }

    #[test]
    fn apply_research_costs_per_added() {
        let state = init_empty_game_state();

        let total_cost = get_upgrade_cost(&state, &vec![get_test_upgrade("TestUpgrade"), get_test_upgrade("TestEdictUpgrade")]);

        for i in 0..COST_PER_UPGRADE.len() {
            assert_eq!(total_cost[i].amount, COST_PER_UPGRADE[i].1 * 2);
        }
    }

    #[test]
    fn apply_research_costs_per_removed() {
        let mut state = init_empty_game_state();
        state.upgrades.insert("TestUpgrade".to_owned());
        state.upgrades.insert("TestEdictUpgrade".to_owned());

        let total_cost = get_upgrade_cost(&state, &vec![get_test_upgrade("TestUpgrade")]);

        for i in 0..COST_PER_UPGRADE.len() {
            assert_eq!(total_cost[i].amount, COST_PER_UPGRADE[i].1);
        }
    }

    #[test]
    fn apply_research_costs_per_toggle() {
        let mut state = init_empty_game_state();
        state.upgrades.insert("TestUpgrade".to_owned());

        let total_cost = get_upgrade_cost(&state, &vec![get_test_upgrade("TestEdictUpgrade")]);

        for i in 0..COST_PER_UPGRADE.len() {
            assert_eq!(total_cost[i].amount, COST_PER_UPGRADE[i].1 * 2);
        }
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
