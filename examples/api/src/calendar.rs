use calendar::{Date, LiturgicalDay, BCP1979_CALENDAR};
use lectionary::{Reading, BCP1979_DAILY_OFFICE_LECTIONARY};
use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
pub struct Readings {
    observed: Vec<Reading>,
    alternate: Option<Vec<Reading>>,
}

#[derive(Serialize)]
pub struct Summary {
    day: LiturgicalDay,
    daily_office_readings: Readings,
}

#[get("/day?<year>&<month>&<day>&<evening>")]
pub fn day(year: u16, month: u8, day: u8, evening: bool) -> Json<Summary> {
    let date = Date::from_ymd(year, month, day);
    let day = BCP1979_CALENDAR.liturgical_day(date, evening);
    let daily_office_readings = Readings {
        observed: BCP1979_DAILY_OFFICE_LECTIONARY
            .readings_by_day(&day.observed, &day)
            .collect(),
        alternate: day.alternate.map(|alternate| {
            BCP1979_DAILY_OFFICE_LECTIONARY
                .readings_by_day(&alternate, &day)
                .collect()
        }),
    };
    Json(Summary {
        day,
        daily_office_readings,
    })
}
