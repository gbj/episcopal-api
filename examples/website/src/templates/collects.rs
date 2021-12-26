use crate::components::*;
use library::rite2::collects::COLLECTS_CONTEMPORARY;
use liturgy::{Content, Document, Version};
use perseus::{t, Html, RenderFnResult, RenderFnResultWithCause, Template};
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
    version: Version,
    collects: Vec<Document>,
}

pub async fn get_static_paths() -> RenderFnResult<Vec<String>> {
    Ok(vec!["RiteI".into(), "RiteII".into()])
}

#[perseus::autoserde(build_state)]
pub async fn get_static_props(
    path: String,
    _locale: String,
) -> RenderFnResultWithCause<CollectPageProps> {
    let version = path_to_version(path);

    let collects = match version {
        // TODO add Rite I
        // Version::RiteI => ,
        _ => COLLECTS_CONTEMPORARY
            .iter()
            .map(|(_, document)| document)
            .cloned()
            .collect::<Vec<_>>(),
    };

    Ok(CollectPageProps { version, collects })
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
    let search = Signal::new(String::default());
    let collects = View::new_fragment(
        props
            .collects
            .into_iter()
            .map(|document| {
                let search = search.clone();
                let is_included = create_selector({
                    let document = document.clone();
                    move || {
                        let label_matches = if let Some(label) = &document.label {
                            label.contains(&*search.get())
                        } else {
                            false
                        };
                        let text_matches = if let Content::Text(text) = &document.content {
                            text.text.contains(&*search.get())
                        } else {
                            false
                        };
                        label_matches || text_matches
                    }
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

    view! {
      main {
        h1 {
          (title)
        }
        input(type = "search", bind:value=search)
        (collects)
      }
    }
}
