use serde::{Deserialize, Serialize};

/// Language that can be assigned to a [Document](liturgy::Document)
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Language {
    /// English
    En,
    /// Spanish
    Es,
    /// French
    Fr,
    /// Haitian Creole
    Ht,
}

impl Default for Language {
    fn default() -> Self {
        Self::En
    }
}

impl Language {
    // TODO i18n when other languages are added
    pub fn i18n(&self, string: &str) -> String {
        match (self, string) {
            (Language::En, "Mon") => "Monday",
            (Language::En, "Tue") => "Tuesday",
            (Language::En, "Wed") => "Wednesday",
            (Language::En, "Thu") => "Thursday",
            (Language::En, "Fri") => "Friday",
            (Language::En, "Sat") => "Saturday",
            (Language::En, "Sun") => "Sunday",
            _ => string,
        }
        .to_string()
    }
}
