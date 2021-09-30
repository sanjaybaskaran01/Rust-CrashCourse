<h1 align="center"> Rust Crash Course </h1>

<h2> print.rs </h2>

```rust
pub fn run() {
    // Print to console
    println!("Hello from print.rs");
    // Basic Formatting
    println!("{} is from {}", "Sanjay", "india");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to code!",
        "Sanjay", "india"
    );
    // Named Arguments
    println!("{name} likes to play {activity}",name="Sanjay",activity="Football");

}
```
