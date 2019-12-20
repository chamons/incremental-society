pub mod resources;
pub use self::resources::*;

pub mod state;
pub use self::state::*;

pub mod conversion;
pub use self::conversion::*;

pub fn process_tick(state: &mut GameState) {
    for conversion in state.conversions.iter() {
        if conversion.has_input(&state.resources) {
            state.resources.combine(&conversion.total());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_conversion<'a>() -> Conversion<'a> {
        Conversion::init_single(
            "TestConversion",
            ResourceAmount::init(ResourceKind::Food, 10),
            ResourceAmount::init(ResourceKind::Fuel, 10),
        )
    }

    #[test]
    fn process_tick_has_conversion_input() {
        let mut state = GameState::init();
        state.resources[ResourceKind::Food] = 10;
        state.conversions.push(create_test_conversion());

        process_tick(&mut state);

        assert_eq!(0, state.resources[ResourceKind::Food]);
        assert_eq!(10, state.resources[ResourceKind::Fuel]);
    }

    #[test]
    fn process_tick_no_conversion_input() {
        let mut state = GameState::init();
        state.resources[ResourceKind::Food] = 0;
        state.conversions.push(create_test_conversion());

        process_tick(&mut state);

        assert_eq!(0, state.resources[ResourceKind::Food]);
        assert_eq!(0, state.resources[ResourceKind::Fuel]);
    }
}
