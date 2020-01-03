use crate::building::Building;
use crate::data;
use crate::region::Region;
use crate::resources::*;

use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GameState<'a> {
    pub resources: ResourceTotal,
    pub regions: Vec<Region<'a>>,
    pub ticks: HashMap<&'a str, u32>,
}

impl<'a> GameState<'a> {
    pub fn init() -> GameState<'a> {
        GameState {
            resources: ResourceTotal::init(),
            regions: vec![
                Region::init_with_buildings("Lusitania", vec![data::get_building("Gathering Camp"), data::get_building("Hunting Grounds")]),
                Region::init("Illyricum"),
            ],
            ticks: HashMap::new(),
        }
    }

    pub fn buildings(&self) -> Vec<&Building<'a>> {
        self.regions.iter().flat_map(|x| &x.buildings).collect()
    }

    pub fn conversion_with_counts(&self) -> Vec<(&'a str, u32)> {
        let mut conversion_count = HashMap::new();
        for c in self.regions.iter().flat_map(|x| &x.buildings).flat_map(|x| &x.conversions) {
            let entry = conversion_count.entry(c).or_insert(0);
            *entry += 1;
        }
        conversion_count.iter().map(|x| (**x.0, *x.1)).collect()
    }

    pub fn conversion_names(&self) -> Vec<&'a str> {
        self.regions
            .iter()
            .flat_map(|x| &x.buildings)
            .flat_map(|x| &x.conversions)
            .unique()
            .copied()
            .collect()
    }
}
