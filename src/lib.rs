use std::fmt;

#[derive(Debug)]
pub struct LoadingBar {
    bar: String,
    pub len: u64,
    index: u64,
    pub done: bool,
}
impl LoadingBar {
    pub fn new(len: u64) -> LoadingBar {
        LoadingBar {
            bar: string_from_u64(len, ' '),
            len,
            index: 0,
            done: false,
        }
    }

    pub fn advance(self: &mut LoadingBar) {
        self.bar = self.bar.replacen(' ', "#", 1);
        self.index += 1; // increment
        if self.index == self.len {
            self.done = true;
        }
    }

    pub fn advance_by(self: &mut LoadingBar, index: u64) {
        self.bar = self.bar.replacen(' ', "#", index as usize);
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
        self.bar = self.bar.replacen(' ', "#", index as usize);
        self.index += index; // increment
        if self.index == self.len {
            self.done = true;
        }
    }
}
impl fmt::Display for LoadingBar {
    // fn fmt(&self, f: &mut fmt::Formatter)
    //     -> fmt::Result {
    //         write!(f, " {0} \n[{1}]\n {2} ", string_from_u64(self.len, '\u{2581}'), self.bar, string_from_u64(self.len, '\u{2594}'))
    //     }
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\r[{}]", self.bar)
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
