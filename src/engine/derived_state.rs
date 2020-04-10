use std::collections::HashMap;

pub use super::upgrade;
pub use crate::state::{Building, Conversion, Edict, GameState, Research, ResourceTotal, Upgrade};

#[derive(Debug)]
pub struct DerivedState {
    pub current_building_jobs: HashMap<String, u32>,
    pub storage: ResourceTotal,
    pub available_buildings: Vec<Building>,
    pub available_edicts: Vec<Edict>,
    pub available_research: Vec<Research>,
    pub available_upgrade: Vec<Upgrade>,
    pub available_conversions: Vec<Conversion>,
}

impl DerivedState {
    pub fn init() -> DerivedState {
        DerivedState {
            current_building_jobs: HashMap::new(),
            storage: ResourceTotal::init(),
            available_buildings: vec![],
            available_edicts: vec![],
            available_research: vec![],
            available_upgrade: vec![],
            available_conversions: vec![],
        }
    }

    pub fn calculate(state: &GameState) -> DerivedState {
        DerivedState {
            current_building_jobs: DerivedState::jobs_with_counts(state),
            storage: DerivedState::calculate_storage(state),

            // Items with respect to upgrades
            available_buildings: upgrade::available_to_build(state),
            available_edicts: upgrade::available_to_invoke(state),
            available_research: upgrade::available_to_research(state),
            available_upgrade: upgrade::available_to_upgrade(state),
            available_conversions: upgrade::current_conversions(state),
            // Reassign job resets conversoin
            // Prevent building destruction if would cause some jobs to be oversubscrubed? Or just fix jobs?
            // Fixup existing tests
            // Add UI in game to assign jobs
        }
    }

    fn jobs_with_counts(state: &GameState) -> HashMap<String, u32> {
        let mut counts = HashMap::new();
        for c in state.regions.iter().flat_map(|x| &x.buildings).flat_map(|x| &x.jobs) {
            let entry = counts.entry(c.to_string()).or_insert(0);
            *entry += 1;
        }
        counts
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
        self.available_conversions.iter().filter(|x| x.name == name).nth(0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::tests::*;
    use crate::state::ResourceKind;

    #[test]
    fn jobs_with_counts() {
        let state = init_test_game_state();
        let jobs = &state.derived_state.current_building_jobs;
        assert_eq!(4, *jobs.get("TestChop").unwrap());
        assert_eq!(1, *jobs.get("TestGather").unwrap());
    }

    #[test]
    fn storage() {
        let state = init_test_game_state();
        let storage = state.derived_state.storage;
        assert!(storage[ResourceKind::Food] >= 20);
        assert!(storage[ResourceKind::Fuel] >= 30);
    }
}
