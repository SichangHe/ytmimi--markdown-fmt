/// Collection of common functions / macros used for generating tests

#[allow(dead_code)]
pub fn check_formatted_markdown<'a>(
    input: &'a str,
    expected_output: &str,
) -> std::borrow::Cow<'a, str> {
    let formatted = fmtm_ytmimi_markdown_fmt::MarkdownFormatter::default()
        .format(input)
        .expect("formatting won't fail");

    assert_eq!(formatted, expected_output);
    formatted.into()
}

pub fn init_tracing() {
    _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_ansi(true)
        .try_init();
}

#[macro_export]
macro_rules! test {
    ($input:expr) => {
        test!($input, $input)
    };
    ($input:expr, $output:expr) => {{
        $crate::common::init_tracing();
        let formatted = $crate::common::check_formatted_markdown($input, $output);
        if $input != $output {
            // Perform an idempotency check on the formatted markdown
            $crate::common::check_formatted_markdown(&formatted, &formatted);
        }
        formatted
    }};
}

#[macro_export]
macro_rules! test_identical_markdown_events {
    ($input:expr) => {
        test_identical_markdown_events!($input, $input)
    };
    ($input:expr, $output:expr) => {
        let formatted = $crate::test!($input, $output);

        let mut options = pulldown_cmark::Options::all();
        options.remove(pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION);
        let input_events = pulldown_cmark::Parser::new_ext($input, options.clone()).into_iter()
                .filter(|e| {
                    // We don't care about removing empty text nodes
                    !matches!(e, pulldown_cmark::Event::Text(text) if text.trim().is_empty())
                })
            .collect::<Vec<_>>();
        let formatted_events = pulldown_cmark::Parser::new_ext(&formatted, options)
                .into_iter()
                .filter(|e| {
                    // We don't care about removing empty text nodes
                    !matches!(e, pulldown_cmark::Event::Text(text) if text.trim().is_empty())
                })
                .collect::<Vec<_>>();

        assert_eq!(formatted_events.len(), input_events.len());

        let all_events_equal = input_events.into_iter()
            .zip(formatted_events.into_iter())
            .all(|(input_event, formatted_event)| match (&input_event, &formatted_event)
        {
            (pulldown_cmark::Event::Text(input), pulldown_cmark::Event::Text(formatted)) => {
                input.trim() == formatted.trim()
            },
            _ => input_event == formatted_event
        });
        assert!(all_events_equal)
    };
}
