use std::marker::PhantomData;

use crate::{PickerArg, PickerArgs, PickerNames, UnmatchedArgs};

/// Remaining positional arguments (anything not consumed as an option).
/// - `full`: `[]` (empty — not triggered by any `--` prefix).
/// - `short`: (none)
/// - `positional`: `false` (this is a meta‑argument that collects everything left).
///   This constant is used internally to access any leftover arguments after
///   all defined flags/options have been processed.
pub const REMAINS: PickerArg<PickerArgs> = PickerArg::<PickerArgs> {
    full: PickerNames::Borrowed(&[]),
    short: None,
    positional: false,
    internal_type: PhantomData,
};

/// The arguments no pick consumed, each with its position.
///
/// Like [`REMAINS`], this is a meta-argument that runs last, after every declared argument has
/// had its chance to claim a word. Unlike `REMAINS`, it reports each word's index in the argument
/// list and leaves the end-of-options marker out.
///
/// Pick this instead of [`REMAINS`], not alongside it: both run last, and whichever runs first
/// claims every remaining word.
///
/// ```
/// use arg_picker::{UnmatchedArgs, consts::UNMATCHED, macros::arg, prelude::*};
///
/// let (name, unmatched): (String, UnmatchedArgs) = vec!["--name", "Alice", "stray"]
///     .pick(arg![name: String])
///     .pick(&UNMATCHED)
///     .unwrap();
///
/// assert_eq!(name, "Alice");
/// assert_eq!(unmatched.len(), 1);
/// assert_eq!(unmatched.iter().next().unwrap().index, 2);
/// assert_eq!(unmatched.iter().next().unwrap().raw, "stray");
/// assert_eq!(unmatched.original().join(" "), "--name Alice stray");
/// ```
pub const UNMATCHED: PickerArg<UnmatchedArgs> = PickerArg::<UnmatchedArgs> {
    full: PickerNames::Borrowed(&[]),
    short: None,
    positional: false,
    internal_type: PhantomData,
};
