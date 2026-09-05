use std::time::Duration;

use crate::{PickerArgResult, SinglePickable};

impl SinglePickable for Duration {
    fn pick_single(str: Option<&str>) -> PickerArgResult<Self> {
        str.and_then(parse_duration)
            .map_or(PickerArgResult::NotFound, PickerArgResult::Parsed)
    }
}

/// Parses a duration string.
///
/// A bare number is interpreted as seconds. Optional unit suffixes are also
/// supported: `ns`, `us`, `ms`, `s`, `m`, and `h`.
fn parse_duration(raw: &str) -> Option<Duration> {
    let raw = raw.trim();
    if raw.is_empty() {
        return None;
    }

    let (value, multiplier) = split_unit(raw);
    let number: f64 = value.parse().ok()?;
    Duration::try_from_secs_f64(number * multiplier).ok()
}

/// Splits a duration string into its numeric part and a seconds multiplier.
fn split_unit(raw: &str) -> (&str, f64) {
    const UNITS: [(&str, f64); 6] = [
        ("ns", 1e-9),
        ("us", 1e-6),
        ("ms", 1e-3),
        ("s", 1.0),
        ("m", 60.0),
        ("h", 3_600.0),
    ];

    for (suffix, multiplier) in UNITS {
        if let Some(stripped) = raw.strip_suffix(suffix) {
            return (stripped, multiplier);
        }
    }

    (raw, 1.0)
}
