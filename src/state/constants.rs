use super::{ResourceKind, ResourceQuantity};

pub const BUILD_LENGTH: u32 = 30 * 8;
pub const UPGRADE_LENGTH: u32 = 30 * 12;
pub const SUSTAIN_POP_DURATION: u32 = 80;
pub const DESTROY_LENGTH: u32 = 30 * 5;
pub const REGION_BUILDING_COUNT: usize = 2;
pub const RESEARCH_LENGTH: u32 = 30 * 8;

pub const SHORT_CONVERSION: u32 = 50;
pub const MEDIUM_CONVERSION: u32 = 100;
pub const LONG_CONVERSION: u32 = 150;
pub const EPIC_CONVERSION: u32 = 300;

// Use tuple instead of ResourceAmount as structs not allows constants
pub const COST_PER_UPGRADE: [(ResourceKind, ResourceQuantity); 1] = [(ResourceKind::Knowledge, 25)];
pub const MAX_UPGRADES: usize = 2;
