use crate::rite2::GLORIA_PATRI;
use calendar::Season;
use liturgy::{
    Condition, Content, DisplayFormat, Document, Heading, HeadingLevel, Preces, Rubric, SubLiturgy,
    Text,
};

lazy_static! {
    pub static ref NOONDAY_PRAYER: Document =
        Document::new()
            .label("Noonday Prayer")
            .content(Content::Series(vec![
                // Include the title, date, and day in any case
                Document::from(Heading::from((HeadingLevel::Heading1, "Noonday Prayer"))),
                Document::from(Heading::Date),
                Document::from(Heading::Day),
                // If the Angelus is included, add the Angelus and then a heading for Noonday Prayer proper
                Document::new()
                    .content(Content::SubLiturgy(SubLiturgy::Angelus))
                    .condition(Condition::Preference(
                        String::from("angelus"),
                        String::from("before")
                    )),
                Document::from(Heading::from((HeadingLevel::Heading1, "Noonday Prayer")))
                    .condition(Condition::Preference(
                        String::from("angelus"),
                        String::from("before")
                    )),
                // Opening of Noonday Prayer proper
                Document::from(Preces::from([
                    ("Officiant", "O God, make speed to save us."),
                    ("People", "O Lord, make haste to help us.")
                ])),
                Document::from(Rubric::from("Officiant and People")),
                // TODO: add display format
                GLORIA_PATRI.clone(),
                Document::from(Rubric::from("Except in Lent, add")),
                Document::from(Text::from("Alleluia."))
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
