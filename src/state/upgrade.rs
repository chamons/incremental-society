use super::{ConversionLength, ResourceAmount};

#[derive(Debug, Clone)]
pub enum UpgradeActions {
    AddBuildingPops(u32),
    AddBuildingConversion(String),
    AddBuildingStorage(ResourceAmount),
    ChangeEdictLength(ConversionLength),
}
