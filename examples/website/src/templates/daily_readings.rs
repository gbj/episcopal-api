use calendar::Date;
use perseus::{t, Html, Template};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use sycamore::{
    futures::spawn_local_in_scope,
    prelude::{cloned, component, create_effect, create_memo, view, Signal, View},
};

use crate::utils::time::input_date_now;

use super::api::{self, Summary};

#[derive(Debug, Serialize, Deserialize)]
enum State {
    Idle,
    Loading,
    Error,
    Success(api::Summary),
}

#[perseus::template(DailyReadingsPage)]
#[component(DailyReadingsPage<G>)]
pub fn daily_readings_page() -> View<G> {
    let date_str = Signal::new(input_date_now());
    let date = create_memo(
        cloned!((date_str) => move || Date::parse_from_str(&*date_str.get(), "%Y-%m-%d")),
    );

    let evening = Signal::new(false);

    let state = Signal::new(State::Idle);

    create_effect(cloned!((state) => move || match *date.get() {
        Ok(date) => {
          state.set(State::Loading);
          // TODO proper version
          spawn_local_in_scope(cloned!((state, evening) => async move {
            match fetch_data(date, *evening.get()).await {
                Ok(summary) => state.set(State::Success(summary)),
                Err(_) => state.set(State::Error),
            }
          }))
        },
        Err(_) => state.set(State::Error)
    }));

    view! {
        main {
            input(type="date", bind:value=date_str)
            h1 {
                (t!("daily_readings"))
            }
            pre {
                (serde_json::to_string(&*state.get()).unwrap())
            }
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
    }
}

async fn fetch_data(date: Date, evening: bool) -> Result<Summary, ()> {
    let data = Request::get(&format!(
        "http://127.0.0.1:8000/calendar/day?year={}&month={}&day={}&evening={}",
        date.year(),
        date.month(),
        date.day(),
        evening
    ))
    .send()
    .await
    .map_err(|_| ())?
    .json::<Summary>()
    .await
    .map_err(|_| ())?;
    Ok(data)
}
