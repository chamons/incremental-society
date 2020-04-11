use std::thread::sleep;
use std::time::{Duration, Instant};

use incremental_society::console_ui::{OptionList, Selection};
use incremental_society::engine;
use incremental_society::state::{format_resource_list, Building, DelayedAction, GameState, ResourceKind, Upgrade, MAX_UPGRADES, NUM_RESOURCES};

use pancurses::{Input, Window};

fn main() {
    let mut ui = UI {
        messages: "".to_string(),
        message_timeout: 0,
        term: &term,
    };
    ui.main();
}

struct UI<'a> {
    messages: String,
    message_timeout: u32,
    term: &'a Window,
}

// Need current job position
// Draw one for each job
// Draw current highlightned some how
// Handle + to add-job
// Handle - to sub-job

impl<'a> UI<'a> {
    fn main(&mut self) {
        self.term.keypad(true);
        self.term.nodelay(true);
        pancurses::noecho();

        let mut state = engine::init_new_game_state();
        loop {
            let now = Instant::now();

            if self.handle_input(&mut state) {
                break;
            }

            self.draw(&state);

            if let Some(msg) = engine::process_tick(&mut state) {
                self.set_message(msg);
            }

            const MS_FOR_30_FPS: u128 = 32;
            let processing_duration = now.elapsed().as_millis();
            if processing_duration < MS_FOR_30_FPS {
                let sleep_duration = MS_FOR_30_FPS - processing_duration;
                sleep(Duration::from_millis(sleep_duration as u64));
            }
            self.tick_message();
        }

        pancurses::endwin();
    }

    fn tick_message(&mut self) {
        if self.message_timeout > 0 {
            self.message_timeout -= 1;
        } else {
            self.clear_message();
        }
    }

    fn set_message<S>(&mut self, message: S)
    where
        S: Into<String>,
    {
        self.messages = message.into();
        self.message_timeout = 120;
    }

    fn clear_message(&mut self) {
        self.messages.clear();
        self.message_timeout = 0;
    }

    fn handle_input(&mut self, mut state: &mut GameState) -> bool {
        if let Some(input) = self.term.getch() {
            if let Input::KeyResize = input {
                pancurses::resize_term(0, 0);
            }

            if is_char(input, 'q') {
                return true;
            }

            if is_char(input, 'b') {
                let building_options: Vec<&Building> = state.derived_state.available_buildings.iter().filter(|x| !x.immortal).collect();
                let building_names: Vec<&String> = building_options.iter().map(|x| &x.name).collect();
                let selection = Selection::init_list(
                    &building_names[..],
                    |o| engine::can_build_building(&state, &&building_options[o]).is_ok(),
                    |o| building_options[o].details(),
                );

                match OptionList::init(&self.term, selection).run() {
                    Some(building_index) => {
                        let building = building_options[building_index].clone();
                        let name = building.name.clone();
                        let regions: Vec<String> = state.regions.iter().map(|x| x.name.to_string()).collect();
                        let selection = Selection::init_list(&regions, |o| engine::can_build_in_region(&state, o).is_ok(), |_| vec![]);
                        match OptionList::init(&self.term, selection).run() {
                            Some(region_index) => match engine::build(&mut state, building, region_index) {
                                Err(e) => self.set_message(e.to_string()),
                                _ => self.set_message(format!("Built {}", name)),
                            },
                            None => self.clear_message(),
                        }
                    }
                    None => self.clear_message(),
                }
            }

            if is_char(input, 'd') {
                let regions: Vec<String> = state.regions.iter().map(|x| x.name.to_string()).collect();
                let selection = Selection::init_list(&regions, |_| true /* Any region can have buildings destroyed */, |_| vec![]);
                match OptionList::init(&self.term, selection).run() {
                    Some(region_index) => {
                        let buildings: Vec<String> = state.regions[region_index].buildings.iter().map(|x| x.name.to_string()).collect();
                        if !buildings.is_empty() {
                            let selection = Selection::init_list(&buildings, |o| engine::can_destroy_building(&state, region_index, o).is_ok(), |_| vec![]);
                            match OptionList::init(&self.term, selection).run() {
                                Some(building_index) => {
                                    let building_name = &buildings[building_index];
                                    match engine::destroy(&mut state, region_index, building_index) {
                                        Err(e) => self.set_message(e.to_string()),
                                        _ => self.set_message(format!("Destroying {}", building_name)),
                                    }
                                }
                                None => self.clear_message(),
                            }
                        }
                    }
                    None => self.clear_message(),
                }
            }

            if is_char(input, 'e') {
                let edicts = &state.derived_state.available_edicts;
                let edict_names: Vec<&String> = edicts.iter().map(|x| &x.name).collect();

                let selection = Selection::init_list(
                    &edict_names,
                    |o| engine::can_invoke_edict(&state, &edicts.get(o).unwrap()).is_ok(),
                    |o| edicts.get(o).unwrap().conversion.details(),
                );
                match OptionList::init(&self.term, selection).run() {
                    Some(edict_index) => {
                        let edict = edicts.get(edict_index).unwrap().clone();

                        match engine::edict(&mut state, &edict) {
                            Err(e) => self.set_message(e.to_string()),
                            _ => self.clear_message(),
                        }
                    }
                    None => self.clear_message(),
                }
            }

            if is_char(input, 'r') {
                let research = &state.derived_state.available_research;
                let research_names: Vec<&String> = research.iter().map(|x| &x.name).collect();

                let selection = Selection::init_list(
                    &research_names,
                    |o| engine::can_research(&state, &research.get(o).unwrap()).is_ok(),
                    |o| research.get(o).unwrap().details(),
                );
                match OptionList::init(&self.term, selection).run() {
                    Some(research_index) => {
                        let research = research.get(research_index).unwrap().clone();

                        match engine::research(&mut state, &research) {
                            Err(e) => self.set_message(e.to_string()),
                            _ => self.clear_message(),
                        }
                    }
                    None => self.clear_message(),
                }
            }

            if is_char(input, 'u') {
                let upgrades = &state.derived_state.available_upgrade;
                let upgrade_names: Vec<&String> = upgrades.iter().map(|x| &x.name).collect();

                let selection = Selection::init_list(&upgrade_names, |_| true, |o| upgrades.get(o).unwrap().details());
                match OptionList::init(&self.term, selection).run_multiple_selection(
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
                            Err(e) => self.set_message(e.to_string()),
                            _ => self.clear_message(),
                        }
                    }
                    _ => self.clear_message(),
                }
            }

            #[cfg(debug_assertions)]
            {
                if is_char(input, '~') {
                    let debug_options = vec!["Dump State", "Load Default GameState", "Max Resources", "Complete Actions"];
                    let selection = Selection::init_list(&debug_options, |_| true, |_| vec![]);
                    match OptionList::init(&self.term, selection).run() {
                        Some(debug_index) => match debug_index {
                            0 => engine::dump_state(&state),
                            1 => engine::load_default_state(&mut state),
                            2 => engine::max_resources(&mut state),
                            3 => engine::complete_actions(&mut state),
                            _ => self.clear_message(),
                        },
                        None => self.clear_message(),
                    }
                }
            }
        }

        false
    }
}

fn is_char(input: Input, c: char) -> bool {
    if let Input::Character(i) = input {
        if i == c {
            return true;
        }
    }
    false
}
