use std::collections::HashMap;

use super::die;
use crate::engine::data::{get_building, get_building_names, get_edict, get_edict_names, get_research, get_research_names, get_upgrade, get_upgrade_names};
use crate::state::{Building, Edict, GameState, Research, Upgrade, UpgradeActions};

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

fn apply_edict_upgrade(edict: &mut Edict, upgrade: &UpgradeActions) {
    match upgrade {
        UpgradeActions::ChangeEdictLength(length) => edict.conversion.length = *length,
        UpgradeActions::AddBuildingPops(_) => die(&"AddBuildingPops upgrade on edict"),
        UpgradeActions::AddBuildingConversion(_) => die(&"AddBuildingConversion upgrade on edict"),
        UpgradeActions::AddBuildingStorage(_) => die(&"AddBuildingStorage upgrade on edict"),
    }
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
    use super::*;
    use crate::engine::tests::*;

    // Apply each upgrade to a building
    // Apply each upgrade to an edict
    // Test upgrade unlock by research
    // Test upgrade application when available_to_{build, invoke}

    #[test]
    fn available_to_research_dependencies() {
        let mut state = init_empty_game_state();
        let mut base_research = available_to_research(&state);
        assert_eq!(3, base_research.len());

        state.research.insert("TestNoDeps".to_owned());
        base_research = available_to_research(&state);
        assert_eq!(2, base_research.len());

        state.research.insert("Dep".to_owned());
        base_research = available_to_research(&state);
        assert_eq!(2, base_research.len());

        state.research.insert("TestWithDep".to_owned());
        base_research = available_to_research(&state);
        assert_eq!(1, base_research.len());
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
