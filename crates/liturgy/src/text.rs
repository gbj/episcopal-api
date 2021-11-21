use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Text, without any additional styling or semantics
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Text(String);

impl<T> From<T> for Text
where
    T: Display,
{
    fn from(text: T) -> Self {
        Self(text.to_string())
    }
}
