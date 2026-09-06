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

## `arg-picker` can also...

### Customize the parsing style:

You can call [ParserStyle::set_global_style](https://docs.rs/arg-picker/latest/arg_picker/parselib/struct.ParserStyle.html) before parsing commands to set the global theme style. Three styles are provided by default:

| Style                                                                                                    | Flag Examples    | Argument Separator | Pure Positional Marker | Case Sensitive |
| -------------------------------------------------------------------------------------------------------- | ---------------- | ------------------ | ---------------------- | -------------- |
| [UNIX_STYLE](https://docs.rs/arg-picker/latest/arg_picker/parselib/constant.UNIX_STYLE.html)             | `-f` or `--flag` | `SPACE` or `=`     | Content after `--`     | Yes            |
| [POWERSHELL_STYLE](https://docs.rs/arg-picker/latest/arg_picker/parselib/constant.POWERSHELL_STYLE.html) | `-F` or `-Flag`  | `SPACE` or `:`     | Content after `--`     | No             |
| [WINDOWS_STYLE](https://docs.rs/arg-picker/latest/arg_picker/parselib/constant.WINDOWS_STYLE.html)       | `/F` or `/Flag`  | `SPACE` or `:`     | Content after `--`     | No             |

Usage:

```rust
use arg_picker::parselib::ParserStyle;
use arg_picker::parselib::UNIX_STYLE;
use arg_picker::parselib::POWERSHELL_STYLE;
use arg_picker::parselib::WINDOWS_STYLE;

// --name=Name --age=24 -- Hello
// --name Name --age 24 -- Hello
ParserStyle::set_global_style(&UNIX_STYLE);

// -Name:Name -Age:24 -- Hello
// -Name Name -Age 24 -- Hello
ParserStyle::set_global_style(&POWERSHELL_STYLE);

// /Name:Name /Age:24 -- Hello
// /Name Name /Age 24 -- Hello
ParserStyle::set_global_style(&WINDOWS_STYLE);

// After setting the style, calling Picker will parse arguments according to the specified rules
```

### `Pickable` Derive (feature `derive`)

Enable the optional `derive` feature to get `#[derive(Pickable)]`:

```toml
# Cargo.toml
[dependencies]
arg-picker = { version = "0.3", features = ["derive"] }
```

The derive macro is re-exported at the crate root, so it can be imported directly:

```rust
use arg_picker::Pickable;
```

**1. Deriving for structs**

For a named struct, `#[derive(Pickable)]` generates a `Pickable` implementation.
A field without any `#[arg(...)]` helper is **positional**:

```rust
use arg_picker::Pickable;

#[derive(Pickable)]
struct MyType {
    number: i32,
    name: String,
    info: Option<String>,
}

// ["42", "hello", "extra"]
// -> MyType { number: 42, name: "hello", info: Some("extra") }
```

A field can opt into named arguments with a clap-like `#[arg(...)]` helper:

```rust
use arg_picker::Pickable;

#[derive(Pickable)]
struct MyType {
    #[arg(short = 's', long = "server", aliases = ["host"])]
    name: String,

    #[arg(short)]
    count: i32,

    #[arg(long)]
    user_name: String,
}
```

- `#[arg(short)]` derives the short flag from the first character of the field name.
- `#[arg(long)]` derives the long flag from the field name.
- Explicit forms such as `short = 's'`, `long = "server"`, and `aliases = [...]` are also supported.

The generated `tag` phase is precise: each inner field only claims the arguments it
needs, so a derived composite can be combined with other `Pickable`s without
stealing unrelated flags.

**2. Deriving for enums**

For a unit-only enum, `#[derive(Pickable)]` generates a `SinglePickable`
implementation. User input is converted to `PascalCase` and matched against the
macro-generated `PascalCase` variant name:

```rust
use arg_picker::{Pickable, SinglePickable};

#[derive(Pickable)]
enum Mode {
    FastRun,
    Quiet,
}

assert!(matches!(
    Mode::pick_single(Some("fast_run")),
    arg_picker::PickerArgResult::Parsed(Mode::FastRun)
));
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

## License

Licensed under either of the following at your option:

- **MIT License**
- **Apache License, Version 2.0**

See the [LICENSE-MIT](./LICENSE-MIT) and [LICENSE-APACHE](./LICENSE-APACHE) files for details.
