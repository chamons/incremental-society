use pancurses::Input::Character;
use pancurses::Window;

pub const MODAL_WIDTH: usize = 60;

pub fn draw_border(term: &Window, start_x: i32, start_y: i32, height: i32) {
    let border = "-".repeat(MODAL_WIDTH + 1);

    term.mvaddstr(start_y - 1, start_x, border.clone());

    for i in 0..=height {
        term.mvaddch(start_y + i as i32, start_x, '|');
        term.mvaddch(start_y + i as i32, start_x + MODAL_WIDTH as i32, '|');
    }

    term.mvaddstr(start_y + height as i32, start_x, border);
}

pub fn shutdown_option_display(term: &Window, original_win: &Window) {
    term.nodelay(true);
    original_win.overwrite(&term);
}

pub fn write_with_clear(term: &Window, y: i32, x: i32, text: &str) {
    term.mvaddstr(y, x, " ".repeat(MODAL_WIDTH));
    term.mvaddstr(y, x + 3, text.to_string());
}

pub fn write_with_clear_left(term: &Window, start_x: i32, y: i32, line: &str) {
    term.mvaddstr(y, start_x, " ".repeat(MODAL_WIDTH));
    let x = start_x + MODAL_WIDTH as i32 - line.len() as i32 - 2;
    term.mvaddstr(y, x, line);
}

pub fn run_modal_dialog(term: &Window, text: Vec<String>) {
    let max_x = term.get_max_x() as usize;
    let max_y = term.get_max_y() as usize;

    let start_x = ((max_x - MODAL_WIDTH) / 2) as i32;
    let start_y = ((max_y - text.len()) / 2) as i32;

    let original_win = term.dupwin();

    let mut y: i32 = start_y;
    for l in text.iter() {
        write_with_clear(&term, y, start_x, &l);
        y += 1;
    }
    draw_border(term, start_x, start_y, y - start_y);

    term.mv(y, (start_x + MODAL_WIDTH as i32) as i32);

    term.nodelay(false);

    loop {
        if let Some(input) = term.getch() {
            if let Character(_) = input {
                shutdown_option_display(&term, &original_win);
                return;
            }
        }
    }
}
