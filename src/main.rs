use loading_bar::LoadingBar;
use std::{io::Write, thread, time};

fn main() {
    let mut bar = LoadingBar::new(100);

    print!("\r{}", bar);
    std::io::stdout().flush().unwrap();
    for _i in 0..5 {
        if bar.done {
            break;
        }
        let ten_millis = time::Duration::from_secs(1);
        thread::sleep(ten_millis);
        bar.advance_by_percent(25.0);
        print!("\r{}", bar);
        // flush stdout or else wont work properly
        std::io::stdout().flush().unwrap();
    }
}
