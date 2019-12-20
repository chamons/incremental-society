pub mod resources;
pub use self::resources::*;

pub mod state;
pub use self::state::*;

pub mod conversion;
pub use self::conversion::*;

pub mod buildings;
pub use self::buildings::*;

pub mod regions;
pub use self::regions::*;

pub fn process_tick(state: &mut GameState) {
    let mut resources = state.resources.clone();

    for conversion in state.conversions() {
        conversion.convert(&mut resources);
    }

    state.resources = resources;
}

#[cfg(test)]
mod tests {
    use super::*;
}
