use crate::data::get_conversion;
use crate::state::GameState;

pub fn process_conversions(state: &mut GameState) {
    for c in &state.derived_state.conversion_counts {
        let conversion_length = get_conversion(&c.name).tick_length();

        let entry = state.ticks.entry(c.name.to_string()).or_insert(conversion_length);
        if *entry == 0 {
            *entry = conversion_length;
            let conversion = get_conversion(&c.name);
            for _ in 0..c.count {
                conversion.convert(&mut state.resources);
            }
        } else {
            *entry -= 1;
        }
    }
}

pub fn get_conversion_percentage(state: &GameState, conversion_name: &str) -> Option<f64> {
    match state.ticks.get(conversion_name) {
        Some(x) => {
            let conversion_length = get_conversion(conversion_name).tick_length();
            Some((conversion_length - *x) as f64 / conversion_length as f64)
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::*;

    #[test]
    fn simple_process() {
        let mut state = GameState::init_test_game_state();
        process_conversions(&mut state);

        assert_eq!(0.01, get_conversion_percentage(&state, "TestChop").unwrap());
        assert_eq!(0.01, get_conversion_percentage(&state, "TestGather").unwrap());
    }

    #[test]
    fn get_percentage_with_no_ticks() {
        let state = GameState::init_test_game_state();
        assert!(get_conversion_percentage(&state, "TestChop").is_none());
        assert!(get_conversion_percentage(&state, "TestGather").is_none());
    }

    #[test]
    fn get_non_existent_percentage() {
        let mut state = GameState::init_test_game_state();
        process_conversions(&mut state);

        assert!(get_conversion_percentage(&state, "NonExistentConvert").is_none());
    }

    #[test]
    fn process_conversions_none_ready() {
        let mut state = GameState::init_test_game_state();
        process_conversions(&mut state);
        assert_eq!(0, state.resources[ResourceKind::Food]);
        assert_eq!(0, state.resources[ResourceKind::Fuel]);
    }

    #[test]
    fn process_conversions_one_ready() {
        let mut state = GameState::init_test_game_state();
        *state.ticks.entry("TestChop".to_string()).or_default() = 0;
        process_conversions(&mut state);

        assert_eq!(0, state.resources[ResourceKind::Food]);
        assert_eq!(4, state.resources[ResourceKind::Fuel]);
    }

    #[test]
    fn process_conversions_many_ready() {
        let mut state = GameState::init_test_game_state();
        *state.ticks.entry("TestChop".to_string()).or_default() = 0;
        *state.ticks.entry("TestGather".to_string()).or_default() = 0;
        process_conversions(&mut state);

        assert_eq!(1, state.resources[ResourceKind::Food]);
        assert_eq!(4, state.resources[ResourceKind::Fuel]);
    }
}
