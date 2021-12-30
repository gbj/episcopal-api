use crate::components::*;
use crate::utils::language::locale_to_language;
use calendar::lff2018::LFF_BIOS;
use calendar::{HolyDayId, LiturgicalDayId, Time, LFF2018_CALENDAR};
use lectionary::{ReadingType, LFF2018_LECTIONARY};
use liturgy::{BiblicalCitation, Document};
use perseus::RenderFnResult;
use perseus::{t, Html, RenderFnResultWithCause, Template};
use psalter::bcp1979::BCP1979_PSALTER;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize)]
struct HolyDayPageProps {
    date: String,
    locale: String,
    name: String,
    bio: String,
    readings: Vec<(ReadingType, String)>,
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("holy-day")
        .template(holy_day_page)
        .head(head)
        .build_state_fn(get_build_props)
        .build_paths_fn(get_static_paths)
}

#[perseus::head]
pub fn head<G: Html>(props: HolyDayPageProps) -> View<G> {
    let title = format!("{} – {}", props.name, t!("common_prayer"));
    view! {
        title { (title) }
        link(rel = "stylesheet", href = "/.perseus/static/document.css")
        link(rel = "stylesheet", href = "/.perseus/static/holy-day.css")
    }
}

#[perseus::autoserde(build_state)]
pub async fn get_build_props(
    path: String,
    locale: String,
) -> RenderFnResultWithCause<HolyDayPageProps> {
    let language = locale_to_language(&locale);
    let mut parts = path.split('/');
    parts.next(); // /holy-day
    let feast = parts
        .next()
        .map(|s| serde_json::from_str(&format!("\"{}\"", s)).unwrap())
        .unwrap();
    let date = LFF2018_CALENDAR
        .holy_days
        .iter()
        .find(|(_, s_feast, _, _)| *s_feast == feast)
        .map(|(id, _, _, _)| match id {
            HolyDayId::Date(month, day) => format!("{} {}", language.month_name(*month), day),
            _ => panic!("expected a month/date pair for feast"),
        })
        .expect("could not find info for feast");
    let name = LFF2018_CALENDAR
        .feast_name(feast, language)
        .unwrap()
        .to_string();
    let bio = LFF_BIOS
        .iter()
        .find(|(s_feast, _)| *s_feast == feast)
        .map(|(_, bio)| bio.to_string())
        .unwrap();
    let readings = LFF2018_LECTIONARY
        .readings
        .iter()
        .filter(|(id, _, _, _)| id == &LiturgicalDayId::Feast(feast))
        .map(|(_, _, reading_type, citation)| (*reading_type, citation.to_string()))
        .collect();
    Ok(HolyDayPageProps {
        date,
        locale,
        name,
        bio,
        readings,
    })
}

pub async fn get_static_paths() -> RenderFnResult<Vec<String>> {
    Ok(LFF2018_CALENDAR
        .holy_days
        .iter()
        .filter(|(_, _, time, _)| *time != Time::EveningOnly)
        .map(|(_, feast, _, _)| serde_json::to_string(feast).unwrap().replace('"', ""))
        .collect())
}

#[perseus::template(HolyDayPage)]
#[component(HolyDayPage<G>)]
pub fn holy_day_page(props: HolyDayPageProps) -> View<G> {
    let locale = props.locale;
    let name = props.name;
    let date = props.date;

    let bio = View::new_fragment(
        props
            .bio
            .split("\n\n")
            .map(|para| {
                let p = para.to_string();
                view! {
                  p {
                    (p)
                  }
                }
            })
            .collect(),
    );

    let (lesson_citation, lesson_doc) = filter_readings(&props.readings, ReadingType::FirstReading);
    let first_lesson = reading_view(lesson_citation, lesson_doc);

    let (psalm_citation, psalm_doc) = filter_readings(&props.readings, ReadingType::Psalm);
    let psalm = reading_view(psalm_citation, psalm_doc);

    let (gospel_citation, gospel_doc) = filter_readings(&props.readings, ReadingType::Gospel);
    let gospel = reading_view(gospel_citation, gospel_doc);

    view! {
      header {
        (cloned!((locale) => menu_component(locale)))
        p(class = "page-title") {
            (name)
        }
      }
      main {
        h1 {
          (date) ": " (name)
        }
        h2 {
            (t!("lessons_and_psalm"))
        }
        ul(class = "readings") {
            (first_lesson)
            (psalm)
            (gospel)
        }
        (bio)
      }
    }
}

fn filter_readings(
    readings: &[(ReadingType, String)],
    reading_type: ReadingType,
) -> (String, Option<Document>) {
    let filtered = readings
        .iter()
        .filter(|(s_reading_type, _)| *s_reading_type == reading_type)
        .collect::<Vec<_>>();
    let citation = filtered
        .clone()
        .iter()
        .map(|(_, citation)| citation.to_string())
        .collect::<Vec<_>>()
        .join(&format!(" {} ", t!("or")));
    let document = if reading_type == ReadingType::Psalm {
        Document::choice_or_document(
            &mut filtered
                .into_iter()
                .map(|(_, citation)| {
                    BCP1979_PSALTER
                        .psalms_by_citation(citation.as_str())
                        .iter()
                        .map(|psalm| Document::from(psalm.clone()))
                        .collect::<Vec<_>>()
                })
                .flatten(),
        )
    } else {
        Document::choice_or_document(
            &mut filtered
                .into_iter()
                .map(|(_, citation)| Document::from(BiblicalCitation::from(citation.clone()))),
        )
    };

    (citation, document)
}

fn reading_view<G: GenericNode + Html>(citation: String, document: Option<Document>) -> View<G> {
    if let Some(document) = document {
        view! {
            li {
                details {
                    summary {
                        (citation)
                    }
                    DocumentComponent(Signal::new(document).handle())
                }
            }
        }
    } else {
        view! {
            li {
                (citation)
            }
        }
    }
}
