use crate::building::Building;
use crate::data;
use crate::region::Region;
use crate::resources::*;

use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GameState {
    pub resources: ResourceTotal,
    pub regions: Vec<Region>,
    pub ticks: HashMap<&'static str, u32>,
}

pub struct ConversionTotal {
    pub name: &'static str,
    pub count: u32,
}

impl ConversionTotal {
    pub fn init(name: &'static str, count: u32) -> ConversionTotal {
        ConversionTotal { name, count }
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

    pub fn buildings(&self) -> Vec<&Building> {
        self.regions.iter().flat_map(|x| &x.buildings).collect()
    }

    pub fn conversion_with_counts(&self) -> Vec<ConversionTotal> {
        let mut counts = HashMap::new();
        for c in self.regions.iter().flat_map(|x| &x.buildings).flat_map(|x| &x.conversions) {
            let entry = counts.entry(c).or_insert(0);
            *entry += 1;
        }
        let mut conversion_with_counts = Vec::with_capacity(counts.len());
        for name in self.conversion_names() {
            conversion_with_counts.push(ConversionTotal::init(name, *counts.get(&name).unwrap()));
        }
        conversion_with_counts
    }

    pub fn conversion_names(&self) -> Vec<&'static str> {
        let mut names: Vec<&'static str> = self
            .regions
            .iter()
            .flat_map(|x| &x.buildings)
            .flat_map(|x| &x.conversions)
            .unique()
            .copied()
            .collect();
        names.sort();
        names
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn conversion_with_counts() {
        let state = init_test_game_state();
        let conversions = state.conversion_with_counts();
        assert_eq!("TestEmptyConvert", conversions[0].name);
        assert_eq!(4, conversions[0].count);
        assert_eq!("TestGather", conversions[1].name);
        assert_eq!(1, conversions[1].count);
    }

    #[test]
    fn conversion_names() {
        let state = init_test_game_state();
        let conversions = state.conversion_names();
        assert_eq!("TestEmptyConvert", conversions[0]);
        assert_eq!("TestGather", conversions[1]);
    }

    #[test]
    fn buildings() {
        let state = init_test_game_state();
        let buildings = state.buildings();
        assert_eq!(3, buildings.len());
        assert_eq!("Test Building", buildings.get(0).unwrap().name);
        assert_eq!("Test Building", buildings.get(1).unwrap().name);
        assert_eq!("Test Gather Hut", buildings.get(2).unwrap().name);
    }
}
