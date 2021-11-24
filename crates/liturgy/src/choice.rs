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
        let unique_versions = self.unique_versions();
        let unique_citations = self.unique_citations();
        let unique_canticle_numbers = self.unique_canticle_numbers();

        let label = if matches!(doc.content, Content::Series(_)) {
            doc.label.clone().or_else(|| doc.version_label.clone())
        } else {
            None
        }
        .unwrap_or_else(|| format!("Option {}", index + 1));

        if label.len() > 50 {
            format!("{}...", label.chars().take(50).collect::<String>())
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
        }
        // TODO
        /* else if let Content::BibleReading(reading) = &doc.content {

        } */
        else {
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
                // TODO add BibleReading, BibleCitation
                _ => None,
            })
            .unique()
            .count()
    }

    fn unique_canticle_numbers(&self) -> usize {
        // TODO when canticles are added
        0
    }
}

/*

const uniqueVersions = this.uniqueVersions(),
      uniqueLabels = this.uniqueLabels(),
      uniqueCitations = this.uniqueCitations(),
      uniqueCanticleNumbers = this.uniqueCanticleNumbers();

    let label: string = '';

    if (option.type == 'liturgy') {
      label = option.label || option.version_label || 'Option';
    }
    // Psalm 119 parts => Psalm 119: Aleph
    else if (
      option.type == 'psalm' &&
      uniqueVersions == 1 &&
      option.label &&
      (option.slug?.match(/psalm_119_/) || option.citation?.toString().match(/Ps[^\d]+119/))
    ) {
      label = option.label;
    }
    // Other psalms: Psalm 121
    else if (
      option.type == 'psalm' &&
      option.style == 'psalm' &&
      (option.citation || option.metadata?.number) &&
      uniqueVersions == 1
    ) {
      label = option.citation ? option.citation.toString() : `Psalm ${option.metadata.number}`;
    }
    // Readings of many citations => include truncated text
    else if (
      option.type == 'bible-reading' &&
      uniqueCitations > 1 &&
      option.citation &&
      option.value &&
      option.value.length > 0
    ) {
      const text: string = (option as BibleReading).value
        .map((v) => (v.hasOwnProperty('text') ? (v as BibleReadingVerse).text : v.toString()))
        .join(' ');

      const formattedText = option.style == 'short' ? ` (“${text}”)` : '';

      if (uniqueVersions > 1) {
        label = `${option.citation.toString()} (${option.version})${formattedText}`;
      } else {
        label = `${option.citation.toString()}${formattedText}`;
      }
    }
    // Readings with same citation + different versions => Version
    else if (option.type == 'bible-reading' && uniqueCitations == 1) {
      label = versionToString(option.version);
    }
    // Readings with one version => John 1:1-4
    else if (option.type == 'bible-reading' && option.citation && uniqueVersions == 1) {
      label = option.citation.toString();
    }
    // Readings with multiple versions => John 1:1-4 (Version)
    else if (option.type == 'bible-reading' && option.citation && uniqueVersions > 1) {
      label = `${option.citation.toString()} (${versionToString(option.version)})`;
    }
    // Canticles, if only one version
    else if (
      uniqueVersions == 1 &&
      option.type == 'psalm' &&
      option.style == 'canticle' &&
      option.metadata &&
      option.metadata.localname
    ) {
      label = option.metadata.localname;
    }
    // Canticles, if multiple options for same number
    else if (uniqueVersions > 1 && uniqueCanticleNumbers === 1) {
      label = VERSIONS[versionToString(modifiedVersion(option))];
    }
    // Canticles and invitatories, if multiple options => Venite (EOW)
    else if (uniqueVersions > 1 && option.metadata && option.metadata.hasOwnProperty('localname') && option.version) {
      label = `${option.metadata.localname} (${VERSIONS[versionToString(modifiedVersion(option))]})`;
    }
    // Version label other than BCP 1979 => EOW
    else if (option.version_label && option.version_label !== 'bcp1979') {
      label = option.version_label;
    }
    // If multiple labels, then label => Trisagion, Gloria in Excelsis, Kyrie
    else if (option.label && uniqueLabels > 1 && uniqueVersions == 1) {
      label = option.label;
    }
    // If multiple labels and version, then label (version) => Trisagion (BCP), Gloria in Excelsis (EOW)
    else if (option.label && uniqueLabels > 1 && uniqueVersions > 1) {
      label = `${option.label} (${VERSIONS[versionToString(modifiedVersion(option))]})`;
    }
    // Local name but no version (or version is BCP) => 'The Song of Mary'
    else if (
      option.metadata &&
      option.metadata.hasOwnProperty('localname') &&
      (!option.version_label || option.version_label == 'bcp1979')
    ) {
      label = option.metadata.localname;
    }
    // Fall back to a version label
    else if (uniqueVersions > 1 && option.version) {
      label = VERSIONS[versionToString(modifiedVersion(option))];
    }
    // Fall back to a citation
    else if (option.citation) {
      label = option.citation.toString();
    }
    // Fallback: stripped version of JSON of value
    else if (option.value) {
      label = `“${JSON.stringify(option.value)
        .replace(/[\[\]\{\}\"\'\:]/g, ' ')
        .replace(/\\n/g, ' ')
        .trim()}”`;
    } else {
      throw `Unable to generate a label from ${JSON.stringify(option)}`;
    }

    return label.length > maxLength ? label.substring(0, maxLength) + '...' : label;

*/
