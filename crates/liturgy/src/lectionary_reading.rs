use crate::Lectionaries;
use lectionary::ReadingType;
use serde::{Deserialize, Serialize};

use crate::PreferenceKey;

/// A generic reference to a lectionary reading (i.e., “First Reading” from the Daily Office Lectionary).
/// The [Library](library::Library) will compile this into a [BiblicalReading](crate::BiblicalReading).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct LectionaryReading {
    pub reading_type: ReadingType,
    pub lectionary: LectionaryTable,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum LectionaryTable {
    /// Dynamically the lectionary selected in the preference [GlobalPref::Lectionary](crate::GlobalPref)
    Preference(PreferenceKey),
    /// Statically uses the chosen lectionary
    Selected(Lectionaries),
}
