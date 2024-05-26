use std::{borrow::Cow, str::FromStr};

use crate::list::{ListMarker, OrderedListMarker, ParseListMarkerError, UnorderedListMarker};

#[derive(Clone, Debug, Default)]
pub struct Config {
    pub max_width: Option<usize>,
    pub fixed_zero_padding: Option<usize>,
    pub fixed_number: Option<usize>,
    pub fixed_ordered_list_marker: Option<OrderedListMarker>,
    pub fixed_unordered_list_marker: Option<UnorderedListMarker>,
    pub fixed_indentation: Option<Cow<'static, str>>,
}

impl Config {
    /// Steven Hé (Sīchàng)'s opinion on the style.
    pub fn sichanghe_opinion() -> Self {
        Self {
            max_width: Some(80),
            fixed_zero_padding: Some(0),
            fixed_number: Some(1),
            fixed_ordered_list_marker: Some(OrderedListMarker::Period),
            fixed_unordered_list_marker: Some(UnorderedListMarker::Hyphen),
            fixed_indentation: Some("    ".into()),
        }
    }

    pub fn max_width(&self) -> Option<usize> {
        self.max_width
    }

    pub fn set_max_width(&mut self, value: Option<usize>) {
        self.max_width = value;
    }

    pub fn list_marker(&self, source: &str) -> Result<ListMarker, ParseListMarkerError> {
        Ok(match ListMarker::from_str(source)? {
            ListMarker::Ordered {
                zero_padding,
                number,
                marker,
            } => {
                let zero_padding = match self.fixed_zero_padding {
                    Some(fixed_zero_padding) => fixed_zero_padding,
                    None => zero_padding,
                };
                let number = match self.fixed_number {
                    Some(fixed_number) => fixed_number,
                    None => number,
                };
                let marker = match &self.fixed_ordered_list_marker {
                    Some(fixed_marker) => fixed_marker.clone(),
                    None => marker,
                };
                ListMarker::Ordered {
                    zero_padding,
                    number,
                    marker,
                }
            }
            marker @ ListMarker::Unordered(_) => match &self.fixed_unordered_list_marker {
                Some(fixed_marker) => ListMarker::Unordered(fixed_marker.clone()),
                None => marker,
            },
        })
    }

    /// Internal setter for config options. Used for testing
    #[cfg(test)]
    pub(crate) fn set(&mut self, field: &str, value: &str) {
        match field {
            "max_width" => {
                let value = value.parse::<usize>().unwrap();
                self.max_width = Some(value)
            }
            _ => panic!("unknown configuration {field}"),
        }
    }
}
