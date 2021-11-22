use std::fmt::Display;

use calendar::{Calendar, LiturgicalDay};
use language::Language;
use serde::{Deserialize, Serialize};

use crate::{
    Choice, ClientPreferences, Condition, GloriaPatri, Heading, Preces, Psalm, PsalmCitation,
    Reference, ResponsivePrayer, Rubric, Sentence, Series, SubLiturgy, Text,
};

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Document {
    pub condition: Option<Condition>,
    pub label: Option<String>,
    pub language: Language,
    pub source: Option<Reference>,
    pub content: Content,
}

impl Document {
    pub fn new() -> Self {
        Self {
            condition: None,
            label: None,
            language: Language::default(),
            source: None,
            content: Content::Empty,
        }
    }

    pub fn include(
        &self,
        calendar: &Calendar,
        day: &LiturgicalDay,
        prefs: &impl ClientPreferences,
    ) -> bool {
        match &self.condition {
            None => true,
            Some(condition) => condition.include(calendar, day, prefs),
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

    pub fn source(mut self, source: Reference) -> Self {
        self.source = Some(source);
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
    /// The Gloria Patri is formatted such that it is broken into four lines rather than two if necessary
    GloriaPatri(GloriaPatri),
    /// A title, subtitle, label, or other heading
    Heading(Heading),
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
    Parallel(Vec<Document>),
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

// Create Documents from various content types
impl From<Choice> for Document {
    fn from(content: Choice) -> Self {
        Self::from(Content::Choice(content))
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

impl From<Preces> for Document {
    fn from(content: Preces) -> Self {
        Self::from(Content::Preces(content))
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
