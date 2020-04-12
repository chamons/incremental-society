use crate::console_ui::{Screen, Selection};
use crate::engine;
use crate::state::{format_resource_list, Building, GameState, Upgrade, MAX_UPGRADES};

use pancurses::Input;

fn handle_build_command(screen: &mut Screen, mut state: &mut GameState) {
    let building_options: Vec<&Building> = state.derived_state.available_buildings.iter().filter(|x| !x.immortal).collect();
    let building_names: Vec<&String> = building_options.iter().map(|x| &x.name).collect();
    let selection = Selection::init_list(
        &building_names[..],
        |o| engine::can_build_building(&state, &&building_options[o]).is_ok(),
        |o| building_options[o].details(),
    );

    match screen.show_modal_selection(selection) {
        Some(building_index) => {
            let building = building_options[building_index].clone();
            let name = building.name.clone();
            let regions: Vec<String> = state.regions.iter().map(|x| x.name.to_string()).collect();
            let selection = Selection::init_list(&regions, |o| engine::can_build_in_region(&state, o).is_ok(), |_| vec![]);
            match screen.show_modal_selection(selection) {
                Some(region_index) => match engine::build(&mut state, building, region_index) {
                    Err(e) => screen.set_message(e.to_string()),
                    _ => screen.set_message(format!("Built {}", name)),
                },
                None => screen.clear_message(),
            }
        }
        None => screen.clear_message(),
    }
}

fn handle_destroy_command(screen: &mut Screen, mut state: &mut GameState) {
    let regions: Vec<String> = state.regions.iter().map(|x| x.name.to_string()).collect();
    let selection = Selection::init_list(&regions, |_| true /* Any region can have buildings destroyed */, |_| vec![]);
    match screen.show_modal_selection(selection) {
        Some(region_index) => {
            let buildings: Vec<String> = state.regions[region_index].buildings.iter().map(|x| x.name.to_string()).collect();
            if !buildings.is_empty() {
                let selection = Selection::init_list(&buildings, |o| engine::can_destroy_building(&state, region_index, o).is_ok(), |_| vec![]);
                match screen.show_modal_selection(selection) {
                    Some(building_index) => {
                        let building_name = &buildings[building_index];
                        match engine::destroy(&mut state, region_index, building_index) {
                            Err(e) => screen.set_message(e.to_string()),
                            _ => screen.set_message(format!("Destroying {}", building_name)),
                        }
                    }
                    None => screen.clear_message(),
                }
            }
        }
        None => screen.clear_message(),
    }
}

fn handle_edict_command(screen: &mut Screen, mut state: &mut GameState) {
    let edicts = &state.derived_state.available_edicts;
    let edict_names: Vec<&String> = edicts.iter().map(|x| &x.name).collect();

    let selection = Selection::init_list(
        &edict_names,
        |o| engine::can_invoke_edict(&state, &edicts.get(o).unwrap()).is_ok(),
        |o| edicts.get(o).unwrap().conversion.details(),
    );
    match screen.show_modal_selection(selection) {
        Some(edict_index) => {
            let edict = edicts.get(edict_index).unwrap().clone();

            match engine::edict(&mut state, &edict) {
                Err(e) => screen.set_message(e.to_string()),
                _ => screen.clear_message(),
            }
        }
        None => screen.clear_message(),
    }
}

fn handle_research_command(screen: &mut Screen, mut state: &mut GameState) {
    let research = &state.derived_state.available_research;
    let research_names: Vec<&String> = research.iter().map(|x| &x.name).collect();

    let selection = Selection::init_list(
        &research_names,
        |o| engine::can_research(&state, &research.get(o).unwrap()).is_ok(),
        |o| research.get(o).unwrap().details(),
    );
    match screen.show_modal_selection(selection) {
        Some(research_index) => {
            let research = research.get(research_index).unwrap().clone();

            match engine::research(&mut state, &research) {
                Err(e) => screen.set_message(e.to_string()),
                _ => screen.clear_message(),
            }
        }
        None => screen.clear_message(),
    }
}

#[cfg(debug_assertions)]
fn handle_debug_command(screen: &mut Screen, mut state: &mut GameState) {
    let debug_options = vec!["Dump State", "Load Default GameState", "Max Resources", "Complete Actions"];
    let selection = Selection::init_list(&debug_options, |_| true, |_| vec![]);
    match screen.show_modal_selection(selection) {
        Some(debug_index) => match debug_index {
            0 => engine::dump_state(&state),
            1 => engine::load_default_state(&mut state),
            2 => engine::max_resources(&mut state),
            3 => engine::complete_actions(&mut state),
            _ => screen.clear_message(),
        },
        None => screen.clear_message(),
    }
}

fn handle_upgrade_command(screen: &mut Screen, mut state: &mut GameState) {
    let upgrades = &state.derived_state.available_upgrade;
    let upgrade_names: Vec<&String> = upgrades.iter().map(|x| &x.name).collect();

    let selection = Selection::init_list(&upgrade_names, |_| true, |o| upgrades.get(o).unwrap().details());
    match screen.show_modal_multiple_selection(
        selection,
        upgrade_names.iter().map(|x| state.upgrades.contains(*x)).collect(),
        |selection| {
            let selected_upgrades: Vec<Upgrade> = selection.iter().map(|x| upgrades.get(*x).unwrap().clone()).collect();
            engine::can_apply_upgrades(&state, &selected_upgrades[..]).is_ok()
        },
        |selection| {
            let selected_upgrades: Vec<Upgrade> = selection.iter().map(|x| upgrades.get(*x).unwrap().clone()).collect();
            [
                format!("[Enter] to Accept. ({} of {})", selection.len(), MAX_UPGRADES),
                format_resource_list("", &engine::get_upgrade_cost(&state, &selected_upgrades[..])),
            ]
        },
    ) {
        Some(selected_items) => {
            let selected_upgrades = selected_items.iter().map(|x| upgrades.get(*x).unwrap().clone()).collect();
            match engine::upgrade(&mut state, selected_upgrades) {
                Err(e) => screen.set_message(e.to_string()),
                _ => screen.clear_message(),
            }
        }
        _ => screen.clear_message(),
    }
}

fn handle_job_increase(screen: &mut Screen, state: &mut GameState) {
    match engine::add_job(state, &screen.current_job_name(state)) {
        Err(e) => screen.set_message(e.to_string()),
        _ => screen.clear_message(),
    }
}

fn handle_job_decrease(screen: &mut Screen, state: &mut GameState) {
    match engine::remove_job(state, &screen.current_job_name(state)) {
        Err(e) => screen.set_message(e.to_string()),
        _ => screen.clear_message(),
    }
}

pub fn handle_input(screen: &mut Screen, state: &mut GameState) -> bool {
    if let Some(input) = screen.get_input() {
        match input {
            Input::KeyResize => {
                pancurses::resize_term(0, 0);
            }
            Input::KeyUp => screen.move_job_pos_up(),
            Input::KeyDown => screen.move_job_pos_down(state),
            Input::Character(i) => match i {
                'q' => return true,
                'b' => handle_build_command(screen, state),
                'd' => handle_destroy_command(screen, state),
                'e' => handle_edict_command(screen, state),
                'r' => handle_research_command(screen, state),
                'u' => handle_upgrade_command(screen, state),
                '+' => handle_job_increase(screen, state),
                '-' => handle_job_decrease(screen, state),
                #[cfg(debug_assertions)]
                '~' => handle_debug_command(screen, state),
                _ => {}
            },
            _ => {}
        };
    }

    false
}
