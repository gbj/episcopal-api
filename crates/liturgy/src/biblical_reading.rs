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
    /// Introduction to the reading. The introduction begins as a [BiblicalReadingIntroTemplate](crate::BiblicalReadingIntroTemplate).
    pub intro: Option<BiblicalReadingIntro>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct BiblicalReadingIntro(Box<Document>);

impl From<Document> for BiblicalReadingIntro {
    fn from(document: Document) -> Self {
        Self(Box::new(document))
    }
}
