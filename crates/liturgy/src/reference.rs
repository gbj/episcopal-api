use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    pub source: Source,
    pub page: u16,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Source {
    BCP1979,
    EOW1,
}
