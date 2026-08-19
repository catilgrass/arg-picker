<h1 style="display: flex; align-items: center; gap: 8px; flex-wrap: nowrap;">
  --arg-picker=
  <img src="https://img.shields.io/github/stars/catilgrass/arg-picker.svg?style=social&label=Star" alt="GitHub stars" />
</h1>

<p style="display: flex; align-items: center; gap: 8px; flex-wrap: nowrap;">
  <img src="https://img.shields.io/crates/v/arg-picker?style=flat-square" alt="Crates.io version" />
  <img src="https://img.shields.io/crates/d/arg-picker?style=flat-square" alt="Crates.io downloads" />
  <img src="https://img.shields.io/crates/l/arg-picker?style=flat-square" alt="Crates.io license" />
  <img src="https://img.shields.io/crates/size/arg-picker?style=flat-square" alt="Crates.io size" />
</p>

A lightweight, type-safe Rust CLI argument parser

## Usage

`arg-picker` provides an extremely simple API—just a simple declaration is all it takes to extract types from your argument list:

```rust
use arg_picker::prelude::*;

// User input: greet --name Bob --age 24
let args: Vec<&str> = vec!["--name", "Bob", "--age", "24"];

// Parse
let (name, age) = args
    .pick(&arg![name: String])
    .or(|| "Alice".to_string())
    .pick(&arg![age: i32])
    .or(|| 24)
    .post(|num| num.clamp(0, 120))
    .unwrap();

// Assert
assert_eq!(name, "Bob".to_string());
assert_eq!(age, 24);
```

## Adding `arg-picker` to Your Project

Add the following to your `Cargo.toml`

```toml
# Cargo.toml
[dependencies]
arg-picker = "0.3"
```

or run `cargo add`

```bash
cargo add arg-picker
```
