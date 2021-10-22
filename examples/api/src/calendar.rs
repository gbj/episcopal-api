use calendar::{Date, LiturgicalDay, BCP1979_CALENDAR};
use rocket::serde::{json::Json, Serialize};

#[derive(Debug, Serialize)]
struct Test {
    a: &'static str,
}

#[get("/day?<year>&<month>&<day>&<evening>")]
pub fn day(year: u16, month: u8, day: u8, evening: bool) -> Json<LiturgicalDay> {
    let date = Date::from_ymd(year, month, day);
    let liturgical_day = BCP1979_CALENDAR.liturgical_day(date, evening);
    Json(liturgical_day)
}
