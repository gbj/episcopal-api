use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Words {
    EnI(String),
    EnII(String),
    Es(String),
    Fr(String),
}
