use crate::TagPhaseContext;
use crate::parselib::{ArgMatcher, Matcher, ParserStyle, PositionalMatcher, is_flag_like};

/// `SingleMatcher` is a composite matcher for single-value parameters.
///
/// It delegates to [`PositionalMatcher`] for positional args and
/// [`ArgMatcher`] for named args, adding a guard: a named flag that is not
/// followed by a value is kept in the tag **on its own**, so that
/// [`pick_with`](crate::Pickable::pick_with) is told the argument was named and
/// not valued — which is not the same as its not being there at all.
pub struct SingleMatcher;

impl SingleMatcher {
    /// Match a single positional value, or a named flag with the value that follows it.
    ///
    /// For named args a flag is paired with the word after it only when that word is a value.
    /// A flag whose next word names an option — or is the end-of-options marker — was given no
    /// value, and is tagged alone; the word it was not paired with is left out of the tag, so
    /// that whatever it does name stays free to be matched on its own.
    #[inline]
    #[must_use]
    pub fn tag(ctx: TagPhaseContext) -> Vec<usize> {
        if ctx.arg_info.positional {
            PositionalMatcher::match_one(ctx.into()).map_or_else(Vec::new, |i| vec![i])
        } else {
            let args = ctx.args;
            let positions = ArgMatcher::match_all(ctx.into());
            let style = ParserStyle::global_style();
            let sep = style.value_separator;

            let mut i = 0;
            let mut result = Vec::with_capacity(positions.len());
            while i < positions.len() {
                let flag_idx = positions[i];
                let flag = args.get(flag_idx).unwrap_or_default();

                // `--name=value`: the value is written with the flag, so the flag is the whole
                // of what is tagged.
                if flag.contains(sep) {
                    result.push(flag_idx);
                    i += 1;
                    continue;
                }

                // What follows the flag is its value only when it is a value.
                let value = match positions.get(i + 1) {
                    Some(&next) if !is_flag_like(args.get(next).unwrap_or_default(), style) => {
                        Some(next)
                    }
                    _ => None,
                };

                match value {
                    Some(value_idx) => {
                        result.push(flag_idx);
                        result.push(value_idx);
                        i += 2;
                    }
                    // A flag with no value after it, and nothing else tagged: it is kept on its
                    // own, because a flag that was given no value is not the same as one that was
                    // not given at all, and only the tag can say which. A tag of one flag is how
                    // the pick phase is told to read it as the flag it is.
                    None if result.is_empty() => {
                        result.push(flag_idx);
                        i += 1;
                    }
                    // A further flag once the argument has its value is not this argument's to
                    // claim: it is left untagged, so that whatever names it — another argument,
                    // or none — can still be matched.
                    None => i += 1,
                }
            }
            result
        }
    }
}
