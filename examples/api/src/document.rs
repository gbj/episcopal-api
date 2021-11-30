use std::collections::HashMap;

use calendar::{Date, BCP1979_CALENDAR};
use library::{
    rite2::{COMPLINE, NOONDAY_PRAYER},
    CommonPrayer, Library,
};
use liturgy::Document;
use rocket::{response::content::Html, serde::json::Json};
use web::DocumentView;

fn slug_to_doc(slug: &str) -> Option<Document> {
    match slug {
        "noonday_prayer" => Some(NOONDAY_PRAYER.clone()),
        "compline" => Some(COMPLINE.clone()),
        _ => None,
    }
}

#[get("/doc?<slug>&<year>&<month>&<day>&<evening>")]
pub fn doc_to_json(
    slug: &str,
    year: u16,
    month: u8,
    day: u8,
    evening: bool,
) -> Json<Option<Document>> {
    let date = Date::from_ymd(year, month, day);
    let day = BCP1979_CALENDAR.liturgical_day(date, evening);
    let document = slug_to_doc(slug);
    let prefs = HashMap::new();
    let compiled = document
        .and_then(|doc| CommonPrayer::compile(doc, &BCP1979_CALENDAR, &day, &day.observed, &prefs));

    Json(compiled)
}

#[get("/<slug>/index.html?<year>&<month>&<day>&<evening>")]
pub fn doc_to_html(slug: &str, year: u16, month: u8, day: u8, evening: bool) -> Html<String> {
    let date = Date::from_ymd(year, month, day);
    let day = BCP1979_CALENDAR.liturgical_day(date, evening);
    let document = slug_to_doc(slug);
    let prefs = HashMap::new();
    let compiled = document
        .and_then(|doc| CommonPrayer::compile(doc, &BCP1979_CALENDAR, &day, &day.observed, &prefs));

    let component = DocumentView::from(compiled.unwrap_or_default());
    Html(component.to_html())
}
