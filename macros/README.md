# Mingling Picker Macros

Procedural macros for [Mingling Picker](https://github.com/mingling-rs/mingling/tree/main/mingling_picker), enabled by the `mingling/picker` feature.

```toml
[dependencies.mingling]
version = "0.3.0"
features = [
    "picker"
]
```

## Provided Macros

### Macro `arg!`

Declares a parameter definition for use with `Picker`'s `.pick()` method:

```rust,ignore
use mingling_picker_macros::arg;

// Named flag with a value
let flag = arg![name: String];

// Named flag with short form
let flag = arg![name: String, 'n'];

// Named flag with alias
let flag = arg![name: String, 'n', "nickname"];

// Positional parameter
let flag = arg![String];

// Flag-only switch parameter
let flag = arg![verbose: Flag];
```

### Derive macro `Pickable`

This derive macro is gated behind the `derive` feature of `arg-picker`.

Generates parsing implementations for structs and enums.

For a named struct, it generates a `Pickable` implementation. All field types
must implement `Pickable`, and at most 32 fields are supported. A field
without `#[arg(...)]` is positional (equivalent to `arg![Type]`):

```rust,ignore
use arg_picker::Pickable;

#[derive(Pickable)]
struct MyType {
    number: i32,
    name: String,
    info: Option<String>,
}

// Parses: ["42", "hello", "extra"]
```

A field can use a clap-like `#[arg(...)]` helper attribute to turn it into a
named argument:

```rust,ignore
#[derive(Pickable)]
struct MyType {
    #[arg(short = 's', long = "server", aliases = ["host"])]
    name: String,
}
```

This makes `name` parseable via `-s`, `--server`, or `--host`. When `long` is
given it replaces the default field-name long flag.

`short` and `long` may also be written without a value, in which case they are
derived from the field name:

```rust,ignore
#[derive(Pickable)]
struct MyType {
    #[arg(short)]       // short = first char of field name
    count: i32,

    #[arg(long)]        // long = field name
    user_name: String,

    #[arg(short, long)] // both derived
    server_name: String,
}
```

This allows `-c`, `--user-name`, `--server-name`, and `-s`.

For an enum, it generates a `SinglePickable` implementation. Every variant
must be a unit variant (no data). The user input is converted to `PascalCase`
and matched against the macro-generated PascalCase variant name:

```rust,ignore
use arg_picker::Pickable;

#[derive(Pickable)]
enum Mode {
    FastRun,
    Quiet,
}
```

### Macro `internal_repeat!` (Internal)

Internal macro used by Picker to generate `PickerPattern1..=32` and their parsing logic. Not intended for direct use.
