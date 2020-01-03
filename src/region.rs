use crate::building::Building;

pub const REGION_BUILDING_COUNT: usize = 2;

#[derive(Debug)]
pub struct Region<'a> {
    pub name: &'a str,
    pub buildings: Vec<Building<'a>>,
}

impl<'a> Region<'a> {
    pub fn init(name: &'a str) -> Region<'a> {
        Region { name, buildings: vec![] }
    }

    pub fn init_with_buildings(name: &'a str, buildings: Vec<Building<'a>>) -> Region<'a> {
        Region { name, buildings }
    }

    pub fn add_building(&mut self, building: Building<'a>) {
        self.buildings.push(building);
    }

    pub fn max_building_count(&self) -> usize {
        REGION_BUILDING_COUNT
    }
}
