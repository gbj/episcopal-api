use crate::rite2::GLORIA_PATRI;
use calendar::Season;
use liturgy::{Condition, Content, DisplayFormat, Document, Heading, SubLiturgy};

lazy_static! {
    pub static ref NOONDAY_PRAYER: Document =
        Document::new()
            .label("Noonday Prayer")
            .content(Content::Series(vec![
                // Include the title, date, and day in any case
                Document::new().content(Content::Heading {
                    level: Heading::Heading1,
                    text: Some(String::from("Noonday Prayer"))
                }),
                Document::new().content(Content::Heading {
                    level: Heading::Date,
                    text: None
                }),
                Document::new().content(Content::Heading {
                    level: Heading::Day,
                    text: None
                }),
                // If the Angelus is included, add the Angelus and then a heading for Noonday Prayer proper
                Document::new()
                    .content(Content::SubLiturgy(SubLiturgy::Angelus))
                    .condition(Condition::Preference(
                        String::from("angelus"),
                        String::from("before")
                    )),
                Document::new()
                    .content(Content::Heading {
                        level: Heading::Heading1,
                        text: Some(String::from("Noonday Prayer"))
                    })
                    .condition(Condition::Preference(
                        String::from("angelus"),
                        String::from("before")
                    )),
                // Opening of Noonday Prayer proper
                Document::new()
                    .content(Content::Preces(vec![
                      (String::from("Officiant"), String::from("O God, make speed to save us.")),
                      (String::from("People"), String::from("O Lord, make haste to help us."))
                    ])),
                Document::new()
                    .content(Content::Rubric(String::from("Officiant and People"))),
                GLORIA_PATRI.clone(),
                Document::new()
                    .content(Content::Rubric(String::from("Except in Lent, add"))),
                Document::new()
                    .content(Content::Text(String::from("Alleluia.")))
                    .condition(
                        Condition::Not(Box::new(
                            Condition::Or(
                                Box::new(Condition::Season(Season::Lent)),
                                Box::new(Condition::Season(Season::HolyWeek))
                            )
                        ))
                    ),
            ]));
}
