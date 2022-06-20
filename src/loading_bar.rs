//! Home of the LoadingBar methods and its associated structs and enums.
use colored::{Color, Colorize};
use crossterm::cursor::{RestorePosition, SavePosition};

use std::collections::HashMap;
use std::fmt;
use std::io::stdout;

use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};

pub enum LoadingBarOptions {
    Color(Option<Color>),
    Number(u16),
    Float(f32),
    Pos(u16, u16),
    None,
}

impl LoadingBarOptions {
    fn get_color(&self) -> Option<Color> {
        match self {
            LoadingBarOptions::Color(color) => *color,
            _ => None,
        }
    }

    fn get_number(&self) -> u16 {
        match self {
            LoadingBarOptions::Number(number) => *number,
            _ => 0,
        }
    }

    fn get_float(&self) -> f32 {
        match self {
            LoadingBarOptions::Float(float) => *float,
            _ => 0.0,
        }
    }

    fn get_pos(&self) -> (u16, u16) {
        match self {
            LoadingBarOptions::Pos(x, y) => (*x, *y),
            _ => (0, 0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoadingBar {
    pub(super) len: u16,
    pub(super) index: u16,
    pub(super) done: bool,
    pub(super) color: Option<colored::Color>,
    pub(super) space_left: u16,
    pub(super) half: bool,
    start_pos: (u16, u16),
    pub(super) character: char,
    pub(super) last_character: char,
    pub(super) bracket_color: Option<colored::Color>,
}

/// # These methods are for basic loading bar creation and manipulation.
impl LoadingBar {
    /// this is used to create a new loading bar
    /// # Arguments Len: Length of the loading bar
    /// # Arguments Color: Color of the loading bar, if None, it will be white
    /// # Arguments Pos: Position of the loading bar in the terminal using (x, y)
    pub fn new(len: u16, color: Option<colored::Color>, start_pos: (u16, u16)) -> LoadingBar {
        LoadingBar {
            len,
            index: 0,
            done: false,
            color,
            space_left: len,
            half: false,
            start_pos,
            character: '\u{2589}',
            last_character: '\u{258c}',
            bracket_color: None,
        }
    }

    /// this is used to change the position of the loading bar in the terminal using (x, y)
    /// # Arguments Pos: Position of the loading bar in the terminal using (x, y)
    /// it will cleear the old position and move to the new position (but not print anything)
    pub fn change_pos(&mut self, pos: (u16, u16)) {
        // clear the old bar
        let (x, y) = self.start_pos;
        execute!(stdout(), SavePosition).expect("\x07could not save cursor position\x07");
        execute!(stdout(), MoveTo(x, y)).expect("\x07could not move cursor\x07");
        execute!(stdout(), Clear(ClearType::UntilNewLine)).expect("\x07could not clear line\x07");
        execute!(stdout(), RestorePosition).expect("\x07could not restore cursor position\x07");
        self.start_pos = pos;
    }

    /// this is the same as change_pos but it also prints the the loading bar
    pub fn change_pos_print(&mut self, pos: (u16, u16)) {
        self.change_pos(pos);
        self.print();
    }

    pub fn advance(&mut self) {
        self.adv_index(1);
    }

    pub fn advance_print(&mut self) {
        self.adv_index_print(1);
    }

    pub fn advance_by(&mut self, index: u16) {
        self.adv_index(index);
    }

    pub fn advance_by_print(&mut self, index: u16) {
        self.adv_index_print(index);
    }

    pub fn change_character_type(&mut self, character: char) {
        self.character = character;
    }

    pub fn change_last_character(&mut self, character: char) {
        self.last_character = character;
    }

    pub fn change_bracket_color(&mut self, color: Option<colored::Color>) {
        self.bracket_color = color;
    }

    pub fn change_bracket_color_print(&mut self, color: Option<colored::Color>) {
        self.change_bracket_color(color);
        self.print();
    }

    pub fn change_last_character_print(&mut self, character: char) {
        self.change_last_character(character);
        self.print();
    }

    pub fn change_character_type_print(&mut self, character: char) {
        self.change_character_type(character);
        self.print();
    }

    pub fn advance_by_percent(&mut self, percentage: f32) {
        if percentage > 100.0 {
            panic!("\x07percentage must be between 0 and 100\x07");
        }
        let index = (self.len as f32 * percentage / 100.0) as u16;
        let reminder = (self.len as f32 * percentage % 100.0) as u16;
        if self.half {
            match reminder {
                0 => {
                    self.adv_index(index);
                }
                _ => {
                    self.adv_index(index + 1);
                    self.half = false;
                }
            }
        } else {
            match reminder {
                0 => {
                    self.adv_index(index);
                }
                _ => {
                    self.adv_index(index);
                    self.half = true;
                }
            }
        }
    }

    pub fn advance_by_percent_print(&mut self, percentage: f32) {
        self.advance_by_percent(percentage);
        self.goline_clear_print()
    }

    pub fn change_color(&mut self, color: Option<colored::Color>) {
        self.color = color;
    }

    pub fn change_color_print(&mut self, color: Option<colored::Color>) {
        self.change_color(color);
        self.goline_clear_print()
    }

    fn adv_index(&mut self, adv_val: u16) {
        if self.index + adv_val <= self.len {
            self.index += adv_val;
            self.done = false;
            self.space_left = self.len - self.index;
            if self.space_left == 0 {
                self.done = true;
            }
        } else {
            panic!("\x07 You can't advance more than the length of the bar\x07");
        }
    }

    pub fn change(&mut self, map: HashMap<&str, LoadingBarOptions>, print: bool) {
        for (key, value) in map {
            match key {
                "color" => {
                    self.color = value.get_color();
                }
                "pos" => {
                    self.change_pos(value.get_pos());
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
                _ => {
                    panic!("\x07{} is not a valid option\x07", key);
                }
            }
        }
        if print {
            self.print()
        }
    }

    fn adv_index_print(&mut self, add_val: u16) {
        self.adv_index(add_val);
        self.goline_clear_print();
    }

    fn goline_clear_print(&self) {
        let (x, y) = self.start_pos;
        execute!(stdout(), SavePosition).expect("\x07could not save cursor position\x07");
        execute!(stdout(), MoveTo(x, y)).expect("\x07could not move cursor\x07");
        execute!(stdout(), Clear(ClearType::UntilNewLine)).expect("\x07could not clear line\x07");
        execute!(stdout(), Print(&self)).expect("\x07could not print\x07");
        execute!(stdout(), RestorePosition).expect("\x07failed to restore cursor\x07");
    }

    pub fn print(&self) {
        self.goline_clear_print();
    }
}

impl fmt::Display for LoadingBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut bar = String::new();
        for _ in 0..self.index {
            bar.push('\u{2589}');
        }
        if self.half {
            bar.push('\u{258c}');
        }
        for _ in self.index..self.len {
            bar.push(' ');
        }
        write!(
            f,
            "[{}]",
            bar.color(self.color.unwrap_or(colored::Color::White)) // if we have a color, use it, otherwise use white
        )
    }
}

mod auto_run {
    use super::LoadingBar;
    use crate::{Color, Types};
    use std::{collections::HashMap, fmt, marker, thread, time::Duration};

    /// # these methods/functions are used to auomatically generate and or update the loading bar
    /// Note: because I don't fully understand threads and mutexes, you can't acces the loading bar once it's been used in one of these methods.
    impl LoadingBar {
        pub fn auto_run(
            time_in_seconds: u16,
            len: u16,
            start: u16,
            color: Option<colored::Color>,
            start_pos: (u16, u16),
        ) {
            if start >= len {
                println!();
                panic!("\x07start must be less than len\x07");
            }
            // find the amount of time that has per incremen
            let self_clone = LoadingBar {
                len,
                index: start,
                done: false,
                color,
                space_left: len - start,
                half: false,
                start_pos,
                character: '\u{2589}',
                last_character: '\u{258c}',
                bracket_color: None

            };
            LoadingBar::auto_run_from(self_clone, time_in_seconds)
        }

        pub fn auto_run_change(
            change: Vec<Option<Color>>,
            time_in_seconds: u16,
            len: u16,
            start: u16,
            start_pos: (u16, u16),
        ) {
            if start >= len {
                println!();
                panic!("\x07start must be less than len\x07");
            }
            let mut self_clone = LoadingBar::new(len, change[0], start_pos);
            self_clone.advance_by(start);
            LoadingBar::auto_run_from_change(self_clone, change, time_in_seconds)
        }

        pub fn auto_run_from(mut loading_bar: LoadingBar, time_in_seconds: u16) {
            let index = time_in_seconds as f32 / (loading_bar.space_left + 1) as f32;
            loading_bar.print();
            thread::spawn(move || {
                for _ in 0..(loading_bar.space_left) {
                    loading_bar.advance_print();
                    thread::sleep(Duration::from_secs_f32(index));
                }
            });
        }
        pub fn auto_run_change_points(
            change: HashMap<u16, Option<Color>>,
            time_in_seconds: u16,
            len: u16,
            start: u16,
            start_pos: (u16, u16),
        ) {
            if start >= len {
                println!();
                panic!("\x07start must be less than len\x07");
            }
            let mut self_clone = LoadingBar::new(len, None, start_pos);
            self_clone.advance_by(start);
            LoadingBar::auto_run_from_change_points(
                self_clone,
                change,
                time_in_seconds,
                Types::Index,
            )
        }

        pub fn auto_run_from_change(
            loading_bar: LoadingBar,
            change: Vec<Option<Color>>,
            time_in_seconds: u16,
        ) {
            let change_len = change.len() as u16;
            crate::get_indexes(change_len, loading_bar.space_left + 1, loading_bar.len);
            let change_color = crate::get_index_and_value(
                change_len,
                loading_bar.space_left + 1,
                loading_bar.len,
                &change,
            );
            LoadingBar::auto_run_from_change_points(
                loading_bar,
                change_color,
                time_in_seconds,
                Types::Index,
            )
        }

        // TODO: make a similar function in TextLoadingBar and each one respective non from function
        pub fn auto_run_from_change_points<T, U>(
            mut loading_bar: LoadingBar,
            change: HashMap<T, U>,
            time_in_seconds: u16,
            type_change: Types,
        ) where
            T: Copy + fmt::Debug,
            u16: From<T>,
            f32: From<T>,
            U: Copy + fmt::Debug + marker::Send + 'static,
            Option<Color>: From<U>,
        {
            let index = time_in_seconds as f32 / (loading_bar.space_left + 1) as f32;
            let mut total = loading_bar.len - (loading_bar.space_left);
            let new_hash = crate::generic_to_u16(loading_bar.len, change, type_change);
            loading_bar.print();
            thread::spawn(move || {
                for _ in 0..(loading_bar.space_left) {
                    total += 1;
                    if new_hash.contains_key(&total) {
                        loading_bar.color = Option::<Color>::from(new_hash[&total]);
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
    use super::LoadingBar;
    impl LoadingBar {
        // TODO: implement change at type functions
    }
}
