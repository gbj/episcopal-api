use std::collections::HashMap;
use std::convert::TryFrom;

use calendar::{Date, BCP1979_CALENDAR};
use library::{CommonPrayer, Library};
use liturgy::{Content, Document, LiturgyPreferences, Version};
use perseus::{
    t, GenericErrorWithCause, RenderFnResult, RenderFnResultWithCause, Request, Template,
};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::components::*;
use crate::table_of_contents::{PageType, TABLE_OF_CONTENTS};

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentPageProps {
    locale: String,
    document: Document,
}

#[perseus::template(DocumentPage)]
#[component(DocumentPage<G>)]
pub fn document_page(props: DocumentPageProps) -> View<G> {
    let label = props.document.label.clone().unwrap_or_default();
    let locale = props.locale;
    view! {
        header {
            (cloned!((locale) => menu_component(locale)))
            p(class = "page-title") {
                (label)
            }
        }
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
pub fn head_fn<G: Html>(props: DocumentPageProps) -> View<G> {
    let title = match &props.document.label {
        Some(label) => format!("{} – {}", label, t!("common_prayer")),
        None => t!("common_prayer"),
    };

    view! {
        title { (title) }
        link(rel = "stylesheet", href="/.perseus/static/document.css")
    }
}

fn toc_docs() -> Vec<(String, String, PageType, &'static Document)> {
    TABLE_OF_CONTENTS
        .iter()
        .flat_map(|(category, docs)| {
            docs.iter().map(move |(slug, page_type, doc)| {
                (category.clone(), slug.clone(), page_type.clone(), doc)
            })
        })
        .collect()
}

fn path_to_doc(path: &str) -> Option<Document> {
    let mut parts = path.split('/');
    let _ = parts.next(); // /document
    let category = parts.next().expect("expected a category");
    let slug = parts.next().expect("expected a slug");
    let version: Option<Version> = parts
        .next()
        .map(|version| Version::try_from(version).expect("could not parse version from path"));

    toc_docs()
        .iter()
        .find(|(s_category, s_slug, _, _)| s_category == category && *s_slug == slug)
        .map(|(_, _, page_type, document)| {
            let doc = (*document).clone();
            match (page_type, &doc.content) {
                // if it's a category page, and a choice, then unravel it into a series
                (PageType::Category(label), Content::Choice(choice)) => Document {
                    content: Content::Series(choice.clone().into()),
                    label: Some(label.to_string()),
                    ..doc
                },
                // for all other category pages, just add the category as a label
                // this looks up the translation using the slug as a key
                (PageType::Category(label), _) => Document {
                    label: Some(label.to_string()),
                    ..doc
                },
                _ => doc,
            }
        })
}

#[perseus::autoserde(build_state)]
pub async fn get_build_props(
    path: String,
    locale: String,
) -> RenderFnResultWithCause<DocumentPageProps> {
    let document = path_to_doc(&path)
        .unwrap_or_else(|| panic!("(get_build_props) could not find liturgy for {}", path));
    Ok(DocumentPageProps { locale, document })
}

pub async fn get_static_paths() -> RenderFnResult<Vec<String>> {
    Ok(TABLE_OF_CONTENTS
        .iter()
        .flat_map(|(category, docs)| {
            docs.iter()
                .map(move |(slug, _, doc)| (category.clone(), slug.clone(), doc.version))
        })
        .map(|(category, slug, version)| format!("{category}/{slug}/{:#?}", version))
        .collect())
}

#[perseus::autoserde(request_state)]
pub async fn get_request_state(
    path: String,
    locale: String,
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

        Ok(DocumentPageProps { locale, document })
    } else {
        Err(GenericErrorWithCause {
            error: format!("(get_request_state) document not found at {}", path).into(),
            cause: perseus::ErrorCause::Client(None),
        })
    }
}
