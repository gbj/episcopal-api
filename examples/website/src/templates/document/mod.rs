use liturgy::Document;
use perseus::{RenderFnResult, RenderFnResultWithCause, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::table_of_contents::LITURGIES;
mod biblical_citation;
mod components;
use components::*;
mod lookup;

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentPageProps {
    document: Document,
    date: Option<String>,
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

#[perseus::autoserde(build_state)]
pub async fn get_build_props(
    path: String,
    _locale: String,
) -> RenderFnResultWithCause<DocumentPageProps> {
    let document = LITURGIES
        .iter()
        .find(|(s_slug, _)| path.contains(s_slug))
        .map(|(_, document)| (*document).clone())
        .unwrap_or_else(|| panic!("could not find liturgy for {}", path));
    Ok(DocumentPageProps {
        document,
        date: None,
    })
}

pub async fn get_static_paths() -> RenderFnResult<Vec<String>> {
    Ok(LITURGIES.iter().map(|(slug, _)| slug.to_string()).collect())
}
