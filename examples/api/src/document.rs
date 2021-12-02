use std::collections::HashMap;

use calendar::{Date, BCP1979_CALENDAR};
use library::{
    rite2::{COMPLINE, MORNING_PRAYER_II, NOONDAY_PRAYER},
    CommonPrayer, Library,
};
use liturgy::{Content, Document, GlobalPref, Lectionaries, PreferenceKey, PreferenceValue};
use rocket::{response::content::Html, serde::json::Json};
use web::DocumentView;

use crate::error::{APIError, APIErrorResponder};

fn slug_to_doc(slug: &str) -> Option<Document> {
    match slug {
        "morning-prayer" => Some(MORNING_PRAYER_II.clone()),
        "noonday-prayer" => Some(NOONDAY_PRAYER.clone()),
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

#[get("/?<liturgy>&<date>")]
pub fn doc_to_html(liturgy: &str, date: &str) -> Result<Html<String>, APIErrorResponder> {
    let date = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map_err(|_| APIErrorResponder::from(APIError::DateError(date.to_string())))?;

    let date = Date::from(date);

    let document =
        slug_to_doc(liturgy).ok_or_else(|| APIError::LiturgyError(liturgy.to_string()))?;

    let evening = if let Content::Liturgy(liturgy) = &document.content {
        liturgy.evening
    } else {
        false
    };

    let day = BCP1979_CALENDAR.liturgical_day(date, evening);

    // Default prefs
    let mut prefs = HashMap::new();
    prefs.insert(
        PreferenceKey::from(GlobalPref::PsalmCycle),
        PreferenceValue::Lectionary(Lectionaries::BCP1979DailyOfficePsalms),
    );

    let compiled = CommonPrayer::compile(document, &BCP1979_CALENDAR, &day, &day.observed, &prefs);

    let label = compiled
        .as_ref()
        .and_then(|doc| doc.label.as_ref())
        .cloned()
        .unwrap_or_default();

    let html = DocumentView::from(compiled.unwrap_or_default())
        .mark_as_top_level()
        .to_html();

    Ok(Html(format!(
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                <link rel="stylesheet" href="/assets/document.css">
                <meta name="viewport" content="width=device-width, initial-scale=1.0"> 
                <title>{}</title>
            </head>
            <body>
                <header><h1>{}</h1></header>
                <main>
                {}
                </main>
                <script src="/assets/document.js"></script>
            </body>
        </html>
    "#,
        label, label, html
    )))
}
