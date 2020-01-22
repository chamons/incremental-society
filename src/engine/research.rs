use super::{process, EngineError};
use crate::data;
use crate::state::{DelayedAction, GameState, Research, Waiter};

pub fn can_research(state: &GameState, research: &Research) -> Result<(), EngineError> {
    if state.actions.iter().any(|x| x.action.is_research()) {
        return Err(EngineError::init("Research already in progress"));
    }

    for dep in &research.dependencies {
        if !state.research.contains(dep) {
            return Err(EngineError::init("Unmet dependency for research"));
        }
    }

    for cost in &research.cost {
        if !state.resources.has_amount(&cost) {
            return Err(EngineError::init("Insufficient resources for research"));
        }
    }

    Ok(())
}

pub fn research(state: &mut GameState, research: &Research) -> Result<(), EngineError> {
    can_research(state, research)?;

    state.resources.remove_range(&research.cost);

    let action = Waiter::init_one_shot(
        &format!("Researching {}", research.name)[..],
        data::RESEARCH_LENGTH,
        DelayedAction::Research(research.name.to_string()),
    );
    state.actions.push(action);
    process::recalculate(state);

    Ok(())
}

pub fn apply_research(state: &mut GameState, research: &str) {
    state.research.insert(research.to_owned());
    process::recalculate(state);
}

#[cfg(test)]
mod tests {
    use super::{super::process, *};
    use crate::state::ResourceKind;

    #[test]
    fn research_without_resources() {
        let mut state = process::init_empty_game_state();
        let test_cost_research = data::get_research("TestWithCost");

        assert!(research(&mut state, &test_cost_research).is_err());
        state.resources[ResourceKind::Knowledge] = 10;
        assert!(research(&mut state, &test_cost_research).is_ok());
    }

    #[test]
    fn research_already_in_progress() {
        let mut state = process::init_empty_game_state();
        let nodep_research = data::get_research("TestNoDeps");
        let dep_research = data::get_research("Dep");

        research(&mut state, &nodep_research).unwrap();
        assert!(research(&mut state, &dep_research).is_err());
    }

    #[test]
    fn research_dependency_unmet() {
        let mut state = process::init_empty_game_state();
        let dep_research = data::get_research("TestWithDep");

        assert!(research(&mut state, &dep_research).is_err());
        state.research.insert("Dep".to_owned());
        assert!(research(&mut state, &dep_research).is_ok());
    }

    #[test]
    fn valid_research() {
        let mut state = process::init_empty_game_state();
        let nodep_research = data::get_research("TestNoDeps");

        research(&mut state, &nodep_research).unwrap();

        for _ in 0..data::RESEARCH_LENGTH {
            assert_eq!(0, state.research.len());
            process::process_tick(&mut state);
        }

        assert_eq!(1, state.research.len());
    }
}
