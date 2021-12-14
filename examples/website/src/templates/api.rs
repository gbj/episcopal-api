use calendar::LiturgicalDay;
use lectionary::Reading;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Readings {
    observed: Vec<Reading>,
    alternate: Option<Vec<Reading>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Psalms {
    thirty_day: Vec<Reading>,
    daily_office_lectionary: Readings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Summary {
    day: LiturgicalDay,
    daily_office_readings: Readings,
    daily_office_psalms: Psalms,
}
