use arg_picker::{macros::arg, IntoPicker};

// Basic bool explicit value — present / absent

#[test]
fn test_bool_value_present_true() {
    let parsed: bool = vec!["--verbose", "true"]
        .to_picker()
        .pick(&arg![verbose: bool])
        .or_default()
        .unwrap();
    assert!(parsed);
}

#[test]
fn test_bool_value_present_false() {
    let parsed: bool = vec!["--verbose", "false"]
        .to_picker()
        .pick(&arg![verbose: bool])
        .or_default()
        .unwrap();
    assert!(!parsed);
}

#[test]
fn test_bool_value_absent_uses_default() {
    let parsed: bool = Vec::<&str>::new()
        .to_picker()
        .pick(&arg![verbose: bool])
        .or_default()
        .unwrap();
    assert!(!parsed);
}

// Case-insensitive parsing

#[test]
fn test_bool_value_case_insensitive_upper() {
    let parsed: bool = vec!["--verbose", "TRUE"]
        .to_picker()
        .pick(&arg![verbose: bool])
        .or_default()
        .unwrap();
    assert!(parsed);
}

#[test]
fn test_bool_value_case_insensitive_mixed() {
    let parsed: bool = vec!["--verbose", "FaLsE"]
        .to_picker()
        .pick(&arg![verbose: bool])
        .or_default()
        .unwrap();
    assert!(!parsed);
}

// Short flag — '-v true'

#[test]
fn test_bool_short_value_present() {
    let parsed: bool = vec!["-v", "true"]
        .to_picker()
        .pick(&arg![verbose: bool, 'v'])
        .or_default()
        .unwrap();
    assert!(parsed);
}

// Multiple bool values at once

#[test]
fn test_two_bool_values_both_present() {
    let args = vec!["--flag-a", "true", "--flag-b", "false"];
    let (a, b): (bool, bool) = args
        .to_picker()
        .pick(&arg![flag_a: bool])
        .or_default()
        .pick(&arg![flag_b: bool])
        .or_default()
        .unwrap();
    assert!(a);
    assert!(!b);
}

// Alias matching for bool values

#[test]
fn test_bool_value_with_alias() {
    let parsed: bool = vec!["--cfg", "true"]
        .to_picker()
        .pick(&arg![config: bool, "cfg"])
        .or_default()
        .unwrap();
    assert!(parsed);
}

#[test]
fn test_bool_value_primary_name() {
    let parsed: bool = vec!["--config", "false"]
        .to_picker()
        .pick(&arg![config: bool, "cfg"])
        .or_default()
        .unwrap();
    assert!(!parsed);
}

// Values after `--` (end-of-options marker) are positional and should not
// match a named bool argument.

#[test]
fn test_bool_value_after_end_of_options() {
    let parsed: bool = vec!["--", "--verbose", "true"]
        .to_picker()
        .pick(&arg![verbose: bool])
        .or_default()
        .unwrap();
    assert!(!parsed);
}

// Default values: .or() / .or_default()

#[test]
fn test_or_default_without_args() {
    let parsed: bool = Vec::<&str>::new()
        .to_picker()
        .pick(&arg![verbose: bool])
        .or_default()
        .unwrap();
    assert!(!parsed);
}

#[test]
fn test_or_custom_default() {
    let parsed: bool = Vec::<&str>::new()
        .to_picker()
        .pick(&arg![verbose: bool])
        .or(|| true)
        .unwrap();
    assert!(parsed);
}

#[test]
fn test_bool_invalid_value_uses_custom_default() {
    let parsed: bool = vec!["--verbose", "yes"]
        .to_picker()
        .pick(&arg![verbose: bool])
        .or(|| true)
        .unwrap();
    assert!(parsed);
}

// to_result / to_option interface

#[test]
fn test_to_result_ok() {
    let result: Result<bool, ()> = vec!["--verbose", "true"]
        .to_picker()
        .pick(&arg![verbose: bool])
        .or_default()
        .to_result();
    assert_eq!(result, Ok(true));
}

#[test]
fn test_to_option_some() {
    let opt: Option<bool> = vec!["--verbose", "false"]
        .to_picker()
        .pick(&arg![verbose: bool])
        .or_default()
        .to_option();
    assert_eq!(opt, Some(false));
}

// Chain with_route passthrough

#[test]
fn test_with_route_chain() {
    let parsed: bool = vec!["--flag", "true"]
        .with_route::<String>()
        .pick(&arg![flag: bool])
        .or_default()
        .unwrap();
    assert!(parsed);
}

// Unrelated flag should not match

#[test]
fn test_unrelated_flag_does_not_match() {
    let parsed: bool = vec!["--other", "true"]
        .to_picker()
        .pick(&arg![verbose: bool])
        .or_default()
        .unwrap();
    assert!(!parsed);
}
