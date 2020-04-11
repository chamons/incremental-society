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

    #[serde(skip)]
    #[serde(default = "crate::engine::DerivedState::init")]
    pub derived_state: crate::engine::DerivedState,
}

impl GameState {
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
