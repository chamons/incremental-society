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
}
