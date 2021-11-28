use calendar::{Calendar, LiturgicalDay};
use liturgy::*;
use psalter::{bcp1979::BCP1979_PSALTER, Psalter};
use reference_parser::{BibleReference, Book};

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
                // Compile Biblical reading intros
                Content::BiblicalReading(reading) => {
                    if let BiblicalReadingIntro::Template(template) = &reading.intro {
                        let compiled = Self::compile_biblical_reading_intro(
                            *template.clone(),
                            &reading.citation,
                        );
                        Some(Document {
                            content: Content::BiblicalReading(BiblicalReading {
                                intro: BiblicalReadingIntro::Compiled(Box::new(compiled)),
                                ..reading.clone()
                            }),
                            ..document
                        })
                    } else {
                        Some(document)
                    }
                }
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

    fn compile_biblical_reading_intro(template: Document, citation: &str) -> Document {
        let citation = BibleReference::from(citation);
        let book = citation
            .ranges
            .get(0)
            .and_then(|range| range.start.book)
            .unwrap_or(Book::None);
        let short_name = book.book_short_name(template.language);
        let long_name = book.book_long_name(template.language);

        fn replace_names(base: &str, short_name: &str, long_name: &str) -> String {
            base.replace("${short_name}", short_name)
                .replace("${long_name}", long_name)
                // replace internal "The" (i.e., "A Reading from The Gospel" => "A Reading from the Gospel")
                .replace(" The", " the")
        }

        match &template.content {
            Content::Preces(content) => Document {
                content: Content::Preces(Preces::from(
                    content
                        .iter()
                        .map(|(label, text)| (label, replace_names(text, short_name, long_name))),
                )),
                ..template
            },
            Content::ResponsivePrayer(content) => Document {
                content: Content::ResponsivePrayer(ResponsivePrayer::from(
                    content
                        .iter()
                        .map(|text| replace_names(text, short_name, long_name)),
                )),
                ..template
            },
            Content::Text(content) => Document {
                content: Content::Text(Text {
                    text: replace_names(&content.text, short_name, long_name),
                    ..content.clone()
                }),
                ..template
            },
            _ => template,
        }
    }
}

pub struct CommonPrayer {}

impl Library for CommonPrayer {
    fn psalter(_psalter: Version) -> &'static Psalter {
        &BCP1979_PSALTER
    }
}

#[cfg(test)]
mod tests {
    use liturgy::{BiblicalReading, BiblicalReadingIntro, Document, Preces, Text};

    use crate::{CommonPrayer, Library};

    #[test]
    fn compile_biblical_reading_intros() {
        let intro = Document::from(Text::from("A Reading from ${short_name}."));
        let reading = BiblicalReading {
            citation: String::from("Ecclus. 1:1-14"),
            text: vec![],
            intro: BiblicalReadingIntro::Template(Box::new(intro.clone())),
        };
        assert_eq!(
            CommonPrayer::compile_biblical_reading_intro(intro, &reading.citation),
            Document::from(Text::from("A Reading from Sirach."))
        );

        let intro = Document::from(Text::from("A Reading from ${short_name}."));
        let reading = BiblicalReading {
            citation: String::from("Mark 1:1-14"),
            text: vec![],
            intro: BiblicalReadingIntro::Template(Box::new(intro.clone())),
        };
        assert_eq!(
            CommonPrayer::compile_biblical_reading_intro(intro, &reading.citation),
            Document::from(Text::from("A Reading from Mark."))
        );

        let intro = Document::from(Text::from("A Reading from ${long_name}."));
        let reading = BiblicalReading {
            citation: String::from("Mark 1:1-14"),
            text: vec![],
            intro: BiblicalReadingIntro::Template(Box::new(intro.clone())),
        };
        assert_eq!(
            CommonPrayer::compile_biblical_reading_intro(intro, &reading.citation),
            Document::from(Text::from("A Reading from the Gospel According to Mark."))
        );

        let intro = Document::from(Preces::from([
            (
                "Celebrant",
                "The Holy Gospel of our Lord Jesus Christ according to ${short_name}.",
            ),
            ("People", "Glory to you, Lord Christ."),
        ]));
        let reading = BiblicalReading {
            citation: String::from("Mark 1:1-14"),
            text: vec![],
            intro: BiblicalReadingIntro::Template(Box::new(intro.clone())),
        };
        assert_eq!(
            CommonPrayer::compile_biblical_reading_intro(intro, &reading.citation),
            Document::from(Preces::from([
                (
                    "Celebrant",
                    "The Holy Gospel of our Lord Jesus Christ according to Mark."
                ),
                ("People", "Glory to you, Lord Christ.")
            ]))
        );
    }
}
