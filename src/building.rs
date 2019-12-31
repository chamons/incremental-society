use crate::conversion::Conversion;
use crate::resources::*;

#[derive(Debug)]
pub struct Building<'a> {
    pub name: &'a str,
    pub conversions: Vec<Conversion<'a>>,
    pub build_cost: Vec<ResourceAmount>,
}

impl<'a> Building<'a> {
    pub fn init_single(name: &'a str, conversion: Conversion<'a>, build_cost: Vec<ResourceAmount>) -> Building<'a> {
        Building::init(name, vec![conversion], build_cost)
    }

    pub fn init(name: &'a str, conversions: Vec<Conversion<'a>>, build_cost: Vec<ResourceAmount>) -> Building<'a> {
        Building { name, conversions, build_cost }
    }

    pub fn process_tick(&mut self, resources: &mut ResourceTotal) {
        for c in &mut self.conversions {
            c.process_tick(resources);
        }
    }
}

impl<'a> Clone for Building<'a> {
    fn clone(&self) -> Self {
        Building::init(self.name, self.conversions.clone(), self.build_cost.clone())
    }
}
