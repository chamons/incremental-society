pub use super::upgrade;
pub use crate::state::{Building, Conversion, Edict, GameState, Research, ResourceTotal, Upgrade};

#[derive(Debug)]
pub struct UpgradeState {
    pub available_buildings: Vec<Building>,
    pub available_edicts: Vec<Edict>,
    pub available_research: Vec<Research>,
    pub available_upgrade: Vec<Upgrade>,
    pub available_conversions: Vec<Conversion>,
    pub stability_gain: u32,
}

impl UpgradeState {
    pub fn init() -> UpgradeState {
        UpgradeState {
            available_buildings: vec![],
            available_edicts: vec![],
            available_research: vec![],
            available_upgrade: vec![],
            available_conversions: vec![],
            stability_gain: 0,
        }
    }

    pub fn calculate(state: &GameState) -> UpgradeState {
        UpgradeState {
            available_buildings: upgrade::available_to_build(state),
            available_edicts: upgrade::available_to_invoke(state),
            available_research: upgrade::available_to_research(state),
            available_upgrade: upgrade::available_to_upgrade(state),
            available_conversions: upgrade::current_conversions(state),
            stability_gain: upgrade::get_current_stability_gain(state),
        }
    }

    pub fn find_building(&self, name: &str) -> Building {
        self.available_buildings.iter().filter(|x| x.name == name).nth(0).unwrap().clone()
    }

    pub fn find_edict(&self, name: &str) -> Edict {
        self.available_edicts.iter().filter(|x| x.name == name).nth(0).unwrap().clone()
    }

    pub fn find_research(&self, name: &str) -> Research {
        self.available_research.iter().filter(|x| x.name == name).nth(0).unwrap().clone()
    }

    pub fn find_upgrade(&self, name: &str) -> Upgrade {
        self.available_upgrade.iter().filter(|x| x.name == name).nth(0).unwrap().clone()
    }

    pub fn find_conversion(&self, name: &str) -> Conversion {
        self.available_conversions.iter().filter(|x| x.name == name).nth(0).unwrap().clone()
    }
}
