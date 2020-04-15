use super::{EngineError, GameContext};

use crate::state::{DelayedAction, Research, Waiter, RESEARCH_LENGTH};

pub fn can_research(context: &GameContext, research: &Research) -> Result<(), EngineError> {
    if context.state.actions.iter().any(|x| x.action.is_research()) {
        return Err(EngineError::init("Research already in progress"));
    }

    if !research.is_available(&context.state) {
        return Err(EngineError::init("Unmet dependency for research"));
    }

    for cost in &research.cost {
        if !context.state.resources.has_amount(&cost) {
            return Err(EngineError::init("Insufficient resources for research"));
        }
    }

    Ok(())
}

pub fn research(context: &mut GameContext, research: &Research) -> Result<(), EngineError> {
    can_research(context, research)?;

    context.state.resources.remove_range(&research.cost);

    let action = Waiter::init_one_shot(
        &format!("Researching {}", research.name)[..],
        RESEARCH_LENGTH,
        DelayedAction::Research(research.name.to_string()),
    );
    context.state.actions.push(action);
    context.recalculate();

    Ok(())
}

pub fn apply_research(context: &mut GameContext, research: &str) {
    context.state.research.insert(research.to_owned());
    context.recalculate();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::tests::*;
    use crate::engine::process;

    use crate::state::{ResourceKind, RESEARCH_LENGTH};

    #[test]
    fn research_without_resources() {
        let mut context = GameContext::init_empty_test_game_context();
        let test_cost_research = get_test_research("TestWithCost");

        assert!(research(&mut context, &test_cost_research).is_err());
        context.state.resources[ResourceKind::Knowledge] = 10;
        assert!(research(&mut context, &test_cost_research).is_ok());
    }

    #[test]
    fn research_already_in_progress() {
        let mut context = GameContext::init_empty_test_game_context();
        let nodep_research = get_test_research("TestNoDeps");
        let dep_research = get_test_research("Dep");

        research(&mut context, &nodep_research).unwrap();
        assert!(research(&mut context, &dep_research).is_err());
    }

    #[test]
    fn research_dependency_unmet() {
        let mut context = GameContext::init_empty_test_game_context();
        let dep_research = get_test_research("TestWithDep");

        assert!(research(&mut context, &dep_research).is_err());
        context.state.research.insert("Dep".to_owned());
        assert!(research(&mut context, &dep_research).is_ok());
    }

    #[test]
    fn valid_research() {
        let mut context = GameContext::init_empty_test_game_context();
        let nodep_research = get_test_research("TestNoDeps");

        research(&mut context, &nodep_research).unwrap();

        for _ in 0..RESEARCH_LENGTH {
            assert_eq!(0, context.state.research.len());
            process::process_tick(&mut context);
        }

        assert_eq!(1, context.state.research.len());
    }
}
