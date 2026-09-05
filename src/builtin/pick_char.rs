use crate::{PickerArgResult, SinglePickable};

impl SinglePickable for char {
    fn pick_single(str: Option<&str>) -> PickerArgResult<Self> {
        str.map_or(PickerArgResult::NotFound, |s| {
            s.parse::<Self>()
                .map_or(PickerArgResult::NotFound, PickerArgResult::Parsed)
        })
    }
}
