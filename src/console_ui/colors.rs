use pancurses::Window;

pub enum Colors {
    Red = 1,
}

pub fn init() {
    pancurses::init_pair(Colors::Red as i16, pancurses::COLOR_RED, pancurses::COLOR_BLACK);
}

pub fn set_color(color: Colors, term: &Window) {
    term.attron(pancurses::COLOR_PAIR(color as pancurses::chtype));
}

pub fn clear_color(color: Colors, term: &Window) {
    term.attroff(pancurses::COLOR_PAIR(color as pancurses::chtype));
}
