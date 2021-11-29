use calendar::{Calendar, LiturgicalDay, LiturgicalDayId};
use lectionary::Lectionary;
use liturgy::*;
use psalter::{bcp1979::BCP1979_PSALTER, Psalter};

use serde::{Deserialize, Serialize};

#[macro_use]
extern crate lazy_static;

pub mod conditions;
pub mod rite2;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CommonPrayerLiturgies {
    NoondayPrayer,
    Compline,
}

pub trait Library {
    fn psalter(psalter: Version) -> &'static Psalter;

    fn lectionary(lectionary: Lectionaries) -> &'static Lectionary;

    fn compile(
        document: Document,
        calendar: &Calendar,
        day: &LiturgicalDay,
        observed: &LiturgicalDayId,
        prefs: &impl ClientPreferences,
    ) -> Option<Document> {
        let include = document.include(calendar, day, prefs);
        if !include {
            None
        } else {
            match &document.content {
                // TODO LectionaryReading
                // 1) look up reading from lectionary
                // 2) compile intro
                // 3) look up Biblical text
                // TODO BiblicalCitation
                // 1) compile intro
                // 2) look up Biblical text
                // Insert day/date into heading if necessary

                // Lectionaries
                Content::LectionaryReading(lectionary_reading) => {
                    let chosen_lectionary = match &lectionary_reading.lectionary {
                        LectionaryTable::Preference(key) => match prefs.value(key) {
                            PreferenceValue::Lectionary(lectionary) => lectionary,
                            _ => Lectionaries::default(),
                        },
                        LectionaryTable::Selected(lectionary) => *lectionary,
                    };
                    let lectionary = Self::lectionary(chosen_lectionary);
                    let mut docs = lectionary
                        .reading_by_type(observed, day, lectionary_reading.reading_type)
                        .map(|reading| {
                            let intro = lectionary_reading.intro.as_ref().map(|intro| {
                                BiblicalReadingIntro::from(intro.compile(&reading.citation))
                            });
                            Document::from(BiblicalCitation {
                                citation: reading.citation,
                                intro,
                            })
                        });
                    Document::choice_or_document(&mut docs)
                }

                // Headings
                Content::Heading(heading) => match heading {
                    // ordinary headings are passed through
                    Heading::Text(_, _) => Some(document),

                    // Dates are filled in with the date we're compiling for
                    Heading::Date(_) => Some(Document::from(Heading::Date(Some(day.date)))),

                    // Days need to receive the calendar as well, to allow them to look up feast names etc.
                    Heading::Day(_) => Some(Document::from(Heading::Day(Some(day.clone())))),
                },

                // Lookup types
                Content::PsalmCitation(citation) => {
                    let psalter_pref =
                        match prefs.value(&PreferenceKey::from(GlobalPref::PsalterVersion)) {
                            PreferenceValue::Version(v) => Some(v),
                            _ => None,
                        }
                        .unwrap_or_default();
                    let psalter = Self::psalter(psalter_pref);
                    let psalms: Vec<Psalm> = psalter.psalms_by_citation(citation.as_str());
                    if psalms.is_empty() {
                        None
                    } else if psalms.len() == 1 {
                        Some(Document::from(psalms[0].clone()))
                    } else {
                        Some(Document::from(Series::from(psalms)))
                    }
                }
                // Collection types
                Content::Series(sub) => Some(Document {
                    content: Content::Series(Series::from(
                        sub.iter()
                            .filter_map(|doc| {
                                Self::compile(doc.clone(), calendar, day, observed, prefs)
                            })
                            .collect::<Vec<_>>(),
                    )),
                    ..document
                }),
                Content::Parallel(sub) => Some(Document {
                    content: Content::Parallel(Parallel::from(
                        sub.iter()
                            .filter_map(|doc| {
                                Self::compile(doc.clone(), calendar, day, observed, prefs)
                            })
                            .collect::<Vec<_>>(),
                    )),
                    ..document
                }),
                Content::Choice(sub) => {
                    // try, when filtering selections, to maintain the currently-selected item -- or default to 0
                    let prev_selection = sub.options.get(sub.selected);
                    let index_of_prev_selection = prev_selection
                        .and_then(|prev| sub.options.iter().position(|search| search == prev));

                    Some(Document {
                        content: Content::Choice(Choice {
                            options: sub
                                .options
                                .iter()
                                .filter_map(|doc| {
                                    Self::compile(doc.clone(), calendar, day, observed, prefs)
                                })
                                .collect(),
                            selected: index_of_prev_selection.unwrap_or(0),
                        }),
                        ..document
                    })
                }
                // Every else just passes through as is
                _ => Some(document),
            }
        }
    }
}

pub struct CommonPrayer {}

impl Library for CommonPrayer {
    fn psalter(_psalter: Version) -> &'static Psalter {
        &BCP1979_PSALTER
    }

    fn lectionary(lectionary: Lectionaries) -> &'static Lectionary {
        match lectionary {
            Lectionaries::BCP1979DailyOffice => &lectionary::BCP1979_DAILY_OFFICE_LECTIONARY,
            Lectionaries::BCP1979DailyOfficePsalms => &lectionary::BCP1979_DAILY_OFFICE_PSALTER,
            Lectionaries::BCP1979ThirtyDayPsalms => &lectionary::BCP1979_30_DAY_PSALTER,
            Lectionaries::RCLTrack1 => &lectionary::RCL_TRACK_1,
            Lectionaries::RCLTrack2 => &lectionary::RCL_TRACK_2,
        }
    }
}
