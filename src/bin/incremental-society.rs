use incremental_society::*;

extern crate incremental_society;

fn main() {
    let mut state = state::GameState::init();
    state.process_tick();
}
