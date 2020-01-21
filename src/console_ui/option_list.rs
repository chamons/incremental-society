use super::{clear_color, set_color, Colors};

use pancurses::Input::Character;
use pancurses::Window;

pub struct OptionList<'a> {
    term: &'a Window,
    options: Vec<Selection>,
    start_x: i32,
    start_y: i32,
    border: String,
}

pub struct Selection {
    name: String,
    active: bool,
    details: Vec<String>,
}

impl Selection {
    pub fn init_list<T: AsRef<str>>(names: &[T], active: impl Fn(usize) -> bool, details: impl Fn(usize) -> Vec<String>) -> Vec<Selection> {
        names
            .iter()
            .enumerate()
            .map(|(i, n)| Selection {
                name: n.as_ref().to_string(),
                active: active(i),
                details: details(i),
            })
            .collect()
    }
}

impl<'a> OptionList<'a> {
    const MODAL_WIDTH: usize = 60;

    pub fn init(term: &'a Window, options: Vec<Selection>) -> OptionList<'a> {
        let max_x = term.get_max_x() as usize;
        let max_y = term.get_max_y() as usize;

        let start_x = ((max_x - OptionList::MODAL_WIDTH) / 2) as i32;
        let start_y = ((max_y - options.len()) / 2) as i32;

        OptionList {
            term,
            options,
            start_x,
            start_y,
            border: "-".repeat(OptionList::MODAL_WIDTH + 1),
        }
    }

    fn write_with_clear(&self, y: i32, x: i32, text: &str) {
        self.term.mvaddstr(y, x, " ".repeat(OptionList::MODAL_WIDTH));
        self.term.mvaddstr(y, x + 3, text.to_string());
    }

    pub fn run(&self) -> Option<usize> {
        let original_win = self.term.dupwin();

        let mut y: i32 = self.start_y;
        for (i, o) in self.options.iter().enumerate() {
            // Clear each line
            let option_text = format!("{} - {}", (b'a' + i as u8) as char, o.name);
            if !o.active {
                set_color(Colors::Red, self.term);
            }

            self.write_with_clear(y, self.start_x, &option_text);
            y += 1;
            for l in o.details.iter() {
                self.write_with_clear(y, self.start_x + 2, &l);
                y += 1;
            }

            if !o.active {
                clear_color(Colors::Red, self.term);
            }
        }
        self.draw_border(y - self.start_y);

        self.term.mv(y, (self.start_x + OptionList::MODAL_WIDTH as i32) as i32);

        self.term.nodelay(false);

        let mut selected = None;
        loop {
            if let Some(input) = self.term.getch() {
                if let Character(c) = input {
                    if c.is_ascii_alphabetic() {
                        let index = c as u8 - b'a';
                        if index < self.options.len() as u8 && self.options.get(index as usize).unwrap().active {
                            selected = Some(index as usize);
                            break;
                        }
                    }
                    // Escape
                    if c as u8 == 27 {
                        break;
                    }
                }
            }
        }

        self.term.nodelay(true);
        original_win.overwrite(&self.term);

        selected
    }

    fn draw_border(&self, height: i32) {
        self.term.mvaddstr(self.start_y - 1, self.start_x, self.border.clone());

        for i in 0..=height {
            self.term.mvaddch(self.start_y + i as i32, self.start_x, '|');
            self.term.mvaddch(self.start_y + i as i32, self.start_x + OptionList::MODAL_WIDTH as i32, '|');
        }

        self.term.mvaddstr(self.start_y + height as i32, self.start_x, self.border.clone());
    }
}
