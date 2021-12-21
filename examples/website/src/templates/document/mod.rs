use std::collections::HashMap;

use calendar::{Date, BCP1979_CALENDAR};
use library::{CommonPrayer, Library};
use liturgy::{Content, Document, LiturgyPreferences};
use perseus::{GenericErrorWithCause, RenderFnResult, RenderFnResultWithCause, Request, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::components::*;
use crate::table_of_contents::{CANTICLES, LITURGIES};

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentPageProps {
    document: Document,
}

#[perseus::template(DocumentPage)]
#[component(DocumentPage<G>)]
pub fn document_page(props: DocumentPageProps) -> View<G> {
    view! {
        main {
            DocumentComponent(Signal::new(props.document).handle())
        }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("document")
        .build_state_fn(get_build_props)
        .build_paths_fn(get_static_paths)
        .request_state_fn(get_request_state)
        .template(document_page)
        .head(head_fn)
}

#[perseus::head]
pub fn head_fn<G: Html>() -> View<G> {
    view! {
        title { "" }
        link(rel = "stylesheet", href="/.perseus/static/document.css")
    }
}

fn toc_docs() -> Vec<(&'static str, &'static Document)> {
    let mut docs = Vec::new();
    docs.append(&mut LITURGIES.clone());
    docs.append(&mut CANTICLES.clone());
    docs
}

fn path_to_doc(path: &str) -> Option<Document> {
    let path_slug = path.replace("document/", "");

    toc_docs()
        .iter()
        .find(|(s_slug, _)| *s_slug == path_slug)
        .map(|(_, document)| (*document).clone())
}

#[perseus::autoserde(build_state)]
pub async fn get_build_props(
    path: String,
    _locale: String,
) -> RenderFnResultWithCause<DocumentPageProps> {
    let document = path_to_doc(&path)
        .unwrap_or_else(|| panic!("(get_build_props) could not find liturgy for {}", path));
    Ok(DocumentPageProps { document })
}

pub async fn get_static_paths() -> RenderFnResult<Vec<String>> {
    let liturgies = LITURGIES.iter();
    let canticles = CANTICLES.iter();

    Ok(liturgies
        .chain(canticles)
        .map(|(slug, _)| slug.to_string())
        .collect())
}

#[perseus::autoserde(request_state)]
pub async fn get_request_state(
    path: String,
    _locale: String,
    req: Request,
) -> RenderFnResultWithCause<DocumentPageProps> {
    let document = path_to_doc(&path);
    if let Some(mut document) = document {
        let evening = if let Content::Liturgy(liturgy) = &document.content {
            liturgy.evening
        } else {
            false
        };

        let liturgy_prefs = if let Content::Liturgy(liturgy) = &document.content {
            liturgy.preferences.clone()
        } else {
            LiturgyPreferences::default()
        };

        // parse date and compile if it's present

        let uri = url::Url::parse(&format!("https://commonprayeronline.org/{}", req.uri()))?;
        let query_pairs = uri.query_pairs().collect::<HashMap<_, _>>();
        let date = query_pairs.get("date").ok_or(()).and_then(|date_str| {
            let mut split = date_str.split('-');
            let year = split.next().unwrap_or_default().parse::<u16>();
            let month = split.next().unwrap_or_default().parse::<u8>();
            let day = split.next().unwrap_or_default().parse::<u8>();
            if let (Ok(year), Ok(month), Ok(day)) = (year, month, day) {
                Ok(Date::from_ymd(year, month, day))
            } else {
                Err(())
            }
        });

        if let Ok(date) = date {
            // TODO parse these from other params:
            // - calendar
            // - observed
            // - prefs
            let calendar = &BCP1979_CALENDAR;
            let day = calendar.liturgical_day(date, evening);
            let prefs = HashMap::new();

            document = CommonPrayer::compile(
                document,
                calendar,
                &day,
                &day.observed,
                &prefs,
                &liturgy_prefs,
            )
            .ok_or_else(|| perseus::GenericErrorWithCause {
                error: "Error encountered while compiling this liturgy for this day.".into(),
                cause: perseus::ErrorCause::Server(None),
            })?;
        }

        Ok(DocumentPageProps { document })
    } else {
        Err(GenericErrorWithCause {
            error: format!("(get_request_state) document not found at {}", path).into(),
            cause: perseus::ErrorCause::Client(None),
        })
    }
}
