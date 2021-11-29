use std::collections::HashMap;

use calendar::{Date, BCP1979_CALENDAR};
use library::{rite2::NOONDAY_PRAYER, CommonPrayer, Library};
use liturgy::{Document, Psalm};
use psalter::bcp1979::BCP1979_PSALTER;
use rocket::{response::content::Html, serde::json::Json};
use web::DocumentView;

#[get("/psalm?<number>", rank = 1)]
pub fn psalm_by_number(number: u8) -> Json<Option<Psalm>> {
    Json(BCP1979_PSALTER.psalm_by_number(number).cloned())
}

#[get("/psalm?<citation>", rank = 2)]
pub fn psalms_by_citation(citation: &str) -> Json<Vec<Psalm>> {
    Json(BCP1979_PSALTER.psalms_by_citation(citation))
}

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
        "noonday-prayer" => Some(NOONDAY_PRAYER.clone()),
        _ => None,
    };
    let prefs = HashMap::new();
    let compiled = document
        .and_then(|doc| CommonPrayer::compile(doc, &BCP1979_CALENDAR, &day, &day.observed, &prefs));

    Json(compiled)
}

#[get("/<slug>/index.html?<year>&<month>&<day>&<evening>")]
pub fn doc_to_html(slug: &str, year: u16, month: u8, day: u8, evening: bool) -> Html<String> {
    let date = Date::from_ymd(year, month, day);
    let day = BCP1979_CALENDAR.liturgical_day(date, evening);
    let document: Option<Document> = match slug {
        "noonday-prayer" => Some(NOONDAY_PRAYER.clone()),
        _ => None,
    };
    let prefs = HashMap::new();
    let compiled = document
        .and_then(|doc| CommonPrayer::compile(doc, &BCP1979_CALENDAR, &day, &day.observed, &prefs));

    let component = DocumentView::from(compiled.unwrap_or_default());
    Html(component.to_html())
}
