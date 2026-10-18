use arg_picker::{IntoPicker, Picker, macros::arg};

// The `or` family accepts a closure that returns anything convertible into the expected
// value, not only the value itself.

#[test]
fn test_or_accepts_into() {
    let value: String = Vec::<&str>::new()
        .to_picker()
        .pick(arg![name: String])
        .or(|| "aaa")
        .unwrap();
    assert_eq!(value, "aaa");
}

#[test]
fn test_pick_or_accepts_into() {
    let value: String = Vec::<&str>::new()
        .to_picker()
        .pick_or::<String>(arg![name: String], || "aaa")
        .unwrap();
    assert_eq!(value, "aaa");
}

#[test]
fn test_picker_pick_or_accepts_into() {
    let value: String = Picker::<()>::from(Vec::<&str>::new())
        .pick_or::<String>(arg![name: String], || "aaa")
        .unwrap();
    assert_eq!(value, "aaa");
}

#[test]
fn test_pattern_pick_or_accepts_into() {
    let (first, second): (String, String) = vec!["--first", "Alice"]
        .to_picker()
        .pick(arg![first: String])
        .pick_or::<String>(arg![second: String], || "aaa")
        .unwrap();
    assert_eq!(first, "Alice");
    assert_eq!(second, "aaa");
}

#[test]
fn test_or_route_accepts_into() {
    let result: Result<bool, String> = Vec::<&str>::new()
        .to_picker()
        .with_route::<String>()
        .pick(arg![verbose: bool])
        .or_route(|| "route")
        .to_result();
    assert_eq!(result, Err("route".to_string()));
}

#[test]
fn test_pick_or_route_accepts_into() {
    let result: Result<bool, String> = Vec::<&str>::new()
        .pick_or_route::<bool, String>(arg![verbose: bool], || "route")
        .to_result();
    assert_eq!(result, Err("route".to_string()));
}

#[test]
fn test_picker_pick_or_route_accepts_into() {
    let result: Result<bool, String> = Picker::<String>::from(Vec::<&str>::new())
        .pick_or_route::<bool>(arg![verbose: bool], || "route")
        .to_result();
    assert_eq!(result, Err("route".to_string()));
}
