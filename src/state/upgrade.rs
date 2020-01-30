use std::collections::HashSet;

use itertools::Itertools;

use super::{check_available, ConversionLength, GameState, ResourceAmount};

#[derive(Debug, Clone)]
pub enum UpgradeActions {
    AddBuildingPops(u32),
    AddBuildingConversion(String),
    AddBuildingStorage(ResourceAmount),
    ChangeEdictLength(ConversionLength),
    ChangeConversionLength(ConversionLength),
    ChangeConversionInput(ResourceAmount),
    ChangeConversionOutput(ResourceAmount),
}

impl UpgradeActions {
    pub fn details(&self) -> String {
        match self {
            UpgradeActions::AddBuildingPops(pops) => format!("Adds {} population capacity", pops),
            UpgradeActions::AddBuildingConversion(name) => format!("Adds {} conversion to building", name),
            UpgradeActions::AddBuildingStorage(storage) => format!("Adds {:?} storage", storage),
            UpgradeActions::ChangeEdictLength(length) => format!("Changes edict length to {:?}", length),
            UpgradeActions::ChangeConversionLength(length) => format!("Changes conversion length to {:?}", length),
            UpgradeActions::ChangeConversionInput(input) => format!("Adds {:?} to required conversion input", input),
            UpgradeActions::ChangeConversionOutput(output) => format!("Adds {:?} to required conversion output", output),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Upgrade {
    pub name: String,
    pub upgrades: Vec<UpgradeActions>,
    pub items_upgraded: Vec<String>,
    pub research: HashSet<String>,
    pub cost: Vec<ResourceAmount>,
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
            cost: vec![],
        }
    }

    pub fn with_research(mut self, research: Vec<&str>) -> Upgrade {
        self.research = research.iter().map(|x| (*x).to_owned()).collect();
        self
    }

    pub fn with_cost(mut self, cost: Vec<ResourceAmount>) -> Upgrade {
        self.cost = cost;
        self
    }

    pub fn details(&self) -> Vec<String> {
        let mut details: Vec<String> = vec![];

        if !self.items_upgraded.is_empty() {
            details.push(format!("Upgrades: {}", self.items_upgraded.iter().format(", ")));
        }

        for u in self.upgrades.iter() {
            details.push(u.details());
        }

        if !self.upgrades.is_empty() {
            details.push(format!("{}", self.upgrades.iter().map(|x| x.details()).format("")));
        }

        if !self.research.is_empty() {
            details.push(format!("Requires Research: {}", self.research.iter().format(", ")));
        }

        details
    }
}
