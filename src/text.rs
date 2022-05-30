use std::{
    fmt::{self, Display},
    io::stdout,
};

pub use crate::LoadingBar;
pub use colored::Color;

use colored::Colorize;

use crossterm::{
    cursor::{MoveTo, RestorePosition, SavePosition},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};

// we have 2 structs for the main (TextLoadingbar) struct, so its easier to manage the elements such as (TextItem) and what color they should be etc
// for each of these structs we have to have a position, for crossterm::cursor to be able to use and
// we also have to have a color, so we can use the color struct to set the color of the text (which we might have to change the type from Colored::color to crossterm::style::Color)
#[derive(Debug)]
struct TextItem {
    text: String,
    color: Option<colored::Color>,
}

// in the main struct we have a two text items, one for above the bar and one for below the bar
// we also have a loading bar item, which is the bar itself
// we also have the largest text item, which is the text that is the largest in the bar (used for centering the text)
#[derive(Debug)]
pub struct TextLoadingBar {
    top_text: TextItem,
    bottom_text: TextItem,
    t_bar: LoadingBar,
    t_start_pos: (u16, u16),
}

impl Display for TextLoadingBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}",
            self.top_text
                .text
                .color(self.top_text.color.unwrap_or(colored::Color::White)),
            self.t_bar,
            self.bottom_text
                .text
                .color(self.bottom_text.color.unwrap_or(colored::Color::White)) // if we have a color, use it, otherwise use white
        )
    }
}

impl TextLoadingBar {
    pub fn new(
        top_text: String,
        bottom_text: String,
        len: u16,
        color: (
            Option<colored::Color>, // top text color
            Option<colored::Color>, // bar color
            Option<colored::Color>, // bottom text color
        ),
        t_start_pos: (u16, u16),
    ) -> TextLoadingBar {
        let (top_text_color, bar_color, bottom_text_color) = color;
        TextLoadingBar {
            top_text: TextItem {
                text: top_text,
                color: top_text_color,
            },
            t_bar: LoadingBar::new(len, bar_color, (0, 0)),
            bottom_text: TextItem {
                text: bottom_text,
                color: bottom_text_color,
            },
            t_start_pos,
        }
    }

    pub fn auto_run(
        top_text: String,
        bottom_text: String,
        time_in_seconds: u16,
        len: u16,
        start: u16,
        color: (
            Option<colored::Color>, // top text color
            Option<colored::Color>, // bar color
            Option<colored::Color>, // bottom text color
        ),
        t_start_pos: (u16, u16),
    ) {
        if start > len {
            println!();
            panic!("\x07start must be less than len\x07");
        }
        let index = time_in_seconds as f32 / (len - start) as f32;
        let (top_text_color, bar_color, bottom_text_color) = color;
        let mut self_clone = TextLoadingBar::new(
            top_text,
            bottom_text,
            len,
            (top_text_color, bar_color, bottom_text_color),
            t_start_pos,
        );
        self_clone.adnvance_by_print(start);
        std::thread::spawn(move || {
            for _ in 0..(self_clone.t_bar.space_left) {
                self_clone.adnvance_print();
                std::thread::sleep(std::time::Duration::from_secs_f32(index));
            }
        });
    }

    fn goline_clear_print(&self) {
        let line_count = get_num_lines_witdh(&self.to_string());

        let (x, y) = self.t_start_pos;
        let mut y_copy = y;
        execute!(stdout(), SavePosition).expect("failed to save position");
        for _ in 0..line_count {
            execute!(stdout(), MoveTo(x, y_copy)).expect("failed to move cursor");
            execute!(stdout(), Clear(ClearType::CurrentLine)).expect("failed to clear line");
            execute!(stdout(), RestorePosition).expect("failed to restore cursor");
            y_copy += 1;
        }

        let text = self.to_string();
        let text = text.split("\n").collect::<Vec<&str>>();

        y_copy = y;
        for i in 0..line_count {
            let texts = &text[i as usize];
            execute!(stdout(), MoveTo(x, y_copy)).expect("failed to move cursor");
            execute!(stdout(), Print(texts)).expect("failed to print");
            execute!(stdout(), RestorePosition).expect("failed to restore cursor");
            y_copy += 1;
        }
    }

    pub fn change_top_text(&mut self, text: String) {
        self.top_text.text = text;
    }

    pub fn change_bottom_text(&mut self, text: String) {
        self.bottom_text.text = text;
    }

    pub fn change_top_text_color(&mut self, color: Option<Color>) {
        self.top_text.color = color;
    }

    pub fn change_bottom_text_color(&mut self, color: Option<Color>) {
        self.bottom_text.color = color;
    }

    pub fn change_bar_color(&mut self, color: Option<Color>) {
        self.t_bar.color = color;
    }

    pub fn change_all_text_colors(&mut self, color: Option<Color>) {
        self.top_text.color = color;
        self.bottom_text.color = color;
        self.t_bar.color = color;
    }

    pub fn adnvance(&mut self) {
        self.t_bar.advance();
    }

    pub fn adnvance_print(&mut self) {
        self.adnvance();
        self.goline_clear_print();
    }

    pub fn adnvance_by(&mut self, index: u16) {
        self.t_bar.advance_by(index);
    }

    pub fn adnvance_by_print(&mut self, index: u16) {
        self.adnvance_by(index);
        self.goline_clear_print();
    }

    pub fn adnvance_by_percent(&mut self, percent: f32) {
        self.t_bar.advance_by_percent(percent);
    }

    pub fn adnvance_by_percent_print(&mut self, percent: f32) {
        self.adnvance_by_percent(percent);
        self.goline_clear_print();
    }
}

fn get_num_lines_witdh(text: &str) -> u16 {
    let mut num_lines = 1;
    for c in text.chars() {
        if c == '\n' {
            num_lines += 1;
        }
    }
    num_lines
}
