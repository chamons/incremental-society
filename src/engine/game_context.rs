use super::DerivedState;
use crate::state::GameState;

// Game Context is the full picture of the current state of the game, including:
//
// GameState - The state of the game that we would serialize to disk on save.
//   - Example - Technologies researched and buildings built
// DerivedState - The information calculated from the GameState, can go out of sync
//   - Example - The current storage amount, calculated by summing all buildings in all regions
// RNG - A RNG source for making decisions in game engine
//  - Ideally part of game state possibly. See https://github.com/chamons/incremental-society/issues/100 for details
#[derive(Debug)]
pub struct GameContext {
    pub state: GameState,
    pub derived_state: DerivedState,
}

impl GameContext {
    pub fn init_new_game_context() -> GameContext {
        let state = GameState::init_new_game_state();
        GameContext {
            state,
            derived_state: DerivedState::calculate(&state),
        }
    }

    #[cfg(test)]
    pub fn init_test_game_context() -> GameContext {
        let state = GameState::init_test_game_state();
        GameContext {
            state,
            derived_state: DerivedState::calculate(&state),
        }
    }

    #[cfg(test)]
    pub fn init_empty_test_game_context() -> GameContext {
        let state = GameState::init_test_empty_game_state();
        GameContext {
            state,
            derived_state: DerivedState::calculate(&state),
        }
    }

    pub fn recalculate(&mut self) {
        self.derived_state = DerivedState::calculate(&self.state);
    }
}
