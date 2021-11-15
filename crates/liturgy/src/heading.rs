use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Heading {
    Date,
    Day,
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
}
