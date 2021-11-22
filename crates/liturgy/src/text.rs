use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::DisplayFormat;

/// Text, without any additional styling or semantics
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Text {
    text: String,
    display_format: DisplayFormat,
    response: Option<String>,
}

impl Text {
    pub fn display_format(mut self, display_format: DisplayFormat) -> Self {
        self.display_format = display_format;
        self
    }

    pub fn response<T>(mut self, response: T) -> Self
    where
        T: Display,
    {
        self.response = Some(response.to_string());
        self
    }
}

impl<T> From<T> for Text
where
    T: Display,
{
    fn from(text: T) -> Self {
        Self {
            text: text.to_string(),
            display_format: DisplayFormat::Default,
            response: None,
        }
    }
}
