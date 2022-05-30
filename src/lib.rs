pub use colored::Color;
use colored::Colorize;

use std::fmt;
use std::io::{self, stdout, Write};

pub mod text;

use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};

// TODO: make a display function for the struct
// for now, I'll just reprint the struct each time it gets updated
// this might not work for text::TextLoadingBar and we might have to switch this to not print the struct each time it is changed or make additional functions that update the struct without printing it

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
        let (x, y) = self.start_pos;
        execute!(
            stdout(),
            MoveTo(x, y),
            Clear(ClearType::CurrentLine),
            Print(&self)
        )
        .unwrap();
    }

    // TODO have similar functions for with hashmaps or vectors of colors
    // and or text (and each peice's of txt postion top/bottom) for different percentages
    pub fn auto_run(
        time_in_seconds: u16,
        len: u16,
        start: u16,
        color: Option<colored::Color>,
        start_pos: (u16, u16),
    ) {
        if start > len {
            println!();
            panic!("\x07start must be less than len\x07");
        }
        // find the amount of time that has per incremen
        let index = time_in_seconds as f32 / (len - start) as f32;
        let mut self_clone = LoadingBar {
            len,
            index: start,
            done: false,
            color,
            space_left: len - start,
            half: false,
            start_pos,
        };
        let (x, y) = start_pos;
        execute!(
            stdout(),
            MoveTo(x, y),
            Clear(ClearType::CurrentLine),
            Print(&self_clone)
        )
        .unwrap();
        // flush the buffer
        io::stdout().flush().unwrap();
        std::thread::spawn(move || {
            for _ in 0..(self_clone.space_left) {
                self_clone.advance_print();
                std::thread::sleep(std::time::Duration::from_secs_f32(index));
            }
        });
    }

    pub fn change_color(self: &mut LoadingBar, color: Option<colored::Color>) {
        self.color = color;
    }

    pub fn change_color_print(self: &mut LoadingBar, color: Option<colored::Color>) {
        let (x, y) = self.start_pos;
        self.change_color(color);
        execute!(
            stdout(),
            MoveTo(x, y),
            Clear(ClearType::CurrentLine),
            Print(&self)
        )
        .unwrap();
    }

    fn adv_index(&mut self, adv_val: u16) {
        if self.index + adv_val <= self.len {
            self.index += adv_val;
            self.done = false;
            self.space_left = self.len - self.index;
            if self.space_left == 0 {
                self.done = true;
                execute!(stdout(),).unwrap();
            }
        } else {
            panic!("\x07 You can't advance more than the length of the bar\x07");
        }
    }

    fn adv_index_print(&mut self, add_val: u16) {
        self.adv_index(add_val);
        let (x, y) = self.start_pos;
        execute!(
            stdout(),
            MoveTo(x, y),
            Clear(ClearType::CurrentLine),
            Print(&self)
        )
        .unwrap();
        io::stdout().flush().unwrap();
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
