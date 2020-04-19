pub use super::upgrade;

use std::collections::{HashMap, HashSet};

use crate::data::{get_building, get_conversion, get_edict, get_research, get_upgrade};
use crate::data::{get_building_names, get_conversion_names, get_edict_names, get_research_names, get_upgrade_names};
use crate::state::{Building, Conversion, Edict, GameState, Research, Upgrade, UpgradeActions};

#[derive(Debug)]
pub struct UpgradeState {
    pub available_buildings: Vec<Building>,
    pub available_edicts: Vec<Edict>,
    pub available_research: Vec<Research>,
    pub available_upgrade: Vec<Upgrade>,
    pub available_conversions: Vec<Conversion>,
    pub stability_gain: u32,
}

impl UpgradeState {
    pub fn init() -> UpgradeState {
        UpgradeState {
            available_buildings: vec![],
            available_edicts: vec![],
            available_research: vec![],
            available_upgrade: vec![],
            available_conversions: vec![],
            stability_gain: 0,
        }
    }

    pub fn calculate(state: &GameState) -> UpgradeState {
        UpgradeState {
            available_buildings: available_to_build(state),
            available_edicts: available_to_invoke(state),
            available_research: available_to_research(state),
            available_upgrade: available_to_upgrade(state),
            available_conversions: current_conversions(state),
            stability_gain: get_current_stability_gain(state),
        }
    }

    pub fn find_building(&self, name: &str) -> Building {
        self.available_buildings.iter().filter(|x| x.name == name).nth(0).unwrap().clone()
    }

    pub fn find_edict(&self, name: &str) -> Edict {
        self.available_edicts.iter().filter(|x| x.name == name).nth(0).unwrap().clone()
    }

    pub fn find_research(&self, name: &str) -> Research {
        self.available_research.iter().filter(|x| x.name == name).nth(0).unwrap().clone()
    }

    pub fn find_upgrade(&self, name: &str) -> Upgrade {
        self.available_upgrade.iter().filter(|x| x.name == name).nth(0).unwrap().clone()
    }

    pub fn find_conversion(&self, name: &str) -> Conversion {
        self.available_conversions.iter().filter(|x| x.name == name).nth(0).unwrap().clone()
    }
}

fn current_conversions(state: &GameState) -> Vec<Conversion> {
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

fn get_current_stability_gain(state: &GameState) -> u32 {
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

fn available_to_research(state: &GameState) -> Vec<Research> {
    get_research_by_research(&state)
}

fn available_to_build(state: &GameState) -> Vec<Building> {
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

fn available_to_invoke(state: &GameState) -> Vec<Edict> {
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

fn available_to_upgrade(state: &GameState) -> Vec<Upgrade> {
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
    use crate::engine::GameContext;
    use crate::state::ConversionLength;

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
}
