use std::fmt::Display;

use calendar::{Calendar, LiturgicalDay};
use serde::{Deserialize, Serialize};

use crate::{
    Choice, ClientPreferences, Condition, GloriaPatri, Heading, Preces, Psalm, PsalmCitation,
    Reference, ResponsivePrayer, Rubric, Sentence, Series, SubLiturgy, Text,
};

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Document {
    condition: Option<Condition>,
    label: Option<String>,
    source: Option<Reference>,
    content: Content,
}

impl Document {
    pub fn new() -> Self {
        Self {
            condition: None,
            label: None,
            source: None,
            content: Content::Empty,
        }
    }

    pub fn compile(
        self,
        calendar: &Calendar,
        day: &LiturgicalDay,
        prefs: &impl ClientPreferences,
    ) -> Option<Self> {
        let include = self.include(calendar, day, prefs);
        if !include {
            None
        } else {
            match self.content {
                Content::Series(sub) => Some(Self {
                    content: Content::Series(Series::from(
                        sub.iter()
                            .filter_map(|doc| doc.clone().compile(calendar, day, prefs))
                            .collect::<Vec<_>>(),
                    )),
                    ..self
                }),
                Content::Parallel(sub) => Some(Self {
                    content: Content::Parallel(
                        sub.into_iter()
                            .filter_map(|doc| doc.compile(calendar, day, prefs))
                            .collect::<Vec<_>>(),
                    ),
                    ..self
                }),
                Content::Choice(sub) => {
                    // try, when filtering selections, to maintain the currently-selected item -- or default to 0
                    let prev_selection = sub.options.get(sub.selected);
                    let index_of_prev_selection = prev_selection
                        .and_then(|prev| sub.options.iter().position(|search| search == prev));

                    Some(Self {
                        content: Content::Choice(Choice {
                            options: sub
                                .options
                                .into_iter()
                                .filter_map(|doc| doc.compile(calendar, day, prefs))
                                .collect(),
                            selected: index_of_prev_selection.unwrap_or(0),
                        }),
                        ..self
                    })
                }
                _ => Some(self),
            }
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
    /// A reference to a [Psalm](crate::Psalm), which will be inserted by the compilation process.
    PsalmCitation(PsalmCitation),
    /// Inserts another liturgy, by its identifier
    SubLiturgy(SubLiturgy),
}

// Create Documents from various content types
impl From<Choice> for Document {
    fn from(content: Choice) -> Self {
        Self::new().content(Content::Choice(content))
    }
}

impl From<GloriaPatri> for Document {
    fn from(content: GloriaPatri) -> Self {
        Self::new().content(Content::GloriaPatri(content))
    }
}

impl From<Heading> for Document {
    fn from(content: Heading) -> Self {
        Self::new().content(Content::Heading(content))
    }
}

impl From<Preces> for Document {
    fn from(content: Preces) -> Self {
        Self::new().content(Content::Preces(content))
    }
}

impl From<Psalm> for Document {
    fn from(content: Psalm) -> Self {
        Self::new().content(Content::Psalm(content))
    }
}

impl From<PsalmCitation> for Document {
    fn from(content: PsalmCitation) -> Self {
        Self::new().content(Content::PsalmCitation(content))
    }
}

impl From<ResponsivePrayer> for Document {
    fn from(content: ResponsivePrayer) -> Self {
        Self::new().content(Content::ResponsivePrayer(content))
    }
}

impl From<Rubric> for Document {
    fn from(content: Rubric) -> Self {
        Self::new().content(Content::Rubric(content))
    }
}

impl From<Sentence> for Document {
    fn from(content: Sentence) -> Self {
        Self::new().content(Content::Sentence(content))
    }
}

impl From<SubLiturgy> for Document {
    fn from(content: SubLiturgy) -> Self {
        Self::new().content(Content::SubLiturgy(content))
    }
}

impl From<Text> for Document {
    fn from(content: Text) -> Self {
        Self::new().content(Content::Text(content))
    }
}
