use crate::components::*;
use crate::utils::language::locale_to_language;
use calendar::lff2018::LFF_BIOS;
use calendar::{Time, LFF2018_CALENDAR};
use perseus::RenderFnResult;
use perseus::{t, Html, RenderFnResultWithCause, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize)]
struct HolyDayPageProps {
    locale: String,
    name: String,
    bio: String,
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
    let name = LFF2018_CALENDAR
        .feast_name(feast, language)
        .unwrap()
        .to_string();
    let bio = LFF_BIOS
        .iter()
        .find(|(s_feast, _)| *s_feast == feast)
        .map(|(_, bio)| bio.to_string())
        .unwrap();
    Ok(HolyDayPageProps { locale, name, bio })
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
    let paragraphs = View::new_fragment(
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

    view! {
      header {
        (cloned!((locale) => menu_component(locale)))
        p(class = "page-title") {
            (name)
        }
      }
      main {
        h1 {
          (name)
        }
        (paragraphs)
      }
    }
}
