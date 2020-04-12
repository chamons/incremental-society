use crate::console_ui::{clear_color, set_color, Colors, OptionList, Selection};
use crate::state::{DelayedAction, GameState, ResourceKind, NUM_RESOURCES};
use pancurses::{Input, Window};

pub struct Screen {
    messages: String,
    message_timeout: u32,
    term: Window,
    job_pos: usize,
}

impl Screen {
    pub fn init() -> Screen {
        let term = pancurses::initscr();

        super::init_colors();

        term.keypad(true);
        term.nodelay(true);
        pancurses::noecho();

        Screen {
            messages: "".to_string(),
            message_timeout: 0,
            term: term,
            job_pos: 0,
        }
    }

    pub fn tick_message(&mut self) {
        if self.message_timeout > 0 {
            self.message_timeout -= 1;
        } else {
            self.clear_message();
        }
    }

    pub fn set_message<S>(&mut self, message: S)
    where
        S: Into<String>,
    {
        self.messages = message.into();
        self.message_timeout = 120;
    }

    pub fn clear_message(&mut self) {
        self.messages.clear();
        self.message_timeout = 0;
    }

    pub fn get_input(&self) -> Option<Input> {
        self.term.getch()
    }

    pub fn show_modal_selection(&self, options: Vec<Selection>) -> Option<usize> {
        OptionList::init(&self.term, options).run()
    }

    pub fn show_modal_multiple_selection(
        &self,
        options: Vec<Selection>,
        initial_selection: Vec<bool>,
        valid_selection: impl Fn(&Vec<usize>) -> bool,
        status_line: impl Fn(&Vec<usize>) -> [String; 2],
    ) -> Option<Vec<usize>> {
        OptionList::init(&self.term, options).run_multiple_selection(initial_selection, valid_selection, status_line)
    }

    pub fn current_job_pos(&self) -> usize {
        self.job_pos
    }

    pub fn move_job_pos_up(&mut self) {
        if self.job_pos > 0 {
            self.job_pos -= 1;
        }
    }

    pub fn move_job_pos_down(&mut self, state: &GameState) {
        if self.job_pos < state.derived_state.current_building_jobs.len() - 1 {
            self.job_pos += 1;
        }
    }

    pub fn current_job_name(&mut self, state: &GameState) -> String {
        let mut keys: Vec<&String> = state.derived_state.current_building_jobs.keys().collect();
        keys.sort();
        keys[self.job_pos].to_string()
    }

    #[allow(unused_assignments)]
    pub fn draw(&self, state: &GameState) {
        self.term.clear();

        let mut y = 1;

        // Left Column
        y = self.draw_country_stats(state, y);
        y = self.draw_jobs(state, y);
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
        y += 1;

        for c in &state.actions {
            let percentage = c.percentage();

            // Don't update y, as we have to draw the bar
            if let DelayedAction::Conversion(name) = &c.action {
                let count = state.job_count(name);
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
        if !self.should_draw_buildings(state) {
            return 0;
        }

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
                    self.write("|", Screen::RIGHT_COL, y);
                    self.write("|", Screen::RIGHT_COL, y + 1);
                    self.write("|", Screen::RIGHT_COL, y + 2);
                    self.write("|", Screen::RIGHT_COL + Screen::RIGHT_COL_WIDTH - 1, y);
                    self.write("|", Screen::RIGHT_COL + Screen::RIGHT_COL_WIDTH - 1, y + 1);
                    self.write("|", Screen::RIGHT_COL + Screen::RIGHT_COL_WIDTH - 1, y + 2);

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

    fn should_draw_buildings(&self, state: &GameState) -> bool {
        state.age == "Archaic"
    }

    #[allow(unused_assignments)]
    fn draw_country_stats(&self, state: &GameState, y: i32) -> i32 {
        let mut y = self.write("Elysium", 1, y);
        y += 1;
        y = self.write(format!("{} Age", state.age), 1, y);
        if self.should_draw_buildings(state) {
            y = self.write(format!("Population: {}", state.pops), 1, y + 1);
            let unemployed = state.pops - state.total_jobs_assigned();
            if unemployed > 0 {
                y = self.write(format!("Unemployed: {}", unemployed), 1, y);
            }
        }

        y = self.write(" ----------------", 0, y + 1);

        y
    }

    fn draw_jobs(&self, state: &GameState, y: i32) -> i32 {
        let mut y = y + 1;

        if self.should_draw_buildings(state) {
            let mut jobs: Vec<&String> = state.derived_state.current_building_jobs.keys().collect();
            jobs.sort();

            for (index, job) in jobs.iter().enumerate() {
                let at_selected_job = index == self.job_pos;

                if at_selected_job {
                    set_color(Colors::LightBlue, &self.term);
                }
                let current = state.job_count(job);
                let max = state.derived_state.current_building_jobs[&job.to_string()];
                y = self.write(format!("{} {}/{}", job, current, max), 1, y);
                if at_selected_job {
                    clear_color(Colors::LightBlue, &self.term);
                }
            }

            y += 1;
        }

        y
    }

    fn draw_resources(&self, state: &GameState, y: i32) -> i32 {
        let mut y = y + 1;

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
        self.write(text, x + Screen::RIGHT_COL, y)
    }
}
