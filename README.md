# loading_bar

### Installation

Put the following in your Cargo.toml file under `[dependencies]`: 

```toml
[dependencies]
"loading_bar" = { git = 'https://github.com/mendelsshop/loading_bar' }
```

### For a full example, see the [examples](https://github.com/mendelsshop/load_test).

This my "Awesome" loading bar library.
<br>
The main.rs shows a simple example of how to use the library.
<br>
The loading bar is a simple progress bar that can be used to show the progress of a long running process.
<br>

## You can create a progress bar with the following code:
```rust
use loading_bar::LoadingBar;
use colored::Color; // so we can use the `Color` enum

bar = LoadingBar::new(100, None); // this creates a new loading bar with 100 steps and the default color

bar = LoadingBar::new(100, Some(colored::Color::Green)); // this creates a new loading bar with 100 steps and the green color
``` 
### Note the following examples assume that you have imported the [`colored`](https://crates.io/crates/colored) and [`loading_bar`](https://github.com/mendelsshop/loading_bar) crate.
## You can incremt the progress of the loading bar with the following code:

```rust
bar.advance(); // this increments the loading bar by 1 step
// takes no paramters

bar.advance_by(10); // this increments the loading bar by 10 steps
// takes a paramter of u64

bar.advance_by_percent(100); // this increments the loading bar by 100%
// takes a paramter of f64
```

## To change the color:

```rust
bar.change_color(colored::Color::Red); // this changes the color of the loading bar to red
```

### Note:
color can be found at https://docs.rs/colored/2.0.0/colored/enum.Color.html.
