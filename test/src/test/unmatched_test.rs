use arg_picker::{IntoPicker, PickerArgs, UnmatchedArgs, consts::UNMATCHED, macros::arg};

#[test]
fn test_unmatched_is_empty_when_all_arguments_are_matched() {
    let (name, unmatched): (String, UnmatchedArgs) = vec!["--name", "Alice"]
        .to_picker()
        .pick(arg![name: String])
        .pick(&UNMATCHED)
        .unwrap();

    assert_eq!(name, "Alice");
    assert!(unmatched.is_empty());
}

#[test]
fn test_unmatched_reports_words_and_positions() {
    let (_name, unmatched): (String, UnmatchedArgs) =
        vec!["--name", "Alice", "stray", "--unknown", "value"]
            .to_picker()
            .pick(arg![name: String])
            .pick(&UNMATCHED)
            .unwrap();

    let entries: Vec<(usize, &str)> = unmatched
        .iter()
        .map(|entry| (entry.index, entry.raw.as_str()))
        .collect();
    assert_eq!(entries, vec![(2, "stray"), (3, "--unknown"), (4, "value")]);
    assert_eq!(
        unmatched.original().join(" "),
        "--name Alice stray --unknown value"
    );
}

#[test]
fn test_unmatched_does_not_report_the_end_of_options_marker() {
    let (_name, unmatched): (String, UnmatchedArgs) = vec!["--name", "Alice", "--", "tail"]
        .to_picker()
        .pick(arg![name: String])
        .pick(&UNMATCHED)
        .unwrap();

    let entries: Vec<(usize, &str)> = unmatched
        .iter()
        .map(|entry| (entry.index, entry.raw.as_str()))
        .collect();
    assert_eq!(entries, vec![(3, "tail")]);
    // The copy keeps the marker the entries leave out.
    assert_eq!(unmatched.original().join(" "), "--name Alice -- tail");
}

#[test]
fn test_unmatched_iterates_by_reference() {
    let (_, unmatched): (String, UnmatchedArgs) = vec!["--name", "Alice", "stray"]
        .to_picker()
        .pick(arg![name: String])
        .pick(&UNMATCHED)
        .unwrap();

    let mut seen = Vec::new();
    for entry in &unmatched {
        seen.push(entry.raw.as_str());
    }
    assert_eq!(seen, vec!["stray"]);
}

#[test]
fn test_unmatched_iterates_by_value() {
    let (_, unmatched): (String, UnmatchedArgs) = vec!["--name", "Alice", "stray"]
        .to_picker()
        .pick(arg![name: String])
        .pick(&UNMATCHED)
        .unwrap();

    let mut seen = Vec::new();
    for entry in unmatched {
        seen.push((entry.index, entry.raw));
    }
    assert_eq!(seen, vec![(2, "stray".to_string())]);
}

#[test]
fn test_unmatched_converts_to_picker_args_without_positions() {
    let (_, unmatched): (String, UnmatchedArgs) = vec!["--name", "Alice", "stray", "tail"]
        .to_picker()
        .pick(arg![name: String])
        .pick(&UNMATCHED)
        .unwrap();

    let rest: PickerArgs = unmatched.into();
    assert_eq!(rest.len(), 2);
    assert_eq!(rest.get(0), Some("stray"));
    assert_eq!(rest.get(1), Some("tail"));
}
