# loading_bar

This my "Awesome" loading bar library.
<br>
The main.rs shows a simple example of how to use the library.
<br>
The loading bar is a simple progress bar that can be used to show the progress of a long running process.
<br>
You can create a progress bar with the following code:
```rust
use loading_bar::LoadingBar;

bar = LoadingBar::new(100, None); // this creates a new loading bar with 100 steps and the default color
bar = LoadingBar::new(100, Some(colored::Color::Green)); // this creates a new loading bar with 100 steps and the green color
``` 
You can incremt the progress of the loading bar with the following code:
```rust
bar.advance(); // this increments the loading bar by 1 step
// takes no paramters
bar.advance_by(10); // this increments the loading bar by 10 steps
// takes a paramter of u64
bar.advance_by_percent(100); // this increments the loading bar by 100%
// takes a paramter of f64
```

To change the color:
```rust
bar.change_color(colored::Color::Red); // this changes the color of the loading bar to red
```


