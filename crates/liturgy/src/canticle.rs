use serde::{Deserialize, Serialize};

use crate::{CanticleTables, PreferenceKey, Reference};
use canticle_table::{CanticleId, CanticleNumber};

/// An entry that can be looked up from a [CanticleTable](canticle_table::CanticleTable).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CanticleTableEntry {
    pub nth: CanticleNumber,
    pub table: CanticleTableChoice,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CanticleTableChoice {
    /// Dynamically loads the canticle table selected in the specified preference
    Preference(PreferenceKey),
    /// Statically uses the chosen canticle table
    Selected(CanticleTables),
}

/// A Canticle (i.e., a psalm-like text not found in the Book of Psalms, and used liturgically)
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Canticle {
    /// Unique identifier for the canticle; may be shared between different versions or translations
    pub number: CanticleId,
    /// Present when only a subset of verses should be displayed
    pub citation: Option<String>,
    /// Reference to e.g., a BCP page
    pub reference: Reference,
    /// Name for the section in the psalm's own language (e.g., "Part I" or "Aleph")
    pub local_name: String,
    /// Latin name for the section (e.g., "Beatus vir qui non abiit")
    pub latin_name: String,
    /// The content of the psalm, by section
    pub sections: Vec<CanticleSection>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CanticleSection {
    /// Title of section, if any
    pub title: Option<String>,
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
