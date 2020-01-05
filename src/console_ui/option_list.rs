use pancurses::Input::Character;
use pancurses::Window;

pub struct OptionList<'a> {
    term: &'a Window,
    options: &'a Vec<String>,
    height: usize,
    start_x: i32,
    start_y: i32,
    border: String,
    content_border: String,
}

impl<'a> OptionList<'a> {
    const MODAL_WIDTH: usize = 60;

    pub fn init(term: &'a Window, options: &'a Vec<String>) -> OptionList<'a> {
        let height = options.len() + 2;
        let max_x = term.get_max_x() as usize;
        let max_y = term.get_max_y() as usize;

        let start_x = ((max_x - OptionList::MODAL_WIDTH) / 2) as i32;
        let start_y = ((max_y - height) / 2) as i32;

        OptionList {
            term,
            options,
            height,
            start_x,
            start_y,
            border: "-".repeat(OptionList::MODAL_WIDTH + 1),
            content_border: format!("{}{}{}", "|", " ".repeat(OptionList::MODAL_WIDTH - 1), "|"),
        }
    }

    pub fn run(&self) -> Option<usize> {
        self.draw_border();
        for (i, o) in self.options.iter().enumerate() {
            let option_text = format!("|  {} - {}", ('a' as u8 + i as u8) as char, o);
            self.term.mvaddstr(self.start_y + i as i32, self.start_x, option_text);
        }

        self.term
            .mv(self.start_y + self.height as i32, (self.start_x + OptionList::MODAL_WIDTH as i32 - 1) as i32);

        self.term.nodelay(false);

        let mut selected = None;
        loop {
            match self.term.getch() {
                Some(input) => match input {
                    Character(c) => {
                        if c.is_ascii_alphabetic() {
                            let index = c as u8 - 'a' as u8;
                            if index < self.options.len() as u8 {
                                selected = Some(index as usize);
                                break;
                            }
                        }
                        // Escape
                        if c as u8 == 27 {
                            break;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        self.term.nodelay(true);

        selected
    }

    fn draw_border(&self) {
        self.term.mvaddstr(self.start_y - 1, self.start_x, self.border.clone());

        for i in 0..=self.height {
            self.term.mvaddstr(self.start_y + i as i32, self.start_x, self.content_border.clone());
        }

        // TODO - Not sure why need +1 here but not first border
        self.term.mvaddstr(self.start_y + self.height as i32 + 1, self.start_x, self.border.clone());
    }
}
