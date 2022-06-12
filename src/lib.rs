pub use colored::Color;
use colored::Colorize;
use crossterm::cursor::{RestorePosition, SavePosition};

use std::collections::HashMap;
use std::fmt;
use std::io::stdout;

pub mod text_loading_bar;
pub use auto_run::Types;

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
            LoadingBarOptions::Color(color) => color.clone(),
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
    pub len: u16,
    index: u16,
    pub done: bool,
    color: Option<colored::Color>,
    pub space_left: u16,
    half: bool,
    start_pos: (u16, u16),
}
impl LoadingBar {
    pub fn new(len: u16, color: Option<colored::Color>, start_pos: (u16, u16)) -> LoadingBar {
        LoadingBar {
            len,
            index: 0,
            done: false,
            color,
            space_left: len,
            half: false,
            start_pos,
        }
    }

    pub fn change_pos(&mut self, pos: (u16, u16)) {
        // clear the old bar
        let (x, y) = self.start_pos;
        execute!(stdout(), SavePosition).expect("\x07could not save cursor position\x07");
        execute!(stdout(), MoveTo(x, y)).expect("\x07could not move cursor\x07");
        execute!(stdout(), Clear(ClearType::UntilNewLine)).expect("\x07could not clear line\x07");
        execute!(stdout(), RestorePosition).expect("\x07could not restore cursor position\x07");
        self.start_pos = pos;
    }

    pub fn change_pos_print(&mut self, pos: (u16, u16)) {
        self.change_pos(pos);
        self.print();
    }

    pub fn advance(self: &mut LoadingBar) {
        self.adv_index(1);
    }

    pub fn advance_print(self: &mut LoadingBar) {
        self.adv_index_print(1);
    }

    pub fn advance_by(self: &mut LoadingBar, index: u16) {
        self.adv_index(index);
    }

    pub fn advance_by_print(self: &mut LoadingBar, index: u16) {
        self.adv_index_print(index);
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

    pub fn change_color(self: &mut LoadingBar, color: Option<colored::Color>) {
        self.color = color;
    }

    pub fn change_color_print(self: &mut LoadingBar, color: Option<colored::Color>) {
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

fn get_indexes(num: u16, left: u16, len: u16) -> HashMap<u16, u16> {
    let mut indexes = HashMap::new();
    // get the indexes based on left
    let done = len - left;
    let index = left / num;
    for i in 1..num {
        indexes.insert(done + (index * i), i);
    }
    indexes
}

fn get_index_and_value<T>(num: u16, left: u16, len: u16, value: &Vec<T>) -> HashMap<u16, T>
where
    T: Clone,
{
    let mut index_and_value = HashMap::new();
    let done = len - left;
    let index = left / num;
    for i in 0..num {
        index_and_value.insert(done + (index * i), value[i as usize].clone());
    }
    index_and_value
}

mod auto_run {
    use crate::{Color, LoadingBar};
    use std::{collections::HashMap, fmt};
    #[derive(PartialEq)]
    pub enum Types {
        Percent,
        Index,
    }

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
            let mut self_clone = LoadingBar::new(len, change[0].clone(), start_pos);
            self_clone.advance_by(start);
            LoadingBar::auto_run_from_change(self_clone, change, time_in_seconds)
        }

        pub fn auto_run_from(mut loading_bar: LoadingBar, time_in_seconds: u16) {
            let index = time_in_seconds as f32 / (loading_bar.space_left + 1) as f32;
            loading_bar.print();
            std::thread::spawn(move || {
                for _ in 0..(loading_bar.space_left) {
                    loading_bar.advance_print();
                    std::thread::sleep(std::time::Duration::from_secs_f32(index));
                }
            });
        }
        pub fn auto_run_change_points<T>(
            time_in_seconds: u16,
            len: u16,
            start: u16,
            start_pos: (u16, u16),
            change: HashMap<T, Option<Color>>,
            type_change: Types,
        ) where
            T: Copy + fmt::Debug,
            u16: From<T>,
            f32: From<T>,
        {
            if start >= len {
                println!();
                panic!("\x07start must be less than len\x07");
            }
            let mut self_clone = LoadingBar {
                len,
                index: start,
                done: false,
                color: None,
                space_left: len - start,
                half: false,
                start_pos,
            };
            self_clone.advance_by(start);
            LoadingBar::auto_run_from_change_points(self_clone, change,  time_in_seconds, type_change)
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
        pub fn auto_run_from_change_points<U>(
            mut loading_bar: LoadingBar,
            change: HashMap<U, Option<Color>>,
            time_in_seconds: u16,
            type_change: Types,
        ) where
            U: Copy + fmt::Debug,
            u16: From<U>,
            f32: From<U>,
        {
            let index = time_in_seconds as f32 / (loading_bar.space_left + 1) as f32;
            let mut total = loading_bar.len - (loading_bar.space_left);
            let mut change_color = HashMap::new();

            if Types::Percent == type_change {
                for (key, value) in change.iter() {
                    let key: u16 = (loading_bar.len as f32 * f32::from(*key) / 100.0) as u16;
                    change_color.insert(key, value.clone());
                }
            } else {
                for (key, value) in change.iter() {
                    let change_key: u16 = u16::from(*key);
                    change_color.insert(change_key, value.clone());
                }

                loading_bar.print();
                std::thread::spawn(move || {
                    for _ in 0..(loading_bar.space_left) {
                        total += 1;
                        if change_color.contains_key(&total) {
                            loading_bar.color = change_color[&total].clone();
                        }

                        loading_bar.advance();
                        std::thread::sleep(std::time::Duration::from_secs_f32(index));
                        loading_bar.print()
                    }
                });
            }
        }
    }
}

mod change_at {
    use crate::LoadingBar;
    impl LoadingBar {
        // TODO: implement change at type functions
    }
}
