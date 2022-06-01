use std::{
    collections::HashMap,
    fmt::{self, Display},
    io::stdout,
};

pub use crate::{Color, LoadingBar};

use colored::Colorize;

use crossterm::{
    cursor::{MoveTo, RestorePosition, SavePosition},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};

pub enum TextLoadingBarOptions {
    Text(String),
    Color(Option<Color>),
    Number(u16),
    Float(f32),
    Pos(u16, u16),
    None,
}

impl TextLoadingBarOptions {
    fn get_text(&self) -> &str {
        match self {
            TextLoadingBarOptions::Text(text) => text,
            _ => "",
        }
    }

    fn get_color(&self) -> Option<Color> {
        match self {
            TextLoadingBarOptions::Color(color) => color.clone(),
            _ => None,
        }
    }

    fn get_number(&self) -> u16 {
        match self {
            TextLoadingBarOptions::Number(number) => *number,
            _ => 0,
        }
    }

    fn get_float(&self) -> f32 {
        match self {
            TextLoadingBarOptions::Float(float) => *float,
            _ => 0.0,
        }
    }

    fn get_pos(&self) -> (u16, u16) {
        match self {
            TextLoadingBarOptions::Pos(x, y) => (*x, *y),
            _ => (0, 0),
        }
    }
}

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
        if start >= len {
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
        self_clone.advance_by_print(start);
        std::thread::spawn(move || {
            for _ in 0..(self_clone.t_bar.space_left) {
                self_clone.advance_print();
                std::thread::sleep(std::time::Duration::from_secs_f32(index));
            }
        });
    }

    fn goline_clear_print(&self) {
        let line_count = get_num_lines_witdh(&self.to_string());

        let (x, y) = self.t_start_pos;
        let mut y_copy = y;
        execute!(stdout(), SavePosition).expect("\x07failed to save position\x07");
        for _ in 0..line_count {
            execute!(stdout(), MoveTo(x, y_copy)).expect("\x07failed to move cursor\x07");
            execute!(stdout(), Clear(ClearType::UntilNewLine)).expect("\x07failed to clear\x07");
            execute!(stdout(), RestorePosition).expect("\x07failed to restore cursor\x07");
            y_copy += 1;
        }

        let text = self.to_string();
        let text = text.split("\n").collect::<Vec<&str>>();

        y_copy = y;
        for i in 0..line_count {
            let texts = &text[i as usize];
            execute!(stdout(), MoveTo(x, y_copy)).expect("\x07failed to move cursor\x07");
            execute!(stdout(), Print(texts)).expect("\x07failed to print\x07");
            execute!(stdout(), RestorePosition).expect("\x07failed to restore cursor\x07");
            y_copy += 1;
        }
    }

    pub fn print(&self) {
        self.goline_clear_print();
    }

    pub fn change_pos(&mut self, t_start_pos: (u16, u16)) {
        // first we have to clear the text
        let line_count = get_num_lines_witdh(&self.to_string());

        let (x, y) = self.t_start_pos;
        let mut y_copy = y;
        execute!(stdout(), SavePosition).expect("\x07failed to save position\x07");
        for _ in 0..line_count {
            execute!(stdout(), MoveTo(x, y_copy)).expect("\x07failed to move cursor\x07");
            execute!(stdout(), Clear(ClearType::UntilNewLine)).expect("\x07failed to clear\x07");
            execute!(stdout(), RestorePosition).expect("\x07failed to restore cursor\x07");
            y_copy += 1;
        }

        self.t_start_pos = t_start_pos;
    }

    pub fn change_pos_print(&mut self, t_start_pos: (u16, u16)) {
        self.change_pos(t_start_pos);
        self.print();
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

    pub fn advance(&mut self) {
        self.t_bar.advance();
    }

    pub fn advance_print(&mut self) {
        self.advance();
        self.goline_clear_print();
    }

    pub fn advance_by(&mut self, index: u16) {
        self.t_bar.advance_by(index);
    }

    pub fn advance_by_print(&mut self, index: u16) {
        self.advance_by(index);
        self.goline_clear_print();
    }

    pub fn advance_by_percent(&mut self, percent: f32) {
        self.t_bar.advance_by_percent(percent);
    }

    pub fn advance_by_percent_print(&mut self, percent: f32) {
        self.advance_by_percent(percent);
        self.goline_clear_print();
    }

    // func change this function takes a hashmap
    pub fn change(&mut self, map: HashMap<&str, TextLoadingBarOptions>, print: bool) {
        for (key, value) in map {
            match key {
                "top_text" => {
                    self.change_top_text(value.get_text().to_string());
                }
                "bottom_text" => {
                    self.change_bottom_text(value.get_text().to_string());
                }
                "top_text_color" => {
                    self.change_top_text_color(value.get_color());
                }
                "bottom_text_color" => {
                    self.change_bottom_text_color(value.get_color());
                }
                "bar_color" => {
                    self.change_bar_color(value.get_color());
                }
                "all_text_colors" => {
                    self.change_all_text_colors(value.get_color());
                }
                "advance" => {
                    self.advance();
                }
                "advance_by" => {
                    self.advance_by(value.get_number());
                }
                "advance_by_percent" => {
                    self.advance_by_percent(value.get_float());
                }
                "change_pos" => {
                    self.change_pos(value.get_pos());
                }
                _ => {
                    println!("\x07{}\x07 is not a valid key", key);
                }
            }
        }
        if print {
            self.print();
        }
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
