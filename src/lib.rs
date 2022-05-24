pub use colored::Color;
use colored::Colorize;
use std::fmt;

#[derive(Debug, Clone)]
pub struct LoadingBar {
    pub len: u64,
    index: u64,
    pub done: bool,
    color: Option<colored::Color>,
    pub space_left: u64,
    half: bool,
}
impl LoadingBar {
    pub fn new(len: u64, color: Option<colored::Color>) -> LoadingBar {
        LoadingBar {
            len,
            index: 0,
            done: false,
            color,
            space_left: len,
            half: false,
        }
    }

    pub fn advance(self: &mut LoadingBar) {
        self.adv_index(1);
    }

    pub fn advance_by(self: &mut LoadingBar, index: u64) {
        self.adv_index(index);
    }

    pub fn advance_by_percent(&mut self, percentage: f64) {
        if percentage > 100.0 {
            panic!("percentage must be between 0 and 100");
        }
        let index = (self.len as f64 * percentage / 100.0) as u64;
        let reminder = (self.len as f64 * percentage % 100.0) as u64;
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

    pub fn change_color(self: &mut LoadingBar, color: Option<colored::Color>) {
        self.color = color
    }
    fn adv_index(&mut self, adv_val: u64) {
        if self.index + adv_val <= self.len {
            self.index += adv_val;
            self.done = false;
            self.space_left = self.len - self.index;
            if self.space_left == 0 {
                self.done = true;
            }
        }  else {
            panic!("adv_val must be less than or equal to len");
        }
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
            "\r[{}]",
            bar.color(self.color.unwrap_or(colored::Color::White)) // if we have a color, use it, otherwise use white
        )
    }
}
