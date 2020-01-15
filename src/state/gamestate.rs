use super::actions::Waiter;
use super::building::Building;
use super::region::Region;
use super::resources::*;

use crate::data;

// TODO - BAD
use crate::engine::DerivedState;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub resources: ResourceTotal,
    pub regions: Vec<Region>,
    pub actions: Vec<Waiter>,

    #[serde(skip)]
    #[serde(default = "DerivedState::init")]
    pub derived_state: DerivedState,
}

impl GameState {
    pub fn init() -> GameState {
        let mut state = GameState {
            resources: ResourceTotal::init(),
            regions: vec![],
            actions: vec![],
            derived_state: DerivedState::init(),
        };
        state.recalculate();
        state
    }

    pub fn init_new_game_state() -> GameState {
        let mut state = GameState {
            resources: ResourceTotal::init(),
            regions: vec![
                Region::init_with_buildings("Lusitania", vec![data::get_building("Settlement"), data::get_building("Hunting Grounds")]),
                Region::init("Illyricum"),
            ],
            actions: vec![],
            derived_state: DerivedState::init(),
        };
        state.recalculate();
        state
    }

    #[cfg(test)]
    pub fn init_test_game_state() -> GameState {
        let mut state = GameState {
            resources: ResourceTotal::init(),
            regions: vec![
                Region::init_with_buildings("Lusitania", vec![data::get_building("Test Building"), data::get_building("Test Building")]),
                Region::init_with_buildings("Illyricum", vec![data::get_building("Test Gather Hut")]),
            ],
            actions: vec![],
            derived_state: DerivedState::init(),
        };
        state.recalculate();
        state
    }

    pub fn init_from_json(json: String) -> GameState {
        let mut state: GameState = serde_json::from_str(&json).unwrap();
        state.recalculate();
        state
    }

    pub fn save(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn buildings(&self) -> Vec<&Building> {
        self.regions.iter().flat_map(|x| &x.buildings).collect()
    }

    pub fn action_with_name(&self, name: &str) -> Option<&Waiter> {
        self.actions.iter().filter(|x| x.name == name).nth(0)
    }

    pub fn action_with_name_mut(&mut self, name: &str) -> Option<&mut Waiter> {
        self.actions.iter_mut().filter(|x| x.name == name).nth(0)
    }

    // Despite state.rs being a state and not engine component, we have one lie here
    // We must recalculate some state here
    pub fn recalculate(&mut self) {
        self.derived_state = DerivedState::calculate(&self);
        // See sync_building_to_conversions for the story on why we're doing this :(
        crate::engine::sync_building_to_conversions(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        let state = GameState::init_test_game_state();
        let save = state.save();
        let state = GameState::init_from_json(save);
        assert_eq!(2, state.regions.len());
    }

    #[test]
    fn buildings() {
        let state = GameState::init_test_game_state();
        let buildings = state.buildings();
        assert_eq!(3, buildings.len());
        assert_eq!("Test Building", buildings.get(0).unwrap().name);
        assert_eq!("Test Building", buildings.get(1).unwrap().name);
        assert_eq!("Test Gather Hut", buildings.get(2).unwrap().name);
    }
}
