use crate::UnmatchedArgs;

/// Reports the arguments that no pick consumed.
///
/// A reporter decides what to do with the value: [`AnnotateSnippetsReporter`] renders it as a
/// diagnostic on standard error, and a program can implement this trait to report in its own
/// format.
///
/// [`report`](Self::report) returns whether it reported anything, so a caller can tell an empty
/// report from one that produced output.
///
/// The default implementation does nothing and returns `false`.
pub trait UnmatchedReporter {
    /// Reports `unmatched`, returning `true` if anything was reported.
    fn report(&self, unmatched: UnmatchedArgs) -> bool {
        let _ = unmatched;
        false
    }
}

impl UnmatchedArgs {
    /// Reports these arguments through reporter `R`, returning whether `R` reported anything.
    ///
    /// `R` is constructed with [`Default`], so the call site names the reporter and nothing else:
    ///
    /// ```
    /// use arg_picker::{UnmatchedArgs, consts::UNMATCHED, macros::arg, prelude::*};
    ///
    /// # struct Recorder;
    /// # impl Default for Recorder {
    /// #     fn default() -> Self { Self }
    /// # }
    /// # impl arg_picker::UnmatchedReporter for Recorder {
    /// #     fn report(&self, unmatched: UnmatchedArgs) -> bool {
    /// #         assert!(unmatched.is_empty());
    /// #         false
    /// #     }
    /// # }
    /// let (_, unmatched): (String, UnmatchedArgs) = vec!["--name", "Alice"]
    ///     .pick(arg![name: String])
    ///     .pick(&UNMATCHED)
    ///     .unwrap();
    /// assert!(!unmatched.report::<Recorder>());
    /// ```
    #[must_use]
    pub fn report<R>(&self) -> bool
    where
        R: UnmatchedReporter + Default,
    {
        R::default().report(self.clone())
    }
}

#[cfg(feature = "reporters")]
mod snippets {
    use std::io::IsTerminal;

    use super::UnmatchedReporter;
    use crate::UnmatchedArgs;
    use annotate_snippets::renderer::Style;
    use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet};

    const TITLE: &str = " --> Unmatched arguments";
    const LABEL: &str = "unexpected argument";

    // `annotate-snippets` always prints a line-number column and has no option to leave it out.
    // A one-digit line renders as `1 | `; replacing the digit with a space leaves the two-space
    // gutter this reporter asks for.
    const LINED: &str = "1 | ";
    const GUTTER: &str = "  | ";

    /// Renders unmatched arguments as an [`annotate-snippets`] diagnostic.
    ///
    /// The original argument list is shown as one line and every unmatched word is marked at the
    /// position it held. The output is written to standard error; [`render`](Self::render) builds
    /// the text without printing it.
    ///
    /// The wording is English.
    #[derive(Debug, Default, Clone, Copy)]
    pub struct AnnotateSnippetsReporter;

    impl AnnotateSnippetsReporter {
        /// Returns the diagnostic text for `unmatched`, or an empty string when there is nothing
        /// to report.
        ///
        /// Color is chosen automatically from the environment and standard error; use
        /// [`render_with`](Self::render_with) to decide it explicitly.
        #[must_use]
        pub fn render(unmatched: &UnmatchedArgs) -> String {
            Self::render_with(unmatched, color_enabled())
        }

        /// Returns the diagnostic text for `unmatched`, with color on or off.
        ///
        /// Returns an empty string when there is nothing to report.
        #[must_use]
        pub fn render_with(unmatched: &UnmatchedArgs, color: bool) -> String {
            if unmatched.is_empty() {
                return String::new();
            }

            let original = unmatched.original();
            let source = original.join(" ");

            let mut starts = Vec::with_capacity(original.len());
            let mut offset = 0;
            for word in original {
                starts.push(offset);
                offset += word.len() + 1;
            }

            let mut snippet = Snippet::source(&source).line_start(1);
            for entry in unmatched {
                if let Some(&start) = starts.get(entry.index) {
                    snippet = snippet.annotation(
                        AnnotationKind::Primary
                            .span(start..start + entry.raw.len())
                            .label(LABEL),
                    );
                }
            }

            let report = [Level::ERROR.no_name().primary_title(TITLE).element(snippet)];
            let renderer = if color {
                // The line number is removed below, so it must not carry any escape codes.
                Renderer::styled().line_num(Style::new())
            } else {
                Renderer::plain()
            };
            strip_line_number(&renderer.render(&report))
        }
    }

    fn strip_line_number(rendered: &str) -> String {
        let mut out = String::with_capacity(rendered.len());
        for line in rendered.lines() {
            if let Some(rest) = line.strip_prefix(LINED) {
                out.push_str(GUTTER);
                out.push_str(rest);
            } else {
                out.push_str(line);
            }
            out.push('\n');
        }
        out
    }

    /// Whether standard error should receive ANSI colors.
    ///
    /// `NO_COLOR` wins over everything; `CLICOLOR_FORCE` can force color even when not a
    /// terminal; `CLICOLOR=0` and `TERM=dumb` turn it off; otherwise the choice follows whether
    /// standard error is a terminal.
    fn color_enabled() -> bool {
        if std::env::var_os("NO_COLOR").is_some() {
            return false;
        }
        if std::env::var_os("CLICOLOR_FORCE").is_some_and(|value| value != "0") {
            return true;
        }
        if std::env::var_os("CLICOLOR").is_some_and(|value| value == "0") {
            return false;
        }
        if std::env::var_os("TERM").is_some_and(|value| value == "dumb") {
            return false;
        }
        std::io::stderr().is_terminal()
    }

    impl UnmatchedReporter for AnnotateSnippetsReporter {
        fn report(&self, unmatched: UnmatchedArgs) -> bool {
            let rendered = Self::render(&unmatched);
            if rendered.is_empty() {
                return false;
            }

            eprint!("{rendered}");
            if !rendered.ends_with('\n') {
                eprintln!();
            }
            true
        }
    }
}

#[cfg(feature = "reporters")]
pub use snippets::AnnotateSnippetsReporter;
