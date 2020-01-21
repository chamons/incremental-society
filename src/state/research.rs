use std::collections::HashSet;

use super::{GameState, ResourceAmount};
use crate::data;

use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Research {
    pub name: String,
    pub dependencies: HashSet<String>,
    pub cost: Vec<ResourceAmount>,
}

impl Research {
    pub fn init(name: &str) -> Research {
        Research {
            name: name.to_owned(),
            dependencies: HashSet::new(),
            cost: vec![],
        }
    }

    pub fn with_cost(mut self, cost: Vec<ResourceAmount>) -> Research {
        self.cost = cost;
        self
    }

    pub fn with_dependencies(mut self, cost: Vec<&str>) -> Research {
        self.dependencies = cost.iter().map(|x| (*x).to_owned()).collect();
        self
    }

    pub fn is_available(&self, state: &GameState) -> bool {
        if state.research.contains(&self.name) {
            return false;
        }

        for d in &self.dependencies {
            if !state.research.contains(d) {
                return false;
            }
        }
        true
    }

    pub fn details(&self) -> Vec<String> {
        let mut details: Vec<String> = vec![];
        details.push(format!("Cost: {}", self.cost.iter().map(|x| format!("{} {}", x.amount, x.kind)).format(", ")));
        details
    }
}

pub fn available_to_research(state: &GameState) -> Vec<String> {
    let mut available = vec![];
    for res in data::get_research_names() {
        let res = data::get_research(&res);
        if res.is_available(state) {
            available.push(res.name);
        }
    }

    available
}

pub fn available_to_build(state: &GameState) -> Vec<String> {
    let mut available = vec![];

    for building_name in data::get_building_names() {
        let building = data::get_building(&building_name);
        let has_missing_dep = building.research.iter().any(|x| !state.research.contains(x));

        if !(has_missing_dep || building.immortal) {
            available.push(building_name);
        }
    }

    available
}

pub fn available_to_invoke(state: &GameState) -> Vec<String> {
    let mut available = vec![];

    for edict_name in data::get_edict_names() {
        let edict = data::get_edict(&edict_name);
        let has_missing_dep = edict.research.iter().any(|x| !state.research.contains(x));

        if !(has_missing_dep) {
            available.push(edict_name);
        }
    }

    available
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data;
    use crate::engine::init_empty_game_state;

    #[test]
    fn is_available_no_dependencies() {
        let state = init_empty_game_state();
        let research = data::get_research("TestNoDeps");
        assert!(research.is_available(&state));
    }

    #[test]
    fn is_available_dependencies_met() {
        let mut state = init_empty_game_state();
        state.research.insert("Dep".to_owned());
        let research = data::get_research("TestWithDep");
        assert!(research.is_available(&state));
    }

    #[test]
    fn is_not_available_dependencies_unmet() {
        let state = init_empty_game_state();
        let research = data::get_research("TestWithDep");
        assert!(!research.is_available(&state));
    }

    #[test]
    fn is_not_available_already_researched() {
        let mut state = init_empty_game_state();
        let research = data::get_research("TestNoDeps");
        assert!(research.is_available(&state));
        state.research.insert("TestNoDeps".to_owned());
        assert!(!research.is_available(&state));
    }

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
        assert!(!available_to_build(&state).iter().any(|x| data::get_building(x).immortal));
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
