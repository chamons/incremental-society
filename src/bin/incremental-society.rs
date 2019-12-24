use console::Term;
use std::io;

use incremental_society::resources::*;
use incremental_society::state::*;

extern crate incremental_society;

#[allow(dead_code)]

fn main() {
    let term = Term::stdout();

    let mut state = GameState::init();
    state.process_tick();

    draw(&term, &state).expect("Error writing to console");
}

#[allow(unused_assignments)]
fn draw(t: &Term, state: &GameState) -> io::Result<()> {
    let mut y = 1;

    // Left Column
    y = draw_country_stats(t, state, y)?;
    y += 1;
    y = draw_resources(t, state, y)?;

    // Right Column
    y = 1;
    y = draw_regions(t, state, y)?;
    y += 1;
    y = draw_conversions(t, state, y)?;

    draw_prompt(t)?;
    t.read_char()?;
    Ok(())
}

fn write(t: &Term, text: &str, x: usize, y: usize) -> io::Result<usize> {
    t.move_cursor_to(x, y)?;
    t.write_str(text)?;
    Ok(y + 1)
}

const RIGHT_COL: usize = 50;
const RIGHT_COL_WIDTH: usize = 40;

fn write_right(t: &Term, text: &str, x: usize, y: usize) -> io::Result<usize> {
    write(t, text, x + RIGHT_COL, y)
}

fn draw_conversions(t: &Term, state: &GameState, y: usize) -> io::Result<usize> {
    let mut y = y;
    const CONVERSION_BAR_LENGTH: f64 = 30.0;

    y = write_right(t, "Conversions", 0, y)?;

    for c in state.conversions() {
        // Don't update y, as we have to draw the bar
        write_right(t, c.name, 0, y)?;

        let percentage = c.tick_percentage();
        let filled_width = (CONVERSION_BAR_LENGTH * percentage).round();
        let empty_width = (CONVERSION_BAR_LENGTH - filled_width).round() as usize;
        let filled_width = filled_width as usize;

        let bar_text = format!("{}{}", "#".repeat(filled_width), "-".repeat(empty_width));
        y = write_right(t, &bar_text, c.name.len() + 2, y)?;
    }
    Ok(y)
}

fn write_region_contents(t: &Term, text: &str, x: usize, y: usize) -> io::Result<usize> {
    // RIGHT_COL_WIDTH - 2
    write_right(t, &format!("|{: <38}|", text), x, y)
}

#[allow(unused_assignments)]
fn draw_regions(t: &Term, state: &GameState, y: usize) -> io::Result<usize> {
    let mut y = y;
    for r in &state.regions {
        y = write_right(t, "----------------------------------------", 0, y)?;

        y = write_region_contents(t, r.name, 0, y)?;

        let mut x = 0;
        let building_top_line = y;
        for b in 0..r.max_building_count() {
            if let Some(building) = r.buildings.get(b) {
                y = building_top_line;

                let building_name = building.name;
                let building_name_length = building_name.len();

                // Draw box manually
                write(t, "|", RIGHT_COL, y)?;
                write(t, "|", RIGHT_COL, y + 1)?;
                write(t, "|", RIGHT_COL, y + 2)?;
                write(t, "|", RIGHT_COL + RIGHT_COL_WIDTH - 1, y)?;
                write(t, "|", RIGHT_COL + RIGHT_COL_WIDTH - 1, y + 1)?;
                write(t, "|", RIGHT_COL + RIGHT_COL_WIDTH - 1, y + 2)?;

                y = write_right(t, &"_".repeat(building_name_length + 2), x + 2, y)?;
                y = write_right(t, &format!("|{}|", building_name), x + 2, y)?;
                y = write_right(t, &"-".repeat(building_name_length + 2), x + 2, y)?;

                x += building_name_length + 3;
            }
        }
        y = write_right(t, "----------------------------------------", 0, y)?;
    }

    Ok(y)
}

#[allow(unused_assignments)]
fn draw_country_stats(t: &Term, state: &GameState, y: usize) -> io::Result<usize> {
    let mut y = write(t, "Elysium", 1, y)?;
    y = write(t, "Population: 500", 1, y + 1)?;
    y = write(t, "----------------", 0, y + 1)?;

    Ok(y)
}

fn draw_resources(t: &Term, state: &GameState, y: usize) -> io::Result<usize> {
    let mut y = y;

    for i in 0..NUM_RESOURCES {
        let line = &format!("{}: {}", ResourceKind::name_for_index(i), state.resources[i]);
        y = write(t, line, 1, y)?;
    }
    Ok(y)
}

fn draw_prompt(t: &Term) -> io::Result<()> {
    let term_size = t.size();

    t.move_cursor_to(0, term_size.0 as usize)?;
    Ok(())
}
