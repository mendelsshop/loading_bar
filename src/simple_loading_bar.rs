use std::fmt::Display;

use colored::Colorize;

/// like a loading bar, but only uses `'\r'` instead of `crossterm::cursor` to update the loading bar
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
    pub fn new(
        len: u16,
        index: u16,
        done: bool,
        color: Option<colored::Color>,
        space_left: u16,
        half: bool,
    ) -> SimpleLoadingBar {
        SimpleLoadingBar {
            len,
            index,
            done,
            color,
            space_left,
            half,
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
