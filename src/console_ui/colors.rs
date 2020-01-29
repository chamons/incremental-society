use pancurses::{chtype, Window};

pub enum Colors {
    Red = 1,
    LightBlue = 2,
}

pub fn init_colors() {
    pancurses::start_color();
    pancurses::init_pair(Colors::Red as i16, pancurses::COLOR_RED, pancurses::COLOR_BLACK);
    pancurses::init_pair(Colors::LightBlue as i16, pancurses::COLOR_CYAN, pancurses::COLOR_BLACK);
}

pub fn set_color(color: Colors, term: &Window) {
    term.attron(pancurses::COLOR_PAIR(color as chtype));
}

pub fn clear_color(color: Colors, term: &Window) {
    term.attroff(pancurses::COLOR_PAIR(color as chtype));
}
