use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Different versions that a liturgical [Document](crate::Document) could be (e.g., Rite I, Rite II, EOW)
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Version {
    BCP1979,
    RiteI,
    RiteII,
    EOW,
    Expansive,
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Version::BCP1979 => "1979",
            Version::RiteI => "Rite I",
            Version::RiteII => "Rite II",
            Version::EOW => "EOW",
            Version::Expansive => "Expansive",
        };
        write!(f, "{}", label)
    }
}
