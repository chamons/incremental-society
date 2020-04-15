use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use super::game_context::GameContext;
use crate::state::{GameState, ResourceKind, NUM_RESOURCES};

use dirs::home_dir;

fn get_home_dir_file_path(file_name: &str) -> PathBuf {
    let home = home_dir().unwrap_or_default();
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

pub fn load_default_state() -> GameContext {
    GameContext::init_new_game_context()
}

pub fn max_resources(context: &mut GameContext) {
    for i in 0..NUM_RESOURCES {
        context.state.resources[i] = context.storage[i];
    }
    context.state.resources[ResourceKind::Instability] = 0;
}

pub fn complete_actions(state: &mut GameState) {
    for w in &mut state.actions {
        w.current_tick = 1;
    }
}

pub fn die_unless<T: AsRef<str>>(condition: bool, message: &T) {
    if !condition {
        die(message);
    }
}
pub fn die<T: AsRef<str>>(message: &T) {
    panic!(format!("Debug Assert: {}", message.as_ref()));
}
