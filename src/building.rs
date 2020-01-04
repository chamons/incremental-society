use crate::resources::*;

#[derive(Debug)]
pub struct Building {
    pub name: &'static str,
    pub conversions: Vec<&'static str>,
    pub build_cost: Vec<ResourceAmount>,
}

impl Building {
    pub fn init_single(name: &'static str, conversion: &'static str, build_cost: Vec<ResourceAmount>) -> Building {
        Building {
            name: name,
            conversions: vec![conversion],
            build_cost,
        }
    }

    pub fn init(name: &'static str, conversions: Vec<&'static str>, build_cost: Vec<ResourceAmount>) -> Building {
        Building {
            name: name,
            conversions,
            build_cost,
        }
    }
}

impl<'a> Clone for Building {
    fn clone(&self) -> Self {
        Building::init(&self.name, self.conversions.clone(), self.build_cost.clone())
    }
}
