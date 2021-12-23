use calendar::{Date, Feast};
use language::Language;
use lectionary::Reading;
use library::CommonPrayer;
use perseus::{t, Html, RenderFnResult, RenderFnResultWithCause, Template};
use serde::{Deserialize, Serialize};
use sycamore::{
    futures::spawn_local_in_scope,
    generic_node::GenericNode,
    prelude::{
        cloned, component, create_effect, create_memo, create_selector, view, ReadSignal, Signal,
        View,
    },
};
use web_sys::Event;

use crate::components::*;
use crate::{
    components::date::date_picker,
    utils::time::{current_hour, today},
};
use crate::{utils::input::value, API_BASE};

use api::summary::*;
use liturgy::{BiblicalCitation, Document, Psalm};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
enum State {
    Idle,
    Loading,
    Error,
    // Boxed to avoid huge size differential with other types
    Success(Box<DailySummary>),
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
    date: Option<Date>,
    summary: Option<DailySummary>,
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("daily-readings")
        .template(daily_readings_page)
        .build_state_fn(get_build_props)
        .build_paths_fn(get_static_paths)
        .incremental_generation()
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

// Incremental generation allows us to dynamically generate a page for *any* date at runtime
// and then cache it; the only static/build-time path is /daily-readings, which is the one that always
// loads from the and is not statically generated
pub async fn get_static_paths() -> RenderFnResult<Vec<String>> {
    Ok(vec!["".into()])
}

#[perseus::autoserde(build_state)]
pub async fn get_build_props(
    path: String,
    locale: String,
) -> RenderFnResultWithCause<DailyReadingsPageProps> {
    let language = Language::En; // TODO base on locale

    let date = path
        .split("daily-readings/")
        .last()
        .and_then(|date| Date::parse_from_str(date, "%Y-%m-%d").ok());
    let summary = date.map(|date| CommonPrayer::summarize_date(&date, language));

    Ok(DailyReadingsPageProps {
        locale,
        date,
        summary,
    })
}

#[perseus::template(DailyReadingsPage)]
#[component(DailyReadingsPage<G>)]
pub fn daily_readings_page(props: DailyReadingsPageProps) -> View<G> {
    let locale = props.locale;

    // State
    let state = if let Some(summary) = props.summary {
        State::Success(Box::new(summary))
    } else {
        State::Idle
    };
    let state = Signal::new(state);

    // whenever date changes, load a new DailySummary from server, if different from
    let (date, date_picker) = date_picker(
        "date",
        t!("readings_for_date"),
        props.date.unwrap_or_else(today),
    );
    create_effect({
        let state = state.clone();
        move || {
            if let Some(date) = *date.get() {
                let should_load = match &*state.get() {
                    State::Idle => true,     // in initial empty state, do loa
                    State::Loading => false, // if already loading, don't load again
                    State::Error => false, // if encountered an error, don't try to keep reloading infinitely
                    State::Success(summary) => (*summary).morning.day.date != date, // if date has changed, reload; if same, don't
                };

                if should_load {
                    spawn_local_in_scope({
                        let state = state.clone();
                        async move {
                            state.set(State::Loading);
                            match fetch_data(date).await {
                                Ok(summary) => state.set(State::Success(Box::new(summary))),
                                Err(_) => state.set(State::Error),
                            }
                        }
                    })
                }
            }
        }
    });

    // UI controls to select which data to show
    let (summary_data, controls) = controls(state.handle());

    // Rendered set of day name, psalms, and readings
    let observance_view = observance_view(state.handle(), summary_data, locale);

    view! {
        main {
            h1 {
                (t!("daily_readings"))
            }

            (date_picker)

            (controls)

            (observance_view)
        }
    }
}

// Represents all the data needed for the view, at the most efficient level of granularity,
// so that some of these fields can change without all changing
#[derive(Debug)]
struct SummaryDataSignals {
    localized_day_name: ReadSignal<String>,
    black_letter_days: ReadSignal<Vec<(Feast, String)>>,
    daily_office_readings: ReadSignal<Vec<Reading>>,
    psalms: ReadSignal<Vec<Psalm>>,
}

fn controls<G: GenericNode<EventType = Event>>(
    state: ReadSignal<State>,
) -> (SummaryDataSignals, View<G>) {
    // control to choose Morning/Evening and psalm cycle
    let (evening, time_of_day_picker) = time_of_day_picker();
    let (use_30_day_cycle, psalm_cycle_picker) = psalm_cycle_picker();

    // derived data signals
    let partial_daily_summary = create_memo({
        move || {
            let is_evening = *evening.get();
            let state = (*state.get()).clone();
            match (state, is_evening) {
                (State::Success(summary), true) => Some(summary.evening),
                (State::Success(summary), false) => Some(summary.morning.clone()),
                _ => None,
            }
        }
    });

    // control to choose Default/Alternate observance (depends on partial_daily_summary)
    let (use_alternate, observance_picker) = observance_picker(partial_daily_summary.clone());

    let observance_summary = create_selector({
        let partial_daily_summary = partial_daily_summary.clone();
        move || {
            let partial = (*partial_daily_summary.get()).clone();
            let use_alternate = *use_alternate.get();
            match (partial, use_alternate) {
                // summary not loaded yet => None
                (None, _) => None,
                // use_alternate = false => give observed day (always present)
                (Some(summary), false) => Some(summary.observed),
                // use_alternate = true => give alternate (if present), otherwise give observed day (always present)
                (Some(summary), true) => Some(
                    summary
                        .alternate
                        .clone()
                        .unwrap_or_else(|| summary.observed.clone()),
                ),
            }
        }
    });

    let localized_day_name = create_selector({
        let summary = observance_summary.clone();
        move || {
            (*summary.get())
                .as_ref()
                .map(|summary| summary.localized_name.clone())
                .unwrap_or_default()
        }
    });

    let black_letter_days = create_selector({
        let summary = observance_summary.clone();
        move || {
            (*summary.get())
                .as_ref()
                .map(|summary| summary.black_letter_days.clone())
                .unwrap_or_else(Vec::new)
        }
    });

    let daily_office_readings = create_selector({
        let summary = observance_summary.clone();
        move || {
            (*summary.get())
                .as_ref()
                .map(|summary| summary.daily_office_readings.clone())
                .unwrap_or_else(Vec::new)
        }
    });

    let thirty_day_psalms = create_selector(move || {
        (*partial_daily_summary.get())
            .as_ref()
            .map(|summary| summary.thirty_day_psalms.clone())
            .unwrap_or_else(Vec::new)
    });

    let psalms = create_selector({
        let summary = observance_summary;
        move || {
            let use_30_day_cycle = *use_30_day_cycle.get();
            (*summary.get())
                .as_ref()
                .map(|summary| {
                    if use_30_day_cycle {
                        (*thirty_day_psalms.get()).clone()
                    } else {
                        summary.daily_office_psalms.clone()
                    }
                })
                .unwrap_or_else(Vec::new)
        }
    });

    let data = SummaryDataSignals {
        localized_day_name,
        black_letter_days,
        daily_office_readings,
        psalms,
    };

    let view = view! {
        (time_of_day_picker)
        (observance_picker)
        (psalm_cycle_picker)
    };

    (data, view)
}

// output ReadSignal value is `false` for morning, `true` for evening
fn time_of_day_picker<G: GenericNode<EventType = Event>>() -> (ReadSignal<bool>, View<G>) {
    let is_evening = Signal::new(current_hour() >= 13);

    let evening_check_1 = is_evening.clone();
    let evening_check_2 = is_evening.clone();

    let view = view! {
        fieldset(class = "toggle") {
            input(
                type = "radio",
                id = "morning",
                name = "time_of_day",
                value = "morning",
                checked=!(*evening_check_1.get()),
                on:change=cloned!((is_evening) => move |ev: Event| if value(ev) == "morning" { is_evening.set(false) })
            )
            label(for = "morning") {
                (t!("morning"))
            }
            input(
                type = "radio",
                id = "evening",
                name = "time_of_day",
                value = "evening",
                checked=*evening_check_2.get(),
                on:change=cloned!((is_evening) => move |ev: Event| if value(ev) == "evening" { is_evening.set(true) })
            )
            label(for = "evening") {
                (t!("evening"))
            }
        }
    };

    (is_evening.handle(), view)
}

// output ReadSignal value is `false` for Daily Office lectionary psalms, `true` for 30-day cycle
fn psalm_cycle_picker<G: GenericNode<EventType = Event>>() -> (ReadSignal<bool>, View<G>) {
    let use_30_day_psalms = Signal::new(false);

    let check_1 = use_30_day_psalms.clone();
    let check_2 = use_30_day_psalms.clone();

    let view = view! {
        fieldset(class = "toggle") {
            input(
                type = "radio",
                id = "daily",
                name = "psalm_cycle",
                value = "daily",
                checked = !*check_1.get(),
                on:change=cloned!((use_30_day_psalms) => move |ev: Event| use_30_day_psalms.set(value(ev) == "thirty"))
            )
            label(for = "daily") {
                (t!("daily_office_psalms"))
            }
            input(
                type = "radio",
                id = "thirty",
                name = "psalm_cycle",
                value = "thirty",
                checked = *check_2.get(),
                on:change=cloned!((use_30_day_psalms) => move |ev: Event| use_30_day_psalms.set(value(ev) == "thirty"))
            )
            label(for = "thirty") {
                (t!("thirty_day_psalms"))
            }
        }
    };

    (use_30_day_psalms.handle(), view)
}

// output ReadSignal value is `false` by default (= use observed day), `true` if should use alternate observance
fn observance_picker<G: GenericNode<EventType = Event>>(
    summary: ReadSignal<Option<PartialDailySummary>>,
) -> (ReadSignal<bool>, View<G>) {
    let use_alternate_if_available = Signal::new(false);

    let alternate_check_1 = use_alternate_if_available.clone();
    let alternate_check_2 = use_alternate_if_available.clone();

    let primary_name = create_selector({
        let summary = summary.clone();
        move || {
            (*summary.get())
                .as_ref()
                .map(|summary| summary.observed.localized_name.clone())
                .unwrap_or_default()
        }
    });

    let has_alternate = create_selector({
        let summary = summary.clone();
        move || {
            (*summary.get())
                .as_ref()
                .and_then(|summary| summary.alternate.as_ref())
                .is_some()
        }
    });

    let alternate_name = create_selector({
        move || {
            (*summary.get())
                .as_ref()
                .and_then(|summary| summary.alternate.clone())
                .map(|alternate| alternate.localized_name)
                .unwrap_or_default()
        }
    });

    let alternate = use_alternate_if_available.clone();

    let view = cloned!((alternate_check_1, alternate_check_2, alternate, primary_name, alternate_name) => view! {
        (if *has_alternate.get() {
            let alternate_check_1 = alternate_check_1.clone();
            let alternate_check_2 = alternate_check_2.clone();
            let primary_name = primary_name.clone();
            let alternate_name = alternate_name.clone();

            view! {
                fieldset(class = "toggle") {
                    input(
                        type = "radio",
                        id = "observed",
                        name = "observance",
                        value = "observed",
                        checked = !(*alternate_check_1.get()),
                        on:change=cloned!((alternate) => move |ev: Event| alternate.set(value(ev) == "alternate"))
                    )
                    label(for = "observed") {
                        (*primary_name.get())
                        " "
                        (t!("default"))
                    }
                    input(
                        type = "radio",
                        id = "alternate",
                        name = "observance",
                        value = "alternate",
                        checked = *alternate_check_2.get(),
                        on:change=cloned!((alternate) => move |ev: web_sys::Event| alternate.set(value(ev) == "alternate"))
                    )
                    label(for = "alternate") {
                        (*alternate_name.get())
                        " "
                        (t!("alternate"))
                    }
                }
            }
        } else {
            view! { }
        })
    });

    (use_alternate_if_available.handle(), view)
}

fn observance_view<G: GenericNode + Html>(
    state: ReadSignal<State>,
    data: SummaryDataSignals,
    locale: String,
) -> View<G> {
    let name = data.localized_day_name;
    let black_letter_days = data.black_letter_days;
    let daily_office_readings = data.daily_office_readings;
    let psalms = data.psalms;

    let black_letter_days = create_memo(move || {
        let days = (*black_letter_days.get()).clone();
        if days.is_empty() {
            view! {}
        } else {
            let lines = View::new_fragment(
                days.into_iter()
                    .map(|(feast, name)| {
                        let url = format!("/{}/holy-day/{:#?}", locale, feast);
                        view! {
                            li {
                                a(href = (url)) {
                                    (name)
                                }
                            }
                        }
                    })
                    .collect(),
            );
            view! {
                ul(class = "black-letter-days") {
                    (lines)
                }
            }
        }
    });

    let psalms = create_memo(move || {
        let psalms = (*psalms.get()).clone();

        if psalms.is_empty() {
            view! {}
        } else {
            let psalms =  View::new_fragment(
                psalms
                    .iter()
                    .map(|psalm| {
                        view! {
                            article(class = "document") {
                                DocumentComponent(Signal::new(Document::from(psalm.clone())).handle())
                            }
                        }
                    })
                    .collect(),
            );
            view! {
                section {
                    h2 {
                        (t!("psalms"))
                    }
                    (psalms)
                }
            }
        }
    });

    let readings = create_memo(move || {
        let readings = (*daily_office_readings.get()).clone();

        if readings.is_empty() {
            view! {}
        } else {
            let readings = View::new_fragment(
                readings.iter()
                    .map(|reading| view! {
                        BiblicalCitationComponent(BiblicalCitation::from(reading.citation.clone()))
                    })
                    .collect()
            );

            view! {
                section {
                    h2 {
                        (t!("daily_office_readings"))
                    }
                    (readings)
                }
            }
        }
    });

    let status_view = create_memo(move || match *state.get() {
        State::Loading => view! {
            p(class = "loading centered") {
                (t!("loading"))
            }
        },
        State::Error => view! {
            p(class = "error centered") {
                (t!("daily_readings_error"))
            }
        },
        _ => view! {},
    });

    view! {
        h2(class = "day-name") {
            (name.get())
        }

        (*status_view.get())

        (*black_letter_days.get())
        (*psalms.get())
        (*readings.get())
    }
}

// Client-side data fetching from API, for arbitrary dates that are not the ones pre-rendered
async fn fetch_data(date: Date) -> Result<DailySummary, ()> {
    let url = format!(
        "{}/calendar/daily_summary?year={}&month={}&day={}",
        API_BASE,
        date.year(),
        date.month(),
        date.day(),
    );
    let data = reqwasm::http::Request::get(&url)
        .send()
        .await
        .map_err(|_| ())?
        .json::<DailySummary>()
        .await
        .map_err(|_| ())?;

    Ok(data)
}
