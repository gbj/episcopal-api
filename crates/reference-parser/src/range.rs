use crate::BibleReferenceQuery;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct BibleReferenceRange {
    pub start: BibleReferenceQuery,
    pub end: Option<BibleReferenceQuery>,
    pub bracketed: bool,
}
