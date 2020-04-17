use rand::{rngs::SmallRng, Rng, SeedableRng};

use std::collections::HashMap;

use super::UpgradeState;
use crate::state::{Building, Conversion, Edict, GameState, Research, ResourceTotal, Upgrade};

// Game Context is the full picture of the current state of the game, including:
//
// GameState - The state of the game that we would serialize to disk on save.
//   - Example - Technologies researched and buildings built
// UpgradeState - The current building/edict/etc based upon researched tech/upgrades
// storage/current_building_jobs - Other calculated state

#[derive(Debug)]
pub struct GameContext {
    pub state: GameState,
    upgrade_state: UpgradeState,
    pub current_building_jobs: HashMap<String, u32>,
    pub storage: ResourceTotal,
    rand: SmallRng,
    pub is_lost: bool,
}

impl GameContext {
    pub fn init_from_state(state: GameState, rand: SmallRng) -> GameContext {
        let upgrade_state = UpgradeState::calculate(&state);
        let current_building_jobs = GameContext::jobs_with_counts(&state);
        let storage = GameContext::calculate_storage(&state);

        GameContext {
            state,
            upgrade_state,
            current_building_jobs,
            storage,
            rand,
            is_lost: false,
        }
    }

    pub fn init_new_game_context() -> GameContext {
        let state = GameState::init_new_game_state();
        GameContext::init_from_state(state, SmallRng::from_entropy())
    }

    #[cfg(test)]
    pub fn init_test_game_context() -> GameContext {
        let state = GameState::init_test_game_state();
        GameContext::init_from_state(state, SmallRng::from_seed([42; 16]))
    }

    #[cfg(test)]
    pub fn init_empty_test_game_context() -> GameContext {
        let state = GameState::init_test_empty_game_state();
        GameContext::init_from_state(state, SmallRng::from_seed([42; 16]))
    }

    pub fn recalculate(&mut self) {
        self.upgrade_state = UpgradeState::calculate(&self.state);
        self.current_building_jobs = GameContext::jobs_with_counts(&self.state);
        self.storage = GameContext::calculate_storage(&self.state);
    }

    pub fn random(&mut self, lower: f32, upper: f32) -> f32 {
        self.rand.gen_range(lower, upper)
    }

    fn jobs_with_counts(state: &GameState) -> HashMap<String, u32> {
        let mut counts = HashMap::new();
        for c in state.regions.iter().flat_map(|x| &x.buildings).flat_map(|x| &x.jobs) {
            let entry = counts.entry(c.to_string()).or_insert(0);
            *entry += 1;
        }
        counts
    }

    fn calculate_storage(state: &GameState) -> ResourceTotal {
        let mut storage = ResourceTotal::init();
        for building in state.buildings() {
            for resource in &building.storage {
                storage.add(resource.kind, resource.amount);
            }
        }
        storage
    }

    pub fn find_building(&self, name: &str) -> Building {
        self.upgrade_state.find_building(name)
    }

    pub fn find_edict(&self, name: &str) -> Edict {
        self.upgrade_state.find_edict(name)
    }

    pub fn find_research(&self, name: &str) -> Research {
        self.upgrade_state.find_research(name)
    }

    pub fn find_upgrade(&self, name: &str) -> Upgrade {
        self.upgrade_state.find_upgrade(name)
    }

    pub fn find_conversion(&self, name: &str) -> Conversion {
        self.upgrade_state.find_conversion(name)
    }

    pub fn get_available_buildings(&self) -> &Vec<Building> {
        &self.upgrade_state.available_buildings
    }

    pub fn get_available_edicts(&self) -> &Vec<Edict> {
        &self.upgrade_state.available_edicts
    }

    pub fn get_available_research(&self) -> &Vec<Research> {
        &self.upgrade_state.available_research
    }

    pub fn get_available_upgrade(&self) -> &Vec<Upgrade> {
        &self.upgrade_state.available_upgrade
    }

    pub fn get_available_conversions(&self) -> &Vec<Conversion> {
        &self.upgrade_state.available_conversions
    }
}
