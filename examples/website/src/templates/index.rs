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
            ul(class = "toc-menu") {
              li {
                (t!("calendar_full"))
                ul {
                  li {
                    (make_link("calendar/bcp", "bcp_1979"))
                  }
                  li {
                    (make_link("calendar/lff", "lff_2018"))
                  }
                  li {
                    (make_link("daily-readings", "daily_readings"))
                  }
                }
              }
              li {
                (make_link("daily-office", "daily_office"))
                ul {
                  li {
                    (t!("morning_prayer"))
                    ul {
                      li {
                        (make_link("document/office/morning-prayer/RiteI", "rite_i"))
                      }
                      li {
                        (make_link("document/office/morning-prayer/RiteII", "rite_ii"))
                      }
                    }
                  }
                  li {
                    (make_link("document/office/noonday-prayer/RiteII", "noonday_prayer"))
                  }
                  li {
                    (t!("evening_prayer"))
                    ul {
                      li {
                        (make_link("document/office/evening-prayer/RiteI", "rite_i"))
                      }
                      li {
                        (make_link("document/office/evening-prayer/RiteII", "rite_ii"))
                      }
                    }
                  }
                  li {
                    (make_link("document/office/compline/RiteII", "compline"))
                  }
                  li {
                    (make_link("canticle-table", "canticle_table"))
                  }
                }
              }
              li {
                (make_link("document/litany", "great_litany"))
              }
              li {
                (make_link("collects", "collects"))
                ul {
                  li {
                    (make_link("collects/RiteI", "traditional"))
                  }
                  li {
                    (make_link("collects/RiteII", "contemporary"))
                  }
                }
              }
            }
          }
        }
      })
    }
}

fn make_link<G: GenericNode>(path: &'static str, label_i18n_slug: &'static str) -> View<G> {
    let locale = use_context::<LocaleContext>();

    view! {
      a(href = (format!("/{}/{}", locale.0, path))) {
        (t!(label_i18n_slug))
      }
    }
}
