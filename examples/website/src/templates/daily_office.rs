use crate::components::*;
use liturgy::Version;
use perseus::{t, Html, RenderFnResultWithCause, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::{cloned, component, view, Signal, View};
use web_sys::window;

use crate::utils::time::{current_preferred_liturgy, input_date_now, DEFAULT_OFFICE_TIMES};

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("daily-office")
        .template(daily_office_page)
        .build_state_fn(get_build_props)
        .head(head)
}

#[perseus::autoserde(build_state)]
pub async fn get_build_props(
    _path: String,
    locale: String,
) -> RenderFnResultWithCause<DailyOfficePageProps> {
    Ok(DailyOfficePageProps { locale })
}

#[perseus::head]
pub fn head<G: Html>() -> View<G> {
    let title = format!("{} – {}", t!("daily_office"), t!("common_prayer"));
    view! {
        title { (title) }
    }
}

#[derive(Deserialize, Serialize)]
struct DailyOfficePageProps {
    locale: String,
}

#[perseus::template(DailyOfficePage)]
#[component(DailyOfficePage<G>)]
pub fn daily_office_page(props: DailyOfficePageProps) -> View<G> {
    let date = Signal::new(input_date_now());
    let liturgy = Signal::new(current_preferred_liturgy(&DEFAULT_OFFICE_TIMES).to_string());
    let version = Signal::new(Version::RiteII); // TODO let user choose once Rite I liturgies are added
    let locale = props.locale;

    let on_form_submit = {
        let liturgy = liturgy.clone();
        let date = date.clone();
        let locale = locale.clone();
        move |ev: web_sys::Event| {
            ev.prevent_default();
            window()
                .unwrap()
                .location()
                .set_href(&format!(
                    "/{locale}/document/office/{}/{:#?}/?date={}",
                    liturgy.get(),
                    version.get(),
                    date.get()
                ))
                .unwrap();
        }
    };

    view! {
      header {
        (cloned!((locale) => menu_component(locale)))
        p(class = "page-title") {
            (t!("daily_office"))
        }
      }
      main {
        form(on:submit=on_form_submit) {
          fieldset {
            label(for = "date") { (t!("date")) }
            input(id = "date", name = "date", type = "date", value = (date.get()))
          }
          fieldset {
            label(for = "liturgy") { (t!("liturgy")) }
            select(
              id = "liturgy",
              name = "liturgy",
              bind:value=liturgy
            ) {
              option(value="morning-prayer") { (t!("morning_prayer")) }
              option(value="noonday-prayer") { (t!("noonday_prayer")) }
              option(value="evening-prayer") { (t!("evening_prayer")) }
              option(value="compline") { (t!("compline")) }
            }
          }
          button(type = "submit") { (t!("pray")) }
        }
      }
    }
}
