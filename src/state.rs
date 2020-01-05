use crate::building::Building;
use crate::data;
use crate::region::Region;
use crate::resources::*;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub resources: ResourceTotal,
    pub regions: Vec<Region>,
    pub ticks: HashMap<String, u32>,
}

pub struct ConversionTotal {
    pub name: String,
    pub count: u32,
}

impl ConversionTotal {
    pub fn init(name: &str, count: u32) -> ConversionTotal {
        ConversionTotal { name: name.to_owned(), count }
    }
}

impl GameState {
    pub fn init() -> GameState {
        GameState {
            resources: ResourceTotal::init(),
            regions: vec![],
            ticks: HashMap::new(),
        }
    }

    pub fn init_new_game_state() -> GameState {
        GameState {
            resources: ResourceTotal::init(),
            regions: vec![
                Region::init_with_buildings("Lusitania", vec![data::get_building("Gathering Camp"), data::get_building("Hunting Grounds")]),
                Region::init("Illyricum"),
            ],
            ticks: HashMap::new(),
        }
    }

    pub fn init_from_json(json: String) -> GameState {
        serde_json::from_str(&json).unwrap()
    }

    pub fn save(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn buildings(&self) -> Vec<&Building> {
        self.regions.iter().flat_map(|x| &x.buildings).collect()
    }

    pub fn conversion_with_counts(&self) -> Vec<ConversionTotal> {
        let mut counts: HashMap<&str, u32> = HashMap::new();
        for c in self.regions.iter().flat_map(|x| &x.buildings).flat_map(|x| &x.conversions) {
            let entry = counts.entry(c).or_insert(0);
            *entry += 1;
        }
        let mut conversion_with_counts = Vec::with_capacity(counts.len());
        for name in self.conversion_names() {
            let count = counts.get::<str>(&name).unwrap();
            conversion_with_counts.push(ConversionTotal::init(&name, *count));
        }
        conversion_with_counts
    }

    pub fn conversion_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self
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

    #[cfg(test)]
    pub fn init_test_game_state() -> GameState {
        GameState {
            resources: ResourceTotal::init(),
            regions: vec![
                Region::init_with_buildings("Lusitania", vec![data::get_building("Test Building"), data::get_building("Test Building")]),
                Region::init_with_buildings("Illyricum", vec![data::get_building("Test Gather Hut")]),
            ],
            ticks: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        let state = GameState::init_test_game_state();
        let save = state.save();
        let state = GameState::init_from_json(save);
        assert_eq!(2, state.regions.len());
    }

    #[test]
    fn conversion_with_counts() {
        let state = GameState::init_test_game_state();
        let conversions = state.conversion_with_counts();
        assert_eq!("TestChop", conversions[0].name);
        assert_eq!(4, conversions[0].count);
        assert_eq!("TestGather", conversions[1].name);
        assert_eq!(1, conversions[1].count);
    }

    #[test]
    fn conversion_names() {
        let state = GameState::init_test_game_state();
        let conversions = state.conversion_names();
        assert_eq!("TestChop", conversions[0]);
        assert_eq!("TestGather", conversions[1]);
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
