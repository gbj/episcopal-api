use api::summary::*;
use calendar::{Date, LiturgicalDay, BCP1979_CALENDAR};
use language::Language;
use library::CommonPrayer;
use rocket::{get, serde::json::Json};

#[get("/day?<year>&<month>&<day>&<evening>")]
pub fn day(year: u16, month: u8, day: u8, evening: bool) -> Json<LiturgicalDay> {
    let date = Date::from_ymd(year, month, day);
    let day = BCP1979_CALENDAR.liturgical_day(date, evening);
    Json(day)
}

#[get("/daily_summary?<year>&<month>&<day>")]
pub fn daily_summary(year: u16, month: u8, day: u8) -> Json<DailySummary> {
    let date = Date::from_ymd(year, month, day);
    let summary = CommonPrayer::summarize_date(&date, Language::En);
    Json(summary)
}
