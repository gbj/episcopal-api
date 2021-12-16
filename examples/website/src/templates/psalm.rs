use liturgy::Document;
use perseus::{
    ErrorCause, GenericErrorWithCause, Html, RenderFnResult, RenderFnResultWithCause, Template,
};
use psalter::bcp1979::BCP1979_PSALTER;
use serde::{Deserialize, Serialize};
use sycamore::prelude::{component, view, Signal, View};

use crate::components::*;

#[derive(Serialize, Deserialize)]
pub struct PsalterPageProps {
    psalm: Document,
}

#[perseus::template(PsalterPage)]
#[component(PsalterPage<G>)]
pub fn psalter_page(props: PsalterPageProps) -> View<G> {
    view! {
      DocumentComponent(Signal::new(props.psalm).handle())
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
pub fn head<G: Html>() -> View<G> {
    view! {
        title { "Psalter – Common Prayer" }
        link(rel = "stylesheet", href="/.perseus/static/document.css")
    }
}

#[perseus::autoserde(build_state)]
pub async fn get_static_props(
    path: String,
    _locale: String,
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
    Ok(PsalterPageProps { psalm })
}

pub async fn get_static_paths() -> RenderFnResult<Vec<String>> {
    // generate a page for each psalm
    Ok((1..=150).map(|n| n.to_string()).collect())
}
