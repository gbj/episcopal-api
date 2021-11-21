use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// A title, subtitle, label, or other heading; can be used to automatically insert date/liturgical day name, or text with a level.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Heading {
    Date,
    Day,
    Text(HeadingLevel, String),
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum HeadingLevel {
    Heading1 = 5,
    Heading2 = 4,
    Heading3 = 3,
    Heading4 = 2,
    Heading5 = 1,
}

impl<T> From<(HeadingLevel, T)> for Heading
where
    T: Display,
{
    fn from((level, text): (HeadingLevel, T)) -> Self {
        Self::Text(level, text.to_string())
    }
}
