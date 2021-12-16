use crate::components::*;
use liturgy::{BiblicalCitation, BiblicalReading, Document, Version};
use perseus::{is_server, t};
use reference_parser::{BibleVerse, BibleVersePart, Book};
use reqwasm::http::Request;
use serde::Deserialize;
use sycamore::futures::spawn_local_in_scope;
use sycamore::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
enum State {
    Idle,
    Loading,
    Error,
    Success(Box<Document>),
}

#[component(BiblicalCitationComponent<G>)]
pub fn biblical_citation_component(citation: BiblicalCitation) -> View<G> {
    let state = Signal::new(State::Idle);

    create_effect(
        cloned!((state, citation) => move || if *state.get() == State::Idle {
          state.set(State::Loading);
          // TODO proper version
          spawn_local_in_scope(cloned!((state, citation) => async move {
            match fetch_reading(citation, Version::NRSV).await {
                Ok(reading) => state.set(State::Success(Box::new(reading))),
                Err(_) => state.set(State::Error),
            }
          }))
        }),
    );

    let main = create_memo(cloned!((state) => move || match &*state.get() {
        State::Idle => view! {
          article(class = "biblical-citation") {
            header {
              h4(class = "citation") {
                (citation)
              }
            }
          }
        },
        State::Loading => view! {
          article(class = "biblical-citation") {
            header {
              h4(class = "citation") {
                (citation)
              }
            }
            main {
              (t!("loading"))
            }
          }
        },
        State::Error => {
            let citation = citation.citation.clone();
            let citation_for_error = citation.clone();
            view! {
              article(class = "biblical-citation") {
                header {
                  h4(class = "citation") {
                    (citation)
                  }
                }
                main {
                  (t!("biblical_citation_error", { "citation": citation_for_error.clone() }))
                }
              }
            }
        }
        State::Success(reading) => view! {
          DocumentComponent(Signal::new(*reading.clone()).handle())
        },
    }));

    view! {
      (*main.get())
    }
}

async fn fetch_reading(citation: BiblicalCitation, version: Version) -> Result<Document, ()> {
    if !is_server!() {
        let url = reading_url(&citation.citation, version);

        let reading = Request::get(&url)
            .send()
            .await
            .map_err(|_| ())?
            .json::<BibleReadingFromAPI>()
            .await
            .map_err(|_| ())?
            .api_data_to_biblical_reading(citation);
        Ok(Document::from(reading))
    } else {
        Ok(Document::from(citation))
    }
}

#[derive(Deserialize)]
pub struct BibleReadingFromAPI {
    pub citation: String,
    pub label: String,
    pub version: Version,
    pub value: Vec<serde_json::value::Value>,
}

impl BibleReadingFromAPI {
    fn api_data_to_biblical_reading(&self, citation: BiblicalCitation) -> BiblicalReading {
        BiblicalReading {
            citation: self.citation.clone(),
            intro: citation.intro,
            text: self
                .value
                .iter()
                .filter_map(|line| {
                    let book = line.get("book");
                    let chapter = line.get("chapter");
                    let verse = line.get("verse");
                    let text = line.get("text");
                    let ldf_type = line.get("type");
                    match (ldf_type, book, chapter, verse, text) {
                        (Some(_), _, _, _, _) => None,
                        (_, Some(book), Some(chapter), Some(verse), Some(text)) => {
                            let text = text.as_str().unwrap().to_string();
                            let book = Book::from(book.as_str().unwrap());
                            let chapter = chapter.as_str().unwrap().parse().unwrap();
                            let verse = verse.as_str().unwrap().parse().unwrap();

                            Some((
                                BibleVerse {
                                    book,
                                    chapter,
                                    verse,
                                    verse_part: BibleVersePart::All,
                                },
                                text,
                            ))
                        }
                        _ => None,
                    }
                })
                .collect(),
        }
    }
}

fn reading_url(citation: &str, version: Version) -> String {
    format!(
        "https://us-central1-venite-2.cloudfunctions.net/bible?citation={}&version={}",
        citation, version
    )
}
