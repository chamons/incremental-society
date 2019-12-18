pub mod state;
pub use self::state::*;

pub fn process_tick(state: &mut GameState) {
    for conversion in state.conversions.iter() {
        if conversion.has_input(state) {
            println!("Can Convert: {}", conversion.name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_tick_smoke() {
        let mut state = GameState::init();
        process_tick(&mut state);
    }
}
