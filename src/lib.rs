pub mod resources;
pub use self::resources::*;

pub mod state;
pub use self::state::*;

pub mod conversion;
pub use self::conversion::*;

pub fn process_tick(state: &mut GameState) {
    for conversion in state.conversions.iter() {
        conversion.convert(&mut state.resources);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
