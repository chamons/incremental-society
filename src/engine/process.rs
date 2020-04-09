use std::cmp::min;
use std::collections::{HashMap, HashSet};

use super::DerivedState;
use super::{build, conversions, destroy, edict, research, upgrade};
use crate::state::{DelayedAction, GameState, Region, ResourceKind, ResourceTotal};

pub fn process_tick(state: &mut GameState) -> Option<&'static str> {
    apply_actions(state);
    super::limits::honor_storage_and_floors(state);

    handle_possible_revolt(state);
    None
}

fn handle_possible_revolt(state: &mut GameState) {
    if state.resources[ResourceKind::Instability] > 0 && state.resources[ResourceKind::Instability] == state.derived_state.storage[ResourceKind::Instability] {
        // TODO - Lose Game
    }
}

fn apply_actions(state: &mut GameState) {
    let fired_actions = super::actions::tick_actions(&mut state.actions);
    for action in fired_actions.iter() {
        match action {
            DelayedAction::Edict(name) => edict::apply_edict(state, name),
            DelayedAction::Conversion(name) => {
                // HACK
                let job_count = 1;

                for _ in 0..job_count {
                    conversions::apply_convert(state, name);
                }
            }
            DelayedAction::SustainPops() => sustain_population(state),
            DelayedAction::Build(building, region_index) => build::apply_build(state, building, *region_index),
            DelayedAction::Destroy(region_index, building_index) => destroy::apply_destroy(state, *region_index, *building_index),
            DelayedAction::Research(research) => research::apply_research(state, research),
            DelayedAction::Upgrade(upgrades) => upgrade::apply_upgrade(state, upgrades.iter().map(|x| state.derived_state.find_upgrade(x).clone()).collect()),
        }
    }
}

fn sustain_population(state: &mut GameState) {
    const FOOD_PER_POP: i64 = 5;
    const INSTABILITY_PER_MISSING_FOOD: i64 = 3;

    let required_food = state.pops as i64 * FOOD_PER_POP;
    if state.resources[ResourceKind::Food] >= required_food {
        state.resources.remove(ResourceKind::Food, required_food);
        state
            .resources
            .remove(ResourceKind::Instability, min(state.pops as i64, state.resources[ResourceKind::Instability]));
    } else {
        let missing_food = required_food - state.resources[ResourceKind::Food];
        state.resources.remove(ResourceKind::Food, state.resources[ResourceKind::Food]);
        state.resources.add(ResourceKind::Instability, missing_food * INSTABILITY_PER_MISSING_FOOD);
    }
}

pub fn recalculate(state: &mut GameState) {
    state.derived_state = DerivedState::calculate(&state);
}

use super::data::get_building;

pub fn init_new_game_state() -> GameState {
    let mut state = GameState {
        resources: ResourceTotal::init(),
        regions: vec![Region::init_with_buildings("Lusitania", vec![get_building("Settlement")])],
        actions: vec![],
        pops: 1,
        age: super::data::get_ages()[0].to_string(),
        derived_state: DerivedState::init(),
        research: HashSet::new(),
        upgrades: HashSet::new(),
        jobs: HashMap::new(),
    };
    state.resources[ResourceKind::Food] = 20;

    recalculate(&mut state);
    state
}

#[cfg(test)]
pub fn init_empty_game_state() -> GameState {
    let mut state = GameState {
        resources: ResourceTotal::init(),
        regions: vec![],
        actions: vec![],
        pops: 0,
        age: "Stone".to_string(),
        derived_state: DerivedState::init(),
        research: HashSet::new(),
        upgrades: HashSet::new(),
        jobs: HashMap::new(),
    };
    recalculate(&mut state);
    state
}

#[cfg(test)]
pub fn init_test_game_state() -> GameState {
    let mut state = GameState {
        resources: ResourceTotal::init(),
        regions: vec![
            Region::init_with_buildings("Lusitania", vec![get_building("Test Building"), get_building("Test Building")]),
            Region::init_with_buildings("Illyricum", vec![get_building("Test Gather Hut")]),
        ],
        actions: vec![],
        pops: 1,
        age: super::data::get_ages()[0].to_string(),
        derived_state: DerivedState::init(),
        research: HashSet::new(),
        upgrades: HashSet::new(),
        jobs: HashMap::new(),
    };
    recalculate(&mut state);

    state
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::edict;
    use crate::engine::tests::*;
    use crate::state::{GameState, ResourceKind};

    #[test]
    fn serialization() {
        let state = init_test_game_state();
        let save = state.save();
        let state = GameState::init_from_json(save);
        assert_eq!(2, state.regions.len());
    }

    #[test]
    fn buildings() {
        let state = init_test_game_state();
        let buildings = state.buildings();
        assert_eq!(3, buildings.len());
        assert_eq!("Test Building", buildings.get(0).unwrap().name);
        assert_eq!("Test Building", buildings.get(1).unwrap().name);
        assert_eq!("Test Gather Hut", buildings.get(2).unwrap().name);
    }

    // #[test]
    // fn process_tick_storage_limits_honored() {
    //     let mut state = init_test_game_state();
    //     state.resources[ResourceKind::Food] = state.derived_state.storage[ResourceKind::Food] - 1;
    //     state.resources[ResourceKind::Fuel] = state.derived_state.storage[ResourceKind::Fuel] - 1;
    //     state.action_with_name_mut("TestGather").unwrap().current_tick = 1;
    //     state.action_with_name_mut("TestChop").unwrap().current_tick = 1;

    //     process_tick(&mut state);
    //     assert_eq!(state.resources[ResourceKind::Food], state.derived_state.storage[ResourceKind::Food]);
    //     assert_eq!(state.resources[ResourceKind::Fuel], state.derived_state.storage[ResourceKind::Fuel]);
    // }

    // #[test]
    // fn invoke_takes_times_to_complete() {
    //     let mut state = init_empty_game_state();
    //     state.resources[ResourceKind::Fuel] = 2;
    //     let test_edict = get_test_edict("TestEdict");

    //     edict(&mut state, &test_edict).unwrap();
    //     let edict_length = test_edict.conversion.tick_length();
    //     for _ in 0..edict_length {
    //         assert_eq!(2, state.actions.len());
    //         process_tick(&mut state);
    //     }
    //     assert_eq!(1, state.actions.len());
    // }

    // #[test]
    // fn process_conversions_none_ready() {
    //     let mut state = init_test_game_state();
    //     process_tick(&mut state);
    //     assert_eq!(0, state.resources[ResourceKind::Food]);
    //     assert_eq!(0, state.resources[ResourceKind::Fuel]);
    // }

    // #[test]
    // fn process_conversions_one_ready() {
    //     let mut state = init_test_game_state();
    //     state.action_with_name_mut("TestChop").unwrap().current_tick = 1;
    //     process_tick(&mut state);

    //     assert_eq!(0, state.resources[ResourceKind::Food]);
    //     assert_eq!(4, state.resources[ResourceKind::Fuel]);
    // }

    // #[test]
    // fn process_conversions_many_ready() {
    //     let mut state = init_test_game_state();
    //     state.action_with_name_mut("TestGather").unwrap().current_tick = 1;
    //     state.action_with_name_mut("TestChop").unwrap().current_tick = 1;
    //     process_tick(&mut state);

    //     assert_eq!(1, state.resources[ResourceKind::Food]);
    //     assert_eq!(4, state.resources[ResourceKind::Fuel]);
    // }

    #[test]
    fn sustain_population_with_food() {
        let mut state = init_test_game_state();
        state.resources[ResourceKind::Food] = 30;
        state.resources[ResourceKind::Instability] = 50;
        state.pops = 3;
        sustain_population(&mut state);

        assert_eq!(15, state.resources[ResourceKind::Food]);
        assert!(state.resources[ResourceKind::Instability] < 50);
    }

    #[test]
    fn sustain_population_without_enough_food() {
        let mut state = init_test_game_state();
        state.resources[ResourceKind::Food] = 2;
        sustain_population(&mut state);

        assert_eq!(0, state.resources[ResourceKind::Food]);
        assert!(state.resources[ResourceKind::Instability] > 0);
    }
}
