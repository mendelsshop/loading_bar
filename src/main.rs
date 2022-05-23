use loading_bar::LoadingBar;
use std::{io::Write, thread, time};

fn main() {
    let mut bar = LoadingBar::new(50, Some(colored::Color::Green));

    print!("{}", bar);
    std::io::stdout().flush().unwrap();
    // flush stdout or else wont work properly

    for _i in 0..5 {
        if bar.done {
            // we dont want to advance the bar if it is done
            break;
        }
        let ten_millis = time::Duration::from_secs(3); // wow what a descriptive name
        thread::sleep(ten_millis); // sleep for 1 seconds
        bar.advance_by_percent(25.0); // advancing by 25%
        print!("{}", bar);
        bar.change_color(colored::Color::Red);
        // flush stdout or else wont work properly
        std::io::stdout().flush().unwrap();
    }
    println!("");
}
