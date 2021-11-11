use serde::{Deserialize, Serialize};

use crate::holy_day::HolyDayId;
use crate::lff2018::LFF_BIOS;
use crate::LiturgicalWeek;

pub use crate::bcp1979::BCP1979_FEASTS;
pub use crate::lff2018::LFF2018_FEASTS;

/// Returns the biography for the given feast day.
/// ```
/// # use crate::calendar::{Feast, feasts::{bio_for_feast}};
/// let bio = bio_for_feast(Feast::ChristmasDay);
/// assert!(bio.is_some());
/// assert!(bio.unwrap().starts_with("That Jesus was born is a fact both of history and revelation."));
/// ```
pub fn bio_for_feast(feast: Feast) -> Option<&'static str> {
    LFF_BIOS
        .iter()
        .find(|(f, _)| *f == feast)
        .map(|(_, bio)| *bio)
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Feast {
    // Special Days
    FridayAfterAscension,
    FridayInEasterWeek,
    GoodFriday,
    FridayAfterAshWednesday,
    MondayInEasterWeek,
    MondayInHolyWeek,
    SaturdayAfterAscension,
    EveOfPentecost,
    SaturdayInEasterWeek,
    HolySaturday,
    SaturdayAfterAshWednesday,
    PalmSunday,
    Pentecost,
    EveOfTrinitySunday,
    TrinitySunday,
    ThanksgivingDay,
    AscensionDay,
    ThursdayInEasterWeek,
    MaundyThursday,
    ThursdayAfterAshWednesday,
    TuesdayInEasterWeek,
    TuesdayInHolyWeek,
    EveOfTheAscension,
    WednesdayInEasterWeek,
    WednesdayInHolyWeek,
    AshWednesday,
    EasterSunday,
    LaborDay,
    // Additional RCL selections for special services on select feast days
    ChristmasDayII,
    ChristmasDayIII,
    EasterVigil,
    PentecostVigil,
    LiturgyOfThePalms,
    EasterDayEveningService,
    // Saints' Days
    AbsalomJones,
    AdelaideTeagueCase,
    AelredOfRievaulx,
    AgathaOfSicily,
    AgnesAndCeciliaOfRome,
    AgnesTsaoKouYing,
    AidanOfLindisfarne,
    Alban,
    AlcuinOfYork,
    AlexanderCrummell,
    Alfred,
    AllSaintsDay,
    AllSoulsDay,
    Alphege,
    AmbroseOfMilan,
    Ammonius,
    AnnaEllisonButlerAlexander,
    AnnaJuliaHaywoodCooper,
    AnselmOfCanterbury,
    Anskar,
    AntonyOfEgypt,
    ArgulaVonGrumbach,
    AthanasiusOfAlexandria,
    AugustineOfCanterbury,
    AugustineOfHippo,
    BakhitaJosephineMargaretBakhita,
    BasilOfCaesarea,
    Bede,
    BenedictOfNursia,
    BernardMizeki,
    BernardOfClairvaux,
    BirgittaOfSweden,
    BlandinaAndHerCompanions,
    Boniface,
    BrigidOfKildare,
    CatherineOfAlexandria,
    CatherineOfGenoa,
    CatherineOfSiena,
    ChadOfLichfield,
    ChanningMooreWilliams,
    CharlesDeFoucauld,
    CharlesHenryBrent,
    CharlesSimeon,
    ClareOfAssisi,
    ClementOfAlexandria,
    ClementOfRome,
    CliveStaplesLewis,
    ColumbaOfIona,
    CorneliusTheCenturion,
    Cuthbert,
    CyprianOfCarthage,
    CyrilAndMethodius,
    CyrilOfJerusalem,
    Damien,
    DavidOfWales,
    DavidPendletonOakerhater,
    DietrichBonhoeffer,
    Dominic,
    DorothyLSayers,
    Dunstan,
    EdithCavell,
    EdithSteinTeresaBenedictaOfTheCross,
    Edmund,
    EdwardBouveriePusey,
    ElisabethCruciger,
    ElizabethAnnSeton,
    ElizabethCadyStanton,
    ElizabethOfHungary,
    EmilyMalboneMorgan,
    Enmegahbowh,
    EphremOfNisibis,
    EuphrosynesmaragdusOfAlexandria,
    EvaLeeMatthews,
    EvelynUnderhill,
    EveOfAllSaints,
    Fabian,
    FlorenceLiTimOi,
    FlorenceNightingale,
    FrancesPerkins,
    FrancisDeSales,
    FrancisOfAssisi,
    FrancisXavier,
    FrederickDenisonMaurice,
    FrederickDouglass,
    GeorgeAugustusSelwyn,
    GeorgeHerbert,
    GregoryOfNazianzus,
    GregoryOfNyssa,
    GregoryTheGreat,
    GregoryTheIlluminator,
    HadewijchOfBrabant,
    HannahMore,
    HarrietBedell,
    HarrietMonsell,
    HarrietStarrCannon,
    HelenaOfConstantinople,
    HenryMartyn,
    HermanOfAlaska,
    HilaryOfPoitiers,
    HildaOfWhitby,
    HildegardOfBingen,
    HolyCross,
    HolyInnocents,
    HughLatimerAndNicholasRidley,
    HughOfLincoln,
    IgnatiusOfAntioch,
    IgnatiusOfLoyola,
    IndependenceDayUnitedStates,
    IrenaeusOfLyons,
    IsabelFlorenceHapgood,
    JacksonKemper,
    JamesDeKoven,
    JamesHannington,
    JamesLloydBreck,
    JamesOtisSargentHuntington,
    JamesSolomonRussell,
    JamesTheodoreHolly,
    JananiLuwum,
    JeremyTaylor,
    Jerome,
    Joanna,
    JohannArndtAndJacobBoehme,
    JohannSebasatianBach,
    JohnAndCharlesWesley,
    JohnCassian,
    JohnChrysostom,
    JohnColeridgePatteson,
    JohnDonne,
    JohnHenryHobart,
    JohnKeble,
    JohnMasonNeale,
    JohnOfDamascus,
    JohnOfTheCross,
    JohnRaleighMott,
    JohnXxiiiAngeloGiuseppeRoncalli,
    JonathanMyrickDaniels,
    JosephButler,
    JosephOfArimathea,
    JuanaInesDeLaCruz,
    JuliaChesterEmery,
    JulianOfNorwich,
    Justin,
    KamehamehaAndEmma,
    Kassiani,
    KateriTekakwitha,
    KatharinaVonBora,
    KatharinaZell,
    LancelotAndrewes,
    LaurenceOfRome,
    LeoOfRome,
    Louis,
    LucyOfSyracuse,
    LydiaOfThyatira,
    MacrinaOfCaesarea,
    MancheMasemola,
    MarcellaOfRome,
    MargaretOfCortona,
    MargaretOfScotland,
    MargaretWard,
    MariaSkobtsova,
    MarinaTheMonk,
    MartinLuther,
    MartinLutherKing,
    MartinOfTours,
    MartyrsOfTheReformationEra,
    MaryAndMarthaOfBethany,
    MaryOfEgypt,
    MaryamOfQidun,
    MechthildOfMagdeburg,
    MechthildeOfHackebornAndGertrudeTheGreat,
    MelaniaTheElder,
    Monica,
    MosesTheBlack,
    NicholasFerrar,
    NicholasOfMyra,
    Ninian,
    NinoOfGeorgia,
    OscarRomero,
    PachomiusOfTabenissi,
    PatrickOfIreland,
    PaulJones,
    PaulaAndEustochiumOfRome,
    PauliMurray,
    PerpetuaAndFelicity,
    PeterWilliamsCassey,
    PhilanderChase,
    Philip,
    PhillipsBrooks,
    Phoebe,
    Photini,
    PolycarpOfSmyrna,
    PriscillaAndAquila,
    RemigiusOfRheims,
    RichardHooker,
    RichardMeuxBenson,
    RichardOfChichester,
    RichardRolle,
    RobertGrosseteste,
    Andrew,
    Barnabas,
    Bartholomew,
    JamesOfJerusalem,
    James,
    John,
    Joseph,
    Luke,
    Mark,
    MaryMagdalene,
    Mary,
    Matthew,
    Matthias,
    Michael,
    SimonAndJude,
    Stephen,
    Thomas,
    SamuelIsaacJosephScherechewsky,
    Sarah,
    ScholasticaOfNursia,
    SergiusOfRadonezh,
    TabithaDorcasOfJoppa,
    TeresaOfAvila,
    Annunciation,
    PeterAndPaul,
    PhilipAndJames,
    TheBeheadingOfSaintJohnTheBaptist,
    ConfessionOfStPeter,
    SamuelSeabury,
    ConversionOfStPaul,
    Epiphany,
    EveOfHolyName,
    HolyName,
    TheMartyrsOfJapan,
    TheMartyrsOfMemphis,
    TheMartyrsOfNewGuinea,
    TheMartyrsOfUganda,
    ChristmasDay,
    NativityOfStJohnTheBaptist,
    TheNativityOfTheBlessedVirginMary,
    TheParentsOfTheBlessedVirginMary,
    ThePresentation,
    TheTransfiguration,
    TheVisitation,
    TheclaOfIconium,
    Theodora,
    TheodoreOfTarsus,
    ThereseOfLisieux,
    ThomasAKempis,
    ThomasAquinas,
    ThomasBecket,
    ThomasBray,
    ThomasGallaudetAndHenryWinterSyle,
    ThomasKen,
    ThurgoodMarshall,
    Tikhon,
    TitusAndTimothy,
    ToyohikoKagawa,
    VidaDuttonScudder,
    VincentDePaul,
    VincentOfSaragossa,
    WilliamAugustusMuhlenberg,
    WilliamLaud,
    WilliamLaw,
    WilliamPorcherDubose,
    WilliamReedHuntington,
    WilliamTemple,
    WilliamTyndale,
    WilliamWhite,
    WilliamWilberforce,
    Willibrord,
    WulfstanOfWorcester,
    Zenaida,
    ZitaOfTuscany,
    ChristmasEve,
    December24,
    December29,
    December30,
    December31,
    EveOfHolyCross,
    EveOfStJohnTheBaptist,
    EveOfTheAnnunciation,
    EveOfThePresentation,
    EveOfTheTransfiguration,
    EveOfTheVisitation,
    IndependenceDay,
    January10,
    January11,
    January12,
    January2,
    January3,
    January4,
    January5,
    EveOfEpiphany,
    January7,
    January8,
    January9,
    EveOfEpiphany1,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Time {
    AllDay,
    MorningOnly,
    EveningOnly,
}

/// (month, day, Feast, eve, not observed After this week begins (used for days between Epiphany and Epiphany 1))
pub type KalendarEntry = (HolyDayId, Feast, Time, Option<LiturgicalWeek>);
