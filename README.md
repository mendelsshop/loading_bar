<div align="center">

# <img src="logo.png" width="500" height="500">

# loading_bar 
</div>

[![crates.io](https://img.shields.io/crates/v/loading_bar.svg?label=latest%20version)](https://crates.io/crates/loading_bar)
[![docs.rs](https://img.shields.io/docsrs/loading_bar?logo=Docs.rs)](https://docs.rs/loading_bar/latest/loading_bar)
[![Crates.io](https://img.shields.io/crates/d/loading_bar?label=crates.io%20downloads)](https://crates.io/crates/loading_bar)

## About:
This my "Awesome" loading bar library.
<br>
The loading bar is a simple progress bar that can be used to show the progress of a long running process.
<br>
And different ways to advance the loading bar.
<br>
## Documentation:
[docs.rs](https://docs.rs/loading_bar/latest/loading_bar), [Github Pages](https://mendelsshop.github.io/loading_bar/doc/loading_bar/index.html)
<br>
[LoadingBar](https://github.com/mendelsshop/loading_bar#LoadingBar),
[TextLoadingBar](https://github.com/mendelsshop/loading_bar#TextLoadingBar),
[SimpleLoadingBar](https://github.com/mendelsshop/loading_bar#SimpleLoadingBar)

## Installation:
Put the following in your Cargo.toml file under `[dependencies]`: 
```toml
[dependencies]
loading_bar = "1.1.0"
```
Or the following if you want to test the latest features:

```toml
[dependencies]
"loading_bar" = { git = 'https://github.com/mendelsshop/loading_bar' }
```

### For a full example, see the [example](https://github.com/mendelsshop/load_test) repo.

# LoadingBar
## You can create a progress bar with the following code:
```rust
use loading_bar::loading_bar::LoadingBar;

use loading_bar::{loading_bar::LoadingBar, Color}; // if you want to use the color feature

bar = LoadingBar::new(100, None, (0, 0)); // this creates a new loading bar with 100 steps and the default color at position 0,0

bar = LoadingBar::new(100, Some(Color::Green), (0, 0)); // this creates a new loading bar with 100 steps and the green color
``` 
### Note the following examples assume that you have imported the [`loading_bar`](https://crates.io/crates/loading_bar) crate.

## Show the progress of the loading bar:
```rust
bar.print(); // this shows the loading bar
```

## You can increment the progress of the loading bar with the following code:

```rust
bar.advance(); // this increments the loading bar by 1 step
// takes no paramters

bar.advance_by(10); // this increments the loading bar by 10 steps
// takes a paramter of u16

bar.advance_by_percent(100); // this increments the loading bar by 100%
// takes a paramter of f32

// printsing and updating the loading bar
bar.advance_print(); // this increments the loading bar by 1 step and prints the current progress (each method has a print option)
```

## To change the color:

```rust
bar.change_color(Some(Color::Red)); // this changes the color of the loading bar to red
// the change_color does not immediately change the color of the loading bar, it only changes the color when the next step is incremented when you print the bar next
bar.change_color(None); // changes the color to white (the default color)
```

## We can have a loading bar that that is automatically instaciated and incremented (We have no control over updating it manually):
```rust
LoadingBar::auto_run(10, 50, 10, Some(Color::Red), (0, 0)); // this creates a new loading bar lasts 10 seconds, with a length of 50, and starts at 10 bars with the color red.
LoadingBar::auto_run(10, 50, 10, None, (0, 0)); // as above but with the default color
```

## We can have a loading bar that is already instaciated and then passed to a function that will increment it automatically like an auto run function:
```rust
let mut bar = LoadingBar::new(10, Some(Color::Red), (0, 0)); // this creates a new loading bar with 10 steps and the red color
bar.auto_run_from(bar, 10); // this takes ownership of the bar and increments automatically to the end in a duration of 10 seconds
```

# TextLoadingBar
Not documented yet.
# SimpleLoadingBar
Not documented yet.
# Notes:
- color documentation and source can be found at https://docs.rs/colored/2.0.0/colored/enum.Color.html.
- This uses the [`crossterm`](https://docs.rs/crossterm/latest/crossterm/) crate, so when using this crate you should understand how to use the aforementioned crate (and possibly import and use it for resseting the lines).

# Credits:
- [`Mendel's Shop`](https://github.com/mendelsshop.com) - (Me) for creating it.

- [`Ryan`](https://github.com/BeaconBrigade) - For all the bugs any typos they caught and suggestions they had.
