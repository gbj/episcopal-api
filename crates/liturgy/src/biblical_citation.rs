use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// A reference to a passage of the Bible, which will be inserted as a
/// [BibleReading](crate::BibleReading) by the compilation process.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct BiblicalCitation(String);

impl BiblicalCitation {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for BiblicalCitation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl From<String> for BiblicalCitation {
    fn from(text: String) -> Self {
        Self(text)
    }
}

impl From<&str> for BiblicalCitation {
    fn from(text: &str) -> Self {
        Self(text.to_string())
    }
}
