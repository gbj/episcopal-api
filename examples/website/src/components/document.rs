use super::biblical_citation::*;
use liturgy::*;
use perseus::t;
use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlSelectElement};

use super::{lookup_links, LookupType};

type HeaderAndMain<G> = (Option<View<G>>, View<G>);

#[component(DocumentComponent<G>)]
pub fn document_component(doc: ReadSignal<Document>) -> View<G> {
    let label = create_memo({
        let doc = doc.clone();
        move || {
            let doc = doc.get();
            if matches!(doc.content, Content::Liturgy(_)) {
                None
            } else {
                doc.label.as_ref().cloned()
            }
        }
    });

    let header_and_main = create_memo(move || match doc.get().content.clone() {
        Content::Series(content) => series(content),
        Content::Liturgy(content) => series(content.body),
        Content::Rubric(content) => rubric(content),
        Content::Text(content) => text(content),
        Content::Choice(content) => choice(content),
        Content::Parallel(content) => parallel(content),
        Content::Category(content) => category(content, doc.get().version),
        Content::CollectOfTheDay { allow_multiple } => {
            collect_of_the_day(allow_multiple, doc.get().version)
        }
        Content::Empty => empty(),
        Content::Error(content) => error(content),
        Content::Antiphon(content) => antiphon(content),
        Content::BiblicalCitation(content) => (None, view! { BiblicalCitationComponent(content) }),
        Content::BiblicalReading(content) => biblical_reading(content),
        Content::Canticle(content) => canticle(content),
        Content::CanticleTableEntry(content) => canticle_table_entry(content),
        Content::GloriaPatri(content) => gloria_patri(content),
        Content::Heading(content) => heading(content),
        Content::LectionaryReading(content) => lectionary_reading(content),
        Content::Litany(content) => litany(content),
        Content::Preces(content) => preces(content),
        Content::Psalm(content) => psalm(content),
        Content::PsalmCitation(content) => psalm_citation(content),
        Content::ResponsivePrayer(content) => responsive_prayer(content),
        Content::Sentence(content) => sentence(content),
    });
    let header =
        create_memo(cloned!((header_and_main) => move || (*header_and_main.get()).0.clone()));
    let main = create_memo(move || (*header_and_main.get()).1.clone());

    view! {
      (if let Some(label) = (*label.get()).clone() {
        view! {
          h3 {
            (label)
          }
        }
      } else {
        view! { }
      })
      (if let Some(header) = (*header.get()).clone() {
        view! {
          header {
            (header)
          }
        }
      } else {
        view! {}
      })
      (*main.get())
    }
}

pub fn antiphon<G: Html>(antiphon: Antiphon) -> HeaderAndMain<G> {
    (
        None,
        view! {
          main(class="antiphon") {
            (antiphon)
          }
        },
    )
}

pub fn biblical_reading<G: Html>(reading: BiblicalReading) -> HeaderAndMain<G> {
    let intro = if let Some(intro) = &reading.intro {
        let doc = Document::from(intro.clone());
        view! { DocumentComponent(Signal::new(doc).handle()) }
    } else {
        view! {}
    };

    let citation = reading.citation;
    let header = view! {
      h3(class="citation") {
        (citation)
      }
    };

    let verses = View::new_fragment(
        reading
            .text
            .into_iter()
            .map(|(verse, verse_text)| {
                view! {
                  sup(class="verse-number") {
                    (verse.verse)
                  }
                  span(dangerously_set_inner_html=&verse_text)
                }
            })
            .collect(),
    );

    let main = view! {
      main(class="biblical-reading") {
        (intro)
        (verses)
      }
    };

    (Some(header), main)
}

pub fn canticle<G: Html>(content: Canticle) -> HeaderAndMain<G> {
    let citation = if let Some(citation) = content.citation {
        view! {
            h3(class = "citation") {
              (citation)
            }
        }
    } else {
        view! {}
    };

    let number = content.number;
    let local_name = content.local_name;
    let latin_name = content.latin_name.unwrap_or_default();
    let header = View::new_fragment(vec![
        view! { h3(class = "canticle-number") { (number) } },
        view! { h4(class = "local-name") { (local_name) } },
        view! { em(class = "latin-name") { (latin_name) } },
        citation,
    ]);

    let sections = View::new_fragment(
        content
            .sections
            .into_iter()
            .map(|section| {
                let title = section.title.clone();
                let header = if let Some(title) = title {
                    view! {
                      header {
                        h4(class = "canticle-section-title") {
                          (title)
                        }
                      }
                    }
                } else {
                    view! {}
                };

                let verses = View::new_fragment(
                    section
                        .verses
                        .into_iter()
                        .map(|verse| {
                            let a = verse.a;
                            let b = verse.b;
                            view! {
                              p(class = "verse") {
                                span(class = "a") {
                                  (a)
                                }
                                span(class = "b") {
                                  (b)
                                }
                              }
                            }
                        })
                        .collect(),
                );

                view! {
                  section {
                    (header)
                    main {
                      (verses)
                    }
                  }
                }
            })
            .collect(),
    );

    let main = view! {
      main(class = "canticle") {
        (sections)
      }
    };

    (Some(header), main)
}

pub fn category<G: Html>(content: Category, version: Version) -> HeaderAndMain<G> {
    let name = t!(&format!("category_{:#?}", content.name));
    let href = lookup_links(&LookupType::Category(version, name.clone()));
    (
        None,
        view! {
          main(class="lookup category") {
            a(href=(href)) {
              (name)
            }
          }
        },
    )
}

pub fn choice<G: Html>(choice: Choice) -> HeaderAndMain<G> {
    let selected_str = Signal::new(choice.selected.to_string());
    let selected_idx = create_memo(
        cloned!((selected_str) => move || selected_str.get().parse::<usize>().unwrap_or(0)),
    );

    let options = View::new_fragment(
        choice
            .options
            .iter()
            .enumerate()
            .map(|(idx, option)| {
                let label = choice.option_label(option, idx);
                view! {
                  option(value=(idx)) {
                    (label)
                  }
                }
            })
            .collect(),
    );

    let main = View::new_fragment(
        choice
            .options
            .into_iter()
            .enumerate()
            .map(move |(idx, option)| {
                let class = create_memo(cloned!((selected_idx) => move || {
                    if idx == *(selected_idx.get()) {
                        "selected"
                    } else {
                        "hidden"
                    }
                }));

                view! {
                  li(class = (class.get())) {
                    DocumentComponent(Signal::new(option).handle())
                  }
                }
            })
            .collect(),
    );

    (
        None,
        view! {
          section(class="choice") {
            nav {
              select(
                on:change={
                  move |ev: Event| {
                    let value = ev.target()
                            .unwrap()
                            .unchecked_into::<HtmlSelectElement>()
                            .value();
                    selected_str.set(value);
                }
              }) {
                (options)
              }
            }
            ol {
              (main)
            }
          }
        },
    )
}

pub fn canticle_table_entry<G: Html>(entry: CanticleTableEntry) -> HeaderAndMain<G> {
    let href = lookup_links(&LookupType::Canticle(entry));
    let main = view! {
        main(class="lookup canticle-table-entry") {
            a(href=(href)){
              (t!("canticle_table"))
            }
        }
    };

    (None, main)
}

pub fn collect_of_the_day<G: Html>(_allow_multiple: bool, version: Version) -> HeaderAndMain<G> {
    let name = t!("collect_of_the_day");
    let href = lookup_links(&LookupType::Collect(version));
    (
        None,
        view! {
          main(class="lookup collect-of-the-day") {
            a(href=(href)) {
              (name)
            }
          }
        },
    )
}

pub fn empty<G: Html>() -> HeaderAndMain<G> {
    (None, view! {})
}

pub fn error<G: Html>(error: DocumentError) -> HeaderAndMain<G> {
    (
        None,
        view! {
          main(class="error") {
            pre {
              (error)
            }
          }
        },
    )
}

pub fn gloria_patri<G: Html>(content: GloriaPatri) -> HeaderAndMain<G> {
    let display_format = display_format_as_class(content.display_format);
    let (a, b, c, d) = content.text;
    let main = view! {
      main(class = format!("gloria-patri {}", display_format)) {
        p {
          span(class = "a") { (a) }
          span(class = "b") { (b) }
          span(class = "c") { (c) }
          span(class = "d") { (d) }
        }
      }
    };

    (None, main)
}

pub fn heading<G: Html>(heading: Heading) -> HeaderAndMain<G> {
    let main = match heading {
        Heading::Date(date) => view! {
          main(class = "heading") {
            h2(class = "date") {
              (date)
            }
          }
        },
        Heading::Day {
            name,
            proper,
            holy_days,
        } => {
            let proper = if let Some(proper) = proper {
                view! {
                  h3(class = "proper") {
                    (proper)
                  }
                }
            } else {
                view! {}
            };

            let holy_days = if let Some(holy_days) = holy_days {
                View::new_fragment(
                    holy_days
                        .into_iter()
                        .map(|holy_day| view! { li { (holy_day) }})
                        .collect(),
                )
            } else {
                view! {}
            };

            view! {
              main(class = "heading day") {
                h2(class = "day-name") {
                  (name)
                }
                (proper)
                (holy_days)
              }
            }
        }
        Heading::Text(level, content) => {
            let h = match level {
                HeadingLevel::Heading1 => view! { h1 { (content) } },
                HeadingLevel::Heading2 => view! { h2 { (content) } },
                HeadingLevel::Heading3 => view! { h3 { (content) } },
                HeadingLevel::Heading4 => view! { h4 { (content) } },
                HeadingLevel::Heading5 => view! { h5 { (content) } },
            };
            view! {
                main(class = "heading") {
                  (h)
                }
            }
        }

        // InsertDay and InsertDate can be ignored
        _ => view! {},
    };

    (None, main)
}

pub fn lectionary_reading<G: Html>(entry: LectionaryReading) -> HeaderAndMain<G> {
    let href = lookup_links(&LookupType::Lectionary(entry.lectionary));

    let main = view! {
      main(class = "lookup lectionary") {
        a(href = (href)) {
          (t!("lectionary_readings"))
        }
      }
    };

    (None, main)
}

pub fn litany<G: Html>(litany: Litany) -> HeaderAndMain<G> {
    let response = litany.response;
    let lines = litany.lines;
    let lines = View::new_fragment(
        lines
            .into_iter()
            .map(|line| {
                view! {
                  p {
                    span {
                      (line)
                    }
                    strong(class = "response") {
                      (response)
                    }
                  }
                }
            })
            .collect(),
    );

    let main = view! {
      main(class = "litany") {
        (lines)
      }
    };

    (None, main)
}

pub fn parallel<G: Html>(parallel: Parallel) -> HeaderAndMain<G> {
    let children = View::new_fragment(
        parallel
            .into_vec()
            .into_iter()
            .map(|doc| view! { DocumentComponent(Signal::new(doc).handle()) })
            .collect(),
    );

    (
        None,
        view! {
          section(class="parallel") {
            (children)
          }
        },
    )
}

pub fn preces<G: Html>(preces: Preces) -> HeaderAndMain<G> {
    let lines = View::new_fragment(
        preces
            .into_vec()
            .into_iter()
            .map(|(label, prayer)| {
                view! {
                  p(class = "line") {
                    em(class = "label") { (label) }
                    span(class = "text") { (prayer) }
                  }
                }
            })
            .collect(),
    );
    let main = view! {
      main(class = "preces") {
        (lines)
      }
    };

    (None, main)
}

pub fn psalm<G: Html>(psalm: Psalm) -> HeaderAndMain<G> {
    let psalm_number = psalm.number;
    let sections = View::new_fragment(
        psalm
            .filtered_sections()
            .into_iter()
            .map(|section| {
                let local = section.local_name;
                let latin = section.latin_name;
                let reference = reference(section.reference);

                let verses = View::new_fragment(
                    section
                        .verses
                        .into_iter()
                        .map(|verse| {
                            let number = verse.number;
                            let a = verse.a;
                            let b = verse.b;

                            view! {
                              p(class = "verse") {
                                a(id = format!("{}-{}", psalm_number, number))
                                sup(class = "number") {
                                  (number)
                                }
                                span(class = "a") {
                                  (a)
                                }
                                span(class = "b") {
                                  (b)
                                }
                              }
                            }
                        })
                        .collect(),
                );

                view! {
                  section {
                    header {
                      h3(class = "local-name") {
                        (local)
                      }
                      em(class = "latin-name") {
                        (latin)
                      }
                      (reference)
                    }
                    main {
                      (verses)
                    }
                  }
                }
            })
            .collect(),
    );

    let main = view! {
      main(class = "psalm") {
        (sections)
      }
    };

    (None, main)
}

pub fn psalm_citation<G: Html>(citation: PsalmCitation) -> HeaderAndMain<G> {
    (
        None,
        view! {
          main(class = "psalm-citation") {
            h3 {
              (citation)
            }
          }
        },
    )
}

pub fn reference<G: Html>(reference: Reference) -> View<G> {
    let href = reference.as_url();
    let text = reference.to_string();
    view! {
      a(class = "reference", href=(href)) {
        (text)
      }
    }
}

pub fn responsive_prayer<G: Html>(prayer: ResponsivePrayer) -> HeaderAndMain<G> {
    let lines = View::new_fragment(
        prayer
            .into_vec()
            .into_iter()
            .enumerate()
            .map(|(n, line)| {
                if n % 2 == 1 {
                    view! {
                      span {
                        strong(class = "response") { (line) }
                        br
                      }
                    }
                } else {
                    view! {
                      span {
                        (line)
                        br
                      }
                    }
                }
            })
            .collect(),
    );

    let main = view! {
      main(class = "responsive-prayer") {
        p {
          (lines)
        }
      }
    };

    (None, main)
}

pub fn rubric<G: Html>(rubric: Rubric) -> HeaderAndMain<G> {
    (None, {
        view! {
          p(class="rubric") {
            (rubric)
          }
        }
    })
}

pub fn sentence<G: Html>(sentence: Sentence) -> HeaderAndMain<G> {
    let citation = sentence.citation;
    let response = sentence.response;

    let short_text_response = response.clone().and_then(|doc| match doc.content {
        Content::Text(text) => {
            if text.text.len() <= 5 && text.response.is_none() {
                Some(text)
            } else {
                None
            }
        }
        _ => None,
    });

    let citation = match citation {
        Some(citation) => view! { span( class="citation") { (citation) } },
        None => view! {},
    };

    let text = sentence.text;

    let body = match (response, short_text_response) {
        // No response
        (None, _) => view! {
          p {
            (text)
            (citation)
          }
        },
        (_, Some(response)) => view! {
          p {
            (text)
            strong(class = "response") {
              (response)
            }
            (citation)
          }
        },
        (Some(response), None) => view! {
          div {
            p {
              (text)
              (citation)
            }
            DocumentComponent(Signal::new(*response).handle())
          }
        },
    };

    let main = view! {
      main(class = "sentence") {
        (body)
      }
    };

    (None, main)
}

pub fn series<G: Html>(series: Series) -> HeaderAndMain<G> {
    (
        None,
        View::new_fragment(
            series
                .into_vec()
                .into_iter()
                .map(|doc| {
                    view! {
                      article(class="document") {
                        DocumentComponent(Signal::new(doc).handle())
                      }
                    }
                })
                .collect(),
        ),
    )
}

pub fn text<G: Html>(text: Text) -> HeaderAndMain<G> {
    let paragraphs = View::new_fragment(
        text.text
            .split("\n\n")
            .map(|s| s.to_string())
            .map(|paragraph| view! { (paragraph) })
            .collect(),
    );
    let class = format!("text {}", display_format_as_class(text.display_format));
    (
        None,
        view! {
          main(class=(class)) {
            p {
              (paragraphs)
            }
          }
        },
    )
}

fn display_format_as_class(display_format: DisplayFormat) -> &'static str {
    match display_format {
        DisplayFormat::Default => "default",
        DisplayFormat::Abbreviated => "abbreviated",
        DisplayFormat::Omit => "omit",
        DisplayFormat::Unison => "unison",
    }
}
