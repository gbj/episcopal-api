use calendar::{Date, LiturgicalDay};
use serde::{Deserialize, Serialize};

use crate::Series;

/// A liturgical template that can carry a set of possible preferences and
/// other metadata, as well as sub-documents.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Liturgy {
    pub preferences: (), // TODO
    pub evening: bool,
    pub body: Series,
}

impl Liturgy {
    pub fn evening(mut self, is_evening_liturgy: bool) -> Self {
        self.evening = is_evening_liturgy;
        self
    }
}

impl From<Series> for Liturgy {
    fn from(body: Series) -> Self {
        Self {
            evening: false,
            preferences: (),
            body,
        }
    }
}
