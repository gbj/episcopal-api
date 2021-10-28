use calendar::{Date, LiturgicalDay, BCP1979_CALENDAR};
use lectionary::{Reading, BCP1979_DAILY_OFFICE_LECTIONARY};
use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
pub struct Summary {
    day: LiturgicalDay,
    daily_office_readings: Vec<Reading>,
}

#[get("/day?<year>&<month>&<day>&<evening>")]
pub fn day(year: u16, month: u8, day: u8, evening: bool) -> Json<Summary> {
    let date = Date::from_ymd(year, month, day);
    let day = BCP1979_CALENDAR.liturgical_day(date, evening);
    let daily_office_readings = BCP1979_DAILY_OFFICE_LECTIONARY
        .readings_by_day(&day)
        .collect();
    Json(Summary {
        day,
        daily_office_readings,
    })
}
