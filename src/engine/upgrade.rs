use std::collections::{HashMap, HashSet};

use super::{die, EngineError};
use crate::engine::data::{get_building, get_building_names, get_edict, get_edict_names, get_research, get_research_names, get_upgrade, get_upgrade_names};
use crate::state::{Building, Edict, GameState, Research, ResourceTotal, Upgrade, UpgradeActions, COST_PER_UPGRADE, MAX_UPGRADES};

pub fn can_apply_upgrades(state: &GameState, upgrades: Vec<Upgrade>) -> Result<(), EngineError> {
    let cost = get_upgrade_cost(state, &upgrades);

    if upgrades.len() > MAX_UPGRADES {
        return Err(EngineError::init("Insufficient upgrade slots for upgrade plan."));
    }

    if !state.resources.has_total(&cost) {
        return Err(EngineError::init("Insufficient resources for upgrade cost."));
    }

    Ok(())
}

pub fn apply_upgrades(state: &mut GameState, upgrades: Vec<Upgrade>) -> Result<(), EngineError> {
    can_apply_upgrades(state, upgrades)?;

    // Since we can toggle between upgrades (for a price) it is easier to check "update" redo every building
    Ok(())
}

pub fn get_upgrade_cost(state: &GameState, upgrades: &Vec<Upgrade>) -> ResourceTotal {
    let current: HashSet<&String> = state.upgrades.iter().collect();
    let desired: HashSet<&String> = upgrades.iter().map(|x| &x.name).collect();

    let difference: HashSet<_> = desired.symmetric_difference(&current).collect();

    let mut cost = ResourceTotal::init();
    for _ in 0..difference.len() {
        for c in COST_PER_UPGRADE.iter() {
            cost.add(c.0, c.1);
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
                current.amount += current.amount + storage.amount;
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

fn get_upgrades_by_name(upgrades: &Vec<String>) -> HashMap<String, Vec<Upgrade>> {
    let mut sorted_list: HashMap<String, Vec<Upgrade>> = HashMap::new();
    for upgrade in upgrades {
        let upgrade = get_upgrade(upgrade);
        for item in &upgrade.items_upgraded {
            sorted_list.entry(item.to_string()).or_insert(vec![]).push(upgrade.clone());
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
        if building.is_available(&state) && !building.immortal {
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
    use crate::state::ConversionLength;

    #[test]
    fn available_to_research_has_upgrades_applied() {
        let mut state = init_empty_game_state();

        let available = available_to_build(&state);
        let before = available.iter().filter(|x| x.name == "Test Building").nth(0).unwrap();
        assert_eq!(2, before.conversions.len());

        state.upgrades.push("TestUpgrade".to_owned());

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

        state.upgrades.push("TestEdictUpgrade".to_owned());

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

        state.upgrades.push(available.get(0).unwrap().name.to_string());
        let available = available_to_upgrade(&state);
        assert_eq!(inital_count, available.len());
    }

    #[test]
    fn apply_research_gamestate_has_upgrades_applied() {}

    #[test]
    fn apply_research_gamestate_has_upgrades_removed() {}

    #[test]
    fn apply_research_allow_up_to_cap_selections() {
        // If changes, test need changes
        assert_eq!(2, MAX_UPGRADES);

        let mut state = init_empty_game_state();
        for c in COST_PER_UPGRADE.iter() {
            state.resources.add(c.0, c.1 * 2);
        }

        assert!(can_apply_upgrades(&state, vec![get_test_upgrade("TestUpgrade"), get_test_upgrade("TestEdictUpgrade")]).is_ok());

        let err = can_apply_upgrades(
            &state,
            vec![
                get_test_upgrade("TestUpgrade"),
                get_test_upgrade("TestEdictUpgrade"),
                get_test_upgrade("TestOtherUpgrade"),
            ],
        )
        .unwrap_err();

        assert_eq!("Insufficient upgrade slots for upgrade plan.", err.description());
    }

    #[test]
    fn apply_research_errors_if_too_many_selected() {}

    #[test]
    fn apply_research_costs_per_added() {
        let state = init_empty_game_state();

        let total_cost = get_upgrade_cost(&state, &vec![get_test_upgrade("TestUpgrade"), get_test_upgrade("TestEdictUpgrade")]);

        for c in COST_PER_UPGRADE.iter() {
            assert_eq!(total_cost[c.0], c.1 * 2);
        }
    }

    #[test]
    fn apply_research_costs_per_removed() {
        let mut state = init_empty_game_state();
        state.upgrades.push("TestUpgrade".to_owned());
        state.upgrades.push("TestEdictUpgrade".to_owned());

        let total_cost = get_upgrade_cost(&state, &vec![get_test_upgrade("TestUpgrade")]);

        for c in COST_PER_UPGRADE.iter() {
            assert_eq!(total_cost[c.0], c.1);
        }
    }

    #[test]
    fn apply_research_costs_per_toggle() {
        let mut state = init_empty_game_state();
        state.upgrades.push("TestUpgrade".to_owned());

        let total_cost = get_upgrade_cost(&state, &vec![get_test_upgrade("TestEdictUpgrade")]);

        for c in COST_PER_UPGRADE.iter() {
            assert_eq!(total_cost[c.0], c.1 * 2);
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
    fn available_to_build_does_not_include_immortal() {
        let state = init_empty_game_state();
        assert!(!available_to_build(&state).iter().any(|x| x.immortal));
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
