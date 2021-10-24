use crate::{
    feasts::KalendarEntry, holy_day::HolyDayId, liturgical_week::Cycle, Calendar, Feast,
    LiturgicalWeek, Rank, Weekday,
};

/// [Calendar](Calendar) that calculates dates from the liturgical and sanctoral calendar
/// of the 1979 Book of Common Prayer of The Episcopal Church.
pub const BCP1979_CALENDAR: Calendar = Calendar {
    easter_cycle_begins: 7,
    christmas_cycle_begins: 4,
    has_propers: true,
    weeks: &BCP1979_WEEKS,
    holy_days: &BCP1979_FEASTS,
    holy_day_ranks: &BCP1979_HOLY_DAY_RANKS,
};

// TODO sort this for presentation
const BCP1979_WEEKS: [(Cycle, u8, LiturgicalWeek); 59] = [
    (Cycle::Easter, 24, LiturgicalWeek::Pentecost10),
    (Cycle::Easter, 25, LiturgicalWeek::Pentecost11),
    (Cycle::Easter, 26, LiturgicalWeek::Pentecost12),
    (Cycle::Easter, 27, LiturgicalWeek::Pentecost13),
    (Cycle::Easter, 28, LiturgicalWeek::Pentecost14),
    (Cycle::Easter, 29, LiturgicalWeek::Pentecost15),
    (Cycle::Easter, 30, LiturgicalWeek::Pentecost16),
    (Cycle::Easter, 31, LiturgicalWeek::Pentecost17),
    (Cycle::Easter, 32, LiturgicalWeek::Pentecost18),
    (Cycle::Easter, 33, LiturgicalWeek::Pentecost19),
    (Cycle::Epiphany, 1, LiturgicalWeek::Epiphany1),
    (Cycle::Easter, 1, LiturgicalWeek::Lent1),
    (Cycle::Christmas, 1, LiturgicalWeek::Christmas1),
    (Cycle::Easter, 34, LiturgicalWeek::Pentecost20),
    (Cycle::Easter, 35, LiturgicalWeek::Pentecost21),
    (Cycle::Easter, 36, LiturgicalWeek::Pentecost22),
    (Cycle::Easter, 37, LiturgicalWeek::Pentecost23),
    (Cycle::Easter, 38, LiturgicalWeek::Pentecost24),
    (Cycle::Easter, 39, LiturgicalWeek::Pentecost25),
    (Cycle::Easter, 40, LiturgicalWeek::Pentecost26),
    (Cycle::Easter, 41, LiturgicalWeek::Pentecost27),
    (Cycle::Easter, 8, LiturgicalWeek::Easter2),
    (Cycle::Epiphany, 2, LiturgicalWeek::Epiphany2),
    (Cycle::Easter, 2, LiturgicalWeek::Lent2),
    (Cycle::Easter, 16, LiturgicalWeek::Pentecost2),
    (Cycle::Christmas, 2, LiturgicalWeek::Christmas2),
    (Cycle::Easter, 9, LiturgicalWeek::Easter3),
    (Cycle::Epiphany, 3, LiturgicalWeek::Epiphany3),
    (Cycle::Easter, 3, LiturgicalWeek::Lent3),
    (Cycle::Easter, 17, LiturgicalWeek::Pentecost3),
    (Cycle::Easter, 10, LiturgicalWeek::Easter4),
    (Cycle::Epiphany, 4, LiturgicalWeek::Epiphany4),
    (Cycle::Easter, 4, LiturgicalWeek::Lent4),
    (Cycle::Easter, 18, LiturgicalWeek::Pentecost4),
    (Cycle::Easter, 11, LiturgicalWeek::Easter5),
    (Cycle::Epiphany, 5, LiturgicalWeek::Epiphany5),
    (Cycle::Easter, 5, LiturgicalWeek::Lent5),
    (Cycle::Easter, 19, LiturgicalWeek::Pentecost5),
    (Cycle::Easter, 12, LiturgicalWeek::Easter6),
    (Cycle::Epiphany, 6, LiturgicalWeek::Epiphany6),
    (Cycle::Easter, 20, LiturgicalWeek::Pentecost6),
    (Cycle::Easter, 13, LiturgicalWeek::Easter7),
    (Cycle::Epiphany, 7, LiturgicalWeek::Epiphany7),
    (Cycle::Easter, 21, LiturgicalWeek::Pentecost7),
    (Cycle::Epiphany, 8, LiturgicalWeek::Epiphany8),
    (Cycle::Easter, 22, LiturgicalWeek::Pentecost8),
    (Cycle::Easter, 23, LiturgicalWeek::Pentecost9),
    (Cycle::Christmas, 0, LiturgicalWeek::Christmas),
    (Cycle::Easter, 7, LiturgicalWeek::Easter),
    (Cycle::Epiphany, 0, LiturgicalWeek::Epiphany),
    (Cycle::Advent, 1, LiturgicalWeek::Advent1),
    (Cycle::Advent, 4, LiturgicalWeek::Advent4),
    (Cycle::Easter, 6, LiturgicalWeek::HolyWeek),
    (Cycle::Advent, 0, LiturgicalWeek::LastPentecost),
    (Cycle::Easter, 0, LiturgicalWeek::LastEpiphany),
    (Cycle::Easter, 14, LiturgicalWeek::Pentecost),
    (Cycle::Advent, 2, LiturgicalWeek::Advent2),
    (Cycle::Advent, 3, LiturgicalWeek::Advent3),
    (Cycle::Easter, 15, LiturgicalWeek::TrinitySunday),
];

const BCP1979_HOLY_DAY_RANKS: [(Feast, Rank); 79] = [
    (Feast::AllSaintsDay, Rank::PrincipalFeast),
    (Feast::ChristmasDay, Rank::PrincipalFeast),
    (Feast::ChristmasEve, Rank::PrincipalFeast),
    (Feast::Epiphany, Rank::PrincipalFeast),
    (Feast::HolyName, Rank::PrincipalFeast),
    (Feast::EveofPentecost, Rank::PrincipalFeast),
    (Feast::PalmSunday, Rank::PrincipalFeast),
    (Feast::Pentecost, Rank::PrincipalFeast),
    (Feast::TrinitySunday, Rank::PrincipalFeast),
    (Feast::AscensionDay, Rank::PrincipalFeast),
    (Feast::EveoftheAscension, Rank::PrincipalFeast),
    (Feast::FridayinEasterWeek, Rank::PrecedenceOverHolyDay),
    (Feast::GoodFriday, Rank::PrecedenceOverHolyDay),
    (Feast::MondayinEasterWeek, Rank::PrecedenceOverHolyDay),
    (Feast::MondayinHolyWeek, Rank::PrecedenceOverHolyDay),
    (Feast::SaturdayinEasterWeek, Rank::PrecedenceOverHolyDay),
    (Feast::HolySaturday, Rank::PrecedenceOverHolyDay),
    (Feast::ThursdayinEasterWeek, Rank::PrecedenceOverHolyDay),
    (Feast::MaundyThursday, Rank::PrecedenceOverHolyDay),
    (Feast::TuesdayinEasterWeek, Rank::PrecedenceOverHolyDay),
    (Feast::TuesdayinHolyWeek, Rank::PrecedenceOverHolyDay),
    (Feast::WednesdayinEasterWeek, Rank::PrecedenceOverHolyDay),
    (Feast::WednesdayinHolyWeek, Rank::PrecedenceOverHolyDay),
    (Feast::AllSoulsDay, Rank::HolyDay),
    (Feast::Annunciation, Rank::HolyDay),
    (Feast::ConfessionOfStPeter, Rank::HolyDay),
    (Feast::ConversionOfStPaul, Rank::HolyDay),
    (Feast::EveOfHolyName, Rank::HolyDay),
    (Feast::EveOfAllSaints, Rank::HolyDay),
    (Feast::EveOfHolyCross, Rank::HolyDay),
    (Feast::EveOfStJohnTheBaptist, Rank::HolyDay),
    (Feast::EveOfTheAnnunciation, Rank::HolyDay),
    (Feast::EveOfThePresentation, Rank::HolyDay),
    (Feast::EveOfTheTransfiguration, Rank::HolyDay),
    (Feast::EveOfTheVisitation, Rank::HolyDay),
    (Feast::FridayAfterAscension, Rank::HolyDay),
    (Feast::FridayafterAshWednesday, Rank::HolyDay),
    (Feast::HolyCross, Rank::HolyDay),
    (Feast::HolyInnocents, Rank::HolyDay),
    (Feast::IndependenceDay, Rank::HolyDay),
    (Feast::NativityOfStJohnTheBaptist, Rank::HolyDay),
    (Feast::SaturdayAfterAscension, Rank::HolyDay),
    (Feast::SaturdayafterAshWednesday, Rank::HolyDay),
    (Feast::PeterAndPaul, Rank::HolyDay),
    (Feast::PhilipAndJames, Rank::HolyDay),
    (Feast::SimonAndJude, Rank::HolyDay),
    (Feast::Andrew, Rank::HolyDay),
    (Feast::Barnabas, Rank::HolyDay),
    (Feast::Bartholomew, Rank::HolyDay),
    (Feast::James, Rank::HolyDay),
    (Feast::JamesOfJerusalem, Rank::HolyDay),
    (Feast::John, Rank::HolyDay),
    (Feast::Joseph, Rank::HolyDay),
    (Feast::Luke, Rank::HolyDay),
    (Feast::Mark, Rank::HolyDay),
    (Feast::MaryMagdalene, Rank::HolyDay),
    (Feast::Mary, Rank::HolyDay),
    (Feast::Matthew, Rank::HolyDay),
    (Feast::Matthias, Rank::HolyDay),
    (Feast::Michael, Rank::HolyDay),
    (Feast::Stephen, Rank::HolyDay),
    (Feast::Thomas, Rank::HolyDay),
    (Feast::EveofTrinitySunday, Rank::HolyDay),
    (Feast::ThanksgivingDay, Rank::HolyDay),
    (Feast::ThePresentation, Rank::HolyDay),
    (Feast::TheTransfiguration, Rank::HolyDay),
    (Feast::TheVisitation, Rank::HolyDay),
    (Feast::ThursdayafterAshWednesday, Rank::HolyDay),
    (Feast::AshWednesday, Rank::HolyDay),
    (Feast::January10, Rank::DaysOfChristmas),
    (Feast::January11, Rank::DaysOfChristmas),
    (Feast::January12, Rank::DaysOfChristmas),
    (Feast::January2, Rank::DaysOfChristmas),
    (Feast::January3, Rank::DaysOfChristmas),
    (Feast::January4, Rank::DaysOfChristmas),
    (Feast::January5, Rank::DaysOfChristmas),
    (Feast::January7, Rank::DaysOfChristmas),
    (Feast::January8, Rank::DaysOfChristmas),
    (Feast::January9, Rank::DaysOfChristmas),
];

/// Array of all observances in the BCP 1979 calendar
// TODO sort this by date for presentation
// TODO add special days
// TODO add Thanksgiving and Labor Day
pub const BCP1979_FEASTS: [KalendarEntry; 195] = [
    // Thanksgiving and Labor Day
    (
        HolyDayId::DayOfMonth {
            month: 9,
            week: 1,
            day: Weekday::Mon,
        },
        Feast::ThanksgivingDay,
        false,
    ),
    (
        HolyDayId::DayOfMonth {
            month: 11,
            week: 4,
            day: Weekday::Thu,
        },
        Feast::ThanksgivingDay,
        false,
    ),
    // Special days
    // TODO sort
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter6, Weekday::Fri),
        Feast::FridayAfterAscension,
        false,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter, Weekday::Fri),
        Feast::FridayinEasterWeek,
        false,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::HolyWeek, Weekday::Fri),
        Feast::GoodFriday,
        false,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::LastEpiphany, Weekday::Fri),
        Feast::FridayafterAshWednesday,
        false,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter, Weekday::Mon),
        Feast::MondayinEasterWeek,
        false,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::HolyWeek, Weekday::Mon),
        Feast::MondayinHolyWeek,
        false,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter6, Weekday::Sat),
        Feast::SaturdayAfterAscension,
        false,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter6, Weekday::Sat),
        Feast::EveofPentecost,
        true,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter, Weekday::Sat),
        Feast::SaturdayinEasterWeek,
        false,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::HolyWeek, Weekday::Sat),
        Feast::HolySaturday,
        false,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::LastEpiphany, Weekday::Sat),
        Feast::SaturdayafterAshWednesday,
        false,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::HolyWeek, Weekday::Sun),
        Feast::PalmSunday,
        false,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Pentecost, Weekday::Sun),
        Feast::Pentecost,
        false,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Pentecost, Weekday::Sat),
        Feast::EveofTrinitySunday,
        true,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::TrinitySunday, Weekday::Sun),
        Feast::TrinitySunday,
        false,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter6, Weekday::Thu),
        Feast::AscensionDay,
        false,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter, Weekday::Thu),
        Feast::ThursdayinEasterWeek,
        false,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::HolyWeek, Weekday::Thu),
        Feast::MaundyThursday,
        false,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::LastEpiphany, Weekday::Thu),
        Feast::ThursdayafterAshWednesday,
        false,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter, Weekday::Tue),
        Feast::TuesdayinEasterWeek,
        false,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::HolyWeek, Weekday::Tue),
        Feast::TuesdayinHolyWeek,
        false,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter6, Weekday::Wed),
        Feast::EveoftheAscension,
        true,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter, Weekday::Wed),
        Feast::WednesdayinEasterWeek,
        false,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::HolyWeek, Weekday::Wed),
        Feast::WednesdayinHolyWeek,
        false,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::LastEpiphany, Weekday::Wed),
        Feast::AshWednesday,
        false,
    ), // MM/DD feast days
    (HolyDayId::Date(2, 13), Feast::AbsalomJones, false),
    (HolyDayId::Date(1, 21), Feast::AgnesAndCeciliaOfRome, false),
    (HolyDayId::Date(8, 31), Feast::AidanOfLindisfarne, false),
    (HolyDayId::Date(6, 22), Feast::Alban, false),
    (HolyDayId::Date(5, 20), Feast::AlcuinOfYork, false),
    (HolyDayId::Date(10, 26), Feast::Alfred, false),
    (HolyDayId::Date(11, 1), Feast::AllSaintsDay, false),
    (HolyDayId::Date(11, 2), Feast::AllSoulsDay, false),
    (HolyDayId::Date(4, 19), Feast::Alphege, false),
    (HolyDayId::Date(12, 7), Feast::AmbroseOfMilan, false),
    (HolyDayId::Date(3, 25), Feast::Annunciation, false),
    (HolyDayId::Date(4, 21), Feast::AnselmOfCanterbury, false),
    (HolyDayId::Date(2, 3), Feast::Anskar, false),
    (HolyDayId::Date(1, 17), Feast::AntonyOfEgypt, false),
    (HolyDayId::Date(5, 2), Feast::AthanasiusOfAlexandria, false),
    (HolyDayId::Date(8, 28), Feast::AugustineOfHippo, false),
    (HolyDayId::Date(5, 26), Feast::AugustineOfCanterbury, false),
    (HolyDayId::Date(6, 14), Feast::BasilOfCaesarea, false),
    (HolyDayId::Date(5, 25), Feast::Bede, false),
    (HolyDayId::Date(7, 11), Feast::BenedictOfNursia, false),
    (HolyDayId::Date(8, 20), Feast::BernardOfClairvaux, false),
    (HolyDayId::Date(6, 18), Feast::BernardMizeki, false),
    (HolyDayId::Date(6, 5), Feast::Boniface, false),
    (HolyDayId::Date(4, 29), Feast::CatherineOfSiena, false),
    (HolyDayId::Date(3, 2), Feast::ChadOfLichfield, false),
    (HolyDayId::Date(12, 2), Feast::ChanningMooreWilliams, false),
    (HolyDayId::Date(3, 27), Feast::CharlesHenryBrent, false),
    (HolyDayId::Date(11, 12), Feast::CharlesSimeon, false),
    (HolyDayId::Date(12, 25), Feast::ChristmasDay, false),
    (HolyDayId::Date(12, 24), Feast::ChristmasEve, true),
    (HolyDayId::Date(8, 11), Feast::ClareOfAssisi, false),
    (HolyDayId::Date(11, 23), Feast::ClementOfRome, false),
    (HolyDayId::Date(12, 5), Feast::ClementOfAlexandria, false),
    (HolyDayId::Date(6, 9), Feast::ColumbaOfIona, false),
    (HolyDayId::Date(1, 18), Feast::ConfessionOfStPeter, false),
    (HolyDayId::Date(11, 14), Feast::SamuelSeabury, false),
    (HolyDayId::Date(1, 25), Feast::ConversionOfStPaul, false),
    (HolyDayId::Date(3, 20), Feast::Cuthbert, false),
    (HolyDayId::Date(9, 13), Feast::CyprianOfCarthage, false),
    (HolyDayId::Date(3, 18), Feast::CyrilOfJerusalem, false),
    (HolyDayId::Date(2, 14), Feast::CyrilAndMethodius, false),
    (HolyDayId::Date(5, 8), Feast::JulianOfNorwich, false),
    (HolyDayId::Date(3, 1), Feast::DavidOfWales, false),
    (HolyDayId::Date(12, 29), Feast::December29, false),
    (HolyDayId::Date(12, 30), Feast::December30, false),
    (HolyDayId::Date(8, 8), Feast::Dominic, false),
    (HolyDayId::Date(5, 19), Feast::Dunstan, false),
    (HolyDayId::Date(9, 18), Feast::EdwardBouveriePusey, false),
    (HolyDayId::Date(11, 19), Feast::ElizabethOfHungary, false),
    (HolyDayId::Date(6, 10), Feast::EphremOfNisibis, false),
    (HolyDayId::Date(1, 6), Feast::Epiphany, false),
    (HolyDayId::Date(10, 31), Feast::EveOfAllSaints, true),
    (HolyDayId::Date(9, 13), Feast::EveOfHolyCross, true),
    (HolyDayId::Date(6, 23), Feast::EveOfStJohnTheBaptist, true),
    (HolyDayId::Date(3, 24), Feast::EveOfTheAnnunciation, true),
    (HolyDayId::Date(2, 1), Feast::EveOfThePresentation, true),
    (HolyDayId::Date(8, 5), Feast::EveOfTheTransfiguration, true),
    (HolyDayId::Date(5, 30), Feast::EveOfTheVisitation, true),
    (HolyDayId::Date(1, 20), Feast::Fabian, false),
    (HolyDayId::Date(10, 4), Feast::FrancisOfAssisi, false),
    (HolyDayId::Date(4, 1), Feast::FrederickDenisonMaurice, false),
    (HolyDayId::Date(4, 11), Feast::GeorgeAugustusSelwyn, false),
    (HolyDayId::Date(2, 27), Feast::GeorgeHerbert, false),
    (HolyDayId::Date(5, 9), Feast::GregoryOfNazianzus, false),
    (HolyDayId::Date(3, 12), Feast::GregoryTheGreat, false),
    (HolyDayId::Date(3, 23), Feast::GregoryTheIlluminator, false),
    (HolyDayId::Date(10, 19), Feast::HenryMartyn, false),
    (HolyDayId::Date(1, 13), Feast::HilaryOfPoitiers, false),
    (HolyDayId::Date(11, 18), Feast::HildaOfWhitby, false),
    (HolyDayId::Date(9, 14), Feast::HolyCross, false),
    (HolyDayId::Date(12, 28), Feast::HolyInnocents, false),
    (HolyDayId::Date(12, 31), Feast::EveOfHolyName, true),
    (HolyDayId::Date(1, 1), Feast::HolyName, false),
    (HolyDayId::Date(11, 17), Feast::HughOfLincoln, false),
    (
        HolyDayId::Date(10, 16),
        Feast::HughLatimerAndNicholasRidley,
        false,
    ),
    (HolyDayId::Date(10, 17), Feast::IgnatiusOfAntioch, false),
    (HolyDayId::Date(7, 4), Feast::IndependenceDay, false),
    (HolyDayId::Date(6, 28), Feast::IrenaeusOfLyons, false),
    (HolyDayId::Date(5, 24), Feast::JacksonKemper, false),
    (HolyDayId::Date(3, 22), Feast::JamesDeKoven, false),
    (HolyDayId::Date(10, 29), Feast::JamesHannington, false),
    (HolyDayId::Date(4, 2), Feast::JamesLloydBreck, false),
    (HolyDayId::Date(1, 10), Feast::January10, false),
    (HolyDayId::Date(1, 11), Feast::January11, false),
    (HolyDayId::Date(1, 12), Feast::January12, false),
    (HolyDayId::Date(1, 2), Feast::January2, false),
    (HolyDayId::Date(1, 3), Feast::January3, false),
    (HolyDayId::Date(1, 4), Feast::January4, false),
    (HolyDayId::Date(1, 5), Feast::January5, false),
    (HolyDayId::Date(1, 7), Feast::January7, false),
    (HolyDayId::Date(1, 8), Feast::January8, false),
    (HolyDayId::Date(1, 9), Feast::January9, false),
    (HolyDayId::Date(8, 13), Feast::JeremyTaylor, false),
    (HolyDayId::Date(9, 30), Feast::Jerome, false),
    (HolyDayId::Date(3, 3), Feast::JohnAndCharlesWesley, false),
    (HolyDayId::Date(1, 27), Feast::JohnChrysostom, false),
    (HolyDayId::Date(9, 20), Feast::JohnColeridgePatteson, false),
    (HolyDayId::Date(3, 31), Feast::JohnDonne, false),
    (HolyDayId::Date(9, 12), Feast::JohnHenryHobart, false),
    (HolyDayId::Date(3, 29), Feast::JohnKeble, false),
    (HolyDayId::Date(8, 7), Feast::JohnMasonNeale, false),
    (HolyDayId::Date(12, 4), Feast::JohnOfDamascus, false),
    (HolyDayId::Date(6, 16), Feast::JosephButler, false),
    (HolyDayId::Date(7, 31), Feast::JosephOfArimathea, false),
    (HolyDayId::Date(6, 1), Feast::Justin, false),
    (HolyDayId::Date(9, 26), Feast::LancelotAndrewes, false),
    (HolyDayId::Date(8, 10), Feast::LaurenceOfRome, false),
    (HolyDayId::Date(11, 10), Feast::LeoOfRome, false),
    (HolyDayId::Date(8, 25), Feast::Louis, false),
    (HolyDayId::Date(11, 16), Feast::MargaretOfScotland, false),
    (HolyDayId::Date(11, 11), Feast::MartinOfTours, false),
    (HolyDayId::Date(5, 4), Feast::Monica, false),
    (
        HolyDayId::Date(6, 24),
        Feast::NativityOfStJohnTheBaptist,
        false,
    ),
    (HolyDayId::Date(12, 6), Feast::NicholasOfMyra, false),
    (HolyDayId::Date(12, 1), Feast::NicholasFerrar, false),
    (HolyDayId::Date(9, 16), Feast::Ninian, false),
    (HolyDayId::Date(3, 17), Feast::PatrickOfIreland, false),
    (HolyDayId::Date(3, 7), Feast::PerpetuaAndFelicity, false),
    (HolyDayId::Date(1, 23), Feast::PhillipsBrooks, false),
    (HolyDayId::Date(2, 23), Feast::PolycarpOfSmyrna, false),
    (HolyDayId::Date(10, 1), Feast::RemigiusOfRheims, false),
    (HolyDayId::Date(4, 3), Feast::RichardOfChichester, false),
    (HolyDayId::Date(11, 3), Feast::RichardHooker, false),
    (HolyDayId::Date(10, 9), Feast::RobertGrosseteste, false),
    (
        HolyDayId::Date(10, 15),
        Feast::SamuelIsaacJosephScherechewsky,
        false,
    ),
    (HolyDayId::Date(9, 25), Feast::SergiusOfRadonezh, false),
    (HolyDayId::Date(6, 29), Feast::PeterAndPaul, false),
    (HolyDayId::Date(5, 1), Feast::PhilipAndJames, false),
    (HolyDayId::Date(10, 28), Feast::SimonAndJude, false),
    (HolyDayId::Date(11, 30), Feast::Andrew, false),
    (HolyDayId::Date(6, 11), Feast::Barnabas, false),
    (HolyDayId::Date(8, 24), Feast::Bartholomew, false),
    (HolyDayId::Date(7, 25), Feast::James, false),
    (HolyDayId::Date(10, 23), Feast::JamesOfJerusalem, false),
    (HolyDayId::Date(12, 27), Feast::John, false),
    (HolyDayId::Date(3, 19), Feast::Joseph, false),
    (HolyDayId::Date(10, 18), Feast::Luke, false),
    (HolyDayId::Date(4, 25), Feast::Mark, false),
    (HolyDayId::Date(7, 22), Feast::MaryMagdalene, false),
    (HolyDayId::Date(8, 15), Feast::Mary, false),
    (HolyDayId::Date(9, 21), Feast::Matthew, false),
    (HolyDayId::Date(2, 24), Feast::Matthias, false),
    (HolyDayId::Date(9, 29), Feast::Michael, false),
    (HolyDayId::Date(12, 26), Feast::Stephen, false),
    (HolyDayId::Date(12, 21), Feast::Thomas, false),
    (HolyDayId::Date(2, 5), Feast::TheMartyrsOfJapan, false),
    (HolyDayId::Date(9, 2), Feast::TheMartyrsOfNewGuinea, false),
    (HolyDayId::Date(6, 3), Feast::TheMartyrsOfUganda, false),
    (HolyDayId::Date(2, 2), Feast::ThePresentation, false),
    (HolyDayId::Date(8, 6), Feast::TheTransfiguration, false),
    (HolyDayId::Date(5, 31), Feast::TheVisitation, false),
    (HolyDayId::Date(9, 19), Feast::TheodoreOfTarsus, false),
    (HolyDayId::Date(7, 24), Feast::ThomasAKempis, false),
    (HolyDayId::Date(1, 28), Feast::ThomasAquinas, false),
    (HolyDayId::Date(2, 15), Feast::ThomasBray, false),
    (HolyDayId::Date(3, 21), Feast::ThomasKen, false),
    (HolyDayId::Date(1, 26), Feast::TitusAndTimothy, false),
    (HolyDayId::Date(1, 22), Feast::VincentOfSaragossa, false),
    (
        HolyDayId::Date(4, 8),
        Feast::WilliamAugustusMuhlenberg,
        false,
    ),
    (HolyDayId::Date(1, 10), Feast::WilliamLaud, false),
    (HolyDayId::Date(4, 9), Feast::WilliamLaw, false),
    (HolyDayId::Date(8, 18), Feast::WilliamPorcherDubose, false),
    (HolyDayId::Date(7, 27), Feast::WilliamReedHuntington, false),
    (HolyDayId::Date(10, 6), Feast::WilliamTyndale, false),
    (HolyDayId::Date(7, 17), Feast::WilliamWhite, false),
    (HolyDayId::Date(7, 30), Feast::WilliamWilberforce, false),
    (HolyDayId::Date(11, 7), Feast::Willibrord, false),
    (HolyDayId::Date(6, 19), Feast::WulfstanOfWorcester, false),
];

#[cfg(test)]
mod tests {
    use crate::LiturgicalDayId;

    use super::super::*;

    #[test]
    fn thanksgiving_day() {
        let tday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2021, 11, 25), false);
        assert_eq!(
            tday.observed,
            LiturgicalDayId::Feast(Feast::ThanksgivingDay)
        );
        let tday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2017, 11, 23), false);
        assert_eq!(
            tday.observed,
            LiturgicalDayId::Feast(Feast::ThanksgivingDay)
        );
    }
}
