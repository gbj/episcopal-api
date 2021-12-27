use liturgy::Version;
use perseus::{t, Html, Template};
use sycamore::prelude::{component, view, Signal, View};
use web_sys::window;

use crate::utils::time::{current_preferred_liturgy, input_date_now, DEFAULT_OFFICE_TIMES};

#[perseus::template(IndexPage)]
#[component(IndexPage<G>)]
pub fn index_page() -> View<G> {
    let date = Signal::new(input_date_now());
    let liturgy = Signal::new(current_preferred_liturgy(&DEFAULT_OFFICE_TIMES).to_string());
    let version = Signal::new(Version::RiteII); // TODO let user choose once Rite I liturgies are added

    let on_form_submit = {
        let liturgy = liturgy.clone();
        let date = date.clone();
        move |ev: web_sys::Event| {
            ev.prevent_default();
            window()
                .unwrap()
                .location()
                .set_href(&format!(
                    "/document/office/{}/{:#?}/?date={}",
                    liturgy.get(),
                    version.get(),
                    date.get()
                ))
                .unwrap();
        }
    };

    view! {
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

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page).head(head)
}

#[perseus::head]
pub fn head<G: Html>() -> View<G> {
    view! {
        title { "Common Prayer" }
    }
}
