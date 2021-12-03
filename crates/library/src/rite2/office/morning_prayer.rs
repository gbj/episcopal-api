use crate::conditions::{EASTER_SEASON, NOT_INSERT_GLORIA, NOT_LENT};
use crate::rite2::{GLORIA_PATRI, WORD_OF_THE_LORD};
use calendar::Weekday;
use lectionary::ReadingType;
use liturgy::{
    Antiphon, BiblicalReadingIntroTemplate, Choice, Condition, DisplayFormat, Document, GlobalPref,
    Heading, HeadingLevel, Lectionaries, LectionaryReading, LectionaryTable, Liturgy,
    LiturgyPreference, LiturgyPreferenceOption, Preces, PreferenceKey, PreferenceValue,
    PsalmCitation, ReadingTypeTable, Reference, Rubric, Sentence, Series, Show, Text, Version,
};

lazy_static! {
    pub static ref MORNING_PRAYER_II: Document = Document::from(
      Liturgy::from(Series::from([
          Document::from(Heading::from((HeadingLevel::Heading1, "Daily Morning Prayer: Rite Two"))),
          Document::from(Heading::Date(None)),
          Document::from(Heading::Day(None)),

          // Fore-Office -- optionally omitted with "omitForeOffice" preference
          Document::from(Series::from([
            Document::from(Rubric::from("The Officiant begins the service with one or more of these sentences of Scripture, or with the versicle “Lord, open our lips.”")).display(Show::TemplateOnly),
            // TODO lookup by category
            Document::from(Sentence::from("").citation("")).version(Version::RiteII),
            Document::from(Rubric::from("The following Confession of Sin may then be said; or the Office may continue at once with “Lord, open our lips.”")),
            Document::from(Heading::from((HeadingLevel::Heading3, "Confession of Sin"))),
            Document::from(Rubric::from("The Officiant says to the people")),
            Document::from(Choice::from([
                  Document::from(Text::from("Dearly beloved, we have come together in the presence of Almighty God our heavenly Father, to set forth his praise, to hear his holy Word, and to ask, for ourselves and on behalf of others, those things that are necessary for our life and our salvation. And so that we may prepare ourselves in heart and mind to worship him, let us kneel in silence, and with penitent and obedient hearts confess our sins, that we may obtain forgiveness by his infinite goodness and mercy. ")).version_label("Long"),
                  Document::from(Text::from("Let us confess our sins against God and our neighbor.")).version_label("Short")
                ])).source(Reference::from(79)),
            Document::from(Rubric::from("Silence may be kept.\n\nOfficiant and People together, all kneeling")),
            Document::from(Text::from("Most merciful God,\nwe confess that we have sinned against you\nin thought, word, and deed,\nby what we have done,\nand by what we have left undone.\nWe have not loved you with our whole heart;\nwe have not loved our neighbors as ourselves.\nWe are truly sorry and we humbly repent.\nFor the sake of your Son Jesus Christ,\nhave mercy on us and forgive us;\nthat we may delight in your will,\nand walk in your ways,\nto the glory of your Name.").response("Amen.").display_format(DisplayFormat::Unison)),
            Document::from(Rubric::from("The Priest alone stands and says")),
            Document::from(Text::from("Almighty God have mercy on you, forgive you all your sins through our Lord Jesus Christ, strengthen you in all goodness, and by the power of the Holy Spirit keep you in eternal life.").response("Amen.")).version_label("Priest"),
            Document::from(Rubric::from("A deacon or lay person using the preceding form remains kneeling, and substitutes “us” for “you” and “our” for “your.”")),
          ])).condition(Condition::Not(
            Box::new(Condition::Preference(PreferenceKey::from("omitForeOffice"), PreferenceValue::from("yes"))))
          ),

          // Invitatory and Psalter
          Document::from(Heading::from((HeadingLevel::Heading2, "The Invitatory and Psalter"))),
          Document::from(Rubric::from("All stand")),
          Document::from(Preces::from([
            ("Officiant", "Lord, open our lips."),
            ("People", "And our mouth shall proclaim your praise.")
          ])).source(Reference::from(80)),
          Document::from(Rubric::from("Officiant and People")),
          Document::from(GLORIA_PATRI.clone()),
          Document::from(Rubric::from("Except in Lent,")).display(Show::TemplateOnly),
          Document::from(Text::from("Alleluia.")).condition(NOT_LENT.clone()),
          Document::from(Rubric::from("may be added.")).display(Show::TemplateOnly),
          Document::from(Rubric::from("One of the following Antiphons may be sung or said with the Invitatory Psalm")).display(Show::TemplateOnly),

          // TODO: lookup by category
          Document::from(Antiphon::from(""))
            .version(Version::RiteII)
            .display(Show::TemplateOnly),
          Document::from(Rubric::from("Then follows one of the Invitatory Psalms, Venite or Jubilate.")).display(Show::TemplateOnly),
          Document::from(Rubric::from("In Easter Week, in place of an Invitatory Psalm, the Pascha Nostrum is sung or said. It may also be used daily until the Day of Pentecost."))
            .condition(EASTER_SEASON.clone())
            .display(Show::TemplateOnly),

          // TODO: insert invitatories as printed
          Document::from(Choice::from([
            // TODO Document::from(undefined).version(Version::RiteII).label("Venite").source(Reference::from(82)).condition(/* {"conditions":[{"preference":{"key":"psalmCycle","value":"30day-psalter","is":false}},{"day_of_month":{"neq":"19"}}],"mode":"or"} */),
            // TODO Document::from(undefined).version(Version::RiteII).label("Jubilate").source(Reference::from(82)),
            // TODO Document::from(undefined).version(Version::RiteII).label("Christ our Passover").version_label("Pascha Nostrum").source(Reference::from(83)).condition(/* {"mode":"and","conditions":[{"season":{"except":[],"only":["Easter","Ascension","Pentecost"]}}]} */)
          ])),

          // Psalms
          Document::from(Rubric::from("Then follows")),
          Document::from(Heading::from((HeadingLevel::Heading3, "The Psalm or Psalms Appointed"))).display(Show::TemplateOnly),
          Document::from(LectionaryReading {
            reading_type: ReadingTypeTable::Selected(ReadingType::MorningPsalm),
            lectionary: LectionaryTable::Preference(PreferenceKey::from(GlobalPref::PsalmCycle)),
            intro: None,
          }),
          Document::from(Rubric::from("At the end of the Psalms is sung or said")).condition(NOT_INSERT_GLORIA.clone()),
          Document::from(GLORIA_PATRI.clone()).condition(NOT_INSERT_GLORIA.clone()),

          // Lessons
          Document::from(Heading::from((HeadingLevel::Heading2, "The Lessons"))),
          Document::from(Rubric::from("One or two lessons, as appointed, are read, the Reader first saying")).display(Show::TemplateOnly),
          Document::from(Text::from("A Reading (Lesson) from _______________.")).display(Show::TemplateOnly),
          Document::from(Rubric::from("A citation giving chapter and verse may be added.")).display(Show::TemplateOnly),
          Document::from(Rubric::from("After each Lesson the Reader may say")).display(Show::TemplateOnly),
          Document::from(WORD_OF_THE_LORD.clone()).display(Show::TemplateOnly),
          Document::from(Rubric::from("Or the Reader may say")).display(Show::TemplateOnly),
          Document::from(Text::from("Here ends the Lesson (Reading).")).display(Show::TemplateOnly),
          // TODO a link to the daily readings page, TemplateOnly
          // {           "compile_hidden": true,           "lookup": {             "type": "lectionary"           }         }
          Document::from(Rubric::from("Silence may be kept after each Reading. One of the Canticles is sung or said after each Reading. If three Lessons are used, the Lesson from the Gospel is read after the second Canticle.")).display(Show::TemplateOnly),
          // TODO a link to the canticle table, TemplateOnly
          // {           "compile_hidden": true,           "lookup": {             "type": "canticle"           }         }

          // First Lesson
          Document::from(LectionaryReading {
            reading_type: ReadingTypeTable::Preference(PreferenceKey::from(GlobalPref::ReadingA)),
            lectionary: LectionaryTable::Preference(PreferenceKey::from(GlobalPref::Lectionary)),
            intro: Some(BiblicalReadingIntroTemplate::from(Text::from("A Reading from ${longName}."))),
          }).label("The First Lesson"),
          Document::from(WORD_OF_THE_LORD.clone()),

          // Second Lesson
          Document::from(Series::from([
            Document::from(LectionaryReading {
              reading_type: ReadingTypeTable::Preference(PreferenceKey::from(GlobalPref::ReadingB)),
              lectionary: LectionaryTable::Preference(PreferenceKey::from(GlobalPref::Lectionary)),
              intro: Some(BiblicalReadingIntroTemplate::from(Text::from("A Reading from ${longName}."))),
            }).label("The Second Lesson"),
            Document::from(WORD_OF_THE_LORD.clone()),
          ])).condition(  // include lesson and response unless the reading preference is set to "—"
            Condition::Not(Box::new(
              Condition::Preference(
                PreferenceKey::from(GlobalPref::ReadingB),
                PreferenceValue::from(ReadingType::Empty))
              )
            )),

          // Third Lesson
          Document::from(Series::from([
            Document::from(LectionaryReading {
              reading_type: ReadingTypeTable::Preference(PreferenceKey::from(GlobalPref::ReadingC)),
              lectionary: LectionaryTable::Preference(PreferenceKey::from(GlobalPref::Lectionary)),
              intro: Some(BiblicalReadingIntroTemplate::from(Text::from("A Reading from ${longName}."))),
            }).label("The Third Lesson"),
            Document::from(WORD_OF_THE_LORD.clone()),
          ])).condition( // include lesson and response unless the reading preference is set to "—"
            Condition::Not(Box::new(
              Condition::Preference(
                PreferenceKey::from(GlobalPref::ReadingC),
                PreferenceValue::from(ReadingType::Empty))
              )
            ))
      ]))
      .preferences([
        // Translations
        LiturgyPreference::from((
          PreferenceKey::from(GlobalPref::BibleVersion),
          "Bible Version",
          [
            LiturgyPreferenceOption::from(Version::NRSV),
            LiturgyPreferenceOption::from(Version::CEB),
            LiturgyPreferenceOption::from(Version::ESV),
            LiturgyPreferenceOption::from(Version::KJV)
          ]
        )).default_value(PreferenceValue::from(ReadingType::FirstReading)),

        // Lectionaries
        LiturgyPreference::from((
          PreferenceKey::from(GlobalPref::PsalmCycle),
          "Psalm Cycle",
          [
            LiturgyPreferenceOption::from(("Daily Office Lectionary", PreferenceValue::from(Lectionaries::BCP1979DailyOfficePsalms))),
            LiturgyPreferenceOption::from(("30-day Cycle", PreferenceValue::from(Lectionaries::BCP1979ThirtyDayPsalms))),
            LiturgyPreferenceOption::from(("RCL (Track 1)", PreferenceValue::from(Lectionaries::RCLTrack1))),
            LiturgyPreferenceOption::from(("RCL (Track 2)", PreferenceValue::from(Lectionaries::RCLTrack2)))
          ]
        )).default_value(PreferenceValue::from(ReadingType::FirstReading)),

        LiturgyPreference::from((
          PreferenceKey::from(GlobalPref::PsalmCycle),
          "Lectionary",
          [
            LiturgyPreferenceOption::from(("Daily Office Lectionary", PreferenceValue::from(Lectionaries::BCP1979DailyOffice))),
            LiturgyPreferenceOption::from(("RCL (Track 1)", PreferenceValue::from(Lectionaries::RCLTrack1))),
            LiturgyPreferenceOption::from(("RCL (Track 2)", PreferenceValue::from(Lectionaries::RCLTrack2)))
          ]
        )).default_value(PreferenceValue::from(ReadingType::FirstReading)),

        // Readings
        LiturgyPreference::from((
          PreferenceKey::from(GlobalPref::ReadingA),
          "First Lesson",
          [
            LiturgyPreferenceOption::from(("First Lesson", PreferenceValue::from(ReadingType::FirstReading))),
            LiturgyPreferenceOption::from(("Second Lesson", PreferenceValue::from(ReadingType::SecondReading))),
            LiturgyPreferenceOption::from(("Gospel", PreferenceValue::from(ReadingType::Gospel)))
          ]
        )).default_value(PreferenceValue::from(ReadingType::FirstReading)),
        LiturgyPreference::from((
          PreferenceKey::from(GlobalPref::ReadingB),
          "Second Lesson",
          [
            LiturgyPreferenceOption::from(("—", PreferenceValue::from(ReadingType::Empty))),
            LiturgyPreferenceOption::from(("First Lesson", PreferenceValue::from(ReadingType::FirstReading))),
            LiturgyPreferenceOption::from(("Second Lesson", PreferenceValue::from(ReadingType::SecondReading))),
            LiturgyPreferenceOption::from(("Gospel", PreferenceValue::from(ReadingType::Gospel)))
          ]
        )).default_value(PreferenceValue::from(ReadingType::SecondReading)),
        LiturgyPreference::from((
          PreferenceKey::from(GlobalPref::ReadingC),
          "Third Lesson",
          [
            LiturgyPreferenceOption::from(("—", PreferenceValue::from(ReadingType::Empty))),
            LiturgyPreferenceOption::from(("First Lesson", PreferenceValue::from(ReadingType::FirstReading))),
            LiturgyPreferenceOption::from(("Second Lesson", PreferenceValue::from(ReadingType::SecondReading))),
            LiturgyPreferenceOption::from(("Gospel", PreferenceValue::from(ReadingType::Gospel)))
          ]
        )).default_value(PreferenceValue::from(ReadingType::Empty))
      ])
    )
    .version(Version::RiteII)
    .label("Morning Prayer");
}
