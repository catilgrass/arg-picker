use crate::{
    PickerArgInfo,
    parselib::{MaskedArg, ParserStyle},
};

/// Builds a list of possible flag strings for the given argument info
///
/// This function generates formatted flag strings (e.g., `-h`, `--help`) from the short flag,
/// long flag, and any aliases defined in the argument info. The long flag and alias names
/// are converted according to the style's naming case convention before being formatted.
#[must_use]
pub fn build_possible_flags(style: &ParserStyle, arg_info: &PickerArgInfo) -> Vec<String> {
    let mut possible_flags = vec![];

    if let Some(short) = arg_info.short {
        possible_flags.push(style.flag_string(short));
    }

    if let Some(long) = arg_info.long {
        let converted = style.naming_case.convert(long.to_string());
        possible_flags.push(style.flag_string(&converted));
    }

    if let Some(aliases) = &arg_info.alias {
        for alias in aliases {
            let converted = style.naming_case.convert(alias.to_string());
            possible_flags.push(style.flag_string(&converted));
        }
    }

    possible_flags
}

/// Extract a single value from the raw strings tagged by [`SingleMatcher`](crate::parselib::SingleMatcher).
///
/// Returns `None` if no value is available (empty slice),
/// the inline value after the style separator if present (eq mode),
/// or the value directly (positional or flag-following).
///
/// This is the standard `pick` helper for all `Single`-type
/// [`Pickable`](crate::Pickable) implementations.
#[must_use]
pub fn seek_single<'a>(raw_strs: &'a [&'a str]) -> Option<&'a str> {
    match raw_strs.len() {
        0 => None,
        1 => {
            let s = raw_strs[0];
            let sep = ParserStyle::global_style().value_separator;
            s.rfind(sep).map_or(Some(s), |pos| Some(&s[pos + 1..]))
        }
        _ => Some(raw_strs[1]),
    }
}

/// Extract a single value, with the marker of the argument it is being read for.
///
/// As [`seek_single`], except that a **named** argument whose one tagged word is what a flag
/// looks like has no value: the word is the flag itself, left in the tag because no value
/// followed it. A positional argument is never read that way — after the end-of-options marker
/// its value is allowed to look like anything.
#[must_use]
pub fn seek_single_with<'a>(raw_strs: &'a [&'a str], info: &PickerArgInfo) -> Option<&'a str> {
    if let [only] = raw_strs {
        let style = ParserStyle::global_style();

        // An inline value is written with the flag, so a word that carries one is more than a
        // flag and is read as such below.
        if !info.positional && !only.contains(style.value_separator) && is_flag_like(only, style) {
            return None;
        }
    }

    seek_single(raw_strs)
}

/// Whether `raw` names an option rather than being a value.
///
/// A word that starts with the style's long or short prefix names an option, and the
/// end-of-options marker is a boundary rather than a word of its own: neither is ever the value
/// of a flag, so a flag that is followed by one was given none. A lone prefix — `-` — is not a
/// flag: it is a value that happens to start with one, which is what a program that reads a lone
/// `-` as its standard input is told.
#[must_use]
pub fn is_flag_like(raw: &str, style: &ParserStyle) -> bool {
    let names_an_option = |prefix: &str| {
        raw.strip_prefix(prefix)
            .is_some_and(|name| !name.is_empty())
    };

    names_an_option(style.long_prefix)
        || names_an_option(style.short_prefix)
        || is_end_of_options(raw, style)
}

/// Whether `raw` is the style's end-of-options marker.
#[must_use]
fn is_end_of_options(raw: &str, style: &ParserStyle) -> bool {
    if style.case_sensitive {
        raw == style.end_of_options
    } else {
        raw.eq_ignore_ascii_case(style.end_of_options)
    }
}

/// Seeks the index of the end-of-options marker (`--`) in the argument list.
///
/// This function searches for the standard end-of-options separator (`--`)
/// in the given argument list, respecting the parser's style settings
/// (e.g., case sensitivity). The end-of-options marker indicates that all
/// subsequent arguments should be treated as positional arguments, not flags.
#[must_use]
pub fn seek_end_of_options(args: &[MaskedArg], style: &ParserStyle) -> Option<usize> {
    args.iter()
        .find(|arg| {
            if style.case_sensitive {
                arg.raw == style.end_of_options
            } else {
                arg.raw.eq_ignore_ascii_case(style.end_of_options)
            }
        })
        .map(|arg| arg.raw_idx)
}

/// Seeks arguments in `args` that are exactly equal to the given `string`.
///
/// Returns the indices of matching arguments.
#[must_use]
#[inline]
pub fn seek_eq(args: &[MaskedArg], string: &str, case_sensitive: bool) -> Vec<usize> {
    args.iter()
        .filter(|arg| {
            if case_sensitive {
                arg.raw == string
            } else {
                arg.raw.eq_ignore_ascii_case(string)
            }
        })
        .map(|arg| arg.raw_idx)
        .collect()
}

/// Seeks arguments in `args` that contain the given `string` as a substring.
///
/// Returns the indices of matching arguments.
#[must_use]
#[inline]
pub fn seek_contains(args: &[MaskedArg], string: &str, case_sensitive: bool) -> Vec<usize> {
    args.iter()
        .filter(|arg| {
            if case_sensitive {
                arg.raw.contains(string)
            } else {
                arg.raw.to_lowercase().contains(&string.to_lowercase())
            }
        })
        .map(|arg| arg.raw_idx)
        .collect()
}

/// Seeks arguments in `args` that start with the given `string`.
///
/// Returns the indices of matching arguments.
#[must_use]
#[inline]
pub fn seek_start_with(args: &[MaskedArg], string: &str, case_sensitive: bool) -> Vec<usize> {
    args.iter()
        .filter(|arg| {
            if case_sensitive {
                arg.raw.starts_with(string)
            } else {
                arg.raw.to_lowercase().starts_with(&string.to_lowercase())
            }
        })
        .map(|arg| arg.raw_idx)
        .collect()
}

/// Seeks arguments in `args` that end with the given `string`.
///
/// Returns the indices of matching arguments.
#[must_use]
#[inline]
pub fn seek_end_with(args: &[MaskedArg], string: &str, case_sensitive: bool) -> Vec<usize> {
    args.iter()
        .filter(|arg| {
            if case_sensitive {
                arg.raw.ends_with(string)
            } else {
                arg.raw.to_lowercase().ends_with(&string.to_lowercase())
            }
        })
        .map(|arg| arg.raw_idx)
        .collect()
}

/// Seeks arguments in `args` that are exactly equal to any of the given `strings`.
///
/// Returns the indices of matching arguments.
#[must_use]
#[inline]
pub fn multi_seek_eq(args: &[MaskedArg], strings: &[&str], case_sensitive: bool) -> Vec<usize> {
    args.iter()
        .filter(|arg| {
            if case_sensitive {
                strings.contains(&arg.raw)
            } else {
                strings.iter().any(|s| arg.raw.eq_ignore_ascii_case(s))
            }
        })
        .map(|arg| arg.raw_idx)
        .collect()
}

/// Seeks arguments in `args` that contain any of the given `strings` as a substring.
///
/// Returns the indices of matching arguments.
#[must_use]
#[inline]
pub fn multi_seek_contains(
    args: &[MaskedArg],
    strings: &[&str],
    case_sensitive: bool,
) -> Vec<usize> {
    args.iter()
        .filter(|arg| {
            if case_sensitive {
                strings.iter().any(|s| arg.raw.contains(s))
            } else {
                let lower_raw = arg.raw.to_lowercase();
                strings
                    .iter()
                    .any(|s| lower_raw.contains(&s.to_lowercase()))
            }
        })
        .map(|arg| arg.raw_idx)
        .collect()
}

/// Seeks arguments in `args` that start with any of the given `strings`.
///
/// Returns the indices of matching arguments.
#[must_use]
#[inline]
pub fn multi_seek_start_with(
    args: &[MaskedArg],
    strings: &[&str],
    case_sensitive: bool,
) -> Vec<usize> {
    args.iter()
        .filter(|arg| {
            if case_sensitive {
                strings.iter().any(|s| arg.raw.starts_with(s))
            } else {
                let lower_raw = arg.raw.to_lowercase();
                strings
                    .iter()
                    .any(|s| lower_raw.starts_with(&s.to_lowercase()))
            }
        })
        .map(|arg| arg.raw_idx)
        .collect()
}

/// Seeks arguments in `args` that end with any of the given `strings`.
///
/// Returns the indices of matching arguments.
#[must_use]
#[inline]
pub fn multi_seek_end_with(
    args: &[MaskedArg],
    strings: &[&str],
    case_sensitive: bool,
) -> Vec<usize> {
    args.iter()
        .filter(|arg| {
            if case_sensitive {
                strings.iter().any(|s| arg.raw.ends_with(s))
            } else {
                let lower_raw = arg.raw.to_lowercase();
                strings
                    .iter()
                    .any(|s| lower_raw.ends_with(&s.to_lowercase()))
            }
        })
        .map(|arg| arg.raw_idx)
        .collect()
}

/// Converts a `&Vec<String>` into a `Vec<&str>` by borrowing each string's slice.
///
/// This is useful for converting owned `String` vectors into borrowed `&str` slices
/// for functions that take `&[&str]` or similar parameters.
#[must_use]
#[inline]
#[doc(hidden)]
pub fn vec_string_to_vec_str(input: &[String]) -> Vec<&str> {
    input.iter().map(String::as_str).collect()
}

/// Converts a `&Vec<String>` into a `Vec<&str>` by borrowing each string's slice.
///
/// This is useful for converting owned `String` vectors into borrowed `&str` slices
/// for functions that take `&[&str]` or similar parameters.
#[macro_export]
#[doc(hidden)]
macro_rules! vec_string_slice {
    ($v:expr) => {
        $v.iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .as_slice()
    };
}

/// Gets the first element from a vector of seek results, if any.
///
/// Returns `Some(index)` if the vector is non-empty, otherwise `None`.
#[must_use]
#[inline]
pub fn get_seeked_first(seeked: Vec<usize>) -> Option<usize> {
    seeked.into_iter().next()
}
