use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{Content, Document};

/// Multiple [Document](crate::Document)s that are displayed one at a time, with a menu to choose between them.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    pub options: Vec<Document>,
    pub selected: usize,
}

impl<T> From<T> for Choice
where
    T: IntoIterator<Item = Document>,
{
    fn from(options: T) -> Self {
        Self {
            selected: 0,
            options: options.into_iter().collect(),
        }
    }
}

impl Choice {
    /// Generates an appropriate label to differentiate this option from all the others
    /// ```
    /// # use crate::liturgy::{Document, Choice};
    /// # use psalter::bcp1979::{PSALM_121, PSALM_126};
    /// let choice = Choice::from([
    ///     Document::from(PSALM_121.clone()),
    ///     Document::from(PSALM_126.clone())
    /// ]);
    /// assert_eq!(choice.option_label(&choice.options[0], 0), "Psalm 121");
    /// ```
    pub fn option_label(&self, doc: &Document, index: usize) -> String {
        let unique_labels = self.unique_labels();
        let unique_versions = self.unique_versions();
        let unique_citations = self.unique_citations();

        let label = if matches!(doc.content, Content::Series(_)) {
            doc.label.clone().or_else(|| doc.version_label.clone())
        } else {
            None
        }
        .unwrap_or_else(|| format!("Option {}", index + 1));

        let label = if doc.version_label.is_some() {
            doc.version_label.clone().unwrap()
        } else if let Content::Psalm(psalm) = &doc.content {
            if psalm.number == 119 && unique_versions == 1 {
                psalm.sections[0].local_name.clone()
            } else {
                let citation = psalm
                    .citation
                    .clone()
                    .unwrap_or_else(|| format!("Psalm {}", psalm.number));
                if unique_versions == 1 {
                    citation
                } else {
                    format!("{} ({})", citation, doc.version)
                }
            }
        } else if let Content::BiblicalReading(reading) = &doc.content {
            if unique_citations > 1 && unique_versions > 1 {
                format!("{} ({})", reading.citation, doc.version)
            } else if unique_versions > 1 {
                doc.version.to_string()
            } else if let Some(text) = reading.text.get(0).map(|(_, text)| text) {
                format!("{} (“{}”)", reading.citation, text)
            } else {
                reading.citation.clone()
            }
        } else if let Content::Sentence(reading) = &doc.content {
            let citation_and_text = if let Some(citation) = &reading.citation {
                format!("{} (“{}”)", citation, reading.text)
            } else {
                reading.text.clone()
            };
            if unique_citations > 1 && unique_versions > 1 {
                format!("{} ({})", citation_and_text, doc.version)
            } else if unique_versions > 1 {
                doc.version.to_string()
            } else {
                citation_and_text
            }
        } else if let Content::Canticle(canticle) = &doc.content {
            let unique_canticle_numbers = self.unique_canticle_numbers();
            let unique_local_names = self.unique_canticle_local_names();

            if unique_canticle_numbers > 1 && unique_local_names > 1 && unique_versions > 1 {
                format!(
                    "{}. {} ({})",
                    canticle.number, canticle.local_name, doc.version
                )
            } else if unique_versions > 1 {
                doc.version.to_string()
            } else {
                format!("{}. {}", canticle.number, canticle.local_name)
            }
        } else if unique_labels > 1 && doc.label.is_some() {
            doc.label.clone().unwrap()
        } else if unique_versions > 1 {
            doc.version.to_string()
        } else {
            label
        };

        if label.len() > 50 {
            format!("{}...", label.chars().take(50).collect::<String>())
        } else {
            label
        }
    }

    fn unique_versions(&self) -> usize {
        self.options.iter().map(|doc| doc.version).unique().count()
    }

    fn unique_labels(&self) -> usize {
        self.options
            .iter()
            .map(|doc| doc.label.as_ref())
            .unique()
            .count()
    }

    fn unique_citations(&self) -> usize {
        self.options
            .iter()
            .filter_map(|doc| match &doc.content {
                Content::PsalmCitation(citation) => Some(citation.as_str().to_string()),
                Content::Psalm(psalm) => psalm.citation.clone(),
                Content::Sentence(sentence) => sentence.citation.clone(),
                Content::BiblicalCitation(citation) => Some(citation.as_str().to_string()),
                Content::BiblicalReading(reading) => Some(reading.citation.clone()),
                _ => None,
            })
            .unique()
            .count()
    }

    fn unique_canticle_numbers(&self) -> usize {
        self.options
            .iter()
            .filter_map(|doc| match &doc.content {
                Content::Canticle(canticle) => Some(canticle.number),
                _ => None,
            })
            .unique()
            .count()
    }

    fn unique_canticle_local_names(&self) -> usize {
        self.options
            .iter()
            .filter_map(|doc| match &doc.content {
                Content::Canticle(canticle) => Some(&canticle.local_name),
                _ => None,
            })
            .unique()
            .count()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use canticle_table::CanticleId;
    use reference_parser::{BibleVerse, BibleVersePart, Book};

    // Canticle label tests
    #[test]
    fn local_names_of_canticles() {
        let choice = Choice::from([
            Document::from(Canticle {
                number: CanticleId::Canticle1,
                citation: None,
                sections: vec![],
                local_name: String::from("A Song of Creation"),
                latin_name: Some(String::from("Benedicite, omnia opera Domini")),
            })
            .version(Version::RiteI),
            Document::from(Canticle {
                number: CanticleId::Canticle2,
                citation: None,
                sections: vec![],
                local_name: String::from("A Song of Praise"),
                latin_name: Some(String::from("Benedictus es, Domine")),
            })
            .version(Version::RiteI),
        ]);
        assert_eq!(
            choice.option_label(&choice.options[0], 0),
            "1. A Song of Creation"
        );
        assert_eq!(
            choice.option_label(&choice.options[1], 1),
            "2. A Song of Praise"
        );
    }

    #[test]
    fn versions_if_same_canticle_by_local_name() {
        let choice = Choice::from([
            Document::from(Canticle {
                number: CanticleId::Canticle1,
                citation: None,
                sections: vec![],
                local_name: String::from("A Song of Creation"),
                latin_name: Some(String::from("Benedicite, omnia opera Domini")),
            })
            .version(Version::RiteI),
            Document::from(Canticle {
                number: CanticleId::Canticle12,
                citation: None,
                sections: vec![],
                local_name: String::from("A Song of Creation"),
                latin_name: Some(String::from("Benedicite, omnia opera Domini")),
            })
            .version(Version::RiteII),
        ]);
        assert_eq!(choice.option_label(&choice.options[0], 0), "Rite I");
        assert_eq!(choice.option_label(&choice.options[1], 1), "Rite II");
    }

    #[test]
    fn versions_if_same_canticle_by_id() {
        let choice = Choice::from([
            Document::from(Canticle {
                number: CanticleId::Canticle12,
                citation: None,
                sections: vec![],
                local_name: String::from("A Song of Creation"),
                latin_name: Some(String::from("Benedicite, omnia opera Domini")),
            })
            .version(Version::RiteII),
            Document::from(Canticle {
                number: CanticleId::Canticle12,
                citation: None,
                sections: vec![],
                local_name: String::from("A Song of Creation"),
                latin_name: Some(String::from("Benedicite, omnia opera Domini")),
            })
            .version(Version::EOW),
        ]);
        assert_eq!(choice.option_label(&choice.options[0], 0), "Rite II");
        assert_eq!(choice.option_label(&choice.options[1], 1), "EOW");
    }

    #[test]
    fn local_names_and_versions_if_necessary() {
        let choice = Choice::from([
            Document::from(Canticle {
                number: CanticleId::Canticle1,
                citation: None,
                sections: vec![],
                local_name: String::from("A Song of Creation"),
                latin_name: Some(String::from("Benedicite, omnia opera Domini")),
            })
            .version(Version::RiteI),
            Document::from(Canticle {
                number: CanticleId::Canticle12,
                citation: None,
                sections: vec![],
                local_name: String::from("A Song of Creation"),
                latin_name: Some(String::from("Benedicite, omnia opera Domini")),
            })
            .version(Version::RiteII),
            Document::from(Canticle {
                number: CanticleId::Canticle2,
                citation: None,
                sections: vec![],
                local_name: String::from("A Song of Praise"),
                latin_name: Some(String::from("Benedictus es, Domine")),
            })
            .version(Version::RiteI),
        ]);
        assert_eq!(
            choice.option_label(&choice.options[0], 0),
            "1. A Song of Creation (Rite I)"
        );
        assert_eq!(
            choice.option_label(&choice.options[1], 1),
            "12. A Song of Creation (Rite II)"
        );
        assert_eq!(
            choice.option_label(&choice.options[2], 2),
            "2. A Song of Praise (Rite I)"
        );
    }

    // Generic
    #[test]
    fn label_if_different_labels() {
        let choice = Choice::from([
            Document::from(Text::from("Kyrie...")).label("Kyrie"),
            Document::from(Text::from("Gloria...")).label("Gloria"),
            Document::from(Text::from("Trisagion...")).label("Trisagion"),
        ]);
        assert_eq!(choice.option_label(&choice.options[0], 0), "Kyrie");
        assert_eq!(choice.option_label(&choice.options[1], 1), "Gloria");
        assert_eq!(choice.option_label(&choice.options[2], 2), "Trisagion");
    }

    // Bible versions
    #[test]
    fn bible_versions_if_different() {
        let choice = Choice::from([
            Document::from(BiblicalReading {
                citation: String::from("John 1:1"),
                text: vec![],
                intro: None,
            })
            .version(Version::NRSV),
            Document::from(BiblicalReading {
                citation: String::from("John 1:1"),
                text: vec![],
                intro: None,
            })
            .version(Version::ESV),
        ]);
        assert_eq!(choice.option_label(&choice.options[0], 0), "NRSV");
        assert_eq!(choice.option_label(&choice.options[1], 1), "ESV");
    }

    #[test]
    fn bible_versions_if_different_sentence() {
        let choice = Choice::from([
            Document::from(Sentence {
                citation: Some(String::from("John 1:1")),
                text: String::from(""),
                response: None,
            })
            .version(Version::NRSV),
            Document::from(Sentence {
                citation: Some(String::from("John 1:1")),
                text: String::from(""),
                response: None,
            })
            .version(Version::ESV),
        ]);
        assert_eq!(choice.option_label(&choice.options[0], 0), "NRSV");
        assert_eq!(choice.option_label(&choice.options[1], 1), "ESV");
    }

    #[test]
    fn citations_and_versions_of_reading() {
        let choice = Choice::from([
            Document::from(BiblicalReading {
                citation: String::from("John 1:1"),
                text: vec![
                    (BibleVerse {book:Book::John,chapter:1,verse:1, verse_part: BibleVersePart::All }, String::from("'In the beginning was the Word, and the Word was with God, and the Word was God."))
                ],
                intro: None
            })
            .version(Version::NRSV),
            Document::from(BiblicalReading {
                citation: String::from("John 1:2"),
                text: vec![
                    (BibleVerse {book:Book::John,chapter:1,verse:2, verse_part: BibleVersePart::All }, String::from("He was in the beginning with God."))
                ],
                intro: None
            })
            .version(Version::ESV),
        ]);
        assert_eq!(
            choice.option_label(&choice.options[0], 0),
            "John 1:1 (NRSV)"
        );
        assert_eq!(choice.option_label(&choice.options[1], 1), "John 1:2 (ESV)");
    }
    #[test]
    fn citations_and_versions_of_sentence() {
        let choice = Choice::from([
            Document::from(Sentence {
                citation: Some(String::from("John 1:1")),
                text:  String::from("In the beginning was the Word, and the Word was with God, and the Word was God."),
                response: None
            })
            .version(Version::NRSV),
            Document::from(Sentence {
                citation: Some(String::from("John 1:2")),
                text: String::from("He was in the beginning with God."),
                response: None,
            })
            .version(Version::NRSV),
        ]);
        assert_eq!(
            choice.option_label(&choice.options[0], 0),
            "John 1:1 (“In the beginning was the Word, and the ..."
        );
        assert_eq!(
            choice.option_label(&choice.options[1], 1),
            "John 1:2 (“He was in the beginning with God.”)"
        );
    }

    #[test]
    fn citations_and_text_if_same_version_of_reading() {
        let choice = Choice::from([
            Document::from(BiblicalReading {
                citation: String::from("John 1:1"),
                text: vec![
                    (BibleVerse {
                        book: Book::John,
                        chapter: 1,
                        verse: 1,
                        verse_part: BibleVersePart::All
                    }, String::from("In the beginning was the Word, and the Word was with God, and the Word was God."))
                ],
                intro: None,
            })
            .version(Version::NRSV),
            Document::from(BiblicalReading {
                citation: String::from("John 1:2"),
                text: vec![
                    (BibleVerse {
                        book: Book::John,
                        chapter: 1,
                        verse: 2,
                        verse_part: BibleVersePart::All
                    }, String::from("He was in the beginning with God."))
                ],
                intro: None,
            })
            .version(Version::NRSV),
        ]);
        assert_eq!(
            choice.option_label(&choice.options[0], 0),
            "John 1:1 (“In the beginning was the Word, and the ..."
        );
        assert_eq!(
            choice.option_label(&choice.options[1], 1),
            "John 1:2 (“He was in the beginning with God.”)"
        );
    }
}
