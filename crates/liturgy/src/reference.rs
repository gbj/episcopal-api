use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    pub source: Source,
    pub page: u16,
}

impl Reference {
    pub fn as_url(&self) -> String {
        let base_url = match self.source {
            Source::BCP1979 => "https://www.episcopalchurch.org/wp-content/uploads/sites/2/2019/11/bcp_compressed.pdf",
            Source::EOW1 => todo!(),
        };

        format!("{}#page={}", base_url, self.page)
    }
}

impl Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} p. {}", self.source, self.page)
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Source {
    BCP1979,
    EOW1,
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Source::BCP1979 => "BCP",
            Source::EOW1 => "EOW 1",
        };
        write!(f, "{}", name)
    }
}
