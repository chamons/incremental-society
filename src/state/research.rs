use std::collections::HashSet;

use super::{format_resource_list, GameState, ResourceAmount};

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

        check_available(&self.dependencies, &state)
    }

    pub fn details(&self) -> Vec<String> {
        let mut details: Vec<String> = vec![];
        details.push(format_resource_list("Cost: ", &self.cost));
        details
    }
}

pub fn check_available(dependencies: &HashSet<String>, state: &GameState) -> bool {
    for d in dependencies {
        if !state.research.contains(d) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::engine::tests::*;

    #[test]
    fn is_available_no_dependencies() {
        let state = init_empty_game_state();
        let research = get_test_research("TestNoDeps");
        assert!(research.is_available(&state));
    }

    #[test]
    fn is_available_dependencies_met() {
        let mut state = init_empty_game_state();
        state.research.insert("Dep".to_owned());
        let research = get_test_research("TestWithDep");
        assert!(research.is_available(&state));
    }

    #[test]
    fn is_not_available_dependencies_unmet() {
        let state = init_empty_game_state();
        let research = get_test_research("TestWithDep");
        assert!(!research.is_available(&state));
    }

    #[test]
    fn is_not_available_already_researched() {
        let mut state = init_empty_game_state();
        let research = get_test_research("TestNoDeps");
        assert!(research.is_available(&state));
        state.research.insert("TestNoDeps".to_owned());
        assert!(!research.is_available(&state));
    }
}
