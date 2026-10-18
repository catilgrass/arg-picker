use crate::{Pickable, PickerArgInfo, parselib::ParserStyle};
use std::marker::PhantomData;

/// The long name and aliases of a [`PickerArg`].
///
/// The tuple constructors hold at most four names — the long name and three aliases — in the
/// value itself, so building an argument allocates nothing. `arg!` and `#[derive(Pickable)]`
/// borrow the slice they already write.
#[derive(Clone, Copy)]
pub enum PickerNames<'a> {
    /// Names borrowed from a slice.
    Borrowed(&'a [&'a str]),

    /// Up to four names held inline; `len` is how many entries of `names` are in use.
    #[non_exhaustive]
    Inline {
        /// The names, with the unused entries beyond `len` left empty.
        names: [&'a str; 4],
        /// The number of used entries in `names`.
        len: u8,
    },
}

impl Default for PickerNames<'_> {
    fn default() -> Self {
        Self::Borrowed(&[])
    }
}

impl<'a> PickerNames<'a> {
    /// Returns the names as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[&'a str] {
        match self {
            Self::Borrowed(names) => names,
            Self::Inline { names, len } => &names[..usize::from(*len)],
        }
    }

    /// Returns `true` if there are no names.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        match self {
            Self::Borrowed(names) => names.is_empty(),
            Self::Inline { len, .. } => *len == 0,
        }
    }

    /// Holds up to four names inline.
    const fn inline(names: [&'a str; 4], len: u8) -> Self {
        Self::Inline { names, len }
    }
}

/// Represents a constraint definition for a parameter selection.
///
/// This structure describes the constraints that a command-line parameter (Picker parameter item)
/// should satisfy, including its full name list (with aliases), short name form, and whether it is
/// positional.
///
/// # Field Descriptions
///
/// - `full`: Full name or alias list. For example, `["config", "cfg"]` means the parameter can be
///   matched with either `--config` or `--cfg`. Must contain at least one non-empty string.
///
/// - `short`: Short name (single character). For example, `Some('c')` means it can be passed using
///   the `-c` form. If set to `None`, the short name form is not supported.
///
/// - `positional`: Whether the parameter is positional (i.e., an argument without a flag).
///   - `true`: The parameter is positional; it is matched by its position in the command line rather
///     than by a `--name` or `-n` flag.
///   - `false`: The parameter is a named (flag-based) parameter.
///
/// - `_type`: [`PhantomData`] to hold the type parameter.
///
/// # Construction
///
/// A definition can be a struct literal, [`PickerArg::new`], or a conversion from its names. The
/// type is explicit in every conversion, because the names say nothing about it:
///
/// ```
/// use arg_picker::PickerArg;
///
/// let short = PickerArg::<String>::from('n');
/// assert!(short.full().is_empty());
/// assert_eq!(short.short(), Some('n'));
///
/// let named = PickerArg::<String>::from(("name", "nick", 'n'));
/// assert_eq!(named.full(), &["name", "nick"][..]);
/// assert_eq!(named.short(), Some('n'));
///
/// let positional = PickerArg::<String>::from(());
/// assert!(positional.full().is_empty());
/// assert!(positional.is_positional());
/// ```
#[derive(Default, Clone, Copy)]
pub struct PickerArg<'a, Type>
where
    Type: Pickable<'a>,
{
    /// Long name and aliases, e.g. `["config", "cfg"]`.
    pub full: PickerNames<'a>,

    /// Short name, e.g., `'c'`.
    pub short: Option<char>,

    /// Whether the parameter is positional (no flag, matched by position).
    pub positional: bool,

    /// [`PhantomData`] to hold the type parameter.
    pub internal_type: PhantomData<Type>,
}

// `Self` cannot be used in an impl header, so the implemented type is spelled out there.
#[allow(clippy::use_self)]
impl<'a, 'b, Type> From<&'b PickerArg<'a, Type>> for PickerArg<'a, Type>
where
    Type: Pickable<'a>,
{
    fn from(value: &'b Self) -> Self {
        Self {
            full: value.full,
            short: value.short,
            positional: value.positional,
            internal_type: PhantomData,
        }
    }
}

impl<'a, Type> PickerArg<'a, Type>
where
    Type: Pickable<'a>,
{
    /// Creates a new `PickerArg` with the provided parameters.
    #[must_use]
    pub const fn new(full: &'a [&'a str], short: Option<char>, positional: bool) -> Self {
        Self {
            full: PickerNames::Borrowed(full),
            short,
            positional,
            internal_type: PhantomData,
        }
    }

    /// Builds an argument from names held inline.
    const fn from_inline(
        names: [&'a str; 4],
        len: u8,
        short: Option<char>,
        positional: bool,
    ) -> Self {
        Self {
            full: PickerNames::inline(names, len),
            short,
            positional,
            internal_type: PhantomData,
        }
    }

    /// Returns the full name list (including aliases).
    #[must_use]
    pub fn full(&self) -> &[&'a str] {
        self.full.as_slice()
    }

    /// Returns the short name, if any.
    #[must_use]
    pub const fn short(&self) -> Option<char> {
        self.short
    }

    /// Returns whether the parameter is positional.
    ///
    /// If `full` is empty or `short` is `None`, the parameter is considered positional
    /// regardless of the stored value.
    #[must_use]
    pub const fn is_positional(&self) -> bool {
        if self.full.is_empty() && self.short.is_none() {
            true
        } else {
            self.positional
        }
    }

    /// Sets the full name list.
    pub const fn set_full(&mut self, full: &'a [&'a str]) {
        self.full = PickerNames::Borrowed(full);
    }

    /// Sets the short name.
    pub const fn set_short(&mut self, short: Option<char>) {
        self.short = short;
    }

    /// Sets whether the parameter is positional.
    pub const fn set_positional(&mut self, positional: bool) {
        self.positional = positional;
    }

    /// Sets the full name list and returns self.
    #[must_use]
    pub const fn with_full(mut self, full: &'a [&'a str]) -> Self {
        self.full = PickerNames::Borrowed(full);
        self
    }

    /// Clears the full name list (sets it to an empty slice) and returns self.
    #[must_use]
    pub const fn without_full(mut self) -> Self {
        self.full = PickerNames::Borrowed(&[]);
        self
    }

    /// Sets the short name to the given character and returns self.
    #[must_use]
    pub const fn with_short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }

    /// Clears the short name (sets it to None) and returns self.
    #[must_use]
    pub const fn without_short(mut self) -> Self {
        self.short = None;
        self
    }

    /// Sets whether the parameter is positional and returns self.
    #[must_use]
    pub const fn with_positional(mut self, positional: bool) -> Self {
        self.positional = positional;
        self
    }

    /// Returns a copy of this definition.
    ///
    /// Every field is `Copy` regardless of `Type`, so this does not require `Type: Copy`
    /// (which the derived `Copy` would).
    #[must_use]
    pub(crate) const fn copied(&self) -> Self {
        Self {
            full: self.full,
            short: self.short,
            positional: self.positional,
            internal_type: PhantomData,
        }
    }

    /// Converts this `PickerArg` into a `PickerArgInfo` value.
    ///
    /// This is a convenience method equivalent to calling `PickerArgInfo::from(self)`.
    #[must_use]
    pub fn into_info(self) -> PickerArgInfo<'a> {
        let names = self.full.as_slice();
        let (long, alias) = if names.is_empty() {
            (None, None)
        } else {
            let long = Some(names[0]);
            let alias = if names.len() > 1 {
                Some(names[1..].to_vec())
            } else {
                None
            };
            (long, alias)
        };

        PickerArgInfo {
            short: self.short,
            long,
            alias,
            positional: self.positional,
            optional: false,
            multi: false,
            is_flag: false,
        }
    }
}

impl<'a, Type> From<char> for PickerArg<'a, Type>
where
    Type: Pickable<'a>,
{
    fn from(short: char) -> Self {
        Self::from_inline(["", "", "", ""], 0, Some(short), false)
    }
}

impl<'a, Type> From<&'a str> for PickerArg<'a, Type>
where
    Type: Pickable<'a>,
{
    fn from(long: &'a str) -> Self {
        Self::from_inline([long, "", "", ""], 1, None, false)
    }
}

impl<'a, Type> From<()> for PickerArg<'a, Type>
where
    Type: Pickable<'a>,
{
    fn from((): ()) -> Self {
        Self::from_inline(["", "", "", ""], 0, None, true)
    }
}

impl<'a, Type> From<(&'a str, char)> for PickerArg<'a, Type>
where
    Type: Pickable<'a>,
{
    fn from((long, short): (&'a str, char)) -> Self {
        Self::from_inline([long, "", "", ""], 1, Some(short), false)
    }
}

impl<'a, Type> From<(&'a str, &'a str)> for PickerArg<'a, Type>
where
    Type: Pickable<'a>,
{
    fn from((long, alias): (&'a str, &'a str)) -> Self {
        Self::from_inline([long, alias, "", ""], 2, None, false)
    }
}

impl<'a, Type> From<(&'a str, &'a str, &'a str)> for PickerArg<'a, Type>
where
    Type: Pickable<'a>,
{
    fn from((long, alias_1, alias_2): (&'a str, &'a str, &'a str)) -> Self {
        Self::from_inline([long, alias_1, alias_2, ""], 3, None, false)
    }
}

impl<'a, Type> From<(&'a str, &'a str, &'a str, &'a str)> for PickerArg<'a, Type>
where
    Type: Pickable<'a>,
{
    fn from((long, alias_1, alias_2, alias_3): (&'a str, &'a str, &'a str, &'a str)) -> Self {
        Self::from_inline((long, alias_1, alias_2, alias_3).into(), 4, None, false)
    }
}

impl<'a, Type> From<(&'a str, &'a str, char)> for PickerArg<'a, Type>
where
    Type: Pickable<'a>,
{
    fn from((long, alias, short): (&'a str, &'a str, char)) -> Self {
        Self::from_inline([long, alias, "", ""], 2, Some(short), false)
    }
}

impl<'a, Type> From<(&'a str, &'a str, &'a str, char)> for PickerArg<'a, Type>
where
    Type: Pickable<'a>,
{
    fn from((long, alias_1, alias_2, short): (&'a str, &'a str, &'a str, char)) -> Self {
        Self::from_inline([long, alias_1, alias_2, ""], 3, Some(short), false)
    }
}

impl<'a, Type> From<(&'a str, &'a str, &'a str, &'a str, char)> for PickerArg<'a, Type>
where
    Type: Pickable<'a>,
{
    fn from(
        (long, alias_1, alias_2, alias_3, short): (&'a str, &'a str, &'a str, &'a str, char),
    ) -> Self {
        Self::from_inline([long, alias_1, alias_2, alias_3], 4, Some(short), false)
    }
}

impl<'a, Type> From<PickerArg<'a, Type>> for Vec<String>
where
    Type: Pickable<'a>,
{
    fn from(value: PickerArg<'a, Type>) -> Self {
        let mut result = Self::new();
        let info = PickerArgInfo::from(value);
        let possible_flags =
            crate::parselib::build_possible_flags(ParserStyle::global_style(), &info);
        for flag in possible_flags {
            result.push(flag);
        }
        result
    }
}

/// Describes the attribute (behavior) of a command-line parameter.
///
/// The ordering reflects parse priority (higher = parsed first):
/// `Postprocess < Final < PositionalMulti < Positional < Flag < Single < Multi < Begin < Preprocess`
///
/// # Variants
///
/// - `Postprocess` — Reserved lowest priority, used only in special cases.
/// - `Final` — Reserved post-processing priority, used only in special cases.
/// - `PositionalMulti` — Positional argument that accepts multiple values (e.g., multiple input files).
/// - `Positional` — Positional argument matched by its position (e.g., an input file).
/// - `Flag` — Boolean flag with no associated value (e.g., `--verbose`).
/// - `Single` — Accepts a single value (e.g., `--name Alice`).
/// - `Multi` — Accepts multiple values (e.g., `--file a.txt --file b.txt`).
/// - `Begin` — Reserved pre-processing priority, used only in special cases.
/// - `Preprocess` — Reserved highest priority, used only in special cases.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PickerArgAttr {
    /// Reserved lowest priority, used only in special cases.
    Postprocess,

    /// Reserved post-processing priority, used only in special cases.
    Final,

    /// Positional argument that accepts multiple values (e.g., multiple input files).
    PositionalMulti,

    /// Positional argument matched by its position (e.g., an input file).
    #[default]
    Positional,

    /// Boolean flag with no associated value (e.g., `--verbose`).
    Flag,

    /// Accepts a single value (e.g., `--name Alice`).
    Single,

    /// Accepts multiple values (e.g., `--file a.txt --file b.txt`).
    Multi,

    /// Reserved pre-processing priority, used only in special cases.
    Begin,

    /// Reserved highest priority, used only in special cases.
    Preprocess,
}

impl PickerArgAttr {
    /// Determines if the given `PickerArg` represents a positional parameter.
    ///
    /// If the flag is positional (determined by `flag.is_positional()`), returns
    /// `Self::Positional`. Otherwise, invokes the `other` closure to
    /// produce and return a `Self`.
    ///
    /// # Parameters
    ///
    /// - `flag`: A reference to the [`PickerArg`] to evaluate.
    /// - `other`: A closure that returns a [`PickerArgAttr`] when the flag is
    ///   **not** positional.
    #[inline]
    pub fn positional_or_else<'a, T>(flag: &PickerArg<'a, T>, other: fn() -> Self) -> Self
    where
        T: Pickable<'a>,
    {
        if flag.is_positional() {
            Self::Positional
        } else {
            other()
        }
    }

    /// Determines if the given `PickerArg` represents a positional parameter and returns
    /// `Self::Positional` if so. Otherwise, returns the provided `default` attribute.
    ///
    /// # Parameters
    ///
    /// - `flag`: A reference to the [`PickerArg`] to evaluate.
    /// - `default`: The [`PickerArgAttr`] to return if the flag is not positional.
    #[must_use]
    #[inline]
    pub const fn positional_or<'a, T>(flag: &PickerArg<'a, T>, default: Self) -> Self
    where
        T: Pickable<'a>,
    {
        if flag.is_positional() {
            Self::Positional
        } else {
            default
        }
    }

    /// Determines if the given `PickerArg` represents a positional parameter and returns
    /// `Self::Positional` if so. Otherwise, returns `Self::Single`.
    ///
    /// # Parameters
    ///
    /// - `flag`: A reference to the [`PickerArg`] to evaluate.
    #[must_use]
    #[inline]
    pub const fn positional_or_single<'a, T>(flag: &PickerArg<'a, T>) -> Self
    where
        T: Pickable<'a>,
    {
        if flag.is_positional() {
            Self::Positional
        } else {
            Self::Single
        }
    }

    /// Determines if the given `PickerArg` represents a positional parameter and returns
    /// `Self::PositionalMulti` if so. Otherwise, returns `Self::Multi`.
    ///
    /// # Parameters
    ///
    /// - `flag`: A reference to the [`PickerArg`] to evaluate.
    #[must_use]
    #[inline]
    pub const fn positional_or_multi<'a, T>(flag: &PickerArg<'a, T>) -> Self
    where
        T: Pickable<'a>,
    {
        if flag.is_positional() {
            Self::PositionalMulti
        } else {
            Self::Multi
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_picker_flag_attr_ordering() {
        // Multi > Single > Flag > Positional > PositionalMulti
        assert!(PickerArgAttr::Multi > PickerArgAttr::Single);
        assert!(PickerArgAttr::Multi > PickerArgAttr::Flag);
        assert!(PickerArgAttr::Multi > PickerArgAttr::Positional);
        assert!(PickerArgAttr::Multi > PickerArgAttr::PositionalMulti);

        assert!(PickerArgAttr::Single > PickerArgAttr::Flag);
        assert!(PickerArgAttr::Single > PickerArgAttr::Positional);
        assert!(PickerArgAttr::Single > PickerArgAttr::PositionalMulti);

        assert!(PickerArgAttr::Flag > PickerArgAttr::Positional);
        assert!(PickerArgAttr::Flag > PickerArgAttr::PositionalMulti);

        assert!(PickerArgAttr::Positional > PickerArgAttr::PositionalMulti);

        // PartialOrd
        assert!(PickerArgAttr::Multi >= PickerArgAttr::Single);
        assert!(PickerArgAttr::Single >= PickerArgAttr::Flag);
        assert!(PickerArgAttr::Flag >= PickerArgAttr::Positional);
        assert!(PickerArgAttr::Positional >= PickerArgAttr::PositionalMulti);

        assert!(PickerArgAttr::PositionalMulti < PickerArgAttr::Positional);
        assert!(PickerArgAttr::Positional < PickerArgAttr::Flag);
        assert!(PickerArgAttr::Flag < PickerArgAttr::Single);
        assert!(PickerArgAttr::Single < PickerArgAttr::Multi);
    }

    #[test]
    fn test_picker_flag_attr_sorting() {
        // Sort
        let mut values = vec![
            PickerArgAttr::Flag,
            PickerArgAttr::Single,
            PickerArgAttr::Positional,
            PickerArgAttr::Multi,
            PickerArgAttr::PositionalMulti,
        ];
        values.sort();
        assert_eq!(
            values,
            vec![
                PickerArgAttr::PositionalMulti,
                PickerArgAttr::Positional,
                PickerArgAttr::Flag,
                PickerArgAttr::Single,
                PickerArgAttr::Multi,
            ]
        );
    }
}
