use std::ffi::OsString;
use std::time::Duration;

use arg_picker::{macros::arg, IntoPicker};

#[test]
fn test_char_named_present() {
    let val: char = vec!["--separator", ","]
        .to_picker()
        .pick(&arg![separator: char])
        .or(|| '?')
        .unwrap();
    assert_eq!(val, ',');
}

#[test]
fn test_char_named_invalid_uses_fallback() {
    let val: char = vec!["--separator", "ab"]
        .to_picker()
        .pick(&arg![separator: char])
        .or(|| '?')
        .unwrap();
    assert_eq!(val, '?');
}

#[test]
fn test_os_string_named_present() {
    let val: OsString = vec!["--input", "config.json"]
        .to_picker()
        .pick(&arg![input: OsString])
        .or_default()
        .unwrap();
    assert_eq!(val, OsString::from("config.json"));
}

#[test]
fn test_duration_seconds() {
    let val: Duration = vec!["--timeout", "5"]
        .to_picker()
        .pick(&arg![timeout: Duration])
        .or_default()
        .unwrap();
    assert_eq!(val, Duration::from_secs(5));
}

#[test]
fn test_duration_fractional_seconds() {
    let val: Duration = vec!["--timeout", "1.5"]
        .to_picker()
        .pick(&arg![timeout: Duration])
        .or_default()
        .unwrap();
    assert_eq!(val, Duration::from_millis(1500));
}

#[test]
fn test_duration_invalid_uses_default() {
    let val: Duration = vec!["--timeout", "-1"]
        .to_picker()
        .pick(&arg![timeout: Duration])
        .or_default()
        .unwrap();
    assert_eq!(val, Duration::ZERO);
}

#[test]
fn test_duration_with_unit_suffix() {
    let val: Duration = vec!["--timeout", "500ms"]
        .to_picker()
        .pick(&arg![timeout: Duration])
        .or_default()
        .unwrap();
    assert_eq!(val, Duration::from_millis(500));
}

#[test]
fn test_duration_minutes() {
    let val: Duration = vec!["--timeout", "2m"]
        .to_picker()
        .pick(&arg![timeout: Duration])
        .or_default()
        .unwrap();
    assert_eq!(val, Duration::from_secs(120));
}
