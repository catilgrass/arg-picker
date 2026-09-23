use arg_picker::{IntoPicker, Pickable, macros::arg, value::Flag};

// A named argument that was not given at all

#[test]
fn test_option_named_absent_is_none() {
    let result: Result<Option<String>, &'static str> = Vec::<&str>::new()
        .with_route::<&'static str>()
        .pick(&arg![name: Option<String>])
        .or_route(|| "no_value")
        .to_result();
    assert_eq!(result, Ok(None));
}

#[test]
fn test_option_named_present_with_value_is_some() {
    let result: Result<Option<String>, &'static str> = vec!["--name", "Alice"]
        .with_route::<&'static str>()
        .pick(&arg![name: Option<String>])
        .or_route(|| "no_value")
        .to_result();
    assert_eq!(result, Ok(Some("Alice".to_string())));
}

// A named argument that was given and left without a value

#[test]
fn test_option_named_without_value_is_none() {
    // An optional argument that is left without a value is `None`: nothing where a value would
    // be is the same for the type inside as the argument not being there at all.
    let result: Result<Option<String>, &'static str> = vec!["--name"]
        .with_route::<&'static str>()
        .pick(&arg![name: Option<String>])
        .or_route(|| "no_value")
        .to_result();
    assert_eq!(result, Ok(None));
}

#[test]
fn test_required_named_without_value_is_still_a_failure() {
    let result: Result<String, &'static str> = vec!["--name"]
        .with_route::<&'static str>()
        .pick(&arg![name: String])
        .or_route(|| "no_value")
        .to_result();
    assert_eq!(result, Err("no_value"));
}

#[test]
fn test_empty_inline_value_is_a_value() {
    // An inline value is written with the flag, so it is one whatever it is — even empty.
    let result: Result<Option<String>, &'static str> = vec!["--name="]
        .with_route::<&'static str>()
        .pick(&arg![name: Option<String>])
        .or_route(|| "no_value")
        .to_result();
    assert_eq!(result, Ok(Some(String::new())));
}

// What follows a flag: a value, or a word of its own

#[test]
fn test_flag_after_valueless_argument_is_not_taken_as_its_value() {
    // `--name` is given no value, and `--install` is a word of its own rather than that value:
    // it is left to be matched as the flag it names.
    let (name, install) = vec!["--name", "--install"]
        .to_picker()
        .pick(&arg![name: Option<String>])
        .pick(&arg![install: Flag])
        .unpack();

    assert_eq!(name, Some(None));
    assert_eq!(install, Some(Flag::Active));
}

#[test]
fn test_end_of_options_after_a_flag_is_not_taken_as_its_value() {
    let result: Result<Option<String>, &'static str> = vec!["--name", "--"]
        .with_route::<&'static str>()
        .pick(&arg![name: Option<String>])
        .or_route(|| "no_value")
        .to_result();
    assert_eq!(result, Ok(None));
}

#[test]
fn test_value_that_looks_like_a_flag_is_refused() {
    // A word that names an option is not a value, so this is a flag without one ...
    let refused: Result<i32, &'static str> = vec!["--count", "-5"]
        .with_route::<&'static str>()
        .pick(&arg![count: i32])
        .or_route(|| "no_value")
        .to_result();
    assert_eq!(refused, Err("no_value"));

    // ... and writing the value with the flag is how a value like that is given.
    let given: i32 = vec!["--count=-5"]
        .to_picker()
        .pick(&arg![count: i32])
        .or_default()
        .unwrap();
    assert_eq!(given, -5);
}

// Positional arguments

#[test]
fn test_option_positional_absent_is_none_and_present_is_some() {
    let absent: Option<String> = Vec::<&str>::new()
        .to_picker()
        .pick(&arg![Option<String>])
        .or_default()
        .unwrap();
    assert_eq!(absent, None);

    let present: Option<String> = vec!["Alice"]
        .to_picker()
        .pick(&arg![Option<String>])
        .or_default()
        .unwrap();
    assert_eq!(present, Some("Alice".to_string()));
}

#[test]
fn test_flag_like_positional_after_end_of_options_is_a_value() {
    // After `--` everything is positional, so a value is allowed to look like a flag.
    let value: String = vec!["--", "--weird"]
        .to_picker()
        .pick(&arg![String])
        .or_default()
        .unwrap();
    assert_eq!(value, "--weird");
}

// A parameter that takes a list

#[test]
fn test_multi_named_present_with_values_is_the_list() {
    let files: Vec<String> = vec!["--files", "a.txt", "b.txt"]
        .to_picker()
        .pick(&arg![files: Vec<String>])
        .or_default()
        .unwrap();
    assert_eq!(files, vec!["a.txt", "b.txt"]);
}

#[test]
fn test_multi_named_absent_is_empty() {
    let files: Vec<String> = Vec::<&str>::new()
        .to_picker()
        .pick(&arg![files: Vec<String>])
        .or_default()
        .unwrap();
    assert!(files.is_empty());
}

#[test]
fn test_multi_named_without_values_is_empty() {
    // A list that is left without values is the list of none, which is what a list that is not
    // there at all is as well.
    let files: Vec<String> = vec!["--files"]
        .to_picker()
        .pick(&arg![files: Vec<String>])
        .or_default()
        .unwrap();
    assert!(files.is_empty());
}

#[test]
fn test_flag_after_valueless_list_argument_is_not_taken_as_its_value() {
    let (files, install) = vec!["--files", "--install"]
        .to_picker()
        .pick(&arg![files: Vec<String>])
        .pick(&arg![install: Flag])
        .unpack();

    assert_eq!(files, Some(Vec::<String>::new()));
    assert_eq!(install, Some(Flag::Active));
}

#[test]
fn test_a_lone_dash_is_a_value_of_a_list() {
    let files: Vec<String> = vec!["--files", "-"]
        .to_picker()
        .pick(&arg![files: Vec<String>])
        .or_default()
        .unwrap();
    assert_eq!(files, vec!["-"]);
}

// A struct of flags, as a command declares them

#[derive(Pickable, Debug, Default)]
struct GenerateFlags {
    #[arg(long)]
    generate: Option<String>,
    #[arg(long)]
    install: Flag,
}

#[test]
fn test_flag_struct_absent_arguments_are_none_and_inactive() {
    let flags: GenerateFlags = Vec::<&str>::new()
        .to_picker()
        .pick(&arg![GenerateFlags])
        .or_default()
        .unwrap();
    assert_eq!(flags.generate, None);
    assert!(!flags.install.bool());
}

#[test]
fn test_flag_struct_given_a_value_parses() {
    let flags: GenerateFlags = vec!["--generate", "alice", "--install"]
        .to_picker()
        .pick(&arg![GenerateFlags])
        .or_default()
        .unwrap();
    assert_eq!(flags.generate, Some("alice".to_string()));
    assert!(flags.install.bool());
}

#[test]
fn test_flag_struct_with_a_valueless_argument_parses_it_as_none() {
    // What a flag left without a value is, is the field's own type's answer: an optional one is
    // `None`, so the struct parses and its other fields are read as usual.
    let flags: GenerateFlags = vec!["--generate"]
        .to_picker()
        .pick(&arg![GenerateFlags])
        .or_default()
        .unwrap();
    assert_eq!(flags.generate, None);
    assert!(!flags.install.bool());
}

#[test]
fn test_flag_struct_does_not_take_a_flag_as_a_value() {
    // `--generate --install`: the flag that follows is not taken as the value, so it is still
    // the flag it names.
    let flags: GenerateFlags = vec!["--generate", "--install"]
        .to_picker()
        .pick(&arg![GenerateFlags])
        .or_default()
        .unwrap();
    assert_eq!(flags.generate, None);
    assert!(flags.install.bool());
}
