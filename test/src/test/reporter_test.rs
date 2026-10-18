use arg_picker::{
    AnnotateSnippetsReporter, IntoPicker, UnmatchedArgs, UnmatchedReporter, consts::UNMATCHED,
    macros::arg,
};

fn unmatched() -> UnmatchedArgs {
    let (_name, unmatched): (String, UnmatchedArgs) =
        vec!["--name", "Alice", "stray", "--unknown", "value"]
            .to_picker()
            .pick(arg![name: String])
            .pick(&UNMATCHED)
            .unwrap();
    unmatched
}

#[test]
fn test_annotate_snippets_render_mentions_the_unmatched_words() {
    let rendered = AnnotateSnippetsReporter::render_with(&unmatched(), false);
    assert!(rendered.contains(" --> Unmatched arguments"));
    assert!(!rendered.contains("error:"));
    assert!(!rendered.contains("1 | "));
    assert!(rendered.contains("  | --name"));
    assert!(rendered.contains("unexpected argument"));
    assert!(rendered.contains("stray"));
    assert!(rendered.contains("--unknown"));
    assert!(rendered.contains("value"));
}

#[test]
fn test_annotate_snippets_render_is_empty_when_everything_matched() {
    let (_, unmatched): (String, UnmatchedArgs) = vec!["--name", "Alice"]
        .to_picker()
        .pick(arg![name: String])
        .pick(&UNMATCHED)
        .unwrap();
    assert!(AnnotateSnippetsReporter::render_with(&unmatched, false).is_empty());
}

#[test]
fn test_report_calls_the_reporter() {
    struct Recorder;

    impl Default for Recorder {
        fn default() -> Self {
            Self
        }
    }

    impl UnmatchedReporter for Recorder {
        fn report(&self, unmatched: UnmatchedArgs) -> bool {
            assert_eq!(unmatched.len(), 3);
            true
        }
    }

    assert!(unmatched().report::<Recorder>());
}

#[test]
fn test_report_with_the_built_in_reporter() {
    let (_, unmatched): (String, UnmatchedArgs) = vec!["--name", "Alice"]
        .to_picker()
        .pick(arg![name: String])
        .pick(&UNMATCHED)
        .unwrap();

    // Nothing to print: the call must still compile and report nothing.
    assert!(!unmatched.report::<AnnotateSnippetsReporter>());
}

#[test]
fn test_built_in_reporter_reports_when_there_is_something() {
    assert!(AnnotateSnippetsReporter.report(unmatched()));
}

#[test]
fn test_annotate_snippets_color_can_be_forced() {
    let rendered = AnnotateSnippetsReporter::render_with(&unmatched(), true);
    assert!(rendered.contains('\u{1b}'));
}
