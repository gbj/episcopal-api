use crate::Book;
use serde::{Deserialize, Serialize};

type Chapter = u16;
type Verse = u16;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct BibleReferenceQuery {
    pub book: Option<Book>,
    pub chapter: Option<Chapter>,
    pub verse: Option<Verse>,
}
