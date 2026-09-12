# Changelogs

This file tracks all notable changes to the arg-picker project. Each release entry documents new features, bug fixes, optimizations, and breaking changes, helping developers and users understand the evolution of the parser.

The format follows a human-readable changelog convention, with sections organized by release version and change type.

Any contributor making changes to the project must record their changes in this file under the appropriate release section, using the established format and change type categories _(Features, Fixes, Optimizations, Tests, BREAKING CHANGES, etc.)_.

## TOC

- [Unreleased](#unreleased)
- [Release 0.3.1 (Unreleased)](#031-unreleased)
- [Release 0.3.0 (2026-09-06)](#030-2026-09-06)
- [Release 0.2.0 (2026-08-10)](#020-2026-08-10)

## Contents

---

### Unreleased

#### Fixes:

None

#### Optimizations:

None

#### Features:

None

#### **BREAKING CHANGES** (API CHANGES):

None

---

### 0.3.1 (Unreleased)

#### Fixes:

None

#### Optimizations:

None

#### Features:

1. **[`macros`]** Extended the `internal_repeat!` macro with two new expansion forms in the proc-macro internals (`macros/src/internal_repeat.rs`):

    - **`ident$suffix`** — expands to `ident{current}suffix` (e.g. `With$Arg` → `With1Arg`). The suffix is only consumed when `$` is directly followed by an identifier, after `$+` / `$-` / `$^` are ruled out.
    - **`(group,+)^`** — repeats `max - current` times (with separator), the complement of `+`. Used to pad a fixed-length parameter list with the parameters not consumed by this arity.

    The repeat modifier after a parenthesized group is now computed as an explicit `(repeat_count, consumed)` pair, with `+` yielding `current + 1`, `-` yielding `current.saturating_sub(1)`, and `^` yielding `max.saturating_sub(current)`.

2. **[`func`]** Added a `func` module (re-exported at the crate root) providing a dynamically-dispatched callable type and the `Picker::invoke` entry points for invoking closures with anywhere from 1 to 32 arguments:

    - **`PickerFunction<T0, …, T31, R>`** — an enum with one `WithNArg(Box<dyn FnOnce(...) -> R>)` variant per supported arity. Type parameters not consumed by a given arity are set to `()`. `PickerFunction::from` builds one directly from a closure, inferring the argument types and padding the remaining type parameters with `()`.
    - **`IntoPickerFunction<'a, Args>`** — a hidden (per-arity) helper trait powering `PickerFunction::from`, `Picker::invoke`, and `Picker::invoke_args`. Since a 1-tuple, 2-tuple, … are distinct type constructors, the impls stay disjoint, letting a single `from`/`invoke` accept closures of any arity.
    - **`Picker::invoke(args, func)`** — parses `args` as **positional** arguments in order, calls `func` with the parsed values, and returns `func`'s result. The argument types (and therefore the expected number of positional arguments) are inferred from `func`, with every remaining type parameter set to `()`; `func` may take between 1 and 32 arguments.
    - **`Picker::invoke_args(func)`** (in the new `picker::invoke` module) — the “one-liner” form of `invoke`, taking its arguments from [`std::env::args`](https://doc.rust-lang.org/std/env/fn.args.html) while skipping the program name. Together with `invoke`, it provides a shortcut entry point for small scripts.

    Both entry points are **positional-only** — flags, named arguments, defaults, and routes are not supported (use a `pick(..)` chain for those) — and panic if the positional arguments are missing or fail to parse.

    Both the enum body and the per-arity impls are emitted with the `internal_repeat!` macro (using the `(group,+)^` complement repetition to pad the type parameters not consumed by each arity), and the `clippy::type_complexity` lint is allowed locally inside the module.


#### **BREAKING CHANGES** (API CHANGES):

None

---

### 0.3.0 (2026-09-06)

First standalone release after [arg-picker](https://github.com/catilgrass/arg-picker) was migrated out of the [Mingling](https://github.com/mingling-rs/mingling) workspace.

#### Fixes:

None

#### Optimizations:

None

#### Features:

1. **[`project`]** Migrated the crate out of the Mingling workspace into an independent project (`github.com/catilgrass/arg-picker`). The project now owns its own workspace — `arg-picker` plus the `arg-picker-macros` proc-macro sub-crate — along with dual MIT/Apache-2.0 licensing, docs.rs metadata, and a standalone `test` integration crate. The `mingling_support` feature is retained for optional integration with Mingling.

2. **[`docs`]** Rewrote the README for standalone usage: added a badge header (GitHub stars, crates.io version / downloads / license / size), a minimal chained-API usage example, and `cargo add arg-picker` instructions. The `Picker` entry type now carries a plain doc comment instead of `include_str!("../README.md")`, so the crate documentation no longer depends on the README file's content.

3. **[`build`]** Added a `Makefile` exposing common development tasks — `make build`, `make test`, `make clippy`, `make doc` / `make doc-preview`, and `make check` (build + test + clippy). Each task runs across the picker, macros, and test crates, with clippy enforcing `-D warnings`.

4. **[`builtin`]** Added `SinglePickable` implementations for three more common standard-library types:

    - **`char`** — parses a single-character string.
    - **`OsString`** — parses any string argument into an owned OS string.
    - **`Duration`** — parses seconds (bare numeric input) or common suffixed forms such as `500ms`, `2m`, and `1.5h`.

5. **[`macros`]** Added the `#[derive(Pickable)]` derive macro (gated behind the `derive` feature). For named structs it generates a `Pickable` implementation with up to 32 fields; fields without a `#[arg(...)]` helper are positional (equivalent to `arg![Type]`), while fields may customize the generated argument with a clap-like `#[arg(short = 's', long = "long", aliases = ["a", "b"])]` helper attribute to become named arguments; bare `short` and `long` are also supported and derive from the field name. For unit-only enums it generates a `SinglePickable` implementation that parses user input by converting it to `PascalCase` and matching the macro-generated PascalCase variant name. The derive macro is re-exported at the `arg_picker` crate root and in the prelude.

#### **BREAKING CHANGES** (API CHANGES):

1. **[`pickable`]** Renamed the `MultiPickableWithBoundary` trait to `MultiPickable`. All references to the old trait name — including its blanket impl for `Vec<T>`, all inherent implementations, and the `SinglePickable`-adjacent internal call sites in `pickable`, `value::vec_until`, and related modules — have been updated to the shortened `MultiPickable` name. Public API code referencing `MultiPickableWithBoundary` will fail to compile and must migrate to `MultiPickable`.

2. **[`builtin:bool`]** Changed `bool` from a flag-only `Pickable` to an explicit-value [`SinglePickable`](https://docs.rs/arg-picker/latest/arg_picker/trait.SinglePickable.html). It now parses the case-insensitive strings `"true"` / `"false"` as a normal argument value. The old switch behavior is intentionally **not** preserved; users who need a present/absent boolean flag should use the existing `Flag` type instead.

---

### 0.2.0 (2026-08-10)

Last release developed inside the Mingling workspace (tag [picker-0.2.0](https://github.com/mingling-rs/mingling/tree/picker-0.2.0)).

#### Fixes:

None

#### Optimizations:

1. **[`core`]** Enforced `#![deny(clippy::pedantic)]` and `#![deny(clippy::nursery)]` at the crate root and resolved every resulting lint across all modules:

    - Accessors and builder methods (e.g. on `PickerArgInfo`, `PickerArg`) were converted to `const fn` where possible.
    - `#[must_use]` was added to pure query methods, and `#[inline(always)]` was relaxed to `#[inline]`.
    - Lifetimes were simplified to `'_` and `&Self`-style references replaced repetitive generic forms.

    _No behavioral or API changes — purely stylistic, enforced going forward by the crate-level lint gates._

#### Features:

1. **[`value:paths`]** Added filesystem-aware path wrapper types to `arg_picker::value` for validating arguments against the filesystem at parse time:

    - **`FilePath`** — Wraps `PathBuf`, validated at parse time to exist and be a file.
    - **`NoFilePath`** — Wraps `PathBuf`, validated at parse time to _not_ exist as a file.
    - **`DirPath`** — Wraps `PathBuf`, validated at parse time to exist and be a directory.
    - **`NoDirPath`** — Wraps `PathBuf`, validated at parse time to _not_ exist as a directory.
    - **`SymlinkPath`** — Wraps `PathBuf`, validated at parse time to exist and be a symlink.
    - **`NoSymlinkPath`** — Wraps `PathBuf`, validated at parse time to _not_ exist as a symlink.
    - **`NoPath`** — Wraps `PathBuf`, validated at parse time to have no filesystem entry at all.
    - **`RecursiveFiles`** — Wraps `Vec<PathBuf>`. If given a file path, returns a single-element list; if given a directory path, recursively collects all files (and symlinks) under it.

    All single-path types implement `From<PathBuf>`, `From<&PathBuf>`, `AsRef<Path>`, `Deref<Target = PathBuf>`, `DerefMut`, and `Into<PathBuf>`. `RecursiveFiles` additionally provides `len()`, `is_empty()`, `iter()`, `From<Vec<RecursiveFiles>>` for merging multiple collections, and the `IntoRecursiveFiles` trait for ergonomic combination from `Vec<T>`, `&[T]`, and `[T; N]`.

    Each type implements `SinglePickable` (via the new `builtin::pick_paths` module), performing filesystem validation at parse time and returning `NotFound` when the precondition is not met.

2. **[`picker:parse`]** Added `unwrap_or_default`, `unwrap_or_else`, and `expect` convenience methods to the `internal_repeat!`-generated `PickerPattern<T1, T2, ...>` tuple types in `picker::parse`, reducing boilerplate when parsing directly into tuples:

    - **`unwrap_or_default(self)`** — Returns the parsed values, using `Default::default()` for any missing required arguments. Panics if a route was selected.
    - **`unwrap_or_else<F>(self, op: F)`** — Returns the parsed values, using the provided closure to generate defaults for missing arguments. Panics if a route was selected.
    - **`expect(self, msg: &str)`** — Returns the parsed values, or panics with the given message if a route was selected. Requires `Route: std::fmt::Debug`.

3. **[`picker:result`]** Added the same `unwrap_or_default`, `unwrap_or_else`, and `expect` methods to the generated `PickerResult<T1, T2, ...>` tuple result structs in `picker::result`, complementing the existing `unwrap`, `unpack`, `to_result`, and `to_option` methods.

4. **[`picker:conversion`]** Generalized the `impl From<PickerArg<'a, Type>> for Vec<String>` conversion from requiring `Type: SinglePickable` to accepting any `Type: Pickable<'a>`. Flag-style arguments built from any pickable type can now be expanded into their possible flag strings via `ParserStyle::global_style()`.

#### **BREAKING CHANGES** (API CHANGES):

1. **[`parselib:style`]** Removed the `Title`, `Lower`, and `Upper` variants from `ParserStyleNamingCase` — these space-separated naming styles are not valid naming conventions. The `case()` conversion no longer maps those styles; existing code using any of these variants will fail to compile.
