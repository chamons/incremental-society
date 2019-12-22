use crate::buildings::*;
use crate::resources::*;

#[derive(Debug)]
pub struct Region<'a> {
    pub name: &'a str,
    pub buildings: Vec<Building<'a>>,
}

impl<'a> Region<'a> {
    pub fn init(name: &'a str, buildings: Vec<Building<'a>>) -> Region<'a> {
        Region { name, buildings }
    }

    pub fn process_tick(&mut self, resources: &mut ResourceTotal) {
        for b in &mut self.buildings {
            b.process_tick(resources);
        }
    }

    pub fn add_building(&mut self, building: Building<'a>) {
        self.buildings.push(building);
    }
}
