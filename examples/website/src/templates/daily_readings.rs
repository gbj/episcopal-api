use calendar::Date;
use perseus::{t, Html, Template};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use sycamore::{
    futures::spawn_local_in_scope,
    prelude::{cloned, component, create_effect, create_memo, view, Signal, View},
};

use crate::components::*;
use crate::utils::input::value;
use crate::utils::time::input_date_now;

use api::summary::*;
use liturgy::{BiblicalCitation, Document};

#[derive(Clone, Debug, Serialize, Deserialize)]
enum State {
    Loading,
    Error,
    // Boxed to avoid huge size differential with other types
    Success(Box<SummaryWithPsalms>),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum ChosenPsalmCycle {
    Thirty,
    Daily,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum Observance {
    Primary,
    Alternate,
}

#[perseus::template(DailyReadingsPage)]
#[component(DailyReadingsPage<G>)]
pub fn daily_readings_page() -> View<G> {
    let state = Signal::new(State::Loading);

    // Date input
    let date_str = Signal::new(input_date_now());
    let date = create_memo(
        cloned!((date_str) => move || Date::parse_from_str(&*date_str.get(), "%Y-%m-%d")),
    );

    // Choose between primary and alternate observance, if applicable
    let primary_observance = create_memo(cloned!((state) => move || match &*state.get() {
        State::Success(state) => Some(state.day.observed),
        _ => None
    }));
    let alternate_observance = create_memo(cloned!((state) => move || match &*state.get() {
        State::Success(state) => state.day.alternate,
        _ => None
    }));

    // TODO Load defaults for psalm cycle from stored preferences
    let psalm_cycle = Signal::new(ChosenPsalmCycle::Daily);

    // Let people toggle evening
    let evening = Signal::new(false);

    create_effect(cloned!((state, evening) => move || match *date.get() {
        Ok(date) => {
          state.set(State::Loading);
          // TODO proper version
          spawn_local_in_scope(cloned!((state, evening) => async move {
            match fetch_data(date, *evening.get()).await {
                Ok(summary) => state.set(State::Success(Box::new(summary))),
                Err(_) => {
                    state.set(State::Error)
                },
            }
          }))
        },
        Err(_) => state.set(State::Error)
    }));

    let view = create_memo(
        cloned!((state, date_str, psalm_cycle, primary_observance, alternate_observance, evening) => move || match (*state.get()).clone() {
            State::Loading => view! {
                div(class = "loading") {
                    (t!("loading"))
                }
            },
            State::Error => {
                let date_str = date_str.clone();
                view! {
                    div(class = "error") {
                        (t!("daily_readings_error", { "date": (*date_str.get()).clone() }))
                    }
                }
            },
            State::Success(state) => {
                // TODO allow alternate psalms
                let psalms = match *psalm_cycle.get() {
                    ChosenPsalmCycle::Daily => state.daily_office_psalms.daily_office_lectionary.observed,
                    ChosenPsalmCycle::Thirty => state.daily_office_psalms.thirty_day
                };

                let morning_psalms = View::new_fragment(
                    psalms.morning
                        .iter()
                        .map(|psalm| view! {
                            article(class = "document") {
                                DocumentComponent(Signal::new(Document::from(psalm.clone())).handle())
                            }
                        })
                        .collect()
                    );

                let evening_psalms = View::new_fragment(
                    psalms.evening
                    .iter()
                    .map(|psalm| view! {
                        article(class = "document") {
                            DocumentComponent(Signal::new(Document::from(psalm.clone())).handle())
                        }
                    })
                    .collect()
                );

                let readings = state.daily_office_readings;
                let primary = *primary_observance.get();
                let alternate = *alternate_observance.get();
                let evening = *evening.get();

                let readings = View::new_fragment(
                    // TODO allow choice between observed and alternate
                    readings.observed.iter()
                        .map(|reading| view! {
                            BiblicalCitationComponent(BiblicalCitation::from(reading.citation.clone()))
                        })
                        .collect()
                    );

                view! {
                    section {
                        h2 {
                            (format!("{:#?}", primary))
                            (if let Some(alternate) = alternate {
                                format!("{:#?}", alternate)
                            } else {
                                "".into()
                            })
                        }
                    }

                    section(class="psalms") {
                        h2 {
                            (t!("daily_office_psalms"))
                        }
                        div(class="slider") {
                            div(class=(if evening { "hidden" } else { "shown" })) {
                                (morning_psalms)
                            }
                            div(class=(if !evening { "hidden" } else { "shown" })) {
                                (evening_psalms)
                            }
                        }
                    }

                    section {
                        h2 {
                            (t!("daily_office_readings"))
                        }
                        (readings)
                    }
                }
            },
        }),
    );

    let cycle_check_1 = psalm_cycle.clone();
    let cycle_check_2 = psalm_cycle.clone();
    let evening_check_1 = evening.clone();
    let evening_check_2 = evening.clone();

    view! {
        main {
            input(type="date", bind:value=date_str)
            h1 {
                (t!("daily_readings"))
            }

            // Select morning/evening
            fieldset {
                label(for = "morning") {
                    (t!("morning"))
                }
                input(
                    type = "radio",
                    id = "morning",
                    name = "time_of_day",
                    value = "morning",
                    checked = !*evening_check_1.get(),
                    on:change=cloned!((evening) => move |ev: web_sys::Event| evening.set(value(ev) == "evening"))
                )
                label(for = "evening") {
                    (t!("evening"))
                }
                input(
                    type = "radio",
                    id = "evening",
                    name = "time_of_day",
                    value = "evening",
                    checked = *evening_check_2.get(),
                    on:change=cloned!((evening) => move |ev: web_sys::Event| evening.set(value(ev) == "evening"))
                )
            }

            // Select preferred psalter
            fieldset {
                label(for = "30day_psalter") {
                    (t!("thirty_day_psalms"))
                }
                input(
                    type = "radio",
                    id = "30day_psalter",
                    name = "psalm_cycle",
                    value = "30",
                    checked = *cycle_check_1.get() == ChosenPsalmCycle::Thirty,
                    on:change=cloned!((psalm_cycle) => move |_| psalm_cycle.set(ChosenPsalmCycle::Thirty))
                )
                label(for = "daily_psalter") {
                    (t!("daily_office_psalms"))
                }
                input(
                    type = "radio",
                    id = "daily_psalter",
                    name = "psalm_cycle",
                    value = "daily",
                    checked = *cycle_check_2.get() == ChosenPsalmCycle::Daily,
                    on:change=cloned!((psalm_cycle) => move |_| psalm_cycle.set(ChosenPsalmCycle::Daily))
                )
            }

            (*view.get())
        }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("daily-readings")
        .template(daily_readings_page)
        .head(head)
}

#[perseus::head]
pub fn head<G: Html>() -> View<G> {
    view! {
        title { "Daily Readings - Common Prayer" }
        link(rel = "stylesheet", href = "/.perseus/static/daily-readings.css")
        link(rel = "stylesheet", href = "/.perseus/static/document.css")
    }
}

async fn fetch_data(date: Date, evening: bool) -> Result<SummaryWithPsalms, ()> {
    let data = Request::get(&format!(
        "http://127.0.0.1:8000/calendar/day_with_psalms?year={}&month={}&day={}&evening={}",
        date.year(),
        date.month(),
        date.day(),
        evening
    ))
    .send()
    .await
    .map_err(|_| ())?
    .json::<SummaryWithPsalms>()
    .await
    .map_err(|_| ())?;
    Ok(data)
}
