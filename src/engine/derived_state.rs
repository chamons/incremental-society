use std::collections::HashMap;

pub use super::upgrade::{available_to_build, available_to_invoke, available_to_research, available_to_upgrade, current_conversions};
pub use crate::state::{Building, Conversion, Edict, GameState, Research, ResourceTotal, Upgrade};

use itertools::Itertools;

#[derive(Debug)]
pub struct DerivedState {
    pub conversions_names: Vec<String>,
    pub conversions: HashMap<String, u32>,
    pub storage: ResourceTotal,
    pub available_buildings: Vec<Building>,
    pub available_edicts: Vec<Edict>,
    pub available_research: Vec<Research>,
    pub available_upgrade: Vec<Upgrade>,
    pub all_conversions: Vec<Conversion>,
}

impl DerivedState {
    pub fn init() -> DerivedState {
        DerivedState {
            conversions_names: vec![],
            conversions: HashMap::new(),
            storage: ResourceTotal::init(),
            available_buildings: vec![],
            available_edicts: vec![],
            available_research: vec![],
            available_upgrade: vec![],
            all_conversions: vec![],
        }
    }

    pub fn calculate(state: &GameState) -> DerivedState {
        DerivedState {
            conversions_names: DerivedState::conversion_names(state),
            conversions: DerivedState::conversion_with_counts(state),
            storage: DerivedState::calculate_storage(state),
            available_buildings: available_to_build(state),
            available_edicts: available_to_invoke(state),
            available_research: available_to_research(state),
            available_upgrade: available_to_upgrade(state),

            // This needs to be provided jobs
            // Then there there needs to be state on job distribution
            // There needs to be checks on job to prevent destruction under jobs taken
            // There needs to be an unassigned
            // Then every conversion needs to take into account (can you swap a job right before a long one finishes to get entire tick, or does it reset?)
            all_conversions: current_conversions(state),
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

    pub fn find_building(&self, name: &str) -> &Building {
        self.available_buildings.iter().filter(|x| x.name == name).nth(0).unwrap()
    }

    pub fn find_edict(&self, name: &str) -> &Edict {
        self.available_edicts.iter().filter(|x| x.name == name).nth(0).unwrap()
    }

    pub fn find_research(&self, name: &str) -> &Research {
        self.available_research.iter().filter(|x| x.name == name).nth(0).unwrap()
    }

    pub fn find_upgrade(&self, name: &str) -> &Upgrade {
        self.available_upgrade.iter().filter(|x| x.name == name).nth(0).unwrap()
    }

    pub fn find_conversion(&self, name: &str) -> &Conversion {
        self.all_conversions.iter().filter(|x| x.name == name).nth(0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::tests::*;
    use crate::state::ResourceKind;

    #[test]
    fn conversion_with_counts() {
        let state = init_test_game_state();
        let conversions = &state.derived_state.conversions;
        assert_eq!(4, *conversions.get("TestChop").unwrap());
        assert_eq!(1, *conversions.get("TestGather").unwrap());
    }

    #[test]
    fn conversion_names() {
        let state = init_test_game_state();
        let conversions = &state.derived_state.conversions_names;
        assert_eq!("TestChop", conversions[0]);
        assert_eq!("TestGather", conversions[1]);
    }

    #[test]
    fn storage() {
        let state = init_test_game_state();
        let storage = state.derived_state.storage;
        assert!(storage[ResourceKind::Food] >= 20);
        assert!(storage[ResourceKind::Fuel] >= 30);
    }
}
