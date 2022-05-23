use std::fmt;
extern crate colored;

// use colored::*;
// use colored::Color;
pub use colored::Color;
use colored::Colorize;


#[derive(Debug)]
pub struct LoadingBar {
    bar: String,
    pub len: u64,
    index: u64,
    pub done: bool,
    color: Option<colored::Color>,
}
impl LoadingBar {
    pub fn new(len: u64, color: Option<colored::Color>) -> LoadingBar {
        LoadingBar {
            bar: string_from_u64(len, ' '),
            len,
            index: 0,
            done: false,
            color: match color {
                Some(c) => Some(c),
                _ => None,
            },
        }
    }

    pub fn advance(self: &mut LoadingBar) {
        self.bar = self.bar.replacen(' ', "\u{25A0}", 1);
        self.index += 1; // increment
        if self.index == self.len {
            self.done = true;
        }
    }

    pub fn advance_by(self: &mut LoadingBar, index: u64) {
        self.bar = self.bar.replacen(' ', "\u{25A0}", index as usize);
        self.index += index; // increment
        if self.index == self.len {
            self.done = true;
        }
    }

    pub fn advance_by_percent(self: &mut LoadingBar, percentage: f64) {
        // we have to get the percentage of the bar
        // and then we have to get the index of the bar
        // that we want to advance
        let index = (self.len as f64 * percentage / 100.0) as u64;
        self.bar = self.bar.replacen(' ', "\u{25A0}", index as usize);
        self.index += index; // increment
        if self.index == self.len {
            self.done = true;
        }
    }

    pub fn change_color(self: &mut LoadingBar, color: colored::Color) {
        self.color = Some(color);
    }
}
impl fmt::Display for LoadingBar {
    // fn fmt(&self, f: &mut fmt::Formatter)
    //     -> fmt::Result {
    //         write!(f, " {0} \n[{1}]\n {2} ", string_from_u64(self.len, '\u{2581}'), self.bar, string_from_u64(self.len, '\u{2594}'))
    //     }
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\r[{}]",
            self.bar.color(self.color.unwrap_or(colored::Color::White))
        )
    }
}
fn string_from_u64(len: u64, replace: char) -> String {
    let mut string = String::with_capacity(len as usize);
    for _i in 0..len {
        string.push(replace);
    }
    // println!("{}", string);
    string
}
