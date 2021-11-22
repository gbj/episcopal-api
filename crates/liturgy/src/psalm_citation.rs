use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// A reference to a [Psalm](crate::Psalm), which will be inserted by the compilation process.
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
