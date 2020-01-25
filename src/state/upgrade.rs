use std::collections::HashSet;

use super::{check_available, ConversionLength, GameState, ResourceAmount};

#[derive(Debug, Clone)]
pub enum UpgradeActions {
    AddBuildingPops(u32),
    AddBuildingConversion(String),
    AddBuildingStorage(ResourceAmount),
    ChangeEdictLength(ConversionLength),
}

#[derive(Debug, Clone)]
pub struct Upgrade {
    pub name: String,
    pub upgrades: Vec<UpgradeActions>,
    pub items_upgraded: Vec<String>,
    pub research: HashSet<String>,
}

impl Upgrade {
    pub fn is_available(&self, state: &GameState) -> bool {
        check_available(&self.research, &state)
    }

    pub fn init(name: &str, upgrades: Vec<UpgradeActions>, items_upgraded: Vec<String>) -> Upgrade {
        Upgrade {
            name: name.to_owned(),
            upgrades,
            items_upgraded,
            research: HashSet::new(),
        }
    }

    pub fn with_research(mut self, research: Vec<&str>) -> Upgrade {
        self.research = research.iter().map(|x| (*x).to_owned()).collect();
        self
    }
}
