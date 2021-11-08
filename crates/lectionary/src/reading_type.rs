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
    // Easter Vigil and Pentecost Vigil
    Vigil1,
    Vigil2,
    Vigil3,
    Vigil4,
    Vigil5,
    Vigil6,
    Vigil7,
    Vigil8,
    Vigil9,
    VigilPsalm1,
    VigilPsalm2,
    VigilPsalm3,
    VigilPsalm4,
    VigilPsalm5,
    VigilPsalm6,
    VigilPsalm7,
    VigilPsalm8,
    VigilPsalm9,
}
