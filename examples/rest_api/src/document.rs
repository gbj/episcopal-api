use std::collections::HashMap;

use calendar::{Date, BCP1979_CALENDAR};
use library::{
    rite2::{COMPLINE, MORNING_PRAYER_II, NOONDAY_PRAYER},
    CommonPrayer, Library,
};
use liturgy::{Content, Document, LiturgyPreferences};
use rocket::{response::content::Html, serde::json::Json};
use web::Viewer;

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

    let compiled = document.and_then(|doc| {
        let prefs = HashMap::new();

        let liturgy_prefs = if let Content::Liturgy(liturgy) = &doc.content {
            liturgy.preferences.clone()
        } else {
            LiturgyPreferences::default()
        };

        CommonPrayer::compile(
            doc,
            &BCP1979_CALENDAR,
            &day,
            &day.observed,
            &prefs,
            &liturgy_prefs,
        )
    });

    Json(compiled)
}

#[get("/?<liturgy>&<date>")]
pub fn doc_to_html(liturgy: &str, date: Option<&str>) -> Result<Html<String>, APIErrorResponder> {
    let document =
        slug_to_doc(liturgy).ok_or_else(|| APIError::LiturgyError(liturgy.to_string()))?;

    let evening = if let Content::Liturgy(liturgy) = &document.content {
        liturgy.evening
    } else {
        false
    };

    let compiled = if let Some(date) = date {
        let date = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .map_err(|_| APIErrorResponder::from(APIError::DateError(date.to_string())))?;

        let date = Date::from(date);

        let day = BCP1979_CALENDAR.liturgical_day(date, evening);

        // Default prefs
        let prefs = HashMap::new();

        let liturgy_prefs = if let Content::Liturgy(liturgy) = &document.content {
            liturgy.preferences.clone()
        } else {
            LiturgyPreferences::default()
        };

        CommonPrayer::compile(
            document,
            &BCP1979_CALENDAR,
            &day,
            &day.observed,
            &prefs,
            &liturgy_prefs,
        )
    } else {
        Some(document)
    };

    let label = compiled
        .as_ref()
        .and_then(|doc| doc.label.as_ref())
        .cloned()
        .unwrap_or_default();

    let serialized_state = serde_json::to_string(&compiled)
        .map_err(|_| APIError::JsonError)?
        .replace('\\', "\\\\");

    let html = Viewer::from(compiled.unwrap_or_default()).to_html();

    Ok(Html(format!(
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                <link rel="stylesheet" href="/assets/document.css">
                <meta name="viewport" content="width=device-width, initial-scale=1.0"> 
                <title>{}</title>
                <script type="module">
                    import init, {{ initialize_from_json }} from '/pkg/web.js';
                    async function start() {{
                        await init();
                        const state = `{}`;
                        initialize_from_json("main", state);
                    }}
                    start();
                </script>
            </head>
            <body>
                <header><h1>{}</h1></header>
                {}
            </body>
        </html>
    "#,
        label, serialized_state, label, html
    )))
}
