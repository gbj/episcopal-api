use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// A simple responsive prayer in which the leader and participants alternate.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResponsivePrayer(Vec<String>);

impl<T, A> From<T> for ResponsivePrayer
where
    T: IntoIterator<Item = A>,
    A: Display,
{
    fn from(content: T) -> Self {
        Self(content.into_iter().map(|line| line.to_string()).collect())
    }
}
