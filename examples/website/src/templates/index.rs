use crate::components::menu_component;
use perseus::{t, Html, RenderFnResultWithCause, Template};
use serde::{Deserialize, Serialize};
use sycamore::context::*;
use sycamore::prelude::*;

#[derive(Serialize, Deserialize)]
struct IndexPageProps {
    locale: String,
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .template(index_page)
        .head(head)
        .build_state_fn(get_build_props)
}

#[perseus::head]
pub fn head<G: Html>() -> View<G> {
    view! {
        title { (t!("common_prayer")) }
    }
}

#[perseus::autoserde(build_state)]
pub async fn get_build_props(
    _path: String,
    locale: String,
) -> RenderFnResultWithCause<IndexPageProps> {
    Ok(IndexPageProps { locale })
}

#[derive(Clone)]
struct LocaleContext(String);

#[perseus::template(IndexPage)]
#[component(IndexPage<G>)]
pub fn index_page(props: IndexPageProps) -> View<G> {
    let locale = props.locale;
    let value = LocaleContext(locale.clone());

    view! {
      ContextProvider(ContextProviderProps {
        value,
        children: || view! {
          header {
              (cloned!((locale) => menu_component(locale)))
              p(class = "page-title") {
                  (t!("common_prayer"))
              }
          }
          main {
            h1 {
              (t!("table_of_contents"))
            }
            ul(class = "toc-menu") {
              li {
                (t!("calendar_full"))
                ul {
                  li {
                    (make_link("calendar/bcp", "bcp_1979", true))
                  }
                  li {
                    (make_link("calendar/lff", "lff_2018", true))
                  }
                  li {
                    (make_link("daily-readings", "daily_readings", false))
                  }
                }
              }
              li {
                (make_link("daily-office", "daily_office", false))
                ul {
                  li {
                    (t!("morning_prayer"))
                    ul {
                      li {
                        (make_link("document/office/morning-prayer/RiteI", "rite_i", true))
                      }
                      li {
                        (make_link("document/office/morning-prayer/RiteII", "rite_ii", false))
                      }
                    }
                  }
                  li {
                    (make_link("document/office/noonday-prayer/RiteII", "noonday_prayer", false))
                  }
                  li {
                    (t!("evening_prayer"))
                    ul {
                      li {
                        (make_link("document/office/evening-prayer/RiteI", "rite_i", true))
                      }
                      li {
                        (make_link("document/office/evening-prayer/RiteII", "rite_ii", true))
                      }
                    }
                  }
                  li {
                    (make_link("document/office/compline/RiteII", "compline", false))
                  }
                  li {
                    (make_link("canticle-table", "canticle_table", false))
                  }
                }
              }
              li {
                (make_link("document/litany", "great_litany", true))
              }
              li {
                (make_link("collects", "collects", true))
                ul {
                  li {
                    (make_link("collects/RiteI", "traditional", false))
                  }
                  li {
                    (make_link("collects/RiteII", "contemporary", false))
                  }
                }
              }
              li {
                (make_link("document/proper-liturgies", "proper_liturgies", true))
              }
              li {
                (make_link("document/baptism", "holy_baptism", true))
              }
              li {
                (make_link("document/eucharist", "holy_eucharist", true))
              }
              li {
                (t!("pastoral_offices"))
                ul {
                  li {
                    (make_link("document/marriage", "marriage", true))
                  }
                }
              }
              li {
                (make_link("document/episcopal-services", "episcopal_services", true))
              }
              li {
                (make_link("psalter", "psalter_full", true))
              }
              li {
                (make_link("prayers-and-thanksgivings", "prayers_and_thanksgivings", true))
              }
              li {
                (make_link("catechism", "catechism_full", true))
              }
              li {
                (make_link("historical-documents", "historical_documents", true))
              }
            }
          }
        }
      })
    }
}

fn make_link<G: GenericNode>(
    path: &'static str,
    label_i18n_slug: &'static str,
    work_in_progress: bool,
) -> View<G> {
    let locale = use_context::<LocaleContext>();

    if work_in_progress {
        view! {
          (t!(label_i18n_slug))
          " "
          em {
            (t!("work_in_progress"))
          }
        }
    } else {
        view! {
          a(href = (format!("/{}/{}", locale.0, path))) {
            (t!(label_i18n_slug))
          }
        }
    }
}
