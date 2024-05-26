//! Easily format Markdown. [markdown_fmt] supports [CommonMark] and [GitHub Flavored Markdown].
//!
//! [markdown_fmt]: index.html
//! [CommonMark]: https://spec.commonmark.org/
//! [GitHub Flavored Markdown]: https://github.github.com/gfm/
//!
//! # Getting Started
//!
//! ```rust
//! use markdown_fmt::rewrite_markdown;
//!
//! let markdown = r##" # Getting Started
//! 1. numbered lists
//! 1.  are easy!
//! "##;
//!
//! let formatted = r##"# Getting Started
//! 1. numbered lists
//! 1. are easy!
//! "##;
//!
//! let output = rewrite_markdown(markdown)?;
//! # assert_eq!(output, formatted);
//! # Ok::<(), std::fmt::Error>(())
//! ```
//!
//! # Using the [Builder](builder::FormatterBuilder)
//!
//! The builder gives you more control to configure Markdown formatting.
//!
//! ````rust
//! use markdown_fmt::{rewrite_markdown, rewrite_markdown_with_builder, FormatterBuilder};
//!
//! let builder = FormatterBuilder::with_code_block_formatter(|info_string, code_block| {
//!     match info_string.to_lowercase().as_str() {
//!         "markdown" => rewrite_markdown(&code_block).unwrap_or(code_block),
//!         _ => code_block
//!     }
//! });
//!
//! let markdown = r##" # Using the Builder
//! + markdown code block nested in a list
//!   ```markdown
//!   A nested markdown snippet!
//!
//!    * unordered lists
//!    are also pretty easy!
//!    - `-` or `+` can also be used as unordered list markers.
//!    ```
//! "##;
//!
//! let formatted = r##"# Using the Builder
//! + markdown code block nested in a list
//!   ```markdown
//!   A nested markdown snippet!
//!
//!   * unordered lists
//!     are also pretty easy!
//!   - `-` or `+` can also be used as unordered list markers.
//!   ```
//! "##;
//!
//! let output = rewrite_markdown_with_builder(markdown, builder)?;
//! # assert_eq!(output, formatted);
//! # Ok::<(), std::fmt::Error>(())
//! ````

mod adapters;
mod builder;
// TODO: Make pub.
mod config;
mod escape;
mod formatter;
mod links;
mod list;
mod paragraph;
mod table;
#[cfg(test)]
mod test;
mod utils;

pub use builder::FormatterBuilder;
pub use formatter::MarkdownFormatter;

/// Reformat a markdown snippet with all the default settings.
///
/// ```rust
/// # use markdown_fmt::rewrite_markdown;
/// let markdown = r##"  #   Learn Rust Checklist!
/// 1. Read [The Book]
///  2.  Watch tutorials
///   3.   Write some code!
///
/// [The Book]: https://doc.rust-lang.org/book/
/// "##;
///
/// let formatted_markdown = r##"# Learn Rust Checklist!
/// 1. Read [The Book]
/// 2. Watch tutorials
/// 3. Write some code!
///
/// [The Book]: https://doc.rust-lang.org/book/
/// "##;
///
/// let output = rewrite_markdown(markdown).unwrap();
/// assert_eq!(output, formatted_markdown);
/// ```
pub fn rewrite_markdown(input: &str) -> Result<String, std::fmt::Error> {
    rewrite_markdown_with_builder(input, FormatterBuilder::default())
}

/// Reformat a markdown snippet based on Steven Hé (Sīchàng)'s opinion.
///
/// ```rust
/// # use markdown_fmt::rewrite_markdown_sichanghe_opinion;
/// let markdown = r##"  #   Learn Rust Checklist!
/// 1. Read [The Book]
///  2.  Watch tutorials
///   3.   Write some code!
///
/// [The Book]: https://doc.rust-lang.org/book/
/// "##;
///
/// let formatted_markdown = r##"# Learn Rust Checklist!
/// 1. Read [The Book]
/// 1. Watch tutorials
/// 1. Write some code!
///
/// [The Book]: https://doc.rust-lang.org/book/
/// "##;
///
/// let output = rewrite_markdown_sichanghe_opinion(markdown).unwrap();
/// assert_eq!(output, formatted_markdown);
/// ```
pub fn rewrite_markdown_sichanghe_opinion(input: &str) -> Result<String, std::fmt::Error> {
    let mut builder = FormatterBuilder::default();
    builder.sichanghe_config();
    rewrite_markdown_with_builder(input, builder)
}

/// Reformat a markdown snippet with user specified settings
///
/// ```rust
/// # use markdown_fmt::{rewrite_markdown_with_builder, FormatterBuilder};
/// let markdown = r##"  #   Learn Rust Checklist!
/// 1. Read [The Book]
///  2.  Watch tutorials
///   3.   Write some code!
///
/// [The Book]: https://doc.rust-lang.org/book/
/// "##;
///
/// let formatted_markdown = r##"# Learn Rust Checklist!
/// 1. Read [The Book]
/// 2. Watch tutorials
/// 3. Write some code!
///
/// [The Book]: https://doc.rust-lang.org/book/
/// "##;
///
/// let builder = FormatterBuilder::default();
/// let output = rewrite_markdown_with_builder(markdown, builder).unwrap();
/// assert_eq!(output, formatted_markdown);
/// ```
pub fn rewrite_markdown_with_builder(
    input: &str,
    builder: FormatterBuilder,
) -> Result<String, std::fmt::Error> {
    tracing::trace!(?builder);
    let formatter = builder.build();
    formatter.format(input)
}
