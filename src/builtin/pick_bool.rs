use crate::{
    PickerArgResult::{self, NotFound, Parsed},
    SinglePickable,
};

impl SinglePickable for bool {
    fn pick_single(str: Option<&str>) -> PickerArgResult<Self> {
        match str.map(str::trim) {
            Some(s) if s.eq_ignore_ascii_case("true") => Parsed(true),
            Some(s) if s.eq_ignore_ascii_case("false") => Parsed(false),
            _ => NotFound,
        }
    }
}
