use crate::{Color, LoadingBar};

use crossterm::{
    cursor::{self, Hide, MoveDown, MoveUp, MoveLeft, MoveTo, MoveRight, Show, RestorePosition, SavePosition},
    execute, ExecutableCommand, Result, style::Print,
    terminal::{Clear, ClearType},
};


// we have 2 structs for the main (TextLoadingbar) struct, so its easier to manage the elements such as (TextItem) and what color they should be etc
// for each of these structs we have to have a position, for crossterm::cursor to be able to use and
// we also have to have a color, so we can use the color struct to set the color of the text (which we might have to change the type from Colored::color to crossterm::style::Color)
#[derive(Debug)]
struct TextItem {
    text: String,
    color: Option<Color>,
    top_pos: (u16, u16),
    bottom_pos: (u16, u16),
}

#[derive(Debug)]
struct LoadingBarItem {
    bar: LoadingBar,
    pos: (u16, u16),
}

// in the main struct we have a two text items, one for above the bar and one for below the bar
// we also have a loading bar item, which is the bar itself
// we also have the largest text item, which is the text that is the largest in the bar (used for centering the text)
#[derive(Debug)]
pub struct TextLoadingBar {
    top_text: TextItem,
    bottom_text: TextItem,
    bar: LoadingBarItem,
    largest_text_len: u16,
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
        let top_text_pos = get_num_lines_witdh(&top_text);
        let bottom_text_pos = get_num_lines_witdh(&bottom_text);
        let largest = get_largest_line_width(top_text_pos.0, len, bottom_text_pos.0);
        let (top_text_color, bar_color, bottom_text_color) = color;
        TextLoadingBar {
            top_text: TextItem {
                text: top_text,
                color: top_text_color,
                top_pos: (top_text_pos.0, 0),
                bottom_pos: (top_text_pos.0, top_text_pos.1 - 1),
            },
            bar: LoadingBarItem {
                bar: LoadingBar::new(len, bar_color),
                pos: (len, top_text_pos.1),
            },
            bottom_text: TextItem {
                text: bottom_text,
                color: bottom_text_color,
                top_pos: (bottom_text_pos.0, top_text_pos.1 + 1),
                bottom_pos: (bottom_text_pos.0, top_text_pos.1 + 1 + bottom_text_pos.1),
            },
            largest_text_len: largest,
        }
    }
}

// this function is used to get the number of lines and the width of the largest line
fn get_num_lines_witdh(text: &str) -> (u16, u16) {
    let mut num_lines = 1;
    let mut width = 0;
    for c in text.chars() {
        if c == '\n' {
            num_lines += 1;
        } else {
            width += 1;
        }
    }
    (width, num_lines)
}

// this function is used to get the largest line width of the text
fn get_largest_line_width(top: u16, bar: u16, bottom: u16) -> u16 {
    let mut largest = top;
    if bar > largest {
        largest = bar;
    }
    if bottom > largest {
        largest = bottom;
    }
    largest
}
