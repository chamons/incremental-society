use std::collections::BTreeMap;
use std::collections::HashSet;

use super::resources::*;
use super::{check_available, format_resource_list, GameState};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Building {
    pub name: String,
    pub conversions: Vec<String>,
    pub research: HashSet<String>,
    pub build_cost: Vec<ResourceAmount>,
    pub storage: Vec<ResourceAmount>,
    pub housing: u32,
    pub immortal: bool,
}

impl Building {
    pub fn init(name: &'static str) -> Building {
        Building {
            name: name.to_owned(),
            conversions: vec![],
            research: HashSet::new(),
            build_cost: vec![],
            storage: vec![],
            housing: 0,
            immortal: false,
        }
    }

    pub fn with_conversions(mut self, conversions: Vec<&str>) -> Building {
        self.conversions = conversions.iter().map(|x| (*x).to_owned()).collect();
        self
    }

    pub fn with_build_cost(mut self, cost: Vec<ResourceAmount>) -> Building {
        self.build_cost = cost;
        self
    }

    pub fn with_storage(mut self, storage: Vec<ResourceAmount>) -> Building {
        self.storage = storage;
        self
    }

    pub fn with_housing(mut self, housing: u32) -> Building {
        self.housing = housing;
        self
    }

    pub fn with_research(mut self, research: &str) -> Building {
        self.research = HashSet::new();
        self.research.insert(research.to_string());
        self
    }

    pub fn with_many_research(mut self, research: Vec<&str>) -> Building {
        self.research = research.iter().map(|x| (*x).to_owned()).collect();
        self
    }

    pub fn with_immortal(mut self) -> Building {
        self.immortal = true;
        self
    }

    pub fn is_available(&self, state: &GameState) -> bool {
        check_available(&self.research, &state)
    }

    pub fn details(&self) -> Vec<String> {
        let mut details: Vec<String> = vec![];
        if !self.build_cost.is_empty() {
            details.push(format_resource_list("Cost: ", &self.build_cost));
        }

        let mut conversion_count = BTreeMap::new();
        for c in self.conversions.iter() {
            let entry = conversion_count.entry(c).or_insert(0);
            *entry += 1;
        }
        details.push(format!("Provides: {}", conversion_count.iter().map(format_details).format(", ")));

        if !self.storage.is_empty() {
            details.push(format_resource_list("Storage: ", &self.storage));
        }

        if self.housing > 0 {
            details.push(format!("Houses: {}", self.housing));
        }

        details
    }
}

fn format_details((name, val): (&&String, &usize)) -> String {
    if *val < 2 {
        (*name).to_string()
    } else {
        format!("{} ({})", name, val)
    }
}
