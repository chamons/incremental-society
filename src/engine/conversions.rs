use crate::state::GameState;

pub fn apply_convert(state: &mut GameState, name: &str) {
    state.derived_state.find_conversion(name).convert(&mut state.resources);
}
