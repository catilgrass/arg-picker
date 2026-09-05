# Changelogs

This file tracks all notable changes to the arg-picker project. Each release entry documents new features, bug fixes, optimizations, and breaking changes, helping developers and users understand the evolution of the parser.

The format follows a human-readable changelog convention, with sections organized by release version and change type.

Any contributor making changes to the project must record their changes in this file under the appropriate release section, using the established format and change type categories _(Features, Fixes, Optimizations, Tests, BREAKING CHANGES, etc.)_.

## TOC

- [Unreleased](#unreleased)
- [Release 0.3.0 (Unreleased)](#030-unreleased)
- [Release 0.2.0 (2026-08-10)](#020-2026-08-10)

---

## Contents

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

## Contents

### 0.3.0 (Unreleased)

First standalone release after [arg-picker](https://github.com/catilgrass/arg-picker) was migrated out of the [Mingling](https://github.com/mingling-rs/mingling) workspace.

#### Fixes:

None

#### Optimizations:

None

#### Features:

1. **[`project`]** Migrated the crate out of the Mingling workspace into an independent project (`github.com/catilgrass/arg-picker`). The project now owns its own workspace — `arg-picker` plus the `arg-picker-macros` proc-macro sub-crate — along with dual MIT/Apache-2.0 licensing, docs.rs metadata, and a standalone `test` integration crate. The `mingling_support` feature is retained for optional integration with Mingling.

2. **[`docs`]** Rewrote the README for standalone usage: added a badge header (GitHub stars, crates.io version / downloads / license / size), a minimal chained-API usage example, and `cargo add arg-picker` instructions. The `Picker` entry type now carries a plain doc comment instead of `include_str!("../README.md")`, so the crate documentation no longer depends on the README file's content.

3. **[`build`]** Added a `Makefile` exposing common development tasks — `make build`, `make test`, `make clippy`, `make doc` / `make doc-preview`, and `make check` (build + test + clippy). Each task runs across the picker, macros, and test crates, with clippy enforcing `-D warnings`.

#### **BREAKING CHANGES** (API CHANGES):

1. **[`pickable`]** Renamed the `MultiPickableWithBoundary` trait to `MultiPickable`. All references to the old trait name — including its blanket impl for `Vec<T>`, all inherent implementations, and the `SinglePickable`-adjacent internal call sites in `pickable`, `value::vec_until`, and related modules — have been updated to the shortened `MultiPickable` name. Public API code referencing `MultiPickableWithBoundary` will fail to compile and must migrate to `MultiPickable`.

---

## Contents

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
