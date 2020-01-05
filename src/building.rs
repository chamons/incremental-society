use crate::resources::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Building {
    pub name: String,
    pub conversions: Vec<String>,
    pub build_cost: Vec<ResourceAmount>,
}

impl Building {
    pub fn init_single(name: &'static str, conversion: &'static str, build_cost: Vec<ResourceAmount>) -> Building {
        Building {
            name: name.to_owned(),
            conversions: vec![conversion.to_owned()],
            build_cost,
        }
    }

    pub fn init(name: &'static str, conversions: Vec<&'static str>, build_cost: Vec<ResourceAmount>) -> Building {
        Building {
            name: name.to_owned(),
            conversions: conversions.iter().map(|x| x.to_string()).collect(),
            build_cost,
        }
    }
}

impl<'a> Clone for Building {
    fn clone(&self) -> Self {
        Building {
            name: self.name.to_string(),
            conversions: self.conversions.clone(),
            build_cost: self.build_cost.clone(),
        }
    }
}
