use reference_parser::BibleVerse;
use serde::{Deserialize, Serialize};

use crate::Document;

/// A reading that contains the text of a portion of the Bible.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct BiblicalReading {
    /// A citation for the book/chapters/verses included.
    pub citation: String,
    /// The text
    pub text: Vec<(BibleVerse, String)>,
    /// Introduction to the reading. The introduction begins as a template, and is filled in by replacing
    /// ${long_name} or ${short_name} with the name of the appropriate Biblical book.
    pub intro: BiblicalReadingIntro,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum BiblicalReadingIntro {
    None,
    Template(Box<Document>),
    Compiled(Box<Document>),
}
