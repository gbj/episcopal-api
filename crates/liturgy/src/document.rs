use std::fmt::Display;

use calendar::{Calendar, LiturgicalDay};
use serde::{Deserialize, Serialize};

use crate::{ClientPreferences, Condition, Heading, SubLiturgy, Words};

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Document {
    content: Content,
    label: Option<String>,
    condition: Option<Condition>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            label: None,
            content: Content::Empty,
            condition: None,
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
                    content: Content::Series(
                        sub.iter()
                            .filter(|doc| doc.include(calendar, day, prefs))
                            .cloned()
                            .collect::<Vec<_>>(),
                    ),
                    ..self
                }),
                Content::Parallel(sub) => Some(Self {
                    content: Content::Parallel(
                        sub.iter()
                            .filter(|doc| doc.include(calendar, day, prefs))
                            .cloned()
                            .collect::<Vec<_>>(),
                    ),
                    ..self
                }),
                Content::Option(sub) => Some(Self {
                    content: Content::Option(
                        sub.iter()
                            .filter(|doc| doc.include(calendar, day, prefs))
                            .cloned()
                            .collect::<Vec<_>>(),
                    ),
                    ..self
                }),
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
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Content {
    /// A document with no contents
    Empty,
    /// A title, subtitle, label, or other heading
    Heading {
        level: Heading,
        text: Option<String>,
    },
    // Preces: responsive prayer in which each line has a label and its text: V: ___ / R: ___
    Preces(Vec<(Words, Words)>),
    /// A set of multiple [Document]s, organized one after the other
    Series(Vec<Document>),
    /// A set of multiple [Document]s, displayed as parallel options (e.g., in multiple languages or versions)
    Parallel(Vec<Document>),
    /// A set of multiple [Document]s, which are mutually-exclusive choices
    Option(Vec<Document>),
    /// Inserts another liturgy, by its identifier
    SubLiturgy(SubLiturgy),
}
