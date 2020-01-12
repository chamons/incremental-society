use crate::data::get_conversion;
use crate::engine::conversions::single_convert;
use crate::state::{DelayedAction, GameState};

pub fn add_delayed(state: &mut GameState, name: &str, length: u32, action: DelayedAction) {
    assert!(!state.ticks.contains_key(name));
    state.ticks.insert(name.to_string(), length);

    assert!(!state.actions.contains_key(name));
    state.actions.insert(name.to_string(), action);
}

pub fn process_delayed_tick(state: &mut GameState) {
    for a in get_ready_actions(state).iter() {
        state.ticks.remove_entry(a);
        let conversion = get_conversion(a);
        single_convert(state, &conversion);
    }
}

pub fn get_ready_actions(state: &mut GameState) -> Vec<String> {
    let mut ready = vec![];

    for name in state.actions.keys() {
        assert!(state.ticks.contains_key(name));
        let entry = state.ticks.entry(name.to_string()).or_default();
        if *entry == 0 {
            ready.push(name.to_string());
        } else {
            *entry -= 1;
        }
    }

    ready
}

// Right now no way to get starting length of delayed action
// Remove sustain hack from derived state and add as a delayed recuring action?
// Update the conversion listing code to also look here?
