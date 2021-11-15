use liturgy::{Condition, Content, Document, Heading, SubLiturgy, Words};

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
                    ))
                // Opening of Noonday Prayer proper
                ,
                Document::new()
                    .content(Content::Preces(vec![
                      (Words::EnII(String::from("Officiant")), Words::EnII(String::from("O God, make speed to save us."))),
                      (Words::EnII(String::from("People")), Words::EnII(String::from("O Lord, make haste to help us.")))
                    ]))
            ]));
}
