use liturgy::{CanticleTableEntry, LectionaryReading};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum LookupType {
    Category,
    Canticle(CanticleTableEntry),
    Collect,
    Lectionary(LectionaryReading),
}

pub fn lookup_links(_lookup_type: &LookupType) -> String {
    todo!()
}
