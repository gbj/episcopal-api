use crate::conditions::NOT_LENT;
use crate::rite2::GLORIA_PATRI;
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
                Document::from(SubLiturgy::Angelus)
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
                Document::from(GLORIA_PATRI.clone().display_format(DisplayFormat::Unison)),
                Document::from(Rubric::from("Except in Lent, add")).condition(NOT_LENT.clone()),
                Document::from(Text::from("Alleluia.")).condition(NOT_LENT.clone()),
            ]));
}
