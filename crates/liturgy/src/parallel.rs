use serde::{Deserialize, Serialize};

use crate::Document;

/// Multiple [Document](crate::Document)s that are displayed side by side.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Parallel(Vec<Document>);

impl Parallel {
    pub fn iter(&self) -> impl Iterator<Item = &Document> {
        self.0.iter()
    }

    pub fn as_slice(&self) -> &[Document] {
        &self.0
    }
}

impl<T, U> From<T> for Parallel
where
    T: IntoIterator<Item = U>,
    U: Into<Document>,
{
    fn from(items: T) -> Self {
        Self(items.into_iter().map(|item| item.into()).collect())
    }
}
