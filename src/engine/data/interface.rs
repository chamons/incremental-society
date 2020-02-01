use super::{BUILDINGS, CONVERSIONS, EDICTS, RESEARCH, UPGRADE};
use crate::state::{Building, Conversion, Edict, Research, Upgrade};

pub fn get_conversion(name: &str) -> Conversion {
    CONVERSIONS[name].clone()
}

pub fn get_conversion_names() -> Vec<String> {
    CONVERSIONS.keys().map(|x| (*x).to_string()).collect()
}

pub fn get_building(name: &str) -> Building {
    BUILDINGS[name].clone()
}

pub fn get_building_names() -> Vec<String> {
    BUILDINGS.keys().map(|x| (*x).to_string()).collect()
}

pub fn get_edict(name: &str) -> Edict {
    EDICTS[name].clone()
}

pub fn get_edict_names() -> Vec<String> {
    EDICTS.keys().map(|x| (*x).to_string()).collect()
}

pub fn get_research(name: &str) -> Research {
    RESEARCH[name].clone()
}

pub fn get_research_names() -> Vec<String> {
    RESEARCH.keys().map(|x| (*x).to_string()).collect()
}

pub fn get_upgrade(name: &str) -> Upgrade {
    UPGRADE[name].clone()
}

pub fn get_upgrade_names() -> Vec<String> {
    UPGRADE.keys().map(|x| (*x).to_string()).collect()
}
