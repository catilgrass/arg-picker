use crate::{
    matcher_needed::*,
    parselib::{build_possible_flags, is_flag_like, seek_end_of_options},
};

/// `MultiArgMatcher` matches a named flag and **all** consecutive arguments
/// that follow it, stopping at the next flag, the `--` marker, or the end
/// of the argument list.
///
/// This is the tag implementation for `Multi` and `GreedyMulti` types
/// such as `Vec<String>` (`--files a.txt b.txt`).
///
/// A flag followed by no value at all is tagged **on its own** — the same tag a flag with values
/// gets, with none of the values — and what stops the collection is [`is_flag_like`], the same
/// judgment the single-value matchers use: a word that names an option, or the end-of-options
/// marker, is where the values of a flag end. A lone prefix (`-`) ends nothing: it is a value
/// that happens to start with one.
///
/// # Behavior
///
/// | Input | `on_match_all` |
/// |-------|----------------|
/// | `--val a b --val d e` | `[0, 1, 2, 5, 6]` (two groups) |
/// | `--val=1 2` | `[0, 1]` (eq mode + one extra value) |
/// | `--val` | `[0]` (tagged alone) |
/// | `--val --other` | `[0]` (`--other` left for whatever names it) |
///
/// Args after `--` are ignored.
pub struct MultiArgMatcher;

impl Matcher for MultiArgMatcher {
    fn on_match_one(
        args: &[MaskedArg],
        style: &ParserStyle,
        arg_info: &PickerArgInfo,
    ) -> Option<usize> {
        if arg_info.positional {
            return args.first().map(|a| a.raw_idx);
        }

        let possible_flags = build_possible_flags(style, arg_info);
        let end = seek_end_of_options(args, style);
        let sep = style.value_separator;

        for arg in args {
            if end.is_some_and(|e| arg.raw_idx >= e) {
                break;
            }
            let matched = possible_flags
                .iter()
                .any(|f| Self::flag_match(arg.raw, f, style.case_sensitive, sep));
            if matched {
                return Some(arg.raw_idx);
            }
        }
        None
    }

    fn on_match_all(
        args: &[MaskedArg],
        style: &ParserStyle,
        arg_info: &PickerArgInfo,
    ) -> Vec<usize> {
        if arg_info.positional {
            let end = seek_end_of_options(args, style);
            return args
                .iter()
                .take_while(|a| end.is_none_or(|e| a.raw_idx < e))
                .map(|a| a.raw_idx)
                .collect();
        }

        let possible_flags = build_possible_flags(style, arg_info);
        let end = seek_end_of_options(args, style);
        let sep = style.value_separator;
        let is_our_flag = |raw: &str| {
            possible_flags
                .iter()
                .any(|f| Self::flag_match(raw, f, style.case_sensitive, sep))
        };

        let mut result = Vec::new();
        let mut i = 0;
        while i < args.len() {
            if end.is_some_and(|e| args[i].raw_idx >= e) {
                break;
            }

            let matched = is_our_flag(args[i].raw);

            if matched {
                result.push(args[i].raw_idx);

                if Self::is_eq_match(args[i].raw, &possible_flags, style.case_sensitive, sep) {
                    i += 1;
                    while i < args.len()
                        && end.is_none_or(|e| args[i].raw_idx < e)
                        && !is_flag_like(args[i].raw, style)
                    {
                        result.push(args[i].raw_idx);
                        i += 1;
                    }
                    continue;
                }

                i += 1;
                while i < args.len()
                    && end.is_none_or(|e| args[i].raw_idx < e)
                    && !is_flag_like(args[i].raw, style)
                {
                    result.push(args[i].raw_idx);
                    i += 1;
                }
                continue;
            }
            i += 1;
        }

        result
    }
}

impl MultiArgMatcher {
    #[inline]
    fn flag_match(raw: &str, flag_str: &str, case_sensitive: bool, sep: char) -> bool {
        let eq =
            |r: &str, f: &str| r.len() > f.len() && r.as_bytes().get(f.len()) == Some(&(sep as u8));

        if case_sensitive {
            raw == flag_str || (raw.starts_with(flag_str) && eq(raw, flag_str))
        } else {
            raw.eq_ignore_ascii_case(flag_str)
                || (raw.len() > flag_str.len()
                    && raw[..flag_str.len()].eq_ignore_ascii_case(flag_str)
                    && raw.as_bytes()[flag_str.len()] == sep as u8)
        }
    }

    #[inline]
    fn is_eq_match(raw: &str, flags: &[String], case_sensitive: bool, sep: char) -> bool {
        flags.iter().any(|f| {
            Self::flag_match(raw, f, case_sensitive, sep)
                && raw.len() > f.len()
                && raw.as_bytes().get(f.len()) == Some(&(sep as u8))
        })
    }
}
