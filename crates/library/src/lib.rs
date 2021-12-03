use calendar::{Calendar, LiturgicalDay, LiturgicalDayId};
use lectionary::{Lectionary, ReadingType};
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
        liturgy_prefs: &LiturgyPreferences,
    ) -> Option<Document> {
        let preference_value_for_key = |key: &PreferenceKey| {
            prefs
                .value(key)
                .or_else(|| liturgy_prefs.default_value_for_key(key))
        };

        let include = document.include(calendar, day, prefs, liturgy_prefs)
            && document.display != Show::TemplateOnly
            && document.display != Show::Hidden;
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
                        LectionaryTableChoice::Preference(key) => {
                            match preference_value_for_key(key) {
                                Some(PreferenceValue::Lectionary(lectionary)) => *lectionary,
                                _ => Lectionaries::default(),
                            }
                        }
                        LectionaryTableChoice::Selected(lectionary) => *lectionary,
                    };

                    let reading_type = match &lectionary_reading.reading_type {
                        ReadingTypeTable::Preference(key) => match preference_value_for_key(key) {
                            Some(PreferenceValue::ReadingType(reading_type)) => Some(*reading_type),
                            _ => None,
                        },
                        ReadingTypeTable::Selected(reading_type) => Some(*reading_type),
                    };

                    let lectionary = Self::lectionary(chosen_lectionary);
                    if let Some(reading_type) = reading_type {
                        let mut docs = lectionary.reading_by_type(observed, day, reading_type).map(
                            |reading| {
                                if reading_type.is_psalm() {
                                    Self::compile(
                                        Document::from(PsalmCitation::from(reading.citation)),
                                        calendar,
                                        day,
                                        observed,
                                        prefs,
                                        liturgy_prefs,
                                    )
                                    .unwrap()
                                } else {
                                    let intro = lectionary_reading.intro.as_ref().map(|intro| {
                                        BiblicalReadingIntro::from(intro.compile(&reading.citation))
                                    });
                                    Document {
                                        content: Content::BiblicalCitation(BiblicalCitation {
                                            citation: reading.citation,
                                            intro,
                                        }),
                                        ..document.clone()
                                    }
                                }
                            },
                        );

                        // MorningPsalm and EveningPsalm are the only ones that include multiple of the same reading type in sequence
                        if matches!(
                            reading_type,
                            ReadingType::MorningPsalm | ReadingType::EveningPsalm
                        ) {
                            Document::series_or_document(&mut docs)
                        } else {
                            Document::choice_or_document(&mut docs)
                        }
                    } else {
                        Some(Document::from(DocumentError::from(
                            "An invalid reading type preference was selected.",
                        )))
                    }
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
                    let psalter_pref = match preference_value_for_key(&PreferenceKey::from(
                        GlobalPref::PsalterVersion,
                    )) {
                        Some(PreferenceValue::Version(v)) => Some(*v),
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
                Content::Liturgy(liturgy) => Some(Document {
                    content: Content::Liturgy(Liturgy {
                        body: Series::from(
                            liturgy
                                .body
                                .iter()
                                .filter_map(|doc| {
                                    Self::compile(
                                        doc.clone(),
                                        calendar,
                                        day,
                                        observed,
                                        prefs,
                                        liturgy_prefs,
                                    )
                                })
                                .collect::<Vec<_>>(),
                        ),
                        evening: liturgy.evening,
                        preferences: liturgy.preferences.clone(),
                    }),
                    ..document
                }),
                Content::Series(sub) => Some(Document {
                    content: Content::Series(Series::from(
                        sub.iter()
                            .filter_map(|doc| {
                                Self::compile(
                                    doc.clone(),
                                    calendar,
                                    day,
                                    observed,
                                    prefs,
                                    liturgy_prefs,
                                )
                            })
                            .collect::<Vec<_>>(),
                    )),
                    ..document
                }),
                Content::Parallel(sub) => Some(Document {
                    content: Content::Parallel(Parallel::from(
                        sub.iter()
                            .filter_map(|doc| {
                                Self::compile(
                                    doc.clone(),
                                    calendar,
                                    day,
                                    observed,
                                    prefs,
                                    liturgy_prefs,
                                )
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
                                    Self::compile(
                                        doc.clone(),
                                        calendar,
                                        day,
                                        observed,
                                        prefs,
                                        liturgy_prefs,
                                    )
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
