# loading_bar

## About:
This my "Awesome" loading bar library.
<br>
The loading bar is a simple progress bar that can be used to show the progress of a long running process.
<br>
And different ways to advance the loading bar.

## Installation:
Put the following in your Cargo.toml file under `[dependencies]`: 

```toml
[dependencies]
"loading_bar" = { git = 'https://github.com/mendelsshop/loading_bar' }
```

### For a full example, see the [example](https://github.com/mendelsshop/load_test) repo.

## You can create a progress bar with the following code:
```rust
use loading_bar::LoadingBar;

use loading_bar::{LoadingBar, Color}; // if you want to use the color feature

bar = LoadingBar::new(100, None); // this creates a new loading bar with 100 steps and the default color

bar = LoadingBar::new(100, Some(Color::Green)); // this creates a new loading bar with 100 steps and the green color
``` 
### Note the following examples assume that you have imported the [`loading_bar`](https://github.com/mendelsshop/loading_bar) crate.

## You can incremt the progress of the loading bar with the following code:

```rust
bar.advance(); // this increments the loading bar by 1 step
// takes no paramters

bar.advance_by(10); // this increments the loading bar by 10 steps
// takes a paramter of u16

bar.advance_by_percent(100); // this increments the loading bar by 100%
// takes a paramter of f32
```

## To change the color:

```rust
bar.change_color(Some(Color::Red)); // this changes the color of the loading bar to red
// the change_color does not immediately change the color of the loading bar, it only changes the color when the next step is incremented when you print the bar next
bar.change_color(None); // changes the color to white (the default color)
```

## We can have a loading bar that that is automatically instaciated and incremented (We have no control over updating it manually):
```rust
LoadingBar::auto_run(10, 50, 10, Some(Color::Red)); // this creates a new loading bar lasts 10 seconds, with a length of 50, and starts at 10 bars with the color red.
LoadingBar::auto_run(10, 50, 10, None); // as above but with the default color
```

### Note:
color documentation and source can be found at https://docs.rs/colored/2.0.0/colored/enum.Color.html.
