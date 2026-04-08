# Changelog

## [Unreleased]

## [0.0.4](https://github.com/SichangHe/ytmimi--markdown-fmt/compare/v0.0.3...v0.0.4) - 2026-04-08

### Added

- [**breaking**] Unify external formatters;customizable display math formatting ([#2](https://github.com/SichangHe/ytmimi--markdown-fmt/pull/2))
- adapt&rebrand for FMTM;trim unused code-gen

### Fixed

- *(test)* make clippy happy about line width
- *(ci)* bump sccache to avoid failure
- keep wrapped inline code and separators idempotent
- guard newline counting against reversed ranges
- preserve GFM alert markers in blockquotes
- keep nested paragraph softbreak indentation idempotent
- fix last_position check panic on emoji
- fix gitignore to fix parent repo Release-Plz CI

### Other

- fix loose list test clippy warning
- allow missing docs in generated spec crates
- how GFM alerts should be handled
- non-idempotent if item body contain inline code
- release
- release
- use chars instead of bytes whenever possible&sensible
- add failing emoji test
- make my life easier w/ insta
- minor simplification;investigate math
- CIs as standalone repo
- Changes from sh-opinion;ignore lock file
- Fix task list item formatting
- Handle edge case with reference link definition titles
- Add (Event, Range) adapter to transform tight lists into loose lists
- Add a way to configure tests using HTML comments
- Allow users to configure the `max_width`
- Add Paragraph formatter and `max_width` config.
- Add `textwrap` as a dependency to help with paragraph wrapping
- use `rust_search` to search for nested markdown files for testing
- move formatting and idempotence tests to `src/test.rs`
- start work on the configuration system
- make FormatState generic over the (Event, Range) iterator
- Centralize writes through the `std::fmt::Write` impl
- remove whitespace from links and their definitions
- rename `ReferenceLink` -> `ReferenceLinkDefinition`
- Add a broken link callback
- use `std::fmt::Write` instead of `pulldown_cmark::escape::StrWrite`
- Better handling of empty, fenced code blocks
- remove ordered list marker increment feature
- Better handling of empty list markers
- Add trailing newline if input contained trailing newline
- remove space between blockquotes and improve empty blockquote formatting
- Add tracing and log out events
- Add lints to project
- Add crate level documentation to lib.rs
- Set Minimum supported rust version to 1.74.0
- Gate dependencies / binary behind features
- Add Additional Package details
- Add MIT LICENSE
- Fix clippy lints
- Fix typo in generated tests
- Fix typo
- Test license files ([#7](https://github.com/SichangHe/ytmimi--markdown-fmt/pull/7))
- Remove `use Tag::*` from `start_tag` and `end_tag` methods
- Revert commit that consumes compiler internals (PR 3)
- upgrade the CLI using clap and start using anyhow ([#4](https://github.com/SichangHe/ytmimi--markdown-fmt/pull/4))
- Lay the groundwork to consume compiler internals and parse crates ([#3](https://github.com/SichangHe/ytmimi--markdown-fmt/pull/3))
- configure rustfmt to format code blocks in doc comments
- update documentation and add some doc tests
- Add top level `rewrite_markdown` functions
- Use a FormatterBuilder in the public API
- fmt with rustfmt
- Add GitHub Actions to check formatting and run tests
- initial commit

## [0.0.3](https://github.com/SichangHe/fmtm_ytmimi_markdown_fmt/compare/v0.0.2...v0.0.3) - 2024-05-29

### Added
- [**breaking**] Unify external formatters;customizable display math formatting ([#2](https://github.com/SichangHe/fmtm_ytmimi_markdown_fmt/pull/2))

## [0.0.2](https://github.com/SichangHe/fmtm_ytmimi_markdown_fmt/compare/v0.0.1...v0.0.2) - 2024-05-29

### Fixed
- fix last_position check panic on emoji

### Other
- use chars instead of bytes whenever possible&sensible
- add failing emoji test
- make my life easier w/ insta
- minor simplification;investigate math
