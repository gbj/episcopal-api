use calendar::{Date, LiturgicalDayId};
use perseus::{t, Html, RenderFnResultWithCause, Template};
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

#[derive(Serialize, Deserialize)]
pub struct DailyReadingsPageProps {
    locale: String,
}

#[perseus::template(DailyReadingsPage)]
#[component(DailyReadingsPage<G>)]
pub fn daily_readings_page(props: DailyReadingsPageProps) -> View<G> {
    let locale = props.locale;

    let state = Signal::new(State::Loading);

    // Date input
    let date_str = Signal::new(input_date_now());
    let date = create_memo(
        cloned!((date_str) => move || Date::parse_from_str(&*date_str.get(), "%Y-%m-%d")),
    );

    // Choose between primary and alternate observance, if applicable
    let use_alternate_if_available = Signal::new(false);
    let use_alternate = create_memo(
        cloned!((state, use_alternate_if_available) => move || match &*state.get() {
            State::Success(state) => matches!((state.day.alternate, *use_alternate_if_available.get()), (Some(_), true)),
            _ => false
        }),
    );

    let observance = create_memo(
        cloned!((state, use_alternate_if_available) => move || match &*state.get() {
            State::Success(state) => match (state.day.alternate, *use_alternate_if_available.get()) {
                (Some(alternate), true) => Some(alternate),
                _ => Some(state.day.observed)
            },
            _ => None
        }),
    );
    let observance_name = create_memo(
        cloned!((state, use_alternate_if_available) => move || match &*state.get() {
            State::Success(state) => match (state.localized_day_names.alternate.as_ref(), *use_alternate_if_available.get()) {
                (Some(alternate), true) => Some(alternate.to_owned()),
                _ => Some(state.localized_day_names.observed.clone())
            },
            _ => None
        }),
    );

    // Localized versions of day names
    let day_names = create_memo(
        cloned!((state, observance, observance_name, locale) => move || match &*state.get() {
            State::Success(state) => {
                let is_transferred = matches!(*observance.get(), Some(LiturgicalDayId::TransferredFeast(_)));
                let observance_name = (*observance_name.get()).as_ref().cloned().unwrap_or_default();

                let holy_days = state.localized_day_names.holy_days.clone();
                let holy_days = if holy_days.is_empty() {
                    view! {}
                } else {
                    let holy_days = View::new_fragment(
                        holy_days.into_iter()
                            .map(|(feast, name)| {
                                let href = format!("{}/holy-day/{:#?}", &locale, feast);
                                view! {
                                    li {
                                        a(href=(href)) {
                                            (name)
                                        }
                                    }
                                }
                            }
                        )
                        .collect::<Vec<_>>()
                    );
                    view! {
                        ul(class = "holy-days") {
                            (holy_days)
                        }
                    }
                };

                view! {
                    h2(class = "day-name") {
                        (observance_name)
                        (if is_transferred {
                            format!(" ({})", t!("transferred"))
                        } else {
                            "".into()
                        })
                    }
                    (holy_days)
                }
            },
            _ => view! {}
        }),
    );

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
        cloned!((state, date_str, psalm_cycle, use_alternate, evening, day_names) => move || match (*state.get()).clone() {
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
                let use_alternate = *use_alternate.get();

                let psalms = match (*psalm_cycle.get(), use_alternate) {
                    (ChosenPsalmCycle::Daily, false) => state.daily_office_psalms.daily_office_lectionary.observed,
                    (ChosenPsalmCycle::Daily, true) => state.daily_office_psalms.daily_office_lectionary.alternate.unwrap_or(state.daily_office_psalms.daily_office_lectionary.observed),
                    (ChosenPsalmCycle::Thirty, _) => state.daily_office_psalms.thirty_day
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

                let readings = if use_alternate {
                    state.daily_office_readings.alternate.unwrap_or(state.daily_office_readings.observed)
                } else {
                    state.daily_office_readings.observed
                };
                let evening = *evening.get();

                let readings = View::new_fragment(
                    readings.iter()
                        .map(|reading| view! {
                            BiblicalCitationComponent(BiblicalCitation::from(reading.citation.clone()))
                        })
                        .collect()
                    );

                let day_names = &*day_names.get();

                view! {
                    section {
                        (day_names)
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

    let observance_chooser = create_memo(cloned!((state) => move || match (*state.get()).clone() {
        State::Success(summary) => match summary.localized_day_names.alternate {
            Some(alternate_name) => {
                let observed_name = summary.localized_day_names.observed;
                let alternate_check_1 = use_alternate_if_available.clone();
                let alternate_check_2 = use_alternate_if_available.clone();
                view! {
                    fieldset {
                        label(for = "observed") {
                            (observed_name)
                        }
                        input(
                            type = "radio",
                            id = "observed",
                            name = "observance",
                            value = "observed",
                            checked = !*alternate_check_1.get(),
                            on:change=cloned!((use_alternate_if_available) => move |ev: web_sys::Event| use_alternate_if_available.set(value(ev) == "alternate"))
                        )
                        label(for = "alternate") {
                            (alternate_name)
                        }
                        input(
                            type = "radio",
                            id = "alternate",
                            name = "observance",
                            value = "alternate",
                            checked = *alternate_check_2.get(),
                            on:change=cloned!((use_alternate_if_available) => move |ev: web_sys::Event| use_alternate_if_available.set(value(ev) == "alternate"))
                        )
                    }
                }
            }
            _ => view! {},
        },
        _ => view! {},
    }));

    view! {
        main {
            input(type="date", bind:value=date_str)
            h1 {
                (t!("daily_readings"))
            }

            // Select observance, if relevant
            (*observance_chooser.get())

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
        .build_state_fn(get_build_props)
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

#[perseus::autoserde(build_state)]
pub async fn get_build_props(
    _path: String,
    locale: String,
) -> RenderFnResultWithCause<DailyReadingsPageProps> {
    Ok(DailyReadingsPageProps { locale })
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
