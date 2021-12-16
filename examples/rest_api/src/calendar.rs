use api::summary::*;
use calendar::{Calendar, Date, LiturgicalDay, LiturgicalDayId, Weekday, BCP1979_CALENDAR};
use language::Language;
use lectionary::{
    Reading, ReadingType, BCP1979_30_DAY_PSALTER, BCP1979_DAILY_OFFICE_LECTIONARY,
    BCP1979_DAILY_OFFICE_PSALTER,
};
use liturgy::Psalm;
use psalter::{bcp1979::BCP1979_PSALTER, Psalter};
use rocket::{get, serde::json::Json};

#[get("/day?<year>&<month>&<day>&<evening>")]
pub fn day(year: u16, month: u8, day: u8, evening: bool) -> Json<Summary> {
    let date = Date::from_ymd(year, month, day);
    Json(summary_from_date(date, evening))
}

#[get("/day_with_psalms?<year>&<month>&<day>&<evening>")]
pub fn day_with_psalms(year: u16, month: u8, day: u8, evening: bool) -> Json<SummaryWithPsalms> {
    let date = Date::from_ymd(year, month, day);
    let summary = summary_from_date(date, evening);
    let psalter = &BCP1979_PSALTER; // TODO some options here

    fn read_psalms(
        psalter: &Psalter,
        citations: &[Reading],
        reading_type: ReadingType,
    ) -> Vec<Psalm> {
        citations
            .iter()
            .filter_map(|reading| {
                if reading.reading_type == reading_type {
                    Some(psalter.psalms_by_citation(&reading.citation))
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }

    let summary_with_psalms = SummaryWithPsalms {
        day: summary.day,
        localized_day_names: summary.localized_day_names,
        daily_office_readings: summary.daily_office_readings,
        daily_office_psalms: PsalmsWithPsalms {
            thirty_day: PsalmsByTime {
                morning: read_psalms(
                    psalter,
                    &summary.daily_office_psalms.thirty_day,
                    ReadingType::MorningPsalm,
                ),
                evening: read_psalms(
                    psalter,
                    &summary.daily_office_psalms.thirty_day,
                    ReadingType::EveningPsalm,
                ),
            },
            daily_office_lectionary: ReadingsWithPsalms {
                observed: PsalmsByTime {
                    morning: read_psalms(
                        psalter,
                        &summary.daily_office_psalms.daily_office_lectionary.observed,
                        ReadingType::MorningPsalm,
                    ),
                    evening: read_psalms(
                        psalter,
                        &summary.daily_office_psalms.daily_office_lectionary.observed,
                        ReadingType::EveningPsalm,
                    ),
                },
                alternate: summary
                    .daily_office_psalms
                    .daily_office_lectionary
                    .alternate
                    .map(|alternate| PsalmsByTime {
                        morning: read_psalms(psalter, &alternate, ReadingType::MorningPsalm),
                        evening: read_psalms(psalter, &alternate, ReadingType::EveningPsalm),
                    }),
            },
        },
    };
    Json(summary_with_psalms)
}

fn localize_day_names(
    day: &LiturgicalDay,
    calendar: &Calendar,
    language: &Language,
) -> LocalizedDayNames {
    let observed = localize_day_name(day, &day.observed, calendar, language);
    let alternate = day
        .alternate
        .map(|alternate| localize_day_name(day, &alternate, calendar, language));
    let holy_days = day
        .holy_days
        .iter()
        .map(|feast| {
            (
                *feast,
                calendar
                    .feast_name(*feast, *language)
                    .unwrap_or_default()
                    .to_string(),
            )
        })
        .collect();
    LocalizedDayNames {
        observed,
        alternate,
        holy_days,
    }
}

fn localize_day_name(
    day: &LiturgicalDay,
    id: &LiturgicalDayId,
    calendar: &Calendar,
    language: &Language,
) -> String {
    match id {
        LiturgicalDayId::Feast(feast) | LiturgicalDayId::TransferredFeast(feast) => {
            calendar.feast_name(*feast, *language).map(String::from)
        }
        _ => calendar.week_name(day.week, *language).map(|name| {
            if day.weekday == Weekday::Sun {
                name.to_string()
            } else {
                format!(
                    "{} {} {}",
                    language.i18n(&day.weekday.to_string()),
                    language.i18n("after"),
                    name.replace("The", "the")
                )
            }
        }),
    }
    .unwrap_or_default()
}

fn summary_from_date(date: Date, evening: bool) -> Summary {
    let day = BCP1979_CALENDAR.liturgical_day(date, evening);
    // TODO allow calendar/localization
    let localized_day_names = localize_day_names(&day, &BCP1979_CALENDAR, &Language::En);
    let daily_office_readings = Readings {
        observed: BCP1979_DAILY_OFFICE_LECTIONARY
            .readings_by_day(&day.observed, &day)
            .collect(),
        alternate: day.alternate.map(|alternate| {
            BCP1979_DAILY_OFFICE_LECTIONARY
                .readings_by_day(&alternate, &day)
                .collect()
        }),
    };
    let daily_office_psalms = Psalms {
        thirty_day: BCP1979_30_DAY_PSALTER
            .readings_by_day(&day.observed, &day)
            .collect(),
        daily_office_lectionary: Readings {
            observed: BCP1979_DAILY_OFFICE_PSALTER
                .readings_by_day(&day.observed, &day)
                .collect(),
            alternate: day.alternate.map(|alternate| {
                BCP1979_DAILY_OFFICE_PSALTER
                    .readings_by_day(&alternate, &day)
                    .collect()
            }),
        },
    };

    Summary {
        day,
        localized_day_names,
        daily_office_readings,
        daily_office_psalms,
    }
}
