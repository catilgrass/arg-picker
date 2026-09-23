# Changelogs

This file tracks all notable changes to the arg-picker project. Each release entry documents new features, bug fixes, optimizations, and breaking changes, helping developers and users understand the evolution of the parser.

The format follows a human-readable changelog convention, with sections organized by release version and change type.

Any contributor making changes to the project must record their changes in this file under the appropriate release section, using the established format and change type categories _(Features, Fixes, Optimizations, Tests, BREAKING CHANGES, etc.)_.

## TOC

- [Unreleased](#unreleased)
- [Release 0.4.0 (Unreleased)](#040-unreleased)
- [Release 0.3.2 (2026-09-16)](#032-2026-09-16)
- [Release 0.3.1 (2026-09-16)](#031-2026-09-16)
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

### Release 0.4.0 (Unreleased)

#### Fixes:

1. **[`parselib:matcher`]** Fixed named flags swallowing the flag that followed them as their value. A named argument takes the word after it as its value only when that word **is** a value; a word that names an option, or the end-of-options marker, is a word of its own and is left where it is. Previously `--name --other` paired `--name` with `--other`, so `--name` was given a value it was never given and `--other` could no longer be matched by whatever named it.

    - **[`parselib:utils`]** Added `is_flag_like`, telling a word that names an option (`--` / `-` prefix followed by a non-empty name, or the end-of-options marker) from a plain value. A lone prefix (`-`) is **not** a flag — it is a value that happens to start with one.
    - **[`parselib:arg_matcher`]** `ArgMatcher::on_match_all` now claims the word after a named flag only when `is_flag_like` says it is not a flag, so an option-like word is no longer consumed as a value.
    - **[`parselib:single_matcher`]** `SingleMatcher::tag` pairs a flag with the word after it only when that word is a value. A named flag given no value is now kept **on its own** in the tag (previously it was dropped), so the pick phase can tell _named and unvalued_ from _not named at all_; a later flag in the same argument is left untagged.

2. **[`pickable`]** Added the `Pickable::pick_with` method alongside `pick`, taking the [`PickerArgInfo`] of the argument being read for. What a word **is** depends on the argument it is being read for, so the picker now calls `pick_with` rather than `pick`. The default implementation ignores the marker and reads the tagged words as values, exactly as `pick` does.

    - **[`parselib:utils`]** Added `seek_single_with`, which reads like `seek_single` but treats a named argument whose one tagged word is what a flag looks like — and which carries no inline value — as having no value. A positional argument is never read that way: after the end-of-options marker its value is allowed to look like anything.
    - **[`pickable:single_pickable`]** `SinglePickable` now implements `pick_with` via `seek_single_with`, so a flag named but not valued fails as it would for any unparsable value instead of parsing the flag itself as its own value.
    - **[`pickable:single_pickable`]** `SinglePickable` for `Option<S>` also overrides `pick_with`: nothing where a value would be — the argument not being given at all, or being named and left without a value — is `None`, while a word that **is** there is read exactly as `S` reads it, so one that does not parse is still a failure rather than an absence.
    - **[`picker:parse`]** The generated pick phase now calls `T$::pick_with(args, &arg_infos[$-])` instead of `T$::pick(args)`, passing each argument its own marker.

    _Test coverage: the `arg_matcher_test` case for a value that looks like a flag now expects `[0]` rather than `[0, 1]`, and a new `value_valueless_test` module covers valueless named arguments._

3. **[`parselib:multi_arg_matcher`]** Where the words a list collects end is now the same judgment the single-value matchers use (`is_flag_like`): a word that names an option, or the end-of-options marker, ends them, while a lone prefix (`-`) does not — it is a value a list may hold. A flag followed by no value is tagged on its own.

    What a named argument that is given no value reads as is therefore the type's own answer: a type that must have one fails, an optional one is `None`, and a list is the list of none — the same thing each of them reads as when it is not there at all. A **positional** argument is unchanged: after the end-of-options marker its words are values, whatever they look like.

    _Test coverage: `value_valueless_test` covers the outcome each type declares for a named argument given no value, along with an absent argument, an empty inline value, a lone `-` a list holds, a word that looks like a flag needing the separator, and the flag after a valueless argument still being matched; `style_test` covers `is_flag_like` and the matcher guards in all three styles; `multi_value_test`'s "given no value" case expects the empty list._

#### Optimizations:

None

#### Features:

None

#### **BREAKING CHANGES** (API CHANGES):

1. **[`parselib:arg_matcher`]** **[`parselib:single_matcher`]** **[`parselib:multi_arg_matcher`]** A named parameter no longer takes a word that names an option as its value, which changes what several inputs mean. `--name --other` used to give `name` the value `"--other"`, leaving `--other` unmatchable; it now leaves `--name` without a value and `--other` free to be matched by whatever names it. The same holds for the end-of-options marker (`--name --`) and for a list (`--files --other`).

    What a parameter that is left without a value reads as is its type's own answer, and each is the same thing that type reads as when it is not there at all:

    - a type that must have a value — `T`, with no `Option` — **fails**, as it does when it is not given at all;
    - an optional one — `Option<T>` — is `None`;
    - a list — `Vec<T>` / `VecUntil<T>` — is the list of none.

    **Migration**: a value that itself starts with the style's prefix must now be written with the separator — `--name=-5` rather than `--name -5`, or `/Name:/Users/me` rather than `/Name /Users/me` — or passed as a positional argument after the end-of-options marker. Code that relied on a following flag being read as the value should name the value it means instead.

2. **[`pickable`]** The picker now calls `Pickable::pick_with` rather than `Pickable::pick`, so that an implementation can be told which argument the words are being read for. The new method is **additive** — its default forwards to `pick`, and `SinglePickable` implementors are unaffected — but an implementation of `Pickable` that expected `pick` to be the only pick phase will no longer be called by the picker, and one that reads a word differently depending on the argument it is read for must override `pick_with`.

    **Migration**: nothing is required of an implementation that only has `pick`; an implementation that needs the argument it is being read for should override `pick_with` and read the tagged words from there.

---

### 0.3.2 (2026-09-16)

#### Fixes:

1. **[`package`]** Raised the required `arg-picker-macros` version to `0.3.2`, which 0.3.1 should have done. Its requirement was `0.3.0`, while the source it shipped required the `internal_repeat!` forms added in `arg-picker-macros` 0.3.1 — the `ident$suffix` identifier form and the `(group,·)^` complement repetition, both used by `func.rs` to emit `PickerFunction` and the per-arity `Picker::invoke` impls. Because `^0.3.0` is satisfied by `0.3.0`, Cargo never moved an existing lock off it: any project that had already resolved `arg-picker-macros` 0.3.0 kept it, and `arg-picker` then failed to compile with `expected one of ... found Arg` and `expected one of , or >, found ^` — errors pointing into the macro expansion rather than at anything the caller wrote.

    - **[`Cargo.toml`]** `arg-picker-macros` is now required as `0.3.2`, the version released alongside this one, so the crate and the macro that expands its syntax can no longer be resolved apart.
    - **[`macros`]** No change to the macro itself: this release only re-states the requirement 0.3.1 already relied on.

    _A project that hit this can also work around it with `cargo update -p arg-picker-macros --precise 0.3.2`, which is worth doing regardless — it moves off the version whose syntax this crate does not match._

#### Optimizations:

None

#### Features:

None

#### **BREAKING CHANGES** (API CHANGES):

None

---

### 0.3.1 (2026-09-16)

#### Fixes:

1. **[`macros:derive`]** Fixed the `#[derive(Pickable)]` expansion to honour the `mingling_support` feature, which `arg!` already did. Every path the derive generated was hardcoded as `::arg_picker::`, so a crate that depends on Mingling alone could not use it: Mingling's `picker` feature enables `arg-picker/derive` and `arg-picker/mingling_support` and re-exports this crate as `mingling::picker`, yet the expanded `impl` still named `::arg_picker` — a crate that is not in the user's dependency graph, so the expansion failed to resolve.

    - **[`macros`]** Added a single `picker_root()` helper in `macros/src/lib.rs` returning `::mingling::picker` under `mingling_support` and `::arg_picker` otherwise. Both `arg!` (`macros/src/arg.rs`) and the derive (`macros/src/derive.rs`) now build every generated path from it, so the two can no longer disagree about which crate they mean.
    - **[`macros:derive`]** Routed every path the derive generates through that root: `PickerArg`, `IntoPicker`, `PickerArgInfo`, `TagPhaseContext`, `Pickable`, `PickerArgResult`, `PickerArgAttr`, `SinglePickable`, and `__private::to_pascal_case` — the last reached through Mingling's glob re-export of this crate.

    _No change when `mingling_support` is off: the generated code still refers to `::arg_picker` itself._

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
