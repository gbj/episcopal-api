use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Season {
    Advent,
    Christmas,
    Epiphany,
    Lent,
    HolyWeek,
    Easter,
    Ascension,
    Pentecost,
    Saints,
    OrdinaryTime,
    Ember,
    National,
    Rogation,
    Mary,
}