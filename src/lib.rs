pub mod state;
pub use self::state::*;

pub fn process_tick(state: &mut GameState) {
    for conversion in state.conversions.iter() {
        let has_input = conversion
            .input
            .iter()
            .filter_map(|e| e.as_ref())
            .all(|x| state.has(x.resource, x.amount));
        if has_input {
            println!("Can Convert: {}", conversion.name);
        }
    }
}
