use crate::resources::*;
use crate::state::GameState;

use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone)]
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
    pub storage: ResourceTotal,
    pub pops: u32,
    pub used_pops: u32,
}

impl DerivedState {
    pub fn init() -> DerivedState {
        DerivedState {
            conversion_name: vec![],
            conversion_counts: vec![],
            storage: ResourceTotal::init(),
            pops: 0,
            used_pops: 0,
        }
    }

    pub fn calculate(state: &GameState) -> DerivedState {
        let pops = DerivedState::calculate_pops(&state);
        DerivedState {
            conversion_name: DerivedState::conversion_names(&state),
            conversion_counts: DerivedState::conversion_with_counts(&state, pops),
            storage: DerivedState::calculate_storage(&state),
            pops: DerivedState::calculate_pops(&state),
            used_pops: DerivedState::calculate_used_pops(&state),
        }
    }

    fn conversion_with_counts(state: &GameState, pops: u32) -> Vec<ConversionTotal> {
        let mut counts: HashMap<&str, u32> = HashMap::new();
        for c in state.regions.iter().flat_map(|x| &x.buildings).flat_map(|x| &x.conversions) {
            let entry = counts.entry(c).or_insert(0);
            *entry += 1;
        }
        counts.insert("Sustain Population", pops);

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
        // Sustain Population is always last
        names.push(String::from("Sustain Population"));
        names
    }

    fn calculate_storage(state: &GameState) -> ResourceTotal {
        let mut storage = ResourceTotal::init();
        for building in state.regions.iter().flat_map(|x| &x.buildings) {
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

    #[test]
    fn conversion_with_counts() {
        let state = GameState::init_test_game_state();
        let conversions = &state.derived_state.conversion_counts;
        assert_eq!("TestChop", conversions[0].name);
        assert_eq!(4, conversions[0].count);
        assert_eq!("TestGather", conversions[1].name);
        assert_eq!(1, conversions[1].count);
        assert_eq!("Sustain Population", conversions[2].name);
        assert_eq!(4, state.derived_state.pops);
    }

    #[test]
    fn conversion_names() {
        let state = GameState::init_test_game_state();
        let conversions = &state.derived_state.conversion_name;
        assert_eq!("TestChop", conversions[0]);
        assert_eq!("TestGather", conversions[1]);
        assert_eq!("Sustain Population", conversions[2]);
    }

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
