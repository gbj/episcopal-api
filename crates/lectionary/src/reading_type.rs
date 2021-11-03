use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum ReadingType {
    FirstReading,
    Psalm,
    SecondReading,
    Gospel,
    MorningPsalm,
    EveningPsalm,
    Morning1,
    Morning2,
    Evening1,
    Evening2,
    // Special Services
    PalmsGospel,
}
