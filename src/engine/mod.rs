mod actions;
mod build;
mod conversions;
mod derived_state;
mod destroy;
mod disaster;
mod edict;
mod error;
mod limits;
mod process;
mod research;

pub use build::{build, can_build_building, can_build_in_region};
pub use conversions::sync_building_to_conversions;
pub use derived_state::DerivedState;
pub use destroy::{can_destroy_building, destroy};
pub use edict::{can_invoke_edict, edict};
pub use error::EngineError;
pub use process::{init_new_game_state, process_tick};
pub use research::{can_research, research};

#[cfg(test)]
pub use process::init_empty_game_state;
