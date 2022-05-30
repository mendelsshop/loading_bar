use std::{
    fmt::{self, Display},
    io::stdout,
};

pub use crate::LoadingBar;
pub use colored::Color;

use colored::Colorize;

use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Print},
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
    bar: LoadingBar,
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
    ) -> TextLoadingBar {
        let (top_text_color, bar_color, bottom_text_color) = color;
        TextLoadingBar {
            top_text: TextItem {
                text: top_text,
                color: top_text_color,
            },
            bar: LoadingBar::new(len, bar_color, (0, 0)),
            bottom_text: TextItem {
                text: bottom_text,
                color: bottom_text_color,
            },
        }
    }

    fn goline_clear_print(&self) {
        let (x, y) = self.bar.start_pos;
        execute!(
            stdout(),
            MoveTo(x, y),
            Clear(ClearType::CurrentLine),
            Print(&self)
        )
        .unwrap();
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
        self.bar.color = color;
    }

    pub fn change_all_text_colors(&mut self, color: Option<Color>) {
        self.top_text.color = color;
        self.bottom_text.color = color;
        self.bar.color = color;
    }

    pub fn advance(&mut self) {
        self.bar.advance();
    }

    pub fn advance_print(&mut self) {
        self.bar.advance();
        self.goline_clear_print();
    }

    pub fn advance_by(&mut self, index: u16) {
        self.bar.advance_by(index);
    }

    pub fn advance_by_print(&mut self, index: u16) {
        self.bar.advance_by_print(index);
    }

    pub fn advance_by_percent(&mut self, percent: f32) {
        self.bar.advance_by_percent(percent);
    }

    pub fn advance_by_percent_print(&mut self, percent: f32) {
        self.bar.advance_by_percent_print(percent);
    }
}

impl Display for TextLoadingBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n",
            self.top_text
                .text
                .color(self.top_text.color.unwrap_or(colored::Color::White)),
            self.bar,
            self.bottom_text
                .text
                .color(self.bottom_text.color.unwrap_or(colored::Color::White)) // if we have a color, use it, otherwise use white
        )
    }
}

