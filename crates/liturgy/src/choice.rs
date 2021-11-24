use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{Content, Document};

/// An explanatory sentence or direction for the liturgy
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
    pub fn option_label(&self, doc: &Document, index: usize) -> String {
        let unique_labels = self.unique_labels();
        let unique_versions = self.unique_versions();
        let unique_citations = self.unique_citations();
        let unique_canticle_numbers = self.unique_canticle_numbers();

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
                    format!("{} ({})", citation, doc.version.to_string())
                }
            }
        } else if let Content::BiblicalReading(reading) = &doc.content {
            if unique_citations > 1 && unique_versions > 1 {
                format!("{} ({})", reading.citation, doc.version)
            } else if unique_versions > 1 {
                doc.version.to_string()
            } else {
                reading.citation.clone()
            }
        } else if let Content::Canticle(canticle) = &doc.content {
            if unique_canticle_numbers > 1 && unique_versions > 1 {
                format!("Canticle {} ({})", canticle.number, doc.version)
            } else if unique_versions > 1 {
                doc.version.to_string()
            } else {
                format!("Canticle {}", canticle.number)
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
}
