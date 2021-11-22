use serde::{Deserialize, Serialize};

use crate::Document;

/// An explanatory sentence or direction for the liturgy
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Series(Vec<Document>);

impl Series {
    pub fn iter(&self) -> impl Iterator<Item = &Document> {
        self.0.iter()
    }
}

impl<T, U> From<T> for Series
where
    T: IntoIterator<Item = U>,
    U: Into<Document>,
{
    fn from(items: T) -> Self {
        Self(items.into_iter().map(|item| item.into()).collect())
    }
}
