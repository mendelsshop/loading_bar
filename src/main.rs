#[allow(unused_imports)]
use loading_bar::{
    text_loading_bar::{
        self, TextLoadingBar, TextLoadingBarAutoOptions, TextLoadingBarAutoPoint,
        TextLoadingBarOptions,
    },
    Color, LoadingBar, LoadingBarOptions, Types,
};
// use std::collections::HashMap;
use std::collections::HashMap;
use std::process::Command;
use std::{io::stdout, thread, time};

use crossterm::{
    cursor::{self, MoveTo, RestorePosition, SavePosition},
    execute,
    terminal::{Clear, ClearType},
};

fn main() {
    Command::new("powershell")
        .arg("clear")
        .status()
        .expect("\x07failed to clear screen\x07");
    let mut cursors: (u16, u16) = (0, 0);
    for line in 0..15 {
        execute!(
            stdout(),
            MoveTo(0, line as u16),
            Clear(ClearType::CurrentLine),
            SavePosition
        )
        .expect("\x07failed to clear line\x07");
        cursors = cursor::position().expect("Could not get cursor position");
        println!("{:?}", cursors);
    }
    execute!(stdout(), RestorePosition).expect("\x07failed to restore position\x07");

    TextLoadingBar::auto_run_change(
        TextLoadingBarAutoOptions {
            top_text: vec![
                "hello".to_string(),
                "bye".to_string(),
                "goodbye".to_string(),
            ],
            bottom_text: vec!["pinl".to_string(), "bye".to_string()],
            top: vec![Some(Color::Green), Some(Color::Blue)],
            bottom: vec![Some(Color::Blue), Some(Color::Red)],
            bar: vec![Some(Color::Yellow), Some(Color::Red)],
        },
        10,
        50,
        1,
        (7, 0),
    );

    // let mut bar = LoadingBar::new(50, Some(Color::Green), (7, 13));

    // LoadingBar::auto_run_change(
    //     vec![Some(Color::Green), Some(Color::Blue)],
    //     10,
    //     50,
    //     10,
    //     (7, 10),
    // );
    let mut t = HashMap::new();
    t.insert(10u16, Some(Color::Black));
    t.insert(20u16, Some(Color::Red));
    t.insert(30u16, Some(Color::Green));
    t.insert(40u16, Some(Color::Blue));
    t.insert(50u16, Some(Color::Yellow));
    let mut c = HashMap::new();
    c.insert(10u16, Some(Color::Black));
    c.insert(20u16, Some(Color::Red));
    c.insert(30u16, Some(Color::Green));
    c.insert(40u16, None);
    c.insert(50u16, Some(Color::Yellow));
    let mut b = HashMap::new();
    b.insert(10u16, Some(Color::Black));
    b.insert(20u16, Some(Color::Red));
    b.insert(30u16, Some(Color::Green));
    b.insert(40u16, Some(Color::Blue));
    b.insert(50u16, Some(Color::Yellow));
    let mut a = HashMap::new();
    a.insert(10u16, "hello".to_string());
    a.insert(20u16, "bye".to_string());
    a.insert(30u16, "goodbye".to_string());
    a.insert(40u16, "pinl".to_string());
    a.insert(50u16, "bye".to_string());
    let mut q = HashMap::new();
    q.insert(10u16, "hello".to_string());
    q.insert(20u16, "bye".to_string());
    q.insert(30u16, "goodbye".to_string());
    q.insert(40u16, "pinl".to_string());
    q.insert(50u16, "bye".to_string());
    // LoadingBar::auto_run_change_points(b, 10, 50, 10, (7, 10));
    // let option = TextLoadingBarAutoPoint {
    //     top_text: a,
    //     bottom_text: q,
    //     top: t,
    //     bottom: c,
    //     bar: b,
    // };

    // TextLoadingBar::auto_run_change_points(10, 50, 10, (7, 10), option, Types::Index);
    // let v = vec![Some(Color::Green), Some(Color::Blue)];
    // let v = vec!["hello", "bye"];
    // println!("{:?}", z);
    // LoadingBar::auto_run_from_change_points(bar, t, 10, Types::Index);
    // for i in 0..6 {
    //     if bar.done {
    //         // we dont want to advance the bar if it is done
    //         break;
    //     }

    //     let ten_millis = time::Duration::from_secs_f32(2.0); // wow what a descriptive name
    //     thread::sleep(ten_millis); // sleep for 1 seconds

    //     // bar.advance_by_percent(600.0); // this will panic because were going over a 100%
    //     bar.advance_by_print(8);

    //     match i {
    //         0 => {
    //             bar.change_color_print(Some(Color::Black));
    //         }
    //         1 => {
    //             bar.change_color_print(Some(Color::BrightGreen));
    //         }
    //         2 => {
    //             bar.change_color_print(Some(Color::Red));
    //         }
    //         3 => {
    //             bar.change_color_print(Some(Color::Yellow));
    //         }
    //         _ => {
    //             bar.change_color_print(None);
    //         }
    //     }
    // }

    let (x, y) = cursors;
    execute!(stdout(), MoveTo(x, y)).expect("\x07failed to restore cursor\x07");

    // create a new bar
    // let mut bar = LoadingBar::new(50, Some(Color::Green), (0, 0));
    // let mut options = HashMap::new();
    // options.insert("color", LoadingBarOptions::Color(Some(Color::Green)));
    // options.insert("advance_by", LoadingBarOptions::Number(11));
    // bar.change(options, true);
    // let t = LoadingBar::auto_run_from(bar, 10);

    // let mut text = TextLoadingBar::new(
    //     "Loading".to_string(),
    //     "Loading".to_string(),
    //     50,
    //     (Some(Color::Green), Some(Color::Green), Some(Color::Green)),
    //     (0, 0),
    // );
    // text.print();
    // text.advance_by_print(10);
    // TextLoadingBar::auto_run_from(text, 10);
    for _ in 0..10 {
        thread::sleep(time::Duration::from_secs_f32(2.0));
    }
}
