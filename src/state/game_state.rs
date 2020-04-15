use std::collections::{HashMap, HashSet};

use super::actions::Waiter;
use super::building::Building;
use super::region::Region;
use super::resources::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub resources: ResourceTotal,
    pub regions: Vec<Region>,
    pub actions: Vec<Waiter>,
    pub pops: u32,
    pub research: HashSet<String>,
    pub upgrades: HashSet<String>,
    pub age: String,
    pub jobs: HashMap<String, u32>,
}

use crate::data::get_ages;
use crate::data::get_building;

impl GameState {
    pub fn init(regions: Vec<Region>, pops: u32, age: String) -> GameState {
        GameState {
            regions,
            pops,
            age,
            resources: ResourceTotal::init(),
            actions: vec![],
            research: HashSet::new(),
            upgrades: HashSet::new(),
            jobs: HashMap::new(),
        }
    }

    pub fn init_new_game_state() -> GameState {
        let age = get_ages()[0].to_string();
        let mut state = GameState::init(vec![Region::init_with_buildings("Lusitania", vec![get_building("Settlement")])], 1, age);
        state.resources[ResourceKind::Food] = 20;
        state
    }

    #[cfg(test)]
    pub fn init_test_game_state() -> GameState {
        let age = get_ages()[0].to_string();
        let region = vec![
            Region::init_with_buildings("Lusitania", vec![get_building("Test Building"), get_building("Test Building")]),
            Region::init_with_buildings("Illyricum", vec![get_building("Test Gather Hut")]),
        ];
        GameState::init(region, 1, age)
    }

    #[cfg(test)]
    pub fn init_test_empty_game_state() -> GameState {
        GameState::init(vec![], 1, "Archaic".to_string())
    }

    pub fn init_from_json(json: String) -> GameState {
        let state: GameState = serde_json::from_str(&json).unwrap();
        state
    }

    pub fn save(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn buildings(&self) -> Vec<&Building> {
        self.regions.iter().flat_map(|x| &x.buildings).collect()
    }

    pub fn job_count(&self, name: &str) -> u32 {
        match self.jobs.get(&name.to_string()) {
            Some(o) => *o,
            _ => 0,
        }
    }

    pub fn total_jobs_assigned(&self) -> u32 {
        self.jobs.values().sum()
    }

    pub fn conversion_names(&self) -> HashSet<String> {
        self.actions.iter().filter(|x| x.action.is_conversion()).map(|x| x.name.to_string()).collect()
    }

    pub fn action_with_name(&self, name: &str) -> Option<&Waiter> {
        self.actions.iter().filter(|x| x.name == name).nth(0)
    }

    pub fn action_with_name_mut(&mut self, name: &str) -> Option<&mut Waiter> {
        self.actions.iter_mut().filter(|x| x.name == name).nth(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::state::GameState;

    #[test]
    fn serialization() {
        let state = GameState::init_test_game_state();
        let save = state.save();
        let state = GameState::init_from_json(save);
        assert_eq!(2, state.regions.len());
    }

    #[test]
    fn buildings() {
        let state = GameState::init_test_game_state();
        let buildings = state.buildings();
        assert_eq!(3, buildings.len());
        assert_eq!("Test Building", buildings.get(0).unwrap().name);
        assert_eq!("Test Building", buildings.get(1).unwrap().name);
        assert_eq!("Test Gather Hut", buildings.get(2).unwrap().name);
    }
}
