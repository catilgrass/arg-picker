use crate::{
    PickerArgInfo,
    PickerArgResult::Parsed,
    UnmatchedArg, UnmatchedArgs,
    parselib::{ParserStyle, build_masked_args, seek_end_of_options},
    pickable_needed::*,
};

impl<'a> Pickable<'a> for UnmatchedArgs {
    fn get_attr(_flag: &PickerArg<'a, Self>) -> PickerArgAttr {
        // The lowest priority, so every declared argument has had its chance to claim a word.
        PickerArgAttr::Postprocess
    }

    fn tag(ctx: TagPhaseContext) -> Vec<usize> {
        let masked = build_masked_args(ctx.args, ctx.mask);
        // The end-of-options marker is syntax, not an argument: report the words after it, not it.
        let end = seek_end_of_options(&masked, ParserStyle::global_style());
        masked
            .iter()
            .filter(|arg| Some(arg.raw_idx) != end)
            .map(|arg| arg.raw_idx)
            .collect()
    }

    fn pick(raw_strs: &[&str]) -> PickerArgResult<Self> {
        // A direct call has no original list to point into, so positions are the slice's own.
        let entries = raw_strs
            .iter()
            .enumerate()
            .map(|(index, &raw)| UnmatchedArg {
                index,
                raw: raw.to_string(),
            })
            .collect();
        Parsed(Self::new(entries))
    }

    fn pick_indexed(
        raw_strs: &[&str],
        indices: &[usize],
        _info: &PickerArgInfo,
    ) -> PickerArgResult<Self> {
        let entries = raw_strs
            .iter()
            .zip(indices)
            .map(|(&raw, &index)| UnmatchedArg {
                index,
                raw: raw.to_string(),
            })
            .collect();
        Parsed(Self::new(entries))
    }
}
