use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// An explanatory sentence or direction for the liturgy
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PsalmCitation(String);

impl<T> From<T> for PsalmCitation
where
    T: Display,
{
    fn from(text: T) -> Self {
        Self(text.to_string())
    }
}
