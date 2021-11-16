use serde::{Deserialize, Serialize};

use crate::Reference;

/// Represents an entire psalm
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Psalm {
    /// The psalm number (e.g., 8 for Psalm 8)
    pub number: u8,
    /// Present when only a subset of verses should be displayed
    pub range: Option<PsalmVerseRange>,
    /// The content of the psalm, by section
    pub sections: Vec<PsalmSection>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PsalmSection {
    /// Reference to e.g., a BCP page
    pub reference: Reference,
    /// Name for the section in the psalm's own language (e.g., "Part I" or "Aleph")
    pub local_name: String,
    /// Latin name for the section (e.g., "Beatus vir qui non abiit")
    pub latin_name: String,
    /// The set of verses included in this section
    pub verses: Vec<PsalmVerse>,
}

impl PsalmSection {
    /// Verse number of the first verse in this section
    pub fn first_verse(&self) -> u8 {
        self.verses.first().map_or(0, |verse| verse.number)
    }

    /// Verse number of the last verse in this section
    pub fn last_verse(&self) -> u8 {
        self.verses.last().map_or(0, |verse| verse.number)
    }

    /// Tests whether this section includes the given verse number
    pub fn includes_verse(&self, verse_number: u8) -> bool {
        self.verses.iter().any(|verse| verse.number == verse_number)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PsalmVerse {
    /// Verse number
    pub number: u8,
    /// Text of the first half of the verse, up to the asterisk
    pub a: String,
    /// Text of the second half of the verse, after the asterisk
    pub b: String,
}

// TODO
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PsalmVerseRange {}
