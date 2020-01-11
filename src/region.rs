use crate::building::Building;

use serde::{Deserialize, Serialize};

pub const REGION_BUILDING_COUNT: usize = 2;

#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    pub name: String,
    pub buildings: Vec<Building>,
}

impl Region {
    pub fn init(name: &str) -> Region {
        Region {
            name: name.to_string(),
            buildings: vec![],
        }
    }

    pub fn init_with_buildings(name: &str, buildings: Vec<Building>) -> Region {
        Region {
            name: name.to_string(),
            buildings,
        }
    }

    pub fn add_building(&mut self, building: Building) {
        self.buildings.push(building);
    }

    pub fn remove_building(&mut self, index: usize) {
        self.buildings.remove(index);
    }

    pub fn max_building_count(&self) -> usize {
        REGION_BUILDING_COUNT
    }
}
