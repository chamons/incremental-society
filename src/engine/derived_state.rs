use crate::resources::*;
use crate::state::GameState;

use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
pub struct DerivedState {
    pub conversions_names: Vec<String>,
    pub conversions: HashMap<String, u32>,
    pub storage: ResourceTotal,
    pub pops: u32,
    pub used_pops: u32,
}

impl DerivedState {
    pub fn init() -> DerivedState {
        DerivedState {
            conversions_names: vec![],
            conversions: HashMap::new(),
            storage: ResourceTotal::init(),
            pops: 0,
            used_pops: 0,
        }
    }

    pub fn calculate(state: &GameState) -> DerivedState {
        DerivedState {
            conversions_names: DerivedState::conversion_names(state),
            conversions: DerivedState::conversion_with_counts(state),
            storage: DerivedState::calculate_storage(state),
            pops: DerivedState::calculate_pops(state),
            used_pops: DerivedState::calculate_used_pops(state),
        }
    }

    fn conversion_with_counts(state: &GameState) -> HashMap<String, u32> {
        let mut counts = HashMap::new();
        for c in state.regions.iter().flat_map(|x| &x.buildings).flat_map(|x| &x.conversions) {
            let entry = counts.entry(c.to_string()).or_insert(0);
            *entry += 1;
        }
        counts
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

    fn calculate_storage(state: &GameState) -> ResourceTotal {
        let mut storage = ResourceTotal::init();
        for building in state.buildings() {
            for resource in &building.storage {
                storage.add(resource.kind, resource.amount);
            }
        }
        storage
    }

    fn calculate_pops(state: &GameState) -> u32 {
        state.regions.iter().flat_map(|x| &x.buildings).map(|x| x.pops).sum()
    }

    fn calculate_used_pops(state: &GameState) -> u32 {
        state.regions.iter().flat_map(|x| &x.buildings).count() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn conversion_with_counts() {
    //     let state = GameState::init_test_game_state();
    //     let conversions = &state.derived_state.conversion_counts;
    //     assert_eq!("TestChop", conversions[0].name);
    //     assert_eq!(4, conversions[0].count);
    //     assert_eq!("TestGather", conversions[1].name);
    //     assert_eq!(1, conversions[1].count);
    // }

    // #[test]
    // fn conversion_names() {
    //     let state = GameState::init_test_game_state();
    //     let conversions = &state.derived_state.conversion_name;
    //     assert_eq!("TestChop", conversions[0]);
    //     assert_eq!("TestGather", conversions[1]);
    // }

    #[test]
    fn storage() {
        let state = GameState::init_test_game_state();
        let storage = state.derived_state.storage;
        assert!(storage[ResourceKind::Food] >= 20);
        assert!(storage[ResourceKind::Fuel] >= 30);
    }

    #[test]
    fn pops() {
        let state = GameState::init_test_game_state();
        assert!(state.derived_state.pops >= 4);
        assert!(state.derived_state.used_pops >= 3);
    }
}
