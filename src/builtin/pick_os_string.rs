use std::ffi::OsString;

use crate::{PickerArgResult, SinglePickable};

impl SinglePickable for OsString {
    fn pick_single(str: Option<&str>) -> PickerArgResult<Self> {
        str.map_or(PickerArgResult::NotFound, |s| {
            PickerArgResult::Parsed(Self::from(s))
        })
    }
}
