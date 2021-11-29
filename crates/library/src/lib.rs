use calendar::{Calendar, LiturgicalDay};
use liturgy::*;
use psalter::{bcp1979::BCP1979_PSALTER, Psalter};

#[macro_use]
extern crate lazy_static;

pub mod conditions;
pub mod rite2;
pub trait Library {
    fn psalter(psalter: Version) -> &'static Psalter;

    fn compile(
        document: Document,
        calendar: &Calendar,
        day: &LiturgicalDay,
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
                            .filter_map(|doc| Self::compile(doc.clone(), calendar, day, prefs))
                            .collect::<Vec<_>>(),
                    )),
                    ..document
                }),
                Content::Parallel(sub) => Some(Document {
                    content: Content::Parallel(
                        sub.iter()
                            .filter_map(|doc| Self::compile(doc.clone(), calendar, day, prefs))
                            .collect::<Vec<_>>(),
                    ),
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
                                .filter_map(|doc| Self::compile(doc.clone(), calendar, day, prefs))
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
}
