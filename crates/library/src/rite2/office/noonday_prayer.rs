use crate::conditions::{NOT_INSERT_GLORIA, NOT_LENT};
use crate::rite2::{GLORIA_PATRI, LORDS_PRAYER_ABBREV};
use liturgy::{
    Condition, Content, DisplayFormat, Document, Heading, HeadingLevel, Preces, PreferenceKey,
    PreferenceValue, PsalmCitation, ResponsivePrayer, Rubric, Sentence, Series, SubLiturgy, Text,
};

lazy_static! {
    pub static ref NOONDAY_PRAYER: Document =
        Document::new()
            .label("Noonday Prayer")
            .content(Content::Series(Series::from([
                // Include the title, date, and day in any case
                Document::from(Heading::from((HeadingLevel::Heading1, "Noonday Prayer"))),
                Document::from(Heading::Date),
                Document::from(Heading::Day),
                // If the Angelus is included, add the Angelus and then a heading for Noonday Prayer proper
                Document::from(SubLiturgy::Angelus)
                    .condition(Condition::Preference(
                        PreferenceKey::from("angelus"),
                        PreferenceValue::from("before")
                    )),
                Document::from(Heading::from((HeadingLevel::Heading1, "Noonday Prayer")))
                    .condition(Condition::Preference(
                        PreferenceKey::from("angelus"),
                        PreferenceValue::from("before")
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
                Document::from(Sentence::from("The love of God has been poured into our hearts through the Holy Spirit that has been given to us.")
                    .citation("Romans 5:5")
                    .response(Preces::from([("People", "Thanks be to God.")]))
                ),
                Document::from(Sentence::from("From the rising of the sun to its setting my Name shall be great among the nations, and in every place incense shall be offered to my Name, and a pure offering; for my Name shall be great among the nations, says the Lord of Hosts.")
                    .citation("Malachi 1:11")
                    .response(Preces::from([("People", "Thanks be to God.")]))
                ),

                // Prayers
                Document::from(Rubric::from("A meditation, silent or spoken, may follow.")),
                Document::from(Rubric::from("The Officiant then begins the Prayers")),
                Document::from(ResponsivePrayer::from([
                    "Lord, have mercy.",
                    "Christ, have mercy.",
                    "Lord, have mercy."
                ])),
                Document::from(Rubric::from("Officiant and People")),
                Document::from(LORDS_PRAYER_ABBREV.clone()),
                Document::from(Preces::from([
                    ("Officiant", "Lord, hear our prayer."),
                    ("People", "And let our cry come to you."),
                    ("Officiant", "Let us pray.")
                ])),
                Document::from(Rubric::from("The Officiant then says one of the following Collect. If desired, the Collect of the Day may be used."))
            ])));
}
