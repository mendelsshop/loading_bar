//! Home of the TextLoadingBar methods and its associated structs and enums.

use std::{
    collections::HashMap,
    fmt::{self, Display},
    io::stdout,
};

use crate::loading_bar::LoadingBar;
pub use auto_run::{TextLoadingBarAutoOptions, TextLoadingBarAutoPoint};
pub use colored::Color;

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
    Character(char),
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
            TextLoadingBarOptions::Color(color) => *color,
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
    fn get_character(&self) -> char {
        match self {
            TextLoadingBarOptions::Character(character) => *character,
            _ => ' ',
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
        let text = text.split('\n').collect::<Vec<&str>>();

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

    pub fn change_bracket_color(&mut self, color: Option<colored::Color>) {
        self.t_bar.change_bracket_color(color);
    }

    pub fn change_bracket_color_print(&mut self, color: Option<colored::Color>) {
        self.t_bar.change_bracket_color(color);
        self.goline_clear_print();
    }

    pub fn change_character_type(&mut self, character: char) {
        self.t_bar.change_character_type(character);
    }

    pub fn change_character_type_print(&mut self, character: char) {
        self.t_bar.change_character_type(character);
        self.goline_clear_print();
    }

    pub fn change_last_character(&mut self, character: char) {
        self.t_bar.change_last_character(character);
    }

    pub fn change_last_character_print(&mut self, character: char) {
        self.t_bar.change_last_character(character);
        self.goline_clear_print();
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

    pub fn change_all_colors(&mut self, color: Option<Color>) {
        self.top_text.color = color;
        self.bottom_text.color = color;
        self.t_bar.color = color;
        self.t_bar.change_bracket_color(color);
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
                "all_colors" => {
                    self.change_all_colors(value.get_color());
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
                "change_character_type" => {
                    self.change_character_type(value.get_character());
                }
                "change_last_character" => {
                    self.change_last_character(value.get_character());
                }
                "change_bracket_color" => {
                    self.change_bracket_color(value.get_color());
                }
                _ => {
                    panic!("\x07{}\x07 is not a valid key", key);
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

mod auto_run {
    use std::{
        collections::HashMap,
        fmt::{self, Display},
        thread,
        time::Duration,
    };

    use colored::Color;

    use crate::{text_loading_bar::TextLoadingBar, Types};
    #[derive(Debug)]
    pub struct TextLoadingBarAutoOptions {
        pub top_text: Vec<String>,
        pub bottom_text: Vec<String>,
        pub top: Vec<Option<Color>>,
        pub bottom: Vec<Option<Color>>,
        pub bar: Vec<Option<Color>>,
    }

    pub struct TextLoadingBarAutoPoint<T> {
        pub top_text: HashMap<T, String>,
        pub bottom_text: HashMap<T, String>,
        pub top: HashMap<T, Option<Color>>,
        pub bottom: HashMap<T, Option<Color>>,
        pub bar: HashMap<T, Option<Color>>,
    }
    enum TextLoadingBarAutoOptionsType {
        TopText,
        BottomText,
        Top,
        Bottom,
        Bar,
    }

    impl Display for TextLoadingBarAutoOptionsType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                TextLoadingBarAutoOptionsType::TopText => write!(f, "top_text"),
                TextLoadingBarAutoOptionsType::BottomText => write!(f, "bottom_text"),
                TextLoadingBarAutoOptionsType::Top => write!(f, "top"),
                TextLoadingBarAutoOptionsType::Bottom => write!(f, "bottom"),
                TextLoadingBarAutoOptionsType::Bar => write!(f, "bar"),
            }
        }
    }

    impl TextLoadingBarAutoOptions {
        pub fn get_len(&self) -> (u16, u16, u16, u16, u16) {
            (
                TextLoadingBarAutoOptions::check(
                    self.top_text.len(),
                    TextLoadingBarAutoOptionsType::TopText,
                ),
                TextLoadingBarAutoOptions::check(
                    self.bottom_text.len(),
                    TextLoadingBarAutoOptionsType::BottomText,
                ),
                TextLoadingBarAutoOptions::check(
                    self.top.len(),
                    TextLoadingBarAutoOptionsType::Top,
                ),
                TextLoadingBarAutoOptions::check(
                    self.bottom.len(),
                    TextLoadingBarAutoOptionsType::Bottom,
                ),
                TextLoadingBarAutoOptions::check(
                    self.bar.len(),
                    TextLoadingBarAutoOptionsType::Bar,
                ),
            )
        }
        fn check(num: usize, types: TextLoadingBarAutoOptionsType) -> u16 {
            if num == 0 {
                panic!("{} is 0", types);
            } else {
                num as u16
            }
        }
    }
    impl TextLoadingBar {
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
            let (top_text_color, bar_color, bottom_text_color) = color;
            let self_clone = TextLoadingBar::new(
                top_text,
                bottom_text,
                len,
                (top_text_color, bar_color, bottom_text_color),
                t_start_pos,
            );
            TextLoadingBar::auto_run_from(self_clone, time_in_seconds)
        }

        pub fn auto_run_change(
            option: TextLoadingBarAutoOptions,
            time_in_seconds: u16,
            len: u16,
            start: u16,
            start_pos: (u16, u16),
        ) {
            if start >= len {
                println!();
                panic!("\x07start must be less than len\x07");
            }
            let mut self_clone = TextLoadingBar::new(
                option.top_text[0].clone(),
                option.bottom_text[0].clone(),
                len,
                (option.top[0], option.bar[0], option.bottom[0]),
                start_pos,
            );
            self_clone.advance_by(start);
            TextLoadingBar::auto_run_from_change(self_clone, option, time_in_seconds)
        }

        // might have to use default fields for texts and colors
        pub fn auto_run_change_points<T>(
            time_in_seconds: u16,
            len: u16,
            start: u16,
            start_pos: (u16, u16),
            change: TextLoadingBarAutoPoint<T>,
            type_change: Types,
        ) where
            T: Copy + fmt::Debug,
            u16: From<T>,
            f32: From<T>,
        {
            // TODO: implement
            if start >= len {
                println!();
                panic!("\x07start must be less than len\x07");
            }
            let mut self_clone = TextLoadingBar::new(
                "".to_string(),
                "".to_string(),
                len,
                (None, None, None),
                start_pos,
            );
            self_clone.advance_by(start);
            TextLoadingBar::auto_run_from_change_points(
                self_clone,
                change,
                time_in_seconds,
                type_change,
            )
        }
        pub fn auto_run_from(mut text_loading_bar: TextLoadingBar, time_in_seconds: u16) {
            let index = time_in_seconds as f32 / (text_loading_bar.t_bar.space_left + 1) as f32;

            text_loading_bar.print();
            thread::spawn(move || {
                for _ in 0..(text_loading_bar.t_bar.space_left) {
                    text_loading_bar.advance_print();
                    thread::sleep(Duration::from_secs_f32(index));
                }
            });
        }

        pub fn auto_run_from_change(
            text_loading_bar: TextLoadingBar,
            option: TextLoadingBarAutoOptions,
            time_in_seconds: u16,
        ) {
            let (top_len, bottom_len, bar_color_len, top_color_len, bottom_color_len) =
                option.get_len();
            // find the the bar index(s) for each variable we just took from the option struct

            let change = TextLoadingBarAutoPoint {
                bottom: crate::get_index_and_value(
                    bottom_color_len,
                    text_loading_bar.t_bar.space_left + 1,
                    text_loading_bar.t_bar.len,
                    &option.bottom,
                ),
                top: crate::get_index_and_value(
                    top_color_len,
                    text_loading_bar.t_bar.space_left + 1,
                    text_loading_bar.t_bar.len,
                    &option.top,
                ),
                top_text: crate::get_index_and_value(
                    top_len,
                    text_loading_bar.t_bar.space_left + 1,
                    text_loading_bar.t_bar.len,
                    &option.top_text,
                ),
                bottom_text: crate::get_index_and_value(
                    bottom_len,
                    text_loading_bar.t_bar.space_left + 1,
                    text_loading_bar.t_bar.len,
                    &option.bottom_text,
                ),
                bar: crate::get_index_and_value(
                    bar_color_len,
                    text_loading_bar.t_bar.space_left + 1,
                    text_loading_bar.t_bar.len,
                    &option.bar,
                ),
            };
            TextLoadingBar::auto_run_from_change_points(
                text_loading_bar,
                change,
                time_in_seconds,
                Types::Index,
            )
        }

        pub fn auto_run_from_change_points<T>(
            mut loading_bar: TextLoadingBar,
            change: TextLoadingBarAutoPoint<T>,
            time_in_seconds: u16,
            type_change: Types,
        ) where
            T: Copy + fmt::Debug,
            u16: From<T>,
            f32: From<T>,
        {
            // TODO: implement this
            let index = time_in_seconds as f32 / (loading_bar.t_bar.space_left + 1) as f32;
            let mut total = loading_bar.t_bar.len - (loading_bar.t_bar.space_left);
            let top = crate::generic_to_u16(loading_bar.t_bar.len, change.top_text, type_change);
            let bottom =
                crate::generic_to_u16(loading_bar.t_bar.len, change.bottom_text, type_change);
            let bar_color = crate::generic_to_u16(loading_bar.t_bar.len, change.bar, type_change);
            let top_color = crate::generic_to_u16(loading_bar.t_bar.len, change.top, type_change);
            let bottom_color =
                crate::generic_to_u16(loading_bar.t_bar.len, change.bottom, type_change);
            loading_bar.print();
            thread::spawn(move || {
                for _ in 0..(loading_bar.t_bar.space_left) {
                    total += 1;
                    if top.contains_key(&total) {
                        loading_bar.top_text.text = top[&total].clone();
                    }
                    if bottom.contains_key(&total) {
                        loading_bar.bottom_text.text = bottom[&total].clone();
                    }
                    if bar_color.contains_key(&total) {
                        loading_bar.t_bar.color = bar_color[&total];
                    }
                    if top_color.contains_key(&total) {
                        loading_bar.top_text.color = top_color[&total];
                    }
                    if bottom_color.contains_key(&total) {
                        loading_bar.bottom_text.color = bottom_color[&total];
                    }

                    loading_bar.advance();
                    thread::sleep(Duration::from_secs_f32(index));
                    loading_bar.print()
                }
            });
        }
    }
}

mod change_at {
    use crate::text_loading_bar::TextLoadingBar;
    impl TextLoadingBar {
        // TODO: implement change at type functions
    }
}
