use crate::building::Building;

pub const REGION_BUILDING_COUNT: usize = 2;

#[derive(Debug)]
pub struct Region {
    pub name: &'static str,
    pub buildings: Vec<Building>,
}

impl Region {
    pub fn init(name: &'static str) -> Region {
        Region { name, buildings: vec![] }
    }

    pub fn init_with_buildings(name: &'static str, buildings: Vec<Building>) -> Region {
        Region { name, buildings }
    }

    pub fn add_building(&mut self, building: Building) {
        self.buildings.push(building);
    }

    pub fn max_building_count(&self) -> usize {
        REGION_BUILDING_COUNT
    }
}
