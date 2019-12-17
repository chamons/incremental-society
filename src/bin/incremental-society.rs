use incremental_society::*;

extern crate incremental_society;

fn main() {
    let mut state = GameState::init();
    process_tick(&mut state);
}
