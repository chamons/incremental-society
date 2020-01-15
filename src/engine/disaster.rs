use std::cmp;

use super::destroy;
use crate::state::{GameState, ResourceKind, NUM_RESOURCES};

use rand::prelude::*;

fn find_all_vulnerable_building_indexes(state: &GameState) -> Vec<(usize, usize)> {
    let mut building_indexes = vec![];
    for (region_index, region) in state.regions.iter().enumerate() {
        for (building_index, building) in region.buildings.iter().enumerate() {
            // Angry/Starving people don't torch immortal buildings, or those that hold pops/food
            if !building.immortal && building.pops == 0 && building.storage.iter().all(|x| x.kind != ResourceKind::Food) {
                building_indexes.push((region_index, building_index));
            }
        }
    }
    building_indexes
}

fn random_in_overlap_range(rng: &mut ThreadRng, x: usize, y: usize) -> usize {
    if x == y {
        return x;
    }
    rng.gen_range(x, y)
}

pub fn disaster(state: &mut GameState) {
    // At some point this should be a varied list of disaster
    // Today, just wreck buildings and all resources but food
    for i in 0..NUM_RESOURCES {
        state.resources[i] = 0;
    }
    state.resources[ResourceKind::Food] = state.derived_state.storage[ResourceKind::Food] / 2;

    let mut all_buildings = find_all_vulnerable_building_indexes(&state);

    if !all_buildings.is_empty() {
        let mut rng = rand::thread_rng();
        let number_to_destroy = random_in_overlap_range(&mut rng, 1, cmp::min(4, all_buildings.len()));
        for _ in 0..number_to_destroy {
            all_buildings = find_all_vulnerable_building_indexes(&state);
            let index_to_destroy = random_in_overlap_range(&mut rng, 0, all_buildings.len() - 1);

            let (region_index, building_index) = *all_buildings.get(index_to_destroy).unwrap();

            // Ignore buildings unable to be destroy
            let _ = destroy(state, region_index, building_index);
        }
    }

    state.resources[ResourceKind::Instability] = 0;
}

pub fn invoke_disaster_if_needed(state: &mut GameState) -> Option<&'static str> {
    if state.resources[ResourceKind::Instability] > 0 && state.resources[ResourceKind::Instability] == state.derived_state.storage[ResourceKind::Instability] {
        disaster(state);
        return Some("Disaster: People riot due to instability.");
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{super::process, *};
    use crate::data::get_building;
    use crate::state::Region;

    #[test]
    fn invoke_disaster_if_instability_full() {
        let mut state = process::init_test_game_state();
        state.regions[1].add_building(get_building("Stability Building"));
        process::recalculate(&mut state);
        state.resources[ResourceKind::Knowledge] = state.derived_state.storage[ResourceKind::Knowledge];
        state.resources[ResourceKind::Instability] = state.derived_state.storage[ResourceKind::Instability];

        invoke_disaster_if_needed(&mut state).unwrap();

        assert_eq!(0, state.resources[ResourceKind::Knowledge]);
    }

    #[test]
    fn disaster_removes_resources_but_food() {
        let mut state = process::init_empty_game_state();
        state.regions.push(Region::init_with_buildings("Region", vec![get_building("Test Gather Hut")]));
        process::recalculate(&mut state);
        assert_eq!(0, find_all_vulnerable_building_indexes(&state).len());

        state.resources[ResourceKind::Fuel] = 100;

        disaster(&mut state);
        assert_eq!(0, state.resources[ResourceKind::Fuel]);
        assert!(state.resources[ResourceKind::Food] > 0);
    }

    #[test]
    fn disaster_kills_buildings() {
        let mut state = process::init_empty_game_state();
        state.regions.push(Region::init_with_buildings(
            "Region",
            vec![get_building("Empty Building"), get_building("Empty Building"), get_building("Empty Building")],
        ));

        assert_eq!(3, find_all_vulnerable_building_indexes(&state).len());

        for _ in 0..3 {
            disaster(&mut state);
        }
        assert_eq!(0, state.buildings().len());
    }

    #[test]
    fn disaster_kills_buildings_not_immortal() {
        let mut state = process::init_empty_game_state();
        state.regions.push(Region::init_with_buildings("Region", vec![get_building("Test Immortal")]));
        assert_eq!(0, find_all_vulnerable_building_indexes(&state).len());

        disaster(&mut state);
        assert_eq!(1, state.buildings().len());
    }

    #[test]
    fn disaster_resets_instability() {
        let mut state = process::init_empty_game_state();
        state.resources[ResourceKind::Instability] = 100;
        disaster(&mut state);
        assert_eq!(0, state.resources[ResourceKind::Instability]);
    }

    #[test]
    fn random_overlap() {
        let mut rng = rand::thread_rng();

        random_in_overlap_range(&mut rng, 1, 1);
        random_in_overlap_range(&mut rng, 1, 2);
    }
}
