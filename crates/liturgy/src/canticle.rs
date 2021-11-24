use serde::{Deserialize, Serialize};

use crate::Reference;

/// A Canticle (i.e., a psalm-like text not found in the Book of Psalms, and used liturgically)
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Canticle {
    /// Present when only a subset of verses should be displayed
    pub citation: Option<String>,
    /// The content of the psalm, by section
    pub sections: Vec<CanticleSection>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CanticleSection {
    /// Reference to e.g., a BCP page
    pub reference: Reference,
    /// Name for the section in the psalm's own language (e.g., "Part I" or "Aleph")
    pub local_name: String,
    /// Latin name for the section (e.g., "Beatus vir qui non abiit")
    pub latin_name: String,
    /// The set of verses included in this section
    pub verses: Vec<CanticleVerse>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CanticleVerse {
    /// Text of the first half of the verse, up to the asterisk
    pub a: String,
    /// Text of the second half of the verse, after the asterisk
    pub b: String,
}
