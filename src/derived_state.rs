use crate::state::GameState;

use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
pub struct ConversionTotal {
    pub name: String,
    pub count: u32,
}

impl ConversionTotal {
    pub fn init(name: &str, count: u32) -> ConversionTotal {
        ConversionTotal { name: name.to_owned(), count }
    }
}

#[derive(Debug)]
pub struct DerivedState {
    pub conversion_name: Vec<String>,
    pub conversion_counts: Vec<ConversionTotal>,
}

impl DerivedState {
    pub fn init() -> DerivedState {
        DerivedState {
            conversion_name: vec![],
            conversion_counts: vec![],
        }
    }

    pub fn calculate(state: &GameState) -> DerivedState {
        DerivedState {
            conversion_name: DerivedState::conversion_names(&state),
            conversion_counts: DerivedState::conversion_with_counts(&state),
        }
    }

    fn conversion_with_counts(state: &GameState) -> Vec<ConversionTotal> {
        let mut counts: HashMap<&str, u32> = HashMap::new();
        for c in state.regions.iter().flat_map(|x| &x.buildings).flat_map(|x| &x.conversions) {
            let entry = counts.entry(c).or_insert(0);
            *entry += 1;
        }
        let mut conversion_with_counts = Vec::with_capacity(counts.len());
        for name in DerivedState::conversion_names(&state) {
            let count = counts.get::<str>(&name).unwrap();
            conversion_with_counts.push(ConversionTotal::init(&name, *count));
        }
        conversion_with_counts
    }

    fn conversion_names(state: &GameState) -> Vec<String> {
        let mut names: Vec<String> = state
            .regions
            .iter()
            .flat_map(|x| &x.buildings)
            .flat_map(|x| &x.conversions)
            .unique()
            .cloned()
            .collect();
        names.sort();
        names
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_with_counts() {
        let state = GameState::init_test_game_state();
        let conversions = &state.derived_state.conversion_counts;
        assert_eq!("TestChop", conversions[0].name);
        assert_eq!(4, conversions[0].count);
        assert_eq!("TestGather", conversions[1].name);
        assert_eq!(1, conversions[1].count);
    }

    #[test]
    fn conversion_names() {
        let state = GameState::init_test_game_state();
        let conversions = &state.derived_state.conversion_name;
        assert_eq!("TestChop", conversions[0]);
        assert_eq!("TestGather", conversions[1]);
    }
}
