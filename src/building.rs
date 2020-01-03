use crate::resources::*;

#[derive(Debug)]
pub struct Building<'a> {
    pub name: &'a str,
    pub conversions: Vec<&'a str>,
    pub build_cost: Vec<ResourceAmount>,
}

impl<'a> Building<'a> {
    pub fn init_single(name: &'a str, conversion: &'a str, build_cost: Vec<ResourceAmount>) -> Building<'a> {
        Building::init(name, vec![conversion], build_cost)
    }

    pub fn init(name: &'a str, conversions: Vec<&'a str>, build_cost: Vec<ResourceAmount>) -> Building<'a> {
        Building { name, conversions, build_cost }
    }
}

impl<'a> Clone for Building<'a> {
    fn clone(&self) -> Self {
        Building::init(self.name, self.conversions.clone(), self.build_cost.clone())
    }
}
