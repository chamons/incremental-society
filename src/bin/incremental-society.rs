use std::thread::sleep;
use std::time::{Duration, Instant};

use incremental_society::engine;

use incremental_society::console_ui::{handle_input, Screen};

fn main() {
    let mut context = engine::GameContext::init_new_game_context();
    let mut screen = Screen::init();
    loop {
        let now = Instant::now();

        if handle_input(&mut screen, &mut context) {
            break;
        }

        screen.draw(&context);

        if context.is_lost {
            screen.show_modal_dialog(vec!["Game Lost".to_owned()]);
            screen.set_message("Starting new game...");
            context = engine::GameContext::init_new_game_context();
        }

        if let Some(msg) = engine::process_tick(&mut context) {
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
