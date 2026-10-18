use arg_picker::value::Flag;
use arg_picker::{IntoPicker, PickerArg, macros::arg};

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
