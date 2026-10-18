use arg_picker::value::Flag;
use arg_picker::{IntoPicker, PickerArg, macros::arg};

// Tuple constructors. The type is given through the turbofish, since `char`, `&str` and the
// tuples say nothing about it.

#[test]
fn test_arg_from_short_character() {
    let parsed: String = vec!["-n", "Alice"]
        .to_picker()
        .pick::<String>('n')
        .or_default()
        .unwrap();
    assert_eq!(parsed, "Alice");
}

#[test]
fn test_arg_from_long_name() {
    let parsed: String = vec!["--age", "42"]
        .to_picker()
        .pick::<String>("age")
        .or_default()
        .unwrap();
    assert_eq!(parsed, "42");
}

#[test]
fn test_arg_from_positional() {
    let parsed: String = vec!["Alice"]
        .to_picker()
        .pick::<String>(())
        .or_default()
        .unwrap();
    assert_eq!(parsed, "Alice");
}

#[test]
fn test_arg_from_long_and_short() {
    let parsed: String = vec!["--name", "Alice"]
        .to_picker()
        .pick::<String>(("name", 'n'))
        .or_default()
        .unwrap();
    assert_eq!(parsed, "Alice");
}

#[test]
fn test_arg_from_long_alias_and_short() {
    let parsed: String = vec!["--nick", "Alice"]
        .to_picker()
        .pick::<String>(("name", "nick", 'n'))
        .or_default()
        .unwrap();
    assert_eq!(parsed, "Alice");
}

#[test]
fn test_arg_from_long_and_aliases() {
    let parsed: String = vec!["--nmt", "Alice"]
        .to_picker()
        .pick::<String>(("name", "nick", "nmt"))
        .or_default()
        .unwrap();
    assert_eq!(parsed, "Alice");
}

#[test]
fn test_arg_from_long_three_aliases_and_short() {
    let parsed: String = vec!["--nm", "Alice"]
        .to_picker()
        .pick::<String>(("name", "nick", "nmt", "nm", 'n'))
        .or_default()
        .unwrap();
    assert_eq!(parsed, "Alice");
}

#[test]
fn test_arg_from_long_three_aliases() {
    let parsed: String = vec!["--nm", "Alice"]
        .to_picker()
        .pick::<String>(("name", "nick", "nmt", "nm"))
        .or_default()
        .unwrap();
    assert_eq!(parsed, "Alice");
}

#[test]
fn test_arg_from_tuple_has_the_expected_fields() {
    let definition = PickerArg::<String>::from(("name", "nick", "nmt", "nm", 'n'));
    assert_eq!(definition.full(), &["name", "nick", "nmt", "nm"][..]);
    assert_eq!(definition.short(), Some('n'));
    assert!(!definition.is_positional());
}

#[test]
fn test_pick_accepts_arg_by_value() {
    let parsed: String = vec!["--name", "Alice"]
        .to_picker()
        .pick(arg![name: String])
        .or_default()
        .unwrap();
    assert_eq!(parsed, "Alice");
}

#[test]
fn test_pick_accepts_arg_by_ref() {
    let definition = arg![name: String];
    let parsed: String = vec!["--name", "Alice"]
        .to_picker()
        .pick(&definition)
        .or_default()
        .unwrap();
    assert_eq!(parsed, "Alice");
}

#[test]
fn test_chain_takes_args_by_value() {
    let (_name, verbose): (String, Flag) = vec!["--name", "Alice", "--verbose"]
        .to_picker()
        .pick(arg![name: String])
        .or_default()
        .pick(arg![verbose: Flag])
        .unwrap();
    assert!(verbose.bool());
}

#[test]
fn test_into_picker_pick_accepts_arg_by_value() {
    let parsed: String = vec!["--name", "Alice"]
        .pick(arg![name: String])
        .or_default()
        .unwrap();
    assert_eq!(parsed, "Alice");
}

#[test]
fn test_picker_pick_accepts_arg_by_value() {
    let parsed: String = arg_picker::Picker::<()>::from(vec!["--name", "Alice"])
        .pick(arg![name: String])
        .or_default()
        .unwrap();
    assert_eq!(parsed, "Alice");
}

#[test]
fn test_arg_ref_becomes_arg() {
    let definition = arg![name: String];
    let copied: PickerArg<'_, String> = (&definition).into();
    assert_eq!(copied.full(), definition.full());
    assert_eq!(copied.short(), definition.short());
    assert_eq!(copied.is_positional(), definition.is_positional());
}

#[test]
fn test_pick_accepts_custom_into_arg() {
    struct Definition<'a>(PickerArg<'a, String>);

    impl<'a> From<Definition<'a>> for PickerArg<'a, String> {
        fn from(value: Definition<'a>) -> Self {
            value.0
        }
    }

    let parsed: String = vec!["--name", "Alice"]
        .to_picker()
        .pick(Definition(arg![name: String]))
        .or_default()
        .unwrap();
    assert_eq!(parsed, "Alice");
}
