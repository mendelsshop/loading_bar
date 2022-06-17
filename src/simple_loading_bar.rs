//! Home of the SimpleLoadingBar methods and its associated structs and enums.
use std::fmt::Display;

use colored::Colorize;

/// like a LoadingBar, but only uses `'\r'` instead of `crossterm::cursor` to update the loading bar
/// This may makeit easeir to use especially when scrolling is involved
/// Nothing else can be printed to the same shell/terminal while it is running or else you might get weird results
pub struct SimpleLoadingBar {
    len: u16,
    index: u16,
    done: bool,
    color: Option<colored::Color>,
    space_left: u16,
    half: bool,
}

impl SimpleLoadingBar {
    pub fn new(len: u16, color: Option<colored::Color>) -> SimpleLoadingBar {
        SimpleLoadingBar {
            len,
            index: 0,
            done: false,
            color,
            space_left: len,
            half: false,
        }
    }
    pub fn print(&self) {
        print!("\r{}", self);
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
        self.print()
    }

    pub fn change_color(&mut self, color: Option<colored::Color>) {
        self.color = color;
    }

    pub fn change_color_print(&mut self, color: Option<colored::Color>) {
        self.change_color(color);
        self.print()
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

    fn adv_index_print(&mut self, add_val: u16) {
        self.adv_index(add_val);
        self.print();
    }
}

impl Display for SimpleLoadingBar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            "[\r{}]",
            bar.color(self.color.unwrap_or(colored::Color::White)) // if we have a color, use it, otherwise use white
        )
    }
}

mod auto_run {
    use crate::{Color, Types};

    use super::SimpleLoadingBar;
    use std::{collections::HashMap, fmt, marker, thread, time::Duration};

    impl SimpleLoadingBar {
        pub fn auto_run(time_in_seconds: u16, len: u16, start: u16, color: Option<colored::Color>) {
            if start >= len {
                println!();
                panic!("\x07start must be less than len\x07");
            }
            // find the amount of time that has per incremen
            let self_clone = SimpleLoadingBar {
                len,
                index: start,
                done: false,
                color,
                space_left: len - start,
                half: false,
            };
            SimpleLoadingBar::auto_run_from(self_clone, time_in_seconds)
        }
        pub fn auto_run_change(
            change: Vec<Option<Color>>,
            time_in_seconds: u16,
            len: u16,
            start: u16,
        ) {
            if start >= len {
                println!();
                panic!("\x07start must be less than len\x07");
            }
            let mut self_clone = SimpleLoadingBar::new(len, change[0]);
            self_clone.advance_by(start);
            SimpleLoadingBar::auto_run_from_change(self_clone, change, time_in_seconds)
        }

        pub fn auto_run_change_points(
            change: HashMap<u16, Option<Color>>,
            time_in_seconds: u16,
            len: u16,
            start: u16,
        ) {
            if start >= len {
                println!();
                panic!("\x07start must be less than len\x07");
            }
            let mut self_clone = SimpleLoadingBar::new(len, None);
            self_clone.advance_by(start);
            SimpleLoadingBar::auto_run_from_change_points(
                self_clone,
                change,
                time_in_seconds,
                Types::Index,
            )
        }

        pub fn auto_run_from(mut loading_bar: SimpleLoadingBar, time_in_seconds: u16) {
            let index = time_in_seconds as f32 / (loading_bar.space_left + 1) as f32;
            loading_bar.print();
            thread::spawn(move || {
                for _ in 0..(loading_bar.space_left) {
                    loading_bar.advance_print();
                    thread::sleep(Duration::from_secs_f32(index));
                }
            });
        }
        pub fn auto_run_from_change(
            loading_bar: SimpleLoadingBar,
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
            SimpleLoadingBar::auto_run_from_change_points(
                loading_bar,
                change_color,
                time_in_seconds,
                Types::Index,
            )
        }
        pub fn auto_run_from_change_points<T, U>(
            mut loading_bar: SimpleLoadingBar,
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
