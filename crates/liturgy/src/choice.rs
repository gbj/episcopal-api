use serde::{Deserialize, Serialize};

use crate::Document;

/// An explanatory sentence or direction for the liturgy
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    pub options: Vec<Document>,
    pub selected: usize,
}

impl<T> From<T> for Choice
where
    T: IntoIterator<Item = Document>,
{
    fn from(options: T) -> Self {
        Self {
            selected: 0,
            options: options.into_iter().collect(),
        }
    }
}
