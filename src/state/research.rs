use std::collections::HashSet;

use super::{format_resource_list, GameState, ResourceAmount};

#[derive(Clone, Debug)]
pub struct Research {
    pub name: String,
    pub description: String,
    pub dependencies: HashSet<String>,
    pub cost: Vec<ResourceAmount>,
}

impl Research {
    pub fn init(name: &str) -> Research {
        Research {
            name: name.to_owned(),
            description: "".to_owned(),
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

    pub fn with_description(mut self, description: &str) -> Research {
        self.description = description.to_string();
        self
    }

    pub fn is_available(&self, state: &GameState) -> bool {
        if state.research.contains(&self.name) {
            return false;
        }

        check_available_by_research(&self.dependencies, &state)
    }

    pub fn details(&self) -> Vec<String> {
        let mut details: Vec<String> = vec![];
        details.push(format_resource_list("Cost: ", &self.cost));

        if self.description != "" {
            append_string_in_chunks(&mut details, &self.description);
        }
        details
    }
}

fn append_string_in_chunks(details: &mut Vec<String>, description: &str) {
    const DETAIL_LINE_LENGTH: usize = 50;
    let mut line = String::with_capacity(DETAIL_LINE_LENGTH);
    for word in description.split_whitespace() {
        if line.len() + word.len() > DETAIL_LINE_LENGTH {
            details.push(line);
            line = String::with_capacity(DETAIL_LINE_LENGTH);
        }
        line.push_str(word);
        line.push_str(" ");
    }
    details.push(line);
}

pub fn check_available_by_research(dependencies: &HashSet<String>, state: &GameState) -> bool {
    for d in dependencies {
        if !state.research.contains(d) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::data::tests::*;
    use crate::state::GameState;

    #[test]
    fn is_available_no_dependencies() {
        let state = GameState::init_test_empty_game_state();
        let research = get_test_research("TestNoDeps");
        assert!(research.is_available(&state));
    }

    #[test]
    fn is_available_dependencies_met() {
        let mut state = GameState::init_test_empty_game_state();
        state.research.insert("Dep".to_owned());
        let research = get_test_research("TestWithDep");
        assert!(research.is_available(&state));
    }

    #[test]
    fn is_not_available_dependencies_unmet() {
        let state = GameState::init_test_empty_game_state();
        let research = get_test_research("TestWithDep");
        assert!(!research.is_available(&state));
    }

    #[test]
    fn is_not_available_already_researched() {
        let mut state = GameState::init_test_empty_game_state();
        let research = get_test_research("TestNoDeps");
        assert!(research.is_available(&state));
        state.research.insert("TestNoDeps".to_owned());
        assert!(!research.is_available(&state));
    }
}
