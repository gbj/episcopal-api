use std::iter;

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

impl PsalmVerse {
    /// Tests whether this verse is included in the given [PsalmCitation].
    ///     /// ```
    /// # use crate::liturgy::{PsalmCitation, PsalmVerseRange, PsalmVerseId, PsalmVersePart, PsalmVerse};
    /// let citation = PsalmCitation {
    ///     ranges: vec![
    ///         PsalmVerseRange {
    ///             start: PsalmVerseId {
    ///                 verse_number: 1,
    ///                 part: PsalmVersePart::B
    ///             },
    ///             end: PsalmVerseId {
    ///                 verse_number: 3,
    ///                 part: PsalmVersePart::A
    ///             }
    ///         },
    ///         PsalmVerseRange {
    ///             start: PsalmVerseId {
    ///                 verse_number: 5,
    ///                 part: PsalmVersePart::All
    ///             },
    ///             end: PsalmVerseId {
    ///                 verse_number: 9,
    ///                 part: PsalmVersePart::All
    ///             }
    ///         }
    ///     ]
    /// };
    /// let verse_7 = PsalmVerse { number: 7, a: String::from(""), b: String::from("") };
    /// let verse_10 = PsalmVerse { number: 10, a: String::from(""), b: String::from("") };
    /// assert_eq!(verse_7.included_in_citation(&citation), true);
    /// assert_eq!(verse_10.included_in_citation(&citation), false);
    /// ```
    pub fn included_in_citation(&self, citation: &PsalmCitation) -> bool {
        citation
            .to_verse_ids()
            .any(|id| id.verse_number == self.number)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PsalmCitation {
    pub ranges: Vec<PsalmVerseRange>,
}

impl PsalmCitation {
    /// Iterator over the verses included in this range.
    /// ```
    /// # use crate::liturgy::{PsalmCitation, PsalmVerseRange, PsalmVerseId, PsalmVersePart};
    /// let citation = PsalmCitation {
    ///     ranges: vec![
    ///         PsalmVerseRange {
    ///             start: PsalmVerseId {
    ///                 verse_number: 1,
    ///                 part: PsalmVersePart::B
    ///             },
    ///             end: PsalmVerseId {
    ///                 verse_number: 3,
    ///                 part: PsalmVersePart::A
    ///             }
    ///         },
    ///         PsalmVerseRange {
    ///             start: PsalmVerseId {
    ///                 verse_number: 5,
    ///                 part: PsalmVersePart::All
    ///             },
    ///             end: PsalmVerseId {
    ///                 verse_number: 9,
    ///                 part: PsalmVersePart::All
    ///             }
    ///         }
    ///     ]
    /// };
    /// assert_eq!(
    ///     citation.to_verse_ids().collect::<Vec<_>>(),
    ///     vec![
    ///         PsalmVerseId {
    ///             verse_number: 1,
    ///             part: PsalmVersePart::B
    ///         },
    ///         PsalmVerseId {
    ///             verse_number: 2,
    ///             part: PsalmVersePart::All
    ///         },
    ///         PsalmVerseId {
    ///             verse_number: 3,
    ///             part: PsalmVersePart::A
    ///         },
    ///         PsalmVerseId {
    ///             verse_number: 5,
    ///             part: PsalmVersePart::All
    ///         },
    ///         PsalmVerseId {
    ///             verse_number: 6,
    ///             part: PsalmVersePart::All
    ///         },
    ///         PsalmVerseId {
    ///             verse_number: 7,
    ///             part: PsalmVersePart::All
    ///         },
    ///         PsalmVerseId {
    ///             verse_number: 8,
    ///             part: PsalmVersePart::All
    ///         },
    ///         PsalmVerseId {
    ///             verse_number: 9,
    ///             part: PsalmVersePart::All
    ///         }
    ///     ]
    /// )
    /// ```
    pub fn to_verse_ids(&self) -> impl Iterator<Item = PsalmVerseId> + '_ {
        self.ranges.iter().flat_map(|range| {
            let first = range.start;
            let last = range.end;
            let between =
                ((first.verse_number + 1)..(last.verse_number)).map(|verse_number| PsalmVerseId {
                    verse_number,
                    part: PsalmVersePart::All,
                });
            iter::once(first).chain(between).chain(iter::once(last))
        })
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PsalmVerseRange {
    pub start: PsalmVerseId,
    pub end: PsalmVerseId,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PsalmVerseId {
    pub verse_number: u8,
    pub part: PsalmVersePart,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum PsalmVersePart {
    A,
    B,
    All,
}
