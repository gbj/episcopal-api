use crate::components::*;
use library::{rite1::collects::COLLECTS_TRADITIONAL, rite2::collects::COLLECTS_CONTEMPORARY};
use liturgy::{Document, Version};
use perseus::{is_server, t, Html, RenderFnResult, RenderFnResultWithCause, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("collects")
        .template(collect_page)
        .build_state_fn(get_static_props)
        .build_paths_fn(get_static_paths)
        .incremental_generation()
        .head(head)
}
#[derive(Serialize, Deserialize)]
pub struct CollectPageProps {
    locale: String,
    version: Version,
    collects: Vec<Document>,
}

pub async fn get_static_paths() -> RenderFnResult<Vec<String>> {
    Ok(vec!["RiteI".into(), "RiteII".into()])
}

#[perseus::autoserde(build_state)]
pub async fn get_static_props(
    path: String,
    locale: String,
) -> RenderFnResultWithCause<CollectPageProps> {
    let version = path_to_version(path);

    let collects = match version {
        Version::RiteI => COLLECTS_TRADITIONAL.iter(),
        _ => COLLECTS_CONTEMPORARY.iter(),
    }
    .map(|(_, document)| document)
    .cloned()
    .collect::<Vec<_>>();

    Ok(CollectPageProps {
        locale,
        version,
        collects,
    })
}

#[perseus::head]
pub fn head<G: Html>(props: CollectPageProps) -> View<G> {
    let title = page_title(props.version);
    let cp = t!("common_prayer");
    let title = format!("{title} – {cp}");

    view! {
        title { (title) }
        link(rel = "stylesheet", href="/.perseus/static/document.css")
    }
}

fn path_to_version(path: String) -> Version {
    if path.ends_with("RiteI") {
        Version::RiteI
    } else {
        Version::RiteII
    }
}

fn page_title(version: Version) -> String {
    match version {
        Version::RiteI => t!("collects_traditional"),
        Version::RiteII => t!("collects_contemporary"),
        _ => t!("collects"),
    }
}

#[perseus::template(CollectPage)]
#[component(CollectPage<G>)]
pub fn collect_page(props: CollectPageProps) -> View<G> {
    let wasm_loaded = Signal::new(!is_server!());
    let search_placeholder = create_selector({
        let wasm_loaded = wasm_loaded.clone();
        move || {
            if *wasm_loaded.get() {
                "".into()
            } else {
                t!("loading")
            }
        }
    });

    let search = Signal::new(String::default());
    let collects = View::new_fragment(
        props
            .collects
            .into_iter()
            .map(|document| {
                let search = search.clone();
                let is_included = create_selector({
                    let document = document.clone();
                    move || document.contains(&*search.get())
                });
                let class = create_selector(move || if *is_included.get() { "" } else { "hidden" });
                view! {
                    article(class = (*class.get())) {
                        DocumentComponent(Signal::new(document).handle())
                    }
                }
            })
            .collect(),
    );

    let title = page_title(props.version);

    let locale = props.locale;

    view! {
      header {
        (cloned!((locale) => menu_component(locale)))
        p(class = "page-title") {
            (title)
        }
      }
      main {
        h1 {
          (title)
        }
        fieldset(class = "stacked search") {
            label(for = "search") {
                (t!("search"))
            }
            input(
                id = "search",
                type = "search",
                bind:value=search,
                disabled = !(*wasm_loaded.get()),
                placeholder = *search_placeholder.get()
            )
        }
        (collects)
      }
    }
}
