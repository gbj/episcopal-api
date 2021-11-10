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

const BCP1979_WEEKS: [(Cycle, u8, LiturgicalWeek); 59] = [
    (Cycle::Advent, 0, LiturgicalWeek::LastPentecost),
    (Cycle::Advent, 1, LiturgicalWeek::Advent1),
    (Cycle::Advent, 2, LiturgicalWeek::Advent2),
    (Cycle::Advent, 3, LiturgicalWeek::Advent3),
    (Cycle::Advent, 4, LiturgicalWeek::Advent4),
    (Cycle::Christmas, 0, LiturgicalWeek::Christmas),
    (Cycle::Christmas, 1, LiturgicalWeek::Christmas1),
    (Cycle::Christmas, 2, LiturgicalWeek::Christmas2),
    (Cycle::Epiphany, 0, LiturgicalWeek::Epiphany),
    (Cycle::Epiphany, 1, LiturgicalWeek::Epiphany1),
    (Cycle::Epiphany, 2, LiturgicalWeek::Epiphany2),
    (Cycle::Epiphany, 3, LiturgicalWeek::Epiphany3),
    (Cycle::Epiphany, 4, LiturgicalWeek::Epiphany4),
    (Cycle::Epiphany, 5, LiturgicalWeek::Epiphany5),
    (Cycle::Epiphany, 6, LiturgicalWeek::Epiphany6),
    (Cycle::Epiphany, 7, LiturgicalWeek::Epiphany7),
    (Cycle::Epiphany, 8, LiturgicalWeek::Epiphany8),
    (Cycle::Easter, 0, LiturgicalWeek::LastEpiphany),
    (Cycle::Easter, 1, LiturgicalWeek::Lent1),
    (Cycle::Easter, 2, LiturgicalWeek::Lent2),
    (Cycle::Easter, 3, LiturgicalWeek::Lent3),
    (Cycle::Easter, 4, LiturgicalWeek::Lent4),
    (Cycle::Easter, 5, LiturgicalWeek::Lent5),
    (Cycle::Easter, 6, LiturgicalWeek::HolyWeek),
    (Cycle::Easter, 7, LiturgicalWeek::Easter),
    (Cycle::Easter, 8, LiturgicalWeek::Easter2),
    (Cycle::Easter, 9, LiturgicalWeek::Easter3),
    (Cycle::Easter, 10, LiturgicalWeek::Easter4),
    (Cycle::Easter, 11, LiturgicalWeek::Easter5),
    (Cycle::Easter, 12, LiturgicalWeek::Easter6),
    (Cycle::Easter, 13, LiturgicalWeek::Easter7),
    (Cycle::Easter, 14, LiturgicalWeek::Pentecost),
    (Cycle::Easter, 15, LiturgicalWeek::TrinitySunday),
    (Cycle::Easter, 16, LiturgicalWeek::Pentecost2),
    (Cycle::Easter, 17, LiturgicalWeek::Pentecost3),
    (Cycle::Easter, 18, LiturgicalWeek::Pentecost4),
    (Cycle::Easter, 19, LiturgicalWeek::Pentecost5),
    (Cycle::Easter, 20, LiturgicalWeek::Pentecost6),
    (Cycle::Easter, 21, LiturgicalWeek::Pentecost7),
    (Cycle::Easter, 22, LiturgicalWeek::Pentecost8),
    (Cycle::Easter, 23, LiturgicalWeek::Pentecost9),
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
    (Cycle::Easter, 34, LiturgicalWeek::Pentecost20),
    (Cycle::Easter, 35, LiturgicalWeek::Pentecost21),
    (Cycle::Easter, 36, LiturgicalWeek::Pentecost22),
    (Cycle::Easter, 37, LiturgicalWeek::Pentecost23),
    (Cycle::Easter, 38, LiturgicalWeek::Pentecost24),
    (Cycle::Easter, 39, LiturgicalWeek::Pentecost25),
    (Cycle::Easter, 40, LiturgicalWeek::Pentecost26),
    (Cycle::Easter, 41, LiturgicalWeek::Pentecost27),
];

const BCP1979_HOLY_DAY_RANKS: [(Feast, Rank); 83] = [
    (Feast::AllSaintsDay, Rank::PrincipalFeast),
    (Feast::ChristmasDay, Rank::PrincipalFeast),
    (Feast::ChristmasEve, Rank::PrincipalFeast),
    (Feast::Epiphany, Rank::PrincipalFeast),
    (Feast::HolyName, Rank::PrincipalFeast),
    (Feast::EveOfPentecost, Rank::PrincipalFeast),
    (Feast::PalmSunday, Rank::PrincipalFeast),
    (Feast::EasterSunday, Rank::PrincipalFeast),
    (Feast::Pentecost, Rank::PrincipalFeast),
    (Feast::TrinitySunday, Rank::PrincipalFeast),
    (Feast::AscensionDay, Rank::PrincipalFeast),
    (Feast::EveOfTheAscension, Rank::PrincipalFeast),
    (Feast::FridayInEasterWeek, Rank::PrecedenceOverHolyDay),
    (Feast::GoodFriday, Rank::PrecedenceOverHolyDay),
    (Feast::MondayInEasterWeek, Rank::PrecedenceOverHolyDay),
    (Feast::MondayInHolyWeek, Rank::PrecedenceOverHolyDay),
    (Feast::SaturdayInEasterWeek, Rank::PrecedenceOverHolyDay),
    (Feast::HolySaturday, Rank::PrecedenceOverHolyDay),
    (Feast::ThursdayInEasterWeek, Rank::PrecedenceOverHolyDay),
    (Feast::MaundyThursday, Rank::PrecedenceOverHolyDay),
    (Feast::TuesdayInEasterWeek, Rank::PrecedenceOverHolyDay),
    (Feast::TuesdayInHolyWeek, Rank::PrecedenceOverHolyDay),
    (Feast::WednesdayInEasterWeek, Rank::PrecedenceOverHolyDay),
    (Feast::WednesdayInHolyWeek, Rank::PrecedenceOverHolyDay),
    (Feast::AllSoulsDay, Rank::OptionalObservance),
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
    (Feast::FridayAfterAshWednesday, Rank::HolyDay),
    (Feast::HolyCross, Rank::HolyDay),
    (Feast::HolyInnocents, Rank::HolyDay),
    (Feast::IndependenceDay, Rank::HolyDay),
    (Feast::NativityOfStJohnTheBaptist, Rank::HolyDay),
    (Feast::SaturdayAfterAscension, Rank::HolyDay),
    (Feast::SaturdayAfterAshWednesday, Rank::HolyDay),
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
    (Feast::EveOfTrinitySunday, Rank::HolyDay),
    (Feast::ThanksgivingDay, Rank::HolyDay),
    (Feast::ThePresentation, Rank::HolyDay),
    (Feast::TheTransfiguration, Rank::HolyDay),
    (Feast::TheVisitation, Rank::HolyDay),
    (Feast::ThursdayAfterAshWednesday, Rank::HolyDay),
    (Feast::AshWednesday, Rank::HolyDay),
    (Feast::December29, Rank::DaysOfChristmas),
    (Feast::December30, Rank::DaysOfChristmas),
    (Feast::December31, Rank::DaysOfChristmas),
    (Feast::January2, Rank::DaysOfChristmas),
    (Feast::January3, Rank::DaysOfChristmas),
    (Feast::January4, Rank::DaysOfChristmas),
    (Feast::January5, Rank::DaysOfChristmas),
    (Feast::January7, Rank::DaysOfChristmas),
    (Feast::January8, Rank::DaysOfChristmas),
    (Feast::January9, Rank::DaysOfChristmas),
    (Feast::January10, Rank::DaysOfChristmas),
    (Feast::January11, Rank::DaysOfChristmas),
    (Feast::January12, Rank::DaysOfChristmas),
];

/// Array of all observances in the BCP 1979 calendar
pub const BCP1979_FEASTS: [KalendarEntry; 200] = [
    // Thanksgiving, Labor Day, All Saints’ Sunday
    (
        HolyDayId::DayOfMonth {
            month: 9,
            week: 1,
            day: Weekday::Mon,
        },
        Feast::LaborDay,
        false,
        None,
    ),
    (
        HolyDayId::DayOfMonth {
            month: 11,
            week: 4,
            day: Weekday::Thu,
        },
        Feast::ThanksgivingDay,
        false,
        None,
    ),
    // Special days
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Epiphany, Weekday::Sat),
        Feast::EveOfEpiphany1,
        true,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::LastEpiphany, Weekday::Wed),
        Feast::AshWednesday,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::LastEpiphany, Weekday::Thu),
        Feast::ThursdayAfterAshWednesday,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::LastEpiphany, Weekday::Fri),
        Feast::FridayAfterAshWednesday,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::LastEpiphany, Weekday::Sat),
        Feast::SaturdayAfterAshWednesday,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::HolyWeek, Weekday::Sun),
        Feast::PalmSunday,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::HolyWeek, Weekday::Mon),
        Feast::MondayInHolyWeek,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::HolyWeek, Weekday::Tue),
        Feast::TuesdayInHolyWeek,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::HolyWeek, Weekday::Wed),
        Feast::WednesdayInHolyWeek,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::HolyWeek, Weekday::Thu),
        Feast::MaundyThursday,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::HolyWeek, Weekday::Fri),
        Feast::GoodFriday,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::HolyWeek, Weekday::Sat),
        Feast::HolySaturday,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter, Weekday::Sun),
        Feast::EasterSunday,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter, Weekday::Mon),
        Feast::MondayInEasterWeek,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter, Weekday::Tue),
        Feast::TuesdayInEasterWeek,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter, Weekday::Wed),
        Feast::WednesdayInEasterWeek,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter, Weekday::Thu),
        Feast::ThursdayInEasterWeek,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter, Weekday::Fri),
        Feast::FridayInEasterWeek,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter, Weekday::Sat),
        Feast::SaturdayInEasterWeek,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter6, Weekday::Wed),
        Feast::EveOfTheAscension,
        true,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter6, Weekday::Thu),
        Feast::AscensionDay,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter6, Weekday::Fri),
        Feast::FridayAfterAscension,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter6, Weekday::Sat),
        Feast::SaturdayAfterAscension,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Easter7, Weekday::Sat),
        Feast::EveOfPentecost,
        true,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Pentecost, Weekday::Sun),
        Feast::Pentecost,
        false,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::Pentecost, Weekday::Sat),
        Feast::EveOfTrinitySunday,
        true,
        None,
    ),
    (
        HolyDayId::SpecialDay(LiturgicalWeek::TrinitySunday, Weekday::Sun),
        Feast::TrinitySunday,
        false,
        None,
    ),
    // MM/DD feast days
    (HolyDayId::Date(1, 1), Feast::HolyName, false, None),
    (HolyDayId::Date(1, 2), Feast::January2, false, None),
    (HolyDayId::Date(1, 3), Feast::January3, false, None),
    (HolyDayId::Date(1, 4), Feast::January4, false, None),
    (HolyDayId::Date(1, 5), Feast::January5, false, None),
    (HolyDayId::Date(1, 5), Feast::EveOfEpiphany, true, None),
    (HolyDayId::Date(1, 6), Feast::Epiphany, false, None),
    (
        HolyDayId::Date(1, 7),
        Feast::January7,
        false,
        Some(LiturgicalWeek::Epiphany1),
    ),
    (
        HolyDayId::Date(1, 8),
        Feast::January8,
        false,
        Some(LiturgicalWeek::Epiphany1),
    ),
    (
        HolyDayId::Date(1, 9),
        Feast::January9,
        false,
        Some(LiturgicalWeek::Epiphany1),
    ),
    (
        HolyDayId::Date(1, 10),
        Feast::January10,
        false,
        Some(LiturgicalWeek::Epiphany1),
    ),
    (HolyDayId::Date(1, 10), Feast::WilliamLaud, false, None),
    (
        HolyDayId::Date(1, 11),
        Feast::January11,
        false,
        Some(LiturgicalWeek::Epiphany1),
    ),
    (
        HolyDayId::Date(1, 12),
        Feast::January12,
        false,
        Some(LiturgicalWeek::Epiphany1),
    ),
    (HolyDayId::Date(1, 13), Feast::HilaryOfPoitiers, false, None),
    (HolyDayId::Date(1, 17), Feast::AntonyOfEgypt, false, None),
    (
        HolyDayId::Date(1, 18),
        Feast::ConfessionOfStPeter,
        false,
        None,
    ),
    (HolyDayId::Date(1, 20), Feast::Fabian, false, None),
    (
        HolyDayId::Date(1, 21),
        Feast::AgnesAndCeciliaOfRome,
        false,
        None,
    ),
    (
        HolyDayId::Date(1, 22),
        Feast::VincentOfSaragossa,
        false,
        None,
    ),
    (HolyDayId::Date(1, 23), Feast::PhillipsBrooks, false, None),
    (
        HolyDayId::Date(1, 25),
        Feast::ConversionOfStPaul,
        false,
        None,
    ),
    (HolyDayId::Date(1, 26), Feast::TitusAndTimothy, false, None),
    (HolyDayId::Date(1, 27), Feast::JohnChrysostom, false, None),
    (HolyDayId::Date(1, 28), Feast::ThomasAquinas, false, None),
    (
        HolyDayId::Date(2, 1),
        Feast::EveOfThePresentation,
        true,
        None,
    ),
    (HolyDayId::Date(2, 2), Feast::ThePresentation, false, None),
    (HolyDayId::Date(2, 3), Feast::Anskar, false, None),
    (HolyDayId::Date(2, 5), Feast::TheMartyrsOfJapan, false, None),
    (HolyDayId::Date(2, 13), Feast::AbsalomJones, false, None),
    (
        HolyDayId::Date(2, 14),
        Feast::CyrilAndMethodius,
        false,
        None,
    ),
    (HolyDayId::Date(2, 15), Feast::ThomasBray, false, None),
    (HolyDayId::Date(2, 23), Feast::PolycarpOfSmyrna, false, None),
    (HolyDayId::Date(2, 24), Feast::Matthias, false, None),
    (HolyDayId::Date(2, 27), Feast::GeorgeHerbert, false, None),
    (HolyDayId::Date(3, 1), Feast::DavidOfWales, false, None),
    (HolyDayId::Date(3, 2), Feast::ChadOfLichfield, false, None),
    (
        HolyDayId::Date(3, 3),
        Feast::JohnAndCharlesWesley,
        false,
        None,
    ),
    (
        HolyDayId::Date(3, 7),
        Feast::PerpetuaAndFelicity,
        false,
        None,
    ),
    (HolyDayId::Date(3, 12), Feast::GregoryTheGreat, false, None),
    (HolyDayId::Date(3, 17), Feast::PatrickOfIreland, false, None),
    (HolyDayId::Date(3, 18), Feast::CyrilOfJerusalem, false, None),
    (HolyDayId::Date(3, 19), Feast::Joseph, false, None),
    (HolyDayId::Date(3, 20), Feast::Cuthbert, false, None),
    (HolyDayId::Date(3, 21), Feast::ThomasKen, false, None),
    (HolyDayId::Date(3, 22), Feast::JamesDeKoven, false, None),
    (
        HolyDayId::Date(3, 23),
        Feast::GregoryTheIlluminator,
        false,
        None,
    ),
    (
        HolyDayId::Date(3, 24),
        Feast::EveOfTheAnnunciation,
        true,
        None,
    ),
    (HolyDayId::Date(3, 25), Feast::Annunciation, false, None),
    (
        HolyDayId::Date(3, 27),
        Feast::CharlesHenryBrent,
        false,
        None,
    ),
    (HolyDayId::Date(3, 29), Feast::JohnKeble, false, None),
    (HolyDayId::Date(3, 31), Feast::JohnDonne, false, None),
    (
        HolyDayId::Date(4, 1),
        Feast::FrederickDenisonMaurice,
        false,
        None,
    ),
    (HolyDayId::Date(4, 2), Feast::JamesLloydBreck, false, None),
    (
        HolyDayId::Date(4, 3),
        Feast::RichardOfChichester,
        false,
        None,
    ),
    (
        HolyDayId::Date(4, 8),
        Feast::WilliamAugustusMuhlenberg,
        false,
        None,
    ),
    (HolyDayId::Date(4, 9), Feast::WilliamLaw, false, None),
    (
        HolyDayId::Date(4, 11),
        Feast::GeorgeAugustusSelwyn,
        false,
        None,
    ),
    (HolyDayId::Date(4, 19), Feast::Alphege, false, None),
    (
        HolyDayId::Date(4, 21),
        Feast::AnselmOfCanterbury,
        false,
        None,
    ),
    (HolyDayId::Date(4, 25), Feast::Mark, false, None),
    (HolyDayId::Date(4, 29), Feast::CatherineOfSiena, false, None),
    (HolyDayId::Date(5, 1), Feast::PhilipAndJames, false, None),
    (
        HolyDayId::Date(5, 2),
        Feast::AthanasiusOfAlexandria,
        false,
        None,
    ),
    (HolyDayId::Date(5, 4), Feast::Monica, false, None),
    (HolyDayId::Date(5, 8), Feast::JulianOfNorwich, false, None),
    (
        HolyDayId::Date(5, 9),
        Feast::GregoryOfNazianzus,
        false,
        None,
    ),
    (HolyDayId::Date(5, 19), Feast::Dunstan, false, None),
    (HolyDayId::Date(5, 20), Feast::AlcuinOfYork, false, None),
    (HolyDayId::Date(5, 24), Feast::JacksonKemper, false, None),
    (HolyDayId::Date(5, 25), Feast::Bede, false, None),
    (
        HolyDayId::Date(5, 26),
        Feast::AugustineOfCanterbury,
        false,
        None,
    ),
    (
        HolyDayId::Date(5, 30),
        Feast::EveOfTheVisitation,
        true,
        None,
    ),
    (HolyDayId::Date(5, 31), Feast::TheVisitation, false, None),
    (HolyDayId::Date(6, 1), Feast::Justin, false, None),
    (
        HolyDayId::Date(6, 3),
        Feast::TheMartyrsOfUganda,
        false,
        None,
    ),
    (HolyDayId::Date(6, 5), Feast::Boniface, false, None),
    (HolyDayId::Date(6, 9), Feast::ColumbaOfIona, false, None),
    (HolyDayId::Date(6, 10), Feast::EphremOfNisibis, false, None),
    (HolyDayId::Date(6, 11), Feast::Barnabas, false, None),
    (HolyDayId::Date(6, 14), Feast::BasilOfCaesarea, false, None),
    (HolyDayId::Date(6, 16), Feast::JosephButler, false, None),
    (HolyDayId::Date(6, 18), Feast::BernardMizeki, false, None),
    (
        HolyDayId::Date(6, 19),
        Feast::WulfstanOfWorcester,
        false,
        None,
    ),
    (HolyDayId::Date(6, 22), Feast::Alban, false, None),
    (
        HolyDayId::Date(6, 23),
        Feast::EveOfStJohnTheBaptist,
        true,
        None,
    ),
    (
        HolyDayId::Date(6, 24),
        Feast::NativityOfStJohnTheBaptist,
        false,
        None,
    ),
    (HolyDayId::Date(6, 28), Feast::IrenaeusOfLyons, false, None),
    (HolyDayId::Date(6, 29), Feast::PeterAndPaul, false, None),
    (HolyDayId::Date(7, 4), Feast::IndependenceDay, false, None),
    (HolyDayId::Date(7, 11), Feast::BenedictOfNursia, false, None),
    (HolyDayId::Date(7, 17), Feast::WilliamWhite, false, None),
    (HolyDayId::Date(7, 22), Feast::MaryMagdalene, false, None),
    (HolyDayId::Date(7, 24), Feast::ThomasAKempis, false, None),
    (HolyDayId::Date(7, 25), Feast::James, false, None),
    (
        HolyDayId::Date(7, 27),
        Feast::WilliamReedHuntington,
        false,
        None,
    ),
    (
        HolyDayId::Date(7, 30),
        Feast::WilliamWilberforce,
        false,
        None,
    ),
    (
        HolyDayId::Date(7, 31),
        Feast::JosephOfArimathea,
        false,
        None,
    ),
    (
        HolyDayId::Date(8, 5),
        Feast::EveOfTheTransfiguration,
        true,
        None,
    ),
    (
        HolyDayId::Date(8, 6),
        Feast::TheTransfiguration,
        false,
        None,
    ),
    (HolyDayId::Date(8, 7), Feast::JohnMasonNeale, false, None),
    (HolyDayId::Date(8, 8), Feast::Dominic, false, None),
    (HolyDayId::Date(8, 10), Feast::LaurenceOfRome, false, None),
    (HolyDayId::Date(8, 11), Feast::ClareOfAssisi, false, None),
    (HolyDayId::Date(8, 13), Feast::JeremyTaylor, false, None),
    (HolyDayId::Date(8, 15), Feast::Mary, false, None),
    (
        HolyDayId::Date(8, 18),
        Feast::WilliamPorcherDubose,
        false,
        None,
    ),
    (
        HolyDayId::Date(8, 20),
        Feast::BernardOfClairvaux,
        false,
        None,
    ),
    (HolyDayId::Date(8, 24), Feast::Bartholomew, false, None),
    (HolyDayId::Date(8, 25), Feast::Louis, false, None),
    (HolyDayId::Date(8, 28), Feast::AugustineOfHippo, false, None),
    (
        HolyDayId::Date(8, 31),
        Feast::AidanOfLindisfarne,
        false,
        None,
    ),
    (
        HolyDayId::Date(9, 2),
        Feast::TheMartyrsOfNewGuinea,
        false,
        None,
    ),
    (HolyDayId::Date(9, 12), Feast::JohnHenryHobart, false, None),
    (
        HolyDayId::Date(9, 13),
        Feast::CyprianOfCarthage,
        false,
        None,
    ),
    (HolyDayId::Date(9, 13), Feast::EveOfHolyCross, true, None),
    (HolyDayId::Date(9, 14), Feast::HolyCross, false, None),
    (HolyDayId::Date(9, 16), Feast::Ninian, false, None),
    (
        HolyDayId::Date(9, 18),
        Feast::EdwardBouveriePusey,
        false,
        None,
    ),
    (HolyDayId::Date(9, 19), Feast::TheodoreOfTarsus, false, None),
    (
        HolyDayId::Date(9, 20),
        Feast::JohnColeridgePatteson,
        false,
        None,
    ),
    (HolyDayId::Date(9, 21), Feast::Matthew, false, None),
    (
        HolyDayId::Date(9, 25),
        Feast::SergiusOfRadonezh,
        false,
        None,
    ),
    (HolyDayId::Date(9, 26), Feast::LancelotAndrewes, false, None),
    (HolyDayId::Date(9, 29), Feast::Michael, false, None),
    (HolyDayId::Date(9, 30), Feast::Jerome, false, None),
    (HolyDayId::Date(10, 1), Feast::RemigiusOfRheims, false, None),
    (HolyDayId::Date(10, 4), Feast::FrancisOfAssisi, false, None),
    (HolyDayId::Date(10, 6), Feast::WilliamTyndale, false, None),
    (
        HolyDayId::Date(10, 9),
        Feast::RobertGrosseteste,
        false,
        None,
    ),
    (
        HolyDayId::Date(10, 15),
        Feast::SamuelIsaacJosephScherechewsky,
        false,
        None,
    ),
    (
        HolyDayId::Date(10, 16),
        Feast::HughLatimerAndNicholasRidley,
        false,
        None,
    ),
    (
        HolyDayId::Date(10, 17),
        Feast::IgnatiusOfAntioch,
        false,
        None,
    ),
    (HolyDayId::Date(10, 18), Feast::Luke, false, None),
    (HolyDayId::Date(10, 19), Feast::HenryMartyn, false, None),
    (
        HolyDayId::Date(10, 23),
        Feast::JamesOfJerusalem,
        false,
        None,
    ),
    (HolyDayId::Date(10, 26), Feast::Alfred, false, None),
    (HolyDayId::Date(10, 28), Feast::SimonAndJude, false, None),
    (HolyDayId::Date(10, 29), Feast::JamesHannington, false, None),
    (HolyDayId::Date(10, 31), Feast::EveOfAllSaints, true, None),
    (HolyDayId::Date(11, 1), Feast::AllSaintsDay, false, None),
    (HolyDayId::Date(11, 2), Feast::AllSoulsDay, false, None),
    (HolyDayId::Date(11, 3), Feast::RichardHooker, false, None),
    (HolyDayId::Date(11, 7), Feast::Willibrord, false, None),
    (HolyDayId::Date(11, 10), Feast::LeoOfRome, false, None),
    (HolyDayId::Date(11, 11), Feast::MartinOfTours, false, None),
    (HolyDayId::Date(11, 12), Feast::CharlesSimeon, false, None),
    (HolyDayId::Date(11, 14), Feast::SamuelSeabury, false, None),
    (
        HolyDayId::Date(11, 16),
        Feast::MargaretOfScotland,
        false,
        None,
    ),
    (HolyDayId::Date(11, 17), Feast::HughOfLincoln, false, None),
    (HolyDayId::Date(11, 18), Feast::HildaOfWhitby, false, None),
    (
        HolyDayId::Date(11, 19),
        Feast::ElizabethOfHungary,
        false,
        None,
    ),
    (HolyDayId::Date(11, 23), Feast::ClementOfRome, false, None),
    (HolyDayId::Date(11, 30), Feast::Andrew, false, None),
    (HolyDayId::Date(12, 1), Feast::NicholasFerrar, false, None),
    (
        HolyDayId::Date(12, 2),
        Feast::ChanningMooreWilliams,
        false,
        None,
    ),
    (HolyDayId::Date(12, 4), Feast::JohnOfDamascus, false, None),
    (
        HolyDayId::Date(12, 5),
        Feast::ClementOfAlexandria,
        false,
        None,
    ),
    (HolyDayId::Date(12, 6), Feast::NicholasOfMyra, false, None),
    (HolyDayId::Date(12, 7), Feast::AmbroseOfMilan, false, None),
    (HolyDayId::Date(12, 21), Feast::Thomas, false, None),
    (HolyDayId::Date(12, 24), Feast::December24, false, None),
    (HolyDayId::Date(12, 24), Feast::ChristmasEve, true, None),
    (HolyDayId::Date(12, 25), Feast::ChristmasDay, false, None),
    (HolyDayId::Date(12, 26), Feast::Stephen, false, None),
    (HolyDayId::Date(12, 27), Feast::John, false, None),
    (HolyDayId::Date(12, 28), Feast::HolyInnocents, false, None),
    (HolyDayId::Date(12, 29), Feast::December29, false, None),
    (HolyDayId::Date(12, 30), Feast::December30, false, None),
    (HolyDayId::Date(12, 31), Feast::December31, false, None),
    (HolyDayId::Date(12, 31), Feast::EveOfHolyName, true, None),
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

    #[test]
    fn last_week_after_epiphany() {
        let tday = BCP1979_CALENDAR
            .liturgical_day_without_transferred_feasts(Date::from_ymd(2022, 2, 28), false);
        assert_eq!(
            tday.observed,
            LiturgicalDayId::WeekAndDay(LiturgicalWeek::LastEpiphany, Weekday::Mon)
        );
    }
}
