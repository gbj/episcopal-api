mod book_abbrevs;
mod books;
mod query;
mod range;
mod utils;

pub use books::{book_name_to_book, Book};
pub use query::BibleReferenceQuery;
pub use range::BibleReferenceRange;
pub use utils::parse_reference;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BibleReference {
    pub ranges: Vec<BibleReferenceRange>,
}

impl From<&str> for BibleReference {
    fn from(val: &str) -> Self {
        BibleReference {
            ranges: parse_reference(val),
        }
    }
}

impl From<String> for BibleReference {
    fn from(val: String) -> Self {
        BibleReference {
            ranges: parse_reference(&val),
        }
    }
}

impl From<&String> for BibleReference {
    fn from(val: &String) -> Self {
        BibleReference {
            ranges: parse_reference(val),
        }
    }
}
