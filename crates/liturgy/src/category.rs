use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Inserts all documents filed under this category in the library.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    /// If `true`, the compiler will only show one [Document](crate::Document),
    /// rotating by day in a deterministic way. If `false`, it will show them all
    /// as a [Choice](crate::Choice).
    pub rotate: bool,
}

impl<T> From<T> for Category
where
    T: Display,
{
    fn from(name: T) -> Self {
        Self {
            name: name.to_string(),
            rotate: true,
        }
    }
}
