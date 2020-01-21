use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use super::process::init_new_game_state;
use crate::state::{GameState, ResourceKind, NUM_RESOURCES};

use dirs::home_dir;

fn get_home_dir_file_path(file_name: &str) -> PathBuf {
    let home = PathBuf::from(home_dir().unwrap_or(PathBuf::from("")));
    let file = PathBuf::from(file_name);
    home.join(file)
}

fn write_debug_info(file_name: &str, contents: String) {
    let file = File::create(get_home_dir_file_path(file_name));
    if let Ok(mut file) = file {
        let _ = file.write_all(contents.as_bytes());
    }
}

pub fn dump_state(state: &GameState) {
    write_debug_info("incremental-society-state.txt", state.save());
}

pub fn load_default_state(state: &mut GameState) {
    *state = init_new_game_state();
}

pub fn max_resources(state: &mut GameState) {
    for i in 0..NUM_RESOURCES {
        state.resources[i] = state.derived_state.storage[i];
    }
    state.resources[ResourceKind::Instability] = 0;
}

pub fn complete_actions(state: &mut GameState) {
    for w in &mut state.actions {
        w.current_tick = 1;
    }
}

pub fn die_unless<T: AsRef<str>>(condition: bool, message: &T) {
    if !condition {
        panic!(format!("Debug Assert: {}", message.as_ref()));
    }
}
