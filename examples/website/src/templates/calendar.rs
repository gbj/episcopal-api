use calendar::{
    feasts::KalendarEntry, lff2018::LFF2018_FEASTS, Calendar, Date, Feast, HolyDayId, Rank,
    BCP1979_CALENDAR,
};
use language::Language;
use perseus::{t, Html, RenderFnResultWithCause, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("calendar")
        .template(calendar_page)
        .build_state_fn(get_static_props)
        .incremental_generation()
        .head(head)
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
enum DefaultCalendar {
    BCP1979,
    LFF2018,
}

type CalendarListing = Vec<(HolyDayId, Feast, String)>;

#[derive(Serialize, Deserialize)]
pub struct CalendarPageProps {
    locale: String,
    default_calendar: DefaultCalendar,
    bcp1979: CalendarListing,
    lff2018: CalendarListing,
}

#[perseus::head]
pub fn head<G: Html>(props: CalendarPageProps) -> View<G> {
    let title = t!("calendar");
    let cp = t!("common_prayer");
    let title = format!("{title} – {cp}");

    view! {
        title { (title) }
        link(rel = "stylesheet", href="/.perseus/static/calendar.css")
    }
}

fn summarize_calendar(
    language: Language,
    calendar: &Calendar,
    holy_days: impl Iterator<Item = KalendarEntry>,
) -> CalendarListing {
    holy_days
        .filter_map(|(id, feast, _, _)| {
            if matches!(id, HolyDayId::Date(_, _)) {
                let rank = calendar.feast_day_rank(&feast);
                if rank >= Rank::OptionalObservance {
                    let name = calendar.feast_name(feast, language);
                    Some((id, feast, name.map(String::from).unwrap_or_default()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

#[perseus::autoserde(build_state)]
pub async fn get_static_props(
    path: String,
    locale: String,
) -> RenderFnResultWithCause<CalendarPageProps> {
    let language = match locale.as_str() {
        "en-US" => Language::En,
        "es-ES" => Language::Es,
        "fr-FR" => Language::Fr,
        _ => Language::En,
    };

    let default_calendar = if path.ends_with("lff2018") {
        DefaultCalendar::LFF2018
    } else {
        DefaultCalendar::BCP1979
    };
    let bcp1979 = summarize_calendar(
        language,
        &BCP1979_CALENDAR,
        BCP1979_CALENDAR.holy_days.iter().cloned(),
    );
    let lff2018 = summarize_calendar(language, &BCP1979_CALENDAR, LFF2018_FEASTS.iter().cloned());

    Ok(CalendarPageProps {
        locale,
        default_calendar,
        bcp1979,
        lff2018,
    })
}

const MONTHS: [(&str, u8, u8); 12] = [
    ("January", 1, 31),
    ("February", 2, 28),
    ("March", 3, 31),
    ("April", 4, 30),
    ("May", 5, 31),
    ("June", 6, 30),
    ("July", 7, 31),
    ("August", 8, 31),
    ("September", 9, 30),
    ("October", 10, 31),
    ("November", 11, 30),
    ("December", 12, 31),
];

#[perseus::template(CalendarPage)]
#[component(CalendarPage<G>)]
pub fn calendar_page(props: CalendarPageProps) -> View<G> {
    let locale = props.locale;
    let bcp1979 = props.bcp1979;
    let days = View::new_fragment(
        MONTHS
            .iter()
            .map(move |(name, month, days)| {
                // TODO yuck
                let bcp = bcp1979.clone();
                (1..=*days).map(move |day_of_month| {
                    let feast = bcp
                        .iter()
                        .find(|(id, _, _)| *id == HolyDayId::Date(*month, day_of_month))
                        .map(|(_, feast, name)| (feast, name));
                    (
                        month,
                        day_of_month,
                        feast.map(|(feast, name)| (*feast, name.clone())),
                    )
                })
            })
            .flatten()
            .map({
                move |(month, day, feast)| {
                    let mmdd = format!("{}/{}", month, day);
                    let link = feast
                        .map(|(feast, name)| {
                            let link = format!("/{}/holy-days/{:#?}", locale, feast);
                            view! {
                              a(href = link) {
                                (name)
                              }
                            }
                        })
                        .unwrap_or_else(|| view! {});
                    view! {
                      tr {
                        td { (mmdd) }
                        td { (link) }
                      }
                    }
                }
            })
            .collect(),
    );
    view! {
      header {

      }
      main {
        (days)
      }
    }
}
