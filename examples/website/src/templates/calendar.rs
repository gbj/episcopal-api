use crate::{
    components::{date::date_picker, menu_component, Toggle},
    utils::{dom::*, language::locale_to_language, time::today},
};
use calendar::{
    feasts::KalendarEntry, Calendar, Date, Feast, HolyDayId, Rank, Time, BCP1979_CALENDAR,
    LFF2018_CALENDAR,
};
use language::Language;
use perseus::{t, Html, RenderFnResult, RenderFnResultWithCause, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use wasm_bindgen::UnwrapThrowExt;

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

const MONTHS: [(u8, u8); 12] = [
    (1, 31),
    (2, 28),
    (3, 31),
    (4, 30),
    (5, 31),
    (6, 30),
    (7, 31),
    (8, 31),
    (9, 30),
    (10, 31),
    (11, 30),
    (12, 31),
];

#[perseus::template(CalendarPage)]
#[component(CalendarPage<G>)]
pub fn calendar_page(props: CalendarPageProps) -> View<G> {
    let locale = props.locale;

    // Render BCP and LFF calendars and choose between them
    let bcp = calendar_view(CalendarChoice::BCP1979, &locale, &props.bcp1979);
    let lff = calendar_view(CalendarChoice::LFF2018, &locale, &props.lff2018);

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
        let lff_selected = lff_selected.clone();
        move || {
            if *lff_selected.get() {
                "calendar-listing"
            } else {
                "calendar-listing hidden"
            }
        }
    });

    // Scroll to the current day on loading
    let initial_date = location_hash()
        .and_then(|hash| {
            let year = today().year();
            Date::parse_from_str(&format!("{}-{}", year, hash), "%Y-%m-%d").ok()
        })
        .unwrap_or_else(today);
    let (date, date_picker) = date_picker("date", t!("date"), initial_date);
    let current_day = create_memo(move || {
        let date = (*date.get()).unwrap_or_else(today);
        format!("{}-{}", date.month(), date.day())
    });
    create_effect(move || {
        let mmdd = &*current_day.get();
        if !mmdd.is_empty() {
            let choice = if *lff_selected.get() {
                CalendarChoice::LFF2018
            } else {
                CalendarChoice::BCP1979
            };
            let root_id = root_id(choice);
            let el = get_element_by_id(&format!("{}-{}", root_id, *current_day.get()));
            if let Some(el) = el {
                // scroll into view, with some padding at the top for comfort
                let y = el.get_bounding_client_rect().y();
                window()
                    .unwrap_throw()
                    .scroll_to_with_x_and_y(0.0, y - 75.0);
            }
        }
    });

    // Main view
    view! {
      header {
        (cloned!((locale) => menu_component(locale)))
        p(class = "page-title") {
            (t!("calendar"))
        }
      }
      main {
        section(class = "calendar-choice-toggle") {
          (toggle_view)
        }
        (date_picker)
        section(class = (*bcp_class.get())) {
          (bcp)
        }
        section(class = (*lff_class.get())) {
          (lff)
        }
      }
    }
}

fn root_id(calendar: CalendarChoice) -> &'static str {
    match calendar {
        CalendarChoice::BCP1979 => "bcp",
        CalendarChoice::LFF2018 => "lff",
    }
}

fn calendar_view<G: GenericNode>(
    calendar: CalendarChoice,
    locale: &str,
    listing: &CalendarListing,
) -> View<G> {
    let language = locale_to_language(locale);
    let root_id = root_id(calendar);

    View::new_fragment(
        MONTHS
            .iter()
            .map(move |(month, days)| {
                let name = language.month_name(*month);
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
                            let id = format!("{}-{}-{}", root_id, month, day_of_month);
                            view! {
                              tr(id = id) {
                                td { (day_of_month) }
                                td { (link) }
                              }
                            }
                        })
                        .collect(),
                );

                let id = format!("{}-{}", root_id, month);

                view! {
                  h2 {
                    (name)
                  }
                  table(id = id) {
                    (rows)
                  }
                }
            })
            .collect(),
    )
}
