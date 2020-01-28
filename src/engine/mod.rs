mod actions;
mod build;
mod conversions;
mod data;
mod debug;
mod derived_state;
mod destroy;
mod disaster;
mod edict;
mod error;
mod limits;
mod process;
mod research;
mod upgrade;

pub use build::{build, can_build_building, can_build_in_region};
pub use conversions::sync_building_to_conversions;
pub use debug::{complete_actions, die, die_unless, dump_state, load_default_state, max_resources};
pub use derived_state::DerivedState;
pub use destroy::{can_destroy_building, destroy};
pub use edict::{can_invoke_edict, edict};
pub use error::EngineError;
pub use process::{init_new_game_state, process_tick};
pub use research::{can_research, research};
pub use upgrade::{apply_upgrade, upgrade};

#[cfg(test)]
pub mod tests {
    pub use super::data::get_building as get_test_building;
    pub use super::data::get_edict as get_test_edict;
    pub use super::data::get_research as get_test_research;
    pub use super::data::get_upgrade as get_test_upgrade;
    pub use super::process::init_empty_game_state;
    pub use super::process::init_test_game_state;
    pub use super::process::process_tick;
    pub use super::process::recalculate;
}
