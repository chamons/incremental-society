use crate::resources::*;

use std::collections::BTreeMap;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Building {
    pub name: String,
    pub conversions: Vec<String>,
    pub build_cost: Vec<ResourceAmount>,
    pub storage: Vec<ResourceAmount>,
    pub pops: u32,
    pub immortal: bool,
}

impl Building {
    pub fn init_single(name: &'static str, conversion: &'static str, build_cost: Vec<ResourceAmount>, storage: Vec<ResourceAmount>, pops: u32) -> Building {
        Building {
            name: name.to_owned(),
            conversions: vec![conversion.to_owned()],
            build_cost,
            storage,
            pops,
            immortal: false,
        }
    }

    pub fn init(name: &'static str, conversions: Vec<&'static str>, build_cost: Vec<ResourceAmount>, storage: Vec<ResourceAmount>, pops: u32) -> Building {
        Building {
            name: name.to_owned(),
            conversions: conversions.iter().map(|x| (*x).to_string()).collect(),
            build_cost,
            storage,
            pops,
            immortal: false,
        }
    }

    pub fn details(&self) -> Vec<String> {
        let mut details: Vec<String> = vec![];
        if !self.build_cost.is_empty() {
            details.push(format!("Cost: {}", self.build_cost.iter().format(", ")));
        }
        let mut conversion_count = BTreeMap::new();
        for c in self.conversions.iter() {
            let entry = conversion_count.entry(c).or_insert(0);
            *entry += 1;
        }

        details.push(format!(
            "Provides: {}",
            conversion_count
                .iter()
                .map(|(key, val)| {
                    if *val < 2 {
                        return key.to_string();
                    } else {
                        return format!("{} ({})", key, val);
                    }
                })
                .format(", ")
        ));

        details
    }
}

fn format_details((name, val): (&String, &usize)) -> String {
    if *val < 2 {
        return name.to_string();
    } else {
        return format!("{} ({})", name, val);
    }
}
