use std::thread::sleep;
use std::time::{Duration, Instant};

use incremental_society::engine;

use incremental_society::console_ui::{handle_input, Screen};

fn main() {
    let mut state = engine::init_new_game_state();
    let mut screen = Screen::init();
    loop {
        let now = Instant::now();

        if handle_input(&mut screen, &mut state) {
            break;
        }

        screen.draw(&state);

        if let Some(msg) = engine::process_tick(&mut state) {
            screen.set_message(msg);
        }

        const MS_FOR_30_FPS: u128 = 32;
        let processing_duration = now.elapsed().as_millis();
        if processing_duration < MS_FOR_30_FPS {
            let sleep_duration = MS_FOR_30_FPS - processing_duration;
            sleep(Duration::from_millis(sleep_duration as u64));
        }
        screen.tick_message();
    }

    pancurses::endwin();
}

// Need current job position
// Draw one for each job
// Draw current highlightned some how
// Handle + to add-job
// Handle - to sub-job
