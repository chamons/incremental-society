use std::collections::HashSet;

use super::{GameState, ResourceAmount};

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

#[cfg(test)]
mod tests {
    use crate::engine::tests::*;

    #[test]
    fn is_available_no_dependencies() {
        let state = init_empty_game_state();
        let research = get_research("TestNoDeps");
        assert!(research.is_available(&state));
    }

    #[test]
    fn is_available_dependencies_met() {
        let mut state = init_empty_game_state();
        state.research.insert("Dep".to_owned());
        let research = get_research("TestWithDep");
        assert!(research.is_available(&state));
    }

    #[test]
    fn is_not_available_dependencies_unmet() {
        let state = init_empty_game_state();
        let research = get_research("TestWithDep");
        assert!(!research.is_available(&state));
    }

    #[test]
    fn is_not_available_already_researched() {
        let mut state = init_empty_game_state();
        let research = get_research("TestNoDeps");
        assert!(research.is_available(&state));
        state.research.insert("TestNoDeps".to_owned());
        assert!(!research.is_available(&state));
    }
}
