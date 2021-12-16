use calendar::LiturgicalDay;
use lectionary::{Reading, ReadingType};
use liturgy::Psalm;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Summary {
    pub day: LiturgicalDay,
    pub daily_office_readings: Readings,
    pub daily_office_psalms: Psalms,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Readings {
    pub observed: Vec<Reading>,
    pub alternate: Option<Vec<Reading>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Psalms {
    pub thirty_day: Vec<Reading>,
    pub daily_office_lectionary: Readings,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SummaryWithPsalms {
    pub day: LiturgicalDay,
    pub daily_office_readings: Readings,
    pub daily_office_psalms: PsalmsWithPsalms,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PsalmsWithPsalms {
    pub thirty_day: PsalmsByTime,
    pub daily_office_lectionary: ReadingsWithPsalms,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReadingsWithPsalms {
    pub observed: PsalmsByTime,
    pub alternate: Option<PsalmsByTime>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PsalmsByTime {
    pub morning: Vec<Psalm>,
    pub evening: Vec<Psalm>,
}
