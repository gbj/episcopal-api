use std::collections::HashMap;

use calendar::{Date, LiturgicalDay, BCP1979_CALENDAR};
use lectionary::{
    Reading, BCP1979_30_DAY_PSALTER, BCP1979_DAILY_OFFICE_LECTIONARY, BCP1979_DAILY_OFFICE_PSALTER,
};
use library::rite2::NOONDAY_PRAYER;
use liturgy::Document;
use rocket::serde::{json::Json, Serialize};

#[get("/doc?<slug>&<year>&<month>&<day>&<evening>")]
pub fn document(
    slug: &str,
    year: u16,
    month: u8,
    day: u8,
    evening: bool,
) -> Json<Option<Document>> {
    let date = Date::from_ymd(year, month, day);
    let day = BCP1979_CALENDAR.liturgical_day(date, evening);
    let document: Option<Document> = match slug {
        "noonday_prayer" => Some(NOONDAY_PRAYER.clone()),
        _ => None,
    };
    let prefs = HashMap::new();
    let compiled = document.and_then(|doc| doc.compile(&BCP1979_CALENDAR, &day, &prefs));

    Json(compiled)
}
