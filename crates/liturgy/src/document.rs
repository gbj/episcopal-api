use std::fmt::Display;

use calendar::{Calendar, LiturgicalDay};
use language::Language;
use serde::{Deserialize, Serialize};

use crate::{
    Antiphon, BiblicalCitation, BiblicalReading, Canticle, Choice, ClientPreferences, Condition,
    DocumentError, GloriaPatri, Heading, LectionaryReading, Litany, Liturgy, LiturgyPreferences,
    Parallel, Preces, Psalm, PsalmCitation, Reference, ResponsivePrayer, Rubric, Sentence, Series,
    Show, Status, SubLiturgy, Text, Version,
};

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Document {
    pub condition: Option<Condition>,
    pub label: Option<String>,
    pub language: Language,
    pub source: Option<Reference>,
    pub status: Status,
    pub display: Show,
    pub version: Version,
    pub version_label: Option<String>,
    pub content: Content,
}

impl Document {
    pub fn new() -> Self {
        Self {
            condition: None,
            label: None,
            language: Language::default(),
            source: None,
            status: Status::Authorized,
            display: Show::Always,
            version: Version::RiteII,
            version_label: None,
            content: Content::Empty,
        }
    }

    /// Whether a `Document` should be included in the liturgy or omitted, based on its included [Condition]s.
    pub fn include(
        &self,
        calendar: &Calendar,
        day: &LiturgicalDay,
        prefs: &impl ClientPreferences,
        original_prefs: &LiturgyPreferences,
    ) -> bool {
        match &self.condition {
            None => true,
            Some(condition) => condition.include(calendar, day, prefs, original_prefs),
        }
    }

    /// Builds a new Document from an iterator of Documents; either a [Choice] (if multiple Documents) or a single Document.
    pub fn choice_or_document<I>(docs: &mut I) -> Option<Document>
    where
        I: Iterator<Item = Document>,
    {
        match (docs.next(), docs.next()) {
            (None, None) => None,
            (None, Some(doc)) => Some(doc),
            (Some(doc), None) => Some(doc),
            (Some(a), Some(b)) => Some(Document::from(Choice::from(
                std::iter::once(a).chain(std::iter::once(b)).chain(docs),
            ))),
        }
    }

    /// Builds a new Document from an iterator of Documents; either a [Series] (if multiple Documents) or a single Document.
    pub fn series_or_document<I>(docs: &mut I) -> Option<Document>
    where
        I: Iterator<Item = Document>,
    {
        match (docs.next(), docs.next()) {
            (None, None) => None,
            (None, Some(doc)) => Some(doc),
            (Some(doc), None) => Some(doc),
            (Some(a), Some(b)) => Some(Document::from(Series::from(
                std::iter::once(a).chain(std::iter::once(b)).chain(docs),
            ))),
        }
    }

    /// Builds a new Document from an iterator of Documents; either a [Parallel] (if multiple Documents) or a single Document.
    pub fn parallel_or_document<I>(docs: &mut I) -> Option<Document>
    where
        I: Iterator<Item = Document>,
    {
        match (docs.next(), docs.next()) {
            (None, None) => None,
            (None, Some(doc)) => Some(doc),
            (Some(doc), None) => Some(doc),
            (Some(a), Some(b)) => Some(Document::from(Parallel::from(
                std::iter::once(a).chain(std::iter::once(b)).chain(docs),
            ))),
        }
    }

    pub fn content(mut self, content: Content) -> Self {
        self.content = content;
        self
    }

    pub fn label(mut self, label: impl Display) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn condition(mut self, condition: Condition) -> Self {
        self.condition = Some(condition);
        self
    }

    pub fn display(mut self, show: Show) -> Self {
        self.display = show;
        self
    }

    pub fn source(mut self, source: Reference) -> Self {
        self.source = Some(source);
        self
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    pub fn version(mut self, version: Version) -> Self {
        self.version = version;
        self
    }

    pub fn version_label(mut self, version_label: impl Display) -> Self {
        self.version_label = Some(version_label.to_string());
        self
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Content {
    /// # Content Variants
    /// A document with no contents
    Empty,
    /// An error that comes up while compiling a liturgy.
    Error(DocumentError),
    /// A brief passage or verse, usually extracted from a psalm.
    Antiphon(Antiphon),
    /// A reference to a passage of the Bible, which will be inserted as a
    /// [BibleReading](crate::BibleReading) by the compilation process.
    BiblicalCitation(BiblicalCitation),
    /// A reading that contains the text of a portion of the Bible.
    BiblicalReading(BiblicalReading),
    /// A Canticle (i.e., a psalm-like text not found in the Book of Psalms, and used liturgically)
    Canticle(Canticle),
    /// The Gloria Patri is formatted such that it is broken into four lines rather than two if necessary
    GloriaPatri(GloriaPatri),
    /// A title, subtitle, label, or other heading
    Heading(Heading),
    /// A generic reference to a lectionary reading (i.e., “First Reading” from the Daily Office Lectionary).
    LectionaryReading(LectionaryReading),
    /// A responsive prayer in which the same response is given to every petition
    Litany(Litany),
    /// A liturgical template that can carry a set of possible preferences and
    /// other metadata, as well as sub-documents.
    Liturgy(Liturgy),
    /// A responsive prayer in which each line has a label and its text: V: ___ / R: ___
    Preces(Preces),
    /// A psalm.
    Psalm(Psalm),
    /// A simple responsive prayer in which the leader and participants alternate.
    ResponsivePrayer(ResponsivePrayer),
    /// An explanatory sentence or direction for the liturgy
    Rubric(Rubric),
    /// A short Biblical reading, with an optional response.
    Sentence(Sentence),
    /// Text, without any additional styling or semantics
    Text(Text),
    /// # Structural Variants
    /// A set of multiple [Document]s, organized one after the other
    Series(Series),
    /// A set of multiple [Document]s, displayed as parallel options (e.g., in multiple languages or versions)
    Parallel(Parallel),
    /// A set of multiple [Document]s, which are mutually-exclusive choices
    Choice(Choice),
    /// # Lookup Fields
    /// Inserts the Collect of the Day
    CollectOfTheDay,
    /// A reference to a [Psalm](crate::Psalm), which will be inserted by the compilation process.
    PsalmCitation(PsalmCitation),
    /// Inserts another liturgy, by its identifier
    SubLiturgy(SubLiturgy),
}

// Create Document from a Content enum
impl From<Content> for Document {
    fn from(content: Content) -> Self {
        Self::new().content(content)
    }
}

// Plain strings are converted into Text
impl From<&str> for Document {
    fn from(text: &str) -> Self {
        Document::from(Text::from(text))
    }
}

impl From<String> for Document {
    fn from(text: String) -> Self {
        Document::from(Text::from(text))
    }
}

// Create Documents from various content types
impl From<Antiphon> for Document {
    fn from(content: Antiphon) -> Self {
        Self::from(Content::Antiphon(content))
    }
}

impl From<BiblicalCitation> for Document {
    fn from(content: BiblicalCitation) -> Self {
        Self::from(Content::BiblicalCitation(content))
    }
}

impl From<BiblicalReading> for Document {
    fn from(content: BiblicalReading) -> Self {
        Self::from(Content::BiblicalReading(content))
    }
}

impl From<Canticle> for Document {
    fn from(content: Canticle) -> Self {
        Self::from(Content::Canticle(content))
    }
}

impl From<Choice> for Document {
    fn from(content: Choice) -> Self {
        Self::from(Content::Choice(content))
    }
}

impl From<DocumentError> for Document {
    fn from(content: DocumentError) -> Self {
        Self::from(Content::Error(content))
    }
}

impl From<GloriaPatri> for Document {
    fn from(content: GloriaPatri) -> Self {
        Self::from(Content::GloriaPatri(content))
    }
}

impl From<Heading> for Document {
    fn from(content: Heading) -> Self {
        Self::from(Content::Heading(content))
    }
}

impl From<Litany> for Document {
    fn from(content: Litany) -> Self {
        Self::from(Content::Litany(content))
    }
}

impl From<Preces> for Document {
    fn from(content: Preces) -> Self {
        Self::from(Content::Preces(content))
    }
}

impl From<LectionaryReading> for Document {
    fn from(content: LectionaryReading) -> Self {
        Self::from(Content::LectionaryReading(content))
    }
}

impl From<Liturgy> for Document {
    fn from(content: Liturgy) -> Self {
        Self::from(Content::Liturgy(content))
    }
}

impl From<Parallel> for Document {
    fn from(content: Parallel) -> Self {
        Self::from(Content::Parallel(content))
    }
}

impl From<Psalm> for Document {
    fn from(content: Psalm) -> Self {
        Self::from(Content::Psalm(content))
    }
}

impl From<PsalmCitation> for Document {
    fn from(content: PsalmCitation) -> Self {
        Self::from(Content::PsalmCitation(content))
    }
}

impl From<ResponsivePrayer> for Document {
    fn from(content: ResponsivePrayer) -> Self {
        Self::from(Content::ResponsivePrayer(content))
    }
}

impl From<Rubric> for Document {
    fn from(content: Rubric) -> Self {
        Self::from(Content::Rubric(content))
    }
}

impl From<Sentence> for Document {
    fn from(content: Sentence) -> Self {
        Self::from(Content::Sentence(content))
    }
}

impl From<Series> for Document {
    fn from(content: Series) -> Self {
        Self::from(Content::Series(content))
    }
}

impl From<SubLiturgy> for Document {
    fn from(content: SubLiturgy) -> Self {
        Self::from(Content::SubLiturgy(content))
    }
}

impl From<Text> for Document {
    fn from(content: Text) -> Self {
        Self::from(Content::Text(content))
    }
}
