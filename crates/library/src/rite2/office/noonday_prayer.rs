use crate::conditions::{NOT_INSERT_GLORIA, NOT_LENT};
use crate::rite2::GLORIA_PATRI;
use liturgy::{
    Condition, Content, DisplayFormat, Document, Heading, HeadingLevel, Preces, PsalmCitation,
    Rubric, SubLiturgy, Text,
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

                // Psalms
                Document::from(Rubric::from("One or more of the following Psalms is sung or said. Other suitable selections include Psalms 19,67, one or more sections of Psalm 119, or a selection from Psalms 120 through 133.")),
                Document::from(PsalmCitation::from("Psalm 119:105-112")),
                Document::from(PsalmCitation::from("Psalm 121")),
                Document::from(PsalmCitation::from("Psalm 126")),
                Document::from(Rubric::from("At the end of the Psalms is sung or said")).condition(NOT_INSERT_GLORIA.clone()),
                Document::from(GLORIA_PATRI.clone()).condition(NOT_INSERT_GLORIA.clone()),

                // Reading
                Document::from(Rubric::from("One of the following, or some other suitable passage of Scripture, is read")),

            ]));
}
