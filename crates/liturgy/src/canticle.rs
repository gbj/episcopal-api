use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::Reference;

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

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CanticleId {
    Canticle1,
    Canticle2,
    Canticle3,
    Canticle4,
    Canticle5,
    Canticle6,
    Canticle7,
    Canticle8,
    Canticle9,
    Canticle10,
    Canticle11,
    Canticle12,
    Canticle13,
    Canticle14,
    Canticle15,
    Canticle16,
    Canticle17,
    Canticle18,
    Canticle19,
    Canticle20,
    Canticle21,
    CanticleA,
    CanticleB,
    CanticleC,
    CanticleD,
    CanticleE,
    CanticleF,
    CanticleG,
    CanticleH,
    CanticleI,
    CanticleJ,
    CanticleK,
    CanticleL,
    CanticleM,
    CanticleN,
    CanticleO,
    CanticleP,
    CanticleQ,
    CanticleR,
    CanticleS,
}

impl Display for CanticleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CanticleId::Canticle1 => "1",
                CanticleId::Canticle2 => "2",
                CanticleId::Canticle3 => "3",
                CanticleId::Canticle4 => "4",
                CanticleId::Canticle5 => "5",
                CanticleId::Canticle6 => "6",
                CanticleId::Canticle7 => "7",
                CanticleId::Canticle8 => "8",
                CanticleId::Canticle9 => "9",
                CanticleId::Canticle10 => "10",
                CanticleId::Canticle11 => "11",
                CanticleId::Canticle12 => "12",
                CanticleId::Canticle13 => "13",
                CanticleId::Canticle14 => "14",
                CanticleId::Canticle15 => "15",
                CanticleId::Canticle16 => "16",
                CanticleId::Canticle17 => "17",
                CanticleId::Canticle18 => "18",
                CanticleId::Canticle19 => "19",
                CanticleId::Canticle20 => "20",
                CanticleId::Canticle21 => "21",
                CanticleId::CanticleA => "A",
                CanticleId::CanticleB => "B",
                CanticleId::CanticleC => "C",
                CanticleId::CanticleD => "D",
                CanticleId::CanticleE => "E",
                CanticleId::CanticleF => "F",
                CanticleId::CanticleG => "G",
                CanticleId::CanticleH => "H",
                CanticleId::CanticleI => "I",
                CanticleId::CanticleJ => "J",
                CanticleId::CanticleK => "K",
                CanticleId::CanticleL => "L",
                CanticleId::CanticleM => "M",
                CanticleId::CanticleN => "N",
                CanticleId::CanticleO => "O",
                CanticleId::CanticleP => "P",
                CanticleId::CanticleQ => "Q",
                CanticleId::CanticleR => "R",
                CanticleId::CanticleS => "S",
            },
        )
    }
}
