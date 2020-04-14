use std::cmp::min;
use std::collections::{HashMap, HashSet};

use super::GameContext;
use super::{build, conversions, destroy, edict, research, upgrade};
use crate::state::{DelayedAction, GameState, Region, ResourceKind, ResourceTotal};

pub fn process_tick(context: &mut GameContext) -> Option<&'static str> {
    conversions::start_missing_converts(context);

    apply_actions(context);

    super::limits::honor_storage_and_floors(context);

    handle_possible_revolt(context);
    None
}

fn handle_possible_revolt(context: &mut GameContext) {
    if context.state.resources[ResourceKind::Instability] > 0
        && context.state.resources[ResourceKind::Instability] == context.storage[ResourceKind::Instability]
    {
        // TODO - Lose Game
    }
}

fn apply_actions(context: &mut GameContext) {
    let fired_actions = super::actions::tick_actions(&mut context.state.actions);
    for action in fired_actions.iter() {
        match action {
            DelayedAction::Edict(name) => edict::apply_edict(context, name),
            DelayedAction::Conversion(name) => {
                let job_count = *context.state.jobs.get(name).unwrap_or(&0);
                for _ in 0..job_count {
                    conversions::apply_convert(context, name);
                }
            }
            DelayedAction::SustainPops() => sustain_population(context),
            DelayedAction::Build(building, region_index) => build::apply_build(context, building, *region_index),
            DelayedAction::Destroy(region_index, building_index) => destroy::apply_destroy(context, *region_index, *building_index),
            DelayedAction::Research(research) => research::apply_research(context, research),
            DelayedAction::Upgrade(upgrades) => upgrade::apply_upgrade(context, upgrades.iter().map(|x| context.find_upgrade(x).clone()).collect()),
        }
    }
}

fn sustain_population(context: &mut GameContext) {
    const FOOD_PER_POP: i64 = 5;
    const INSTABILITY_PER_MISSING_FOOD: i64 = 3;
    let state = &mut context.state;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::tests::*;
    use crate::engine::{add_job, edict};
    use crate::state::{GameState, ResourceKind};

    #[test]
    fn process_tick_storage_limits_honored() {
        let mut context = GameContext::init_test_game_context();
        context.state.pops = 2;
        add_job(&mut context, "TestGather").unwrap();
        add_job(&mut context, "TestChop").unwrap();
        context.state.resources[ResourceKind::Food] = context.storage[ResourceKind::Food] - 1;
        context.state.resources[ResourceKind::Fuel] = context.storage[ResourceKind::Fuel] - 1;
        process_tick(&mut context);

        context.state.action_with_name_mut("TestGather").unwrap().current_tick = 1;
        context.state.action_with_name_mut("TestChop").unwrap().current_tick = 1;
        process_tick(&mut context);

        assert_eq!(context.state.resources[ResourceKind::Food], context.storage[ResourceKind::Food]);
        assert_eq!(context.state.resources[ResourceKind::Fuel], context.storage[ResourceKind::Fuel]);
    }

    #[test]
    fn invoke_takes_times_to_complete() {
        let mut context = GameContext::init_empty_test_game_context();
        context.state.resources[ResourceKind::Fuel] = 2;
        let test_edict = get_test_edict("TestEdict");

        edict(&mut context, &test_edict).unwrap();
        let edict_length = test_edict.conversion.tick_length();
        for _ in 0..edict_length {
            assert_eq!(1, context.state.actions.iter().filter(|x| x.action.is_edict()).count());
            process_tick(&mut context);
        }
        assert_eq!(0, context.state.actions.iter().filter(|x| x.action.is_edict()).count());
    }

    #[test]
    fn process_conversions_none_ready() {
        let mut context = GameContext::init_test_game_context();
        process_tick(&mut context);
        assert_eq!(0, context.state.resources[ResourceKind::Food]);
        assert_eq!(0, context.state.resources[ResourceKind::Fuel]);
    }

    #[test]
    fn process_conversions_one_ready() {
        let mut context = GameContext::init_test_game_context();
        context.state.pops = 1;
        add_job(&mut context, "TestChop").unwrap();
        process_tick(&mut context);

        context.state.action_with_name_mut("TestChop").unwrap().current_tick = 1;
        process_tick(&mut context);

        assert_eq!(0, context.state.resources[ResourceKind::Food]);
        assert_eq!(1, context.state.resources[ResourceKind::Fuel]);
    }

    #[test]
    fn process_conversions_many_ready() {
        let mut context = GameContext::init_test_game_context();
        context.state.pops = 3;
        add_job(&mut context, "TestGather").unwrap();
        add_job(&mut context, "TestChop").unwrap();
        add_job(&mut context, "TestChop").unwrap();
        process_tick(&mut context);
        context.state.action_with_name_mut("TestGather").unwrap().current_tick = 1;
        context.state.action_with_name_mut("TestChop").unwrap().current_tick = 1;
        process_tick(&mut context);

        assert_eq!(1, context.state.resources[ResourceKind::Food]);
        assert_eq!(2, context.state.resources[ResourceKind::Fuel]);
    }

    #[test]
    fn sustain_population_with_food() {
        let mut context = GameContext::init_test_game_context();
        context.state.resources[ResourceKind::Food] = 30;
        context.state.resources[ResourceKind::Instability] = 50;
        context.state.pops = 3;
        sustain_population(&mut context);

        assert_eq!(15, context.state.resources[ResourceKind::Food]);
        assert!(context.state.resources[ResourceKind::Instability] < 50);
    }

    #[test]
    fn sustain_population_without_enough_food() {
        let mut context = GameContext::init_test_game_context();
        context.state.resources[ResourceKind::Food] = 2;
        sustain_population(&mut context);

        assert_eq!(0, context.state.resources[ResourceKind::Food]);
        assert!(context.state.resources[ResourceKind::Instability] > 0);
    }
}
