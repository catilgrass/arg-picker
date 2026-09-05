use arg_picker::value::Flag;
use arg_picker::{IntoPicker, Pickable, macros::arg};

#[derive(Pickable)]
struct MyType {
    number: i32,
    name: String,
    info: Option<String>,
}

#[test]
fn test_pickialize_parses_full_struct() {
    let raw: &[&str] = &["42", "hello", "extra"];
    let parsed = <MyType as Pickable>::pick(raw).unwrap();
    assert_eq!(parsed.number, 42);
    assert_eq!(parsed.name, "hello");
    assert_eq!(parsed.info, Some("extra".to_string()));
}

#[test]
fn test_pickialize_missing_optional_field_ok() {
    let raw: &[&str] = &["7", "world"];
    let parsed = <MyType as Pickable>::pick(raw).unwrap();
    assert_eq!(parsed.number, 7);
    assert_eq!(parsed.name, "world");
    assert_eq!(parsed.info, None);
}

#[test]
fn test_pickialize_missing_required_field_not_found() {
    let raw: &[&str] = &["1"];
    let result = <MyType as Pickable>::pick(raw);
    assert!(result.is_err());
}

#[derive(Pickable)]
#[allow(non_camel_case_types, non_snake_case)]
enum MyCommand {
    run_fast,
    foo_bar,
}

#[test]
fn test_pickialize_enum_parses_snake_case_input() {
    use arg_picker::SinglePickable;

    let parsed = <MyCommand as SinglePickable>::pick_single(Some("run_fast")).unwrap();
    assert!(matches!(parsed, MyCommand::run_fast));

    let parsed = <MyCommand as SinglePickable>::pick_single(Some("foo-bar")).unwrap();
    assert!(matches!(parsed, MyCommand::foo_bar));
}

#[test]
fn test_pickialize_enum_parses_pascal_input() {
    use arg_picker::SinglePickable;

    let parsed = <MyCommand as SinglePickable>::pick_single(Some("RunFast")).unwrap();
    assert!(matches!(parsed, MyCommand::run_fast));
}

#[test]
fn test_pickialize_enum_unknown_returns_not_found() {
    use arg_picker::SinglePickable;

    let result = <MyCommand as SinglePickable>::pick_single(Some("unknown"));
    assert!(result.is_err());
}

#[derive(Pickable)]
struct ArgStyled {
    #[arg(short = 's', long = "server", aliases = ["host"])]
    name: String,
}

#[test]
fn test_pickialize_struct_arg_attr_short() {
    let raw: &[&str] = &["-s", "alpha"];
    let parsed = <ArgStyled as Pickable>::pick(raw).unwrap();
    assert_eq!(parsed.name, "alpha");
}

#[test]
fn test_pickialize_struct_arg_attr_long() {
    let raw: &[&str] = &["--server", "beta"];
    let parsed = <ArgStyled as Pickable>::pick(raw).unwrap();
    assert_eq!(parsed.name, "beta");
}

#[test]
fn test_pickialize_struct_arg_attr_alias() {
    let raw: &[&str] = &["--host", "gamma"];
    let parsed = <ArgStyled as Pickable>::pick(raw).unwrap();
    assert_eq!(parsed.name, "gamma");
}

#[test]
fn test_pickialize_struct_arg_attr_overrides_field_name() {
    let raw: &[&str] = &["--name", "delta"];
    let result = <ArgStyled as Pickable>::pick(raw);
    assert!(result.is_err());
}

#[derive(Pickable)]
struct ImplicitArgs {
    #[arg(short)]
    count: i32,
    #[arg(long)]
    user_name: String,
    #[arg(short, long)]
    server_name: String,
}

#[test]
fn test_pickialize_implicit_short_long() {
    let raw: &[&str] = &[
        "-c", "3",
        "--user-name", "alice",
        "--server-name", "db",
    ];
    let parsed = <ImplicitArgs as Pickable>::pick(raw).unwrap();
    assert_eq!(parsed.count, 3);
    assert_eq!(parsed.user_name, "alice");
    assert_eq!(parsed.server_name, "db");
}

#[test]
fn test_pickialize_implicit_short_derived_from_field() {
    let raw: &[&str] = &[
        "--count", "7",
        "--user-name", "bob",
        "-s", "web",
    ];
    let parsed = <ImplicitArgs as Pickable>::pick(raw).unwrap();
    assert_eq!(parsed.count, 7);
    assert_eq!(parsed.user_name, "bob");
    assert_eq!(parsed.server_name, "web");
}


#[derive(Pickable)]
struct InnerNamed {
    #[arg(long = "number")]
    number: i32,
    #[arg(long = "name")]
    name: String,
}

#[test]
fn test_composite_tag_does_not_claim_unrelated_flag() {
    let (inner, verbose): (InnerNamed, Flag) = vec![
        "--number", "1",
        "--name", "alice",
        "--verbose",
    ]
    .to_picker()
    .pick(&arg![inner: InnerNamed])
    .pick(&arg![verbose: Flag])
    .unwrap();

    assert_eq!(inner.number, 1);
    assert_eq!(inner.name, "alice");
    assert_eq!(verbose, Flag::Active);
}
