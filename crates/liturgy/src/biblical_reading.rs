use reference_parser::BibleVerse;
use serde::{Deserialize, Serialize};

/// A reading that contains the text of a portion of the Bible.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct BiblicalReading {
    /// A citation for the book/chapters/verses included.
    pub citation: String,
    /// The text
    pub text: Vec<(BibleVerse, String)>,
}
