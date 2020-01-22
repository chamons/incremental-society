use crate::engine::data::{get_building, get_building_names, get_edict, get_edict_names, get_research, get_research_names};
use crate::state::{Building, Edict, GameState, Research};

pub fn available_to_research(state: &GameState) -> Vec<Research> {
    let mut available = vec![];

    for r in available_to_research_names(&state) {
        available.push(get_research(&r));
    }

    available
}

pub fn available_to_build(state: &GameState) -> Vec<Building> {
    let mut available = vec![];

    for b in available_to_build_names(&state) {
        available.push(get_building(&b));
    }

    available
}

pub fn available_to_invoke(state: &GameState) -> Vec<Edict> {
    let mut available = vec![];

    for e in available_to_invoke_names(&state) {
        available.push(get_edict(&e));
    }

    available
}

fn available_to_research_names(state: &GameState) -> Vec<String> {
    let mut available = vec![];
    for res in get_research_names() {
        let res = get_research(&res);
        if res.is_available(state) {
            available.push(res.name);
        }
    }

    available
}

fn available_to_build_names(state: &GameState) -> Vec<String> {
    let mut available = vec![];

    for building_name in get_building_names() {
        let building = get_building(&building_name);
        let has_missing_dep = building.research.iter().any(|x| !state.research.contains(x));

        if !(has_missing_dep || building.immortal) {
            available.push(building_name);
        }
    }

    available
}

fn available_to_invoke_names(state: &GameState) -> Vec<String> {
    let mut available = vec![];

    for edict_name in get_edict_names() {
        let edict = get_edict(&edict_name);
        let has_missing_dep = edict.research.iter().any(|x| !state.research.contains(x));

        if !(has_missing_dep) {
            available.push(edict_name);
        }
    }

    available
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::tests::*;

    #[test]
    fn available_to_research_dependencies() {
        let mut state = init_empty_game_state();
        let mut base_research = available_to_research(&state);
        assert_eq!(3, base_research.len());

        state.research.insert("TestNoDeps".to_owned());
        base_research = available_to_research(&state);
        assert_eq!(2, base_research.len());

        state.research.insert("Dep".to_owned());
        base_research = available_to_research(&state);
        assert_eq!(2, base_research.len());

        state.research.insert("TestWithDep".to_owned());
        base_research = available_to_research(&state);
        assert_eq!(1, base_research.len());
    }

    #[test]
    fn available_to_build_does_not_include_immortal() {
        let state = init_empty_game_state();
        assert!(!available_to_build(&state).iter().any(|x| x.immortal));
    }

    #[test]
    fn available_to_build_changes_with_unlocked_tech() {
        let mut state = init_empty_game_state();
        let base_build = available_to_build(&state);
        state.research.insert("TestNoDeps".to_owned());

        assert_eq!(base_build.len() + 1, available_to_build(&state).len());
    }

    #[test]
    fn available_to_invoke_changes_with_unlocked_tech() {
        let mut state = init_empty_game_state();
        let base_invoke = available_to_invoke(&state);
        state.research.insert("TestNoDeps".to_owned());

        assert_eq!(base_invoke.len() + 1, available_to_invoke(&state).len());
    }
}
