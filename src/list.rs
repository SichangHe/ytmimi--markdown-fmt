use std::borrow::Cow;
use std::num::ParseIntError;
// Including all these spaces might be overkill, but it probably doesn't hurt.
// In practice we'll see far fewer digits in an ordered list.
//
// <https://github.github.com/gfm/#list-items> mentions that:
//
//     An ordered list marker is a sequence of 1–9 arabic digits (0-9), followed by either a .
//     character or a ) character. (The reason for the length limit is that with 10 digits we
//     start seeing integer overflows in some browsers.)
//
const ZERO_PADDING: &str = "00000000000000000000";

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ListMarker {
    Ordered {
        zero_padding: usize,
        number: usize,
        marker: OrderedListMarker,
    },
    Unordered(UnorderedListMarker),
}

impl std::default::Default for ListMarker {
    fn default() -> Self {
        ListMarker::Unordered(UnorderedListMarker::Asterisk)
    }
}

impl ListMarker {
    // TODO(ytmimi) Add a configuration to allow incrementing ordered lists
    #[allow(dead_code)]
    pub(super) fn increment_count(&mut self) {
        match self {
            Self::Ordered { number, .. } => {
                *number += 1;
            }
            Self::Unordered(_) => {}
        }
    }

    pub(super) fn indentation(&self) -> Cow<'static, str> {
        "    ".into() // SH: I fix indentation to 4 spaces.
    }

    pub(super) fn marker_char(&self) -> char {
        match self {
            Self::Ordered { marker, .. } => marker.into(),
            Self::Unordered(marker) => marker.into(),
        }
    }

    pub(super) fn zero_padding(&self) -> &'static str {
        match self {
            Self::Ordered { zero_padding, .. } => &ZERO_PADDING[..*zero_padding],
            Self::Unordered(_) => "",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) enum OrderedListMarker {
    Period,
    Parenthesis,
}

impl From<&OrderedListMarker> for char {
    fn from(value: &OrderedListMarker) -> Self {
        match value {
            OrderedListMarker::Period => '.',
            OrderedListMarker::Parenthesis => ')',
        }
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub(super) struct InvalidMarker(char);

impl TryFrom<char> for OrderedListMarker {
    type Error = InvalidMarker;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(OrderedListMarker::Period),
            ')' => Ok(OrderedListMarker::Parenthesis),
            _ => Err(InvalidMarker(value)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) enum UnorderedListMarker {
    Asterisk,
    Plus,
    Hyphen,
}

impl From<&UnorderedListMarker> for char {
    fn from(value: &UnorderedListMarker) -> Self {
        match value {
            UnorderedListMarker::Asterisk => '*',
            UnorderedListMarker::Plus => '+',
            UnorderedListMarker::Hyphen => '-',
        }
    }
}

impl TryFrom<char> for UnorderedListMarker {
    type Error = InvalidMarker;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '*' => Ok(UnorderedListMarker::Asterisk),
            '+' => Ok(UnorderedListMarker::Plus),
            '-' => Ok(UnorderedListMarker::Hyphen),
            _ => Err(InvalidMarker(value)),
        }
    }
}

/// Some error occured when parsing a ListMarker from a &str
#[derive(Debug, PartialEq, Eq)]
pub(super) enum ParseListMarkerError {
    /// Did not contain the correct list markers.
    NoMarkers,
    /// Invalid char where a list marker was expected
    InvalidMarker(InvalidMarker),
    /// Failed to parse an integer for ordered lists
    ParseIntError(ParseIntError),
}

impl From<InvalidMarker> for ParseListMarkerError {
    fn from(value: InvalidMarker) -> Self {
        Self::InvalidMarker(value)
    }
}

impl From<ParseIntError> for ParseListMarkerError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl std::str::FromStr for ListMarker {
    type Err = ParseListMarkerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(ParseListMarkerError::NoMarkers);
        }

        if let Some('*' | '+' | '-') = s.chars().next() {
            return Ok(ListMarker::Unordered(UnorderedListMarker::Hyphen));
        }

        // SH: I always use `1.` and `-`.
        Ok(ListMarker::Ordered {
            zero_padding: 0,
            number: 1,
            marker: OrderedListMarker::Period,
        })
    }
}
