use liturgy::{Content, Document};
use perseus::{
    t, ErrorCause, GenericErrorWithCause, Html, RenderFnResult, RenderFnResultWithCause, Template,
};
use psalter::bcp1979::BCP1979_PSALTER;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::components::*;

#[derive(Serialize, Deserialize)]
pub struct PsalterPageProps {
    locale: String,
    psalm: Document,
}

fn page_title(props: &PsalterPageProps) -> String {
    let number = match &props.psalm.content {
        Content::Psalm(psalm) => Some(psalm.number.to_string()),
        _ => None,
    };
    number
        .map(|n| t!("psalm", { "number": n }))
        .unwrap_or_else(|| t!("psalter"))
}

#[perseus::template(PsalterPage)]
#[component(PsalterPage<G>)]
pub fn psalter_page(props: PsalterPageProps) -> View<G> {
    let title = page_title(&props);
    let locale = props.locale;

    view! {
        header {
            (cloned!((locale) => menu_component(locale)))
            p(class = "page-title") {
                (title)
            }
        }
        main {
            DocumentComponent(Signal::new(props.psalm).handle())
        }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("psalm")
        .head(head)
        .build_paths_fn(get_static_paths)
        .build_state_fn(get_static_props)
        .template(psalter_page)
}

#[perseus::head]
pub fn head<G: Html>(props: PsalterPageProps) -> View<G> {
    let title = page_title(&props);

    view! {
        title { (title) " – " (t!("common_prayer")) }
        link(rel = "stylesheet", href="/.perseus/static/document.css")
    }
}

#[perseus::autoserde(build_state)]
pub async fn get_static_props(
    path: String,
    locale: String,
) -> RenderFnResultWithCause<PsalterPageProps> {
    let not_found_err = || GenericErrorWithCause {
        error: "psalm not found".into(),
        cause: ErrorCause::Client(Some(404)),
    };
    let psalter = &BCP1979_PSALTER;
    let psalm_number = path
        .split('/')
        .last()
        .ok_or_else(not_found_err)
        .and_then(|n| n.parse::<u8>().map_err(|_| not_found_err()))?;
    let psalm = psalter
        .psalm_by_number(psalm_number)
        .map(|psalm| Document::from(psalm.clone()))
        .ok_or_else(not_found_err)?;
    Ok(PsalterPageProps { locale, psalm })
}

pub async fn get_static_paths() -> RenderFnResult<Vec<String>> {
    // generate a page for each psalm
    Ok((1..=150).map(|n| n.to_string()).collect())
}
