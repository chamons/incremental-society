use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

use incremental_society::console_ui::{self, OptionList, Selection};
use incremental_society::engine;
use incremental_society::state::{format_resource_list, Building, DelayedAction, GameState, ResourceKind, Upgrade, MAX_UPGRADES, NUM_RESOURCES};

use pancurses::{Input, Window};

fn main() {
    let term = pancurses::initscr();
    console_ui::init_colors();

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
                                Err(e) => self.set_message(e.description()),
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
                                        Err(e) => self.set_message(e.description()),
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
                            Err(e) => self.set_message(e.description()),
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
                            Err(e) => self.set_message(e.description()),
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
                            Err(e) => self.set_message(e.description()),
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

    #[allow(unused_assignments)]
    fn draw(&self, state: &GameState) {
        self.term.clear();

        let mut y = 1;

        // Left Column
        y = self.draw_country_stats(state, y);
        y += 1;
        y = self.draw_resources(state, y);

        // Right Column
        y = 1;
        y = self.draw_regions(state, y);
        y += 1;
        y = self.draw_conversions(state, y);

        self.draw_messages();
        self.draw_prompt();
    }

    fn draw_conversions(&self, state: &GameState, y: i32) -> i32 {
        let mut y = y;
        const CONVERSION_BAR_LENGTH: f64 = 30.0;

        y = self.write_right("Conversions", 0, y);

        for c in &state.actions {
            let percentage = c.percentage();

            // Don't update y, as we have to draw the bar
            if let DelayedAction::Conversion(name) = &c.action {
                let count = state.derived_state.conversions.get(name).unwrap();
                self.write_right(&format!("{} ({})", name, count), 0, y);
            } else {
                self.write_right(&c.name, 0, y);
            }

            let filled_width = (CONVERSION_BAR_LENGTH * percentage).round();
            let empty_width = (CONVERSION_BAR_LENGTH - filled_width).round() as usize;
            let filled_width = filled_width as usize;
            let bar_text = format!("{}{}", "#".repeat(filled_width), "-".repeat(empty_width));
            y = self.write_right(&bar_text, c.name.len() as i32 + 5, y);
        }
        y
    }

    fn write_region_contents(&self, text: &str, x: i32, y: i32) -> i32 {
        // RIGHT_COL_WIDTH - 2
        self.write_right(&format!("|{: <67}|", text), x, y)
    }

    #[allow(unused_assignments)]
    fn draw_regions(&self, state: &GameState, y: i32) -> i32 {
        let mut y = y;
        for r in &state.regions {
            y = self.write_right("---------------------------------------------------------------------", 0, y);

            y = self.write_region_contents(&r.name, 0, y);

            let mut x = 0;
            let building_top_line = y;
            for b in 0..r.max_building_count() {
                if let Some(building) = r.buildings.get(b) {
                    y = building_top_line;

                    let building_name = &building.name;
                    let building_name_length: usize = building_name.len();

                    // Draw box manually
                    self.write("|", UI::RIGHT_COL, y);
                    self.write("|", UI::RIGHT_COL, y + 1);
                    self.write("|", UI::RIGHT_COL, y + 2);
                    self.write("|", UI::RIGHT_COL + UI::RIGHT_COL_WIDTH - 1, y);
                    self.write("|", UI::RIGHT_COL + UI::RIGHT_COL_WIDTH - 1, y + 1);
                    self.write("|", UI::RIGHT_COL + UI::RIGHT_COL_WIDTH - 1, y + 2);

                    y = self.write_right(&"_".repeat(building_name_length + 2), x + 2, y);
                    y = self.write_right(&format!("|{}|", building_name), x + 2, y);
                    y = self.write_right(&"-".repeat(building_name_length + 2), x + 2, y);

                    x += building_name_length as i32 + 3;
                }
            }
            y = self.write_right("---------------------------------------------------------------------", 0, y);
        }

        y
    }

    #[allow(unused_assignments)]
    fn draw_country_stats(&self, state: &GameState, y: i32) -> i32 {
        let mut y = self.write("Elysium", 1, y);
        y = self.write(format!("Population: {}", state.derived_state.pops), 1, y + 1);
        y = self.write(format!("Buildings: {} of {}", state.derived_state.used_pops, state.derived_state.pops), 1, y);
        y = self.write("----------------", 0, y + 1);

        y
    }

    fn draw_resources(&self, state: &GameState, y: i32) -> i32 {
        let mut y = y;

        for i in 0..NUM_RESOURCES {
            let line = &format!(
                "{}: {} / {}",
                ResourceKind::name_for_index(i),
                state.resources[i],
                state.derived_state.storage[i]
            );
            y = self.write(line, 1, y);
        }
        y
    }

    fn draw_messages(&self) {
        self.write(&self.messages, 2, self.term.get_max_y() - 2);
    }

    fn draw_prompt(&self) {
        let x = self.term.get_max_x();
        let y = self.term.get_max_y();
        self.write(" ", x - 1, y - 1);
    }

    fn write<S>(&self, text: S, x: i32, y: i32) -> i32
    where
        S: Into<String>,
    {
        self.term.mvaddstr(y, x, text.into());
        y + 1
    }

    const RIGHT_COL: i32 = 50;
    const RIGHT_COL_WIDTH: i32 = 69;

    fn write_right(&self, text: &str, x: i32, y: i32) -> i32 {
        self.write(text, x + UI::RIGHT_COL, y)
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
