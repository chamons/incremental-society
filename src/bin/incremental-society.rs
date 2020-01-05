use pancurses::{Input, Window};

use incremental_society::console_ui::option_list;
use incremental_society::data;
use incremental_society::engine;
use incremental_society::resources::*;
use incremental_society::state::GameState;

use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

extern crate incremental_society;

fn main() {
    let term = pancurses::initscr();
    term.keypad(true);
    term.nodelay(true);
    pancurses::noecho();

    let mut state = GameState::init_new_game_state();
    loop {
        let now = Instant::now();

        if handle_input(&term, &mut state) {
            break;
        }

        draw(&term, &state);

        engine::process_tick(&mut state);

        const MS_FOR_30_FPS: u128 = 32;
        let processing_duration = now.elapsed().as_millis();
        if processing_duration < MS_FOR_30_FPS {
            let sleep_duration = MS_FOR_30_FPS - processing_duration;
            sleep(Duration::from_millis(sleep_duration as u64));
        }
    }

    pancurses::endwin();
}

fn is_char(input: &Input, c: char) -> bool {
    if let Input::Character(i) = input {
        if *i == c {
            return true;
        }
    }
    false
}

fn handle_input(t: &Window, mut state: &mut GameState) -> bool {
    if let Some(input) = t.getch() {
        if is_char(&input, 'q') {
            return true;
        }
        if is_char(&input, 'b') {
            let building_options = data::get_building_names();
            match option_list::OptionList::init(&t, &building_options).run() {
                Some(building_index) => match option_list::OptionList::init(&t, &state.regions.iter().map(|x| x.name).collect()).run() {
                    Some(region_index) => {
                        // Ignore errors for now
                        let _ = engine::build(&mut state, data::get_building(&building_options[building_index]), region_index);
                    }
                    None => {}
                },
                None => {}
            }
        }
    }

    false
}

#[allow(unused_assignments)]
fn draw(t: &Window, state: &GameState) {
    t.clear();

    let mut y = 1;

    // Left Column
    y = draw_country_stats(t, state, y);
    y += 1;
    y = draw_resources(t, state, y);

    // Right Column
    y = 1;
    y = draw_regions(t, state, y);
    y += 1;
    y = draw_conversions(t, state, y);

    draw_prompt(t);
}

fn write(t: &Window, text: &str, x: i32, y: i32) -> i32 {
    t.mvaddstr(y, x, text);
    y + 1
}

const RIGHT_COL: i32 = 50;
const RIGHT_COL_WIDTH: i32 = 40;

fn write_right(t: &Window, text: &str, x: i32, y: i32) -> i32 {
    write(t, text, x + RIGHT_COL, y)
}

fn draw_conversions(t: &Window, state: &GameState, y: i32) -> i32 {
    let mut y = y;
    const CONVERSION_BAR_LENGTH: f64 = 30.0;

    y = write_right(t, "Conversions", 0, y);

    for c in state.conversion_names() {
        match engine::get_conversion_tick(state, &c) {
            Some(ticks) => {
                // Don't update y, as we have to draw the bar
                write_right(t, &c, 0, y);

                let percentage = ticks as f64 / engine::CONVERSION_TICK_START as f64;
                let filled_width = (CONVERSION_BAR_LENGTH * percentage).round();
                let empty_width = (CONVERSION_BAR_LENGTH - filled_width).round() as usize;
                let filled_width = filled_width as usize;
                let bar_text = format!("{}{}", "#".repeat(filled_width), "-".repeat(empty_width));
                y = write_right(t, &bar_text, c.len() as i32 + 2, y);
            }
            _ => {}
        }
    }
    y
}

fn write_region_contents(t: &Window, text: &str, x: i32, y: i32) -> i32 {
    // RIGHT_COL_WIDTH - 2
    write_right(t, &format!("|{: <38}|", text), x, y)
}

#[allow(unused_assignments)]
fn draw_regions(t: &Window, state: &GameState, y: i32) -> i32 {
    let mut y = y;
    for r in &state.regions {
        y = write_right(t, "----------------------------------------", 0, y);

        y = write_region_contents(t, r.name, 0, y);

        let mut x = 0;
        let building_top_line = y;
        for b in 0..r.max_building_count() {
            if let Some(building) = r.buildings.get(b) {
                y = building_top_line;

                let building_name = &building.name;
                let building_name_length: usize = building_name.len();

                // Draw box manually
                write(t, "|", RIGHT_COL, y);
                write(t, "|", RIGHT_COL, y + 1);
                write(t, "|", RIGHT_COL, y + 2);
                write(t, "|", RIGHT_COL + RIGHT_COL_WIDTH - 1, y);
                write(t, "|", RIGHT_COL + RIGHT_COL_WIDTH - 1, y + 1);
                write(t, "|", RIGHT_COL + RIGHT_COL_WIDTH - 1, y + 2);

                y = write_right(t, &"_".repeat(building_name_length + 2), x + 2, y);
                y = write_right(t, &format!("|{}|", building_name), x + 2, y);
                y = write_right(t, &"-".repeat(building_name_length + 2), x + 2, y);

                x += building_name_length as i32 + 3;
            }
        }
        y = write_right(t, "----------------------------------------", 0, y);
    }

    y
}

#[allow(unused_assignments)]
fn draw_country_stats(t: &Window, _state: &GameState, y: i32) -> i32 {
    let mut y = write(t, "Elysium", 1, y);
    y = write(t, "Population: 500", 1, y + 1);
    y = write(t, "----------------", 0, y + 1);

    y
}

fn draw_resources(t: &Window, state: &GameState, y: i32) -> i32 {
    let mut y = y;

    for i in 0..NUM_RESOURCES {
        let line = &format!("{}: {}", ResourceKind::name_for_index(i), state.resources[i]);
        y = write(t, line, 1, y);
    }
    y
}

fn draw_prompt(t: &Window) {
    let y = t.get_max_y();
    t.mv(y, 0);
}
