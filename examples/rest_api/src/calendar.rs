use api::summary::*;
use calendar::{Date, BCP1979_CALENDAR};
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

fn summary_from_date(date: Date, evening: bool) -> Summary {
    let day = BCP1979_CALENDAR.liturgical_day(date, evening);
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
        daily_office_readings,
        daily_office_psalms,
    }
}
