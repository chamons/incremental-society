use crate::data;
use crate::state::{Building, Edict, GameState, Research};

pub fn available_to_research(state: &GameState) -> Vec<Research> {
    let mut available = vec![];

    for r in crate::state::available_to_research(&state) {
        available.push(data::get_research(&r));
    }

    available
}

pub fn available_to_build(state: &GameState) -> Vec<Building> {
    let mut available = vec![];

    for b in crate::state::available_to_build(&state) {
        available.push(data::get_building(&b));
    }

    available
}

pub fn available_to_invoke(state: &GameState) -> Vec<Edict> {
    let mut available = vec![];

    for e in crate::state::available_to_invoke(&state) {
        available.push(data::get_edict(&e));
    }

    available
}
