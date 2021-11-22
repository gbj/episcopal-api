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
