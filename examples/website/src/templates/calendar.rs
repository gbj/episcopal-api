use crate::{
    components::{menu_component, Toggle},
    utils::language::locale_to_language,
};
use calendar::{
    feasts::KalendarEntry, Calendar, Feast, HolyDayId, Rank, Time, BCP1979_CALENDAR,
    LFF2018_CALENDAR,
};
use language::Language;
use perseus::{t, Html, RenderFnResult, RenderFnResultWithCause, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("calendar")
        .template(calendar_page)
        .build_paths_fn(get_static_paths)
        .build_state_fn(get_static_props)
        .incremental_generation()
        .head(head)
}

pub async fn get_static_paths() -> RenderFnResult<Vec<String>> {
    Ok(vec!["".into(), "bcp1979".into(), "lff2018".into()])
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
enum CalendarChoice {
    BCP1979,
    LFF2018,
}

type CalendarListing = Vec<(HolyDayId, Feast, String)>;

#[derive(Serialize, Deserialize)]
pub struct CalendarPageProps {
    locale: String,
    default_calendar: CalendarChoice,
    bcp1979: CalendarListing,
    lff2018: CalendarListing,
}

#[perseus::head]
pub fn head<G: Html>(_props: CalendarPageProps) -> View<G> {
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
        .filter_map(|(id, feast, time, _)| {
            if matches!(id, HolyDayId::Date(_, _)) {
                let rank = calendar.feast_day_rank(&feast);
                // include black-letter and red-letter days, but not weird Daily Office lectionary days like December 29
                // and don't include the Eve of ___ days
                if (rank == Rank::OptionalObservance || rank >= Rank::HolyDay)
                    && time != Time::EveningOnly
                {
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
    let language = locale_to_language(&locale);

    let default_calendar = if path.ends_with("lff2018") {
        CalendarChoice::LFF2018
    } else {
        CalendarChoice::BCP1979
    };
    let bcp1979 = summarize_calendar(
        language,
        &BCP1979_CALENDAR,
        BCP1979_CALENDAR.holy_days.iter().cloned(),
    );
    let lff2018 = summarize_calendar(
        language,
        &LFF2018_CALENDAR,
        LFF2018_CALENDAR.holy_days.iter().cloned(),
    );

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
    let bcp = calendar_view(&locale, &props.bcp1979);
    let lff = calendar_view(&locale, &props.lff2018);

    let use_lff_toggle = Toggle::new(
        "calendar_choice".into(),
        t!("bcp_1979"),
        t!("lff_2018"),
        props.default_calendar == CalendarChoice::LFF2018,
    );
    let lff_selected = use_lff_toggle.toggled();
    let toggle_view = use_lff_toggle.view();

    let bcp_class = create_selector({
        let lff_selected = lff_selected.clone();
        move || {
            if !*lff_selected.get() {
                "calendar-listing"
            } else {
                "calendar-listing hidden"
            }
        }
    });

    let lff_class = create_selector({
        move || {
            if *lff_selected.get() {
                "calendar-listing"
            } else {
                "calendar-listing hidden"
            }
        }
    });

    view! {
      header {
        (cloned!((locale) => menu_component(locale)))
        p(class = "page-title") {
            (t!("daily_office"))
        }
      }
      main {
        section(class = "calendar-choice-toggle") {
          (toggle_view)
        }
        section(class = (*bcp_class.get())) {
          (bcp)
        }
        section(class = (*lff_class.get())) {
          (lff)
        }
      }
    }
}

fn calendar_view<G: GenericNode>(locale: &str, listing: &CalendarListing) -> View<G> {
    View::new_fragment(
        MONTHS
            .iter()
            .map(move |(name, month, days)| {
                // TODO yuck
                let bcp = listing.clone();

                let rows = View::new_fragment(
                    (1..=*days)
                        .map(|day_of_month| {
                            let feast = bcp
                                .iter()
                                .find(|(id, _, _)| *id == HolyDayId::Date(*month, day_of_month))
                                .map(|(_, feast, name)| (feast, name.clone()));
                            let link = feast
                                .and_then(|(feast, name)| {
                                    serde_json::to_string(&feast).ok().map(|feast| {
                                        let link = format!(
                                            "/{}/holy-day/{}",
                                            locale,
                                            feast.replace('"', "")
                                        );
                                        view! {
                                          a(href = link) {
                                            (name)
                                          }
                                        }
                                    })
                                })
                                .unwrap_or_else(|| view! {});
                            view! {
                              tr {
                                td { (day_of_month) }
                                td { (link) }
                              }
                            }
                        })
                        .collect(),
                );

                view! {
                  h2 {
                    (name)
                  }
                  table(id = (month)) {
                    (rows)
                  }
                }
            })
            .collect(),
    )
}
