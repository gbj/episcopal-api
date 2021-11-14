use calendar::{
    Calendar, Date, LiturgicalDay, LiturgicalDayId, LiturgicalWeek, Rank, Season, Weekday,
};
use serde::{Deserialize, Serialize};

use crate::{ClientPreferences, PreferenceKey, PreferenceValue};

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Condition {
    /// # Simple conditions
    /// Included if the given day matches this [LiturgicalDayId](calendar::LiturgicalDayId).
    /// ```
    /// # use crate::liturgy::{Condition, PreferenceKey, PreferenceValue};
    /// # use calendar::{Date, LiturgicalDayId, Feast, BCP1979_CALENDAR};
    /// # let prefs : [(PreferenceKey, PreferenceValue); 0] = [];
    /// let good_friday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 15), false);
    /// let condition = Condition::Day(LiturgicalDayId::Feast(Feast::GoodFriday));
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &good_friday, &prefs), true);
    /// let condition = Condition::Day(LiturgicalDayId::Feast(Feast::HolySaturday));
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &good_friday, &prefs), false);
    /// ```
    Day(LiturgicalDayId),
    /// Included if the given day falls in this [Season](calendar::Season).
    /// ```
    /// # use crate::liturgy::{Condition, PreferenceKey, PreferenceValue};
    /// # use calendar::{Date, LiturgicalDayId, Feast, Season, BCP1979_CALENDAR};
    /// # let prefs : [(PreferenceKey, PreferenceValue); 0] = [];
    /// // applies for days that are unambiguously during a season
    /// let good_friday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 15), false);
    /// let condition = Condition::Season(Season::HolyWeek);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &good_friday, &prefs), true);
    /// let condition = Condition::Season(Season::Easter);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &good_friday, &prefs), false);
    /// // also applies for something like a holy day, when the season is overridden on that day
    /// let condition = Condition::Season(Season::Lent);
    /// let annunciation = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 3, 25), false);
    /// assert_ne!(BCP1979_CALENDAR.season(&annunciation), Season::Lent);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &annunciation, &prefs), true);
    /// ```
    Season(Season),
    /// Included if the given day falls in this [LiturgicalWeek](calendar::LiturgicalWeek).
    /// ```
    /// # use crate::liturgy::{Condition, PreferenceKey, PreferenceValue};
    /// # use calendar::{Date, LiturgicalDayId, Feast, LiturgicalWeek, BCP1979_CALENDAR};
    /// # let prefs : [(PreferenceKey, PreferenceValue); 0] = [];
    /// let good_friday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 15), false);
    /// let condition = Condition::Week(LiturgicalWeek::HolyWeek);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &good_friday, &prefs), true);
    /// let condition = Condition::Week(LiturgicalWeek::Lent1);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &good_friday, &prefs), false);
    /// ```
    Week(LiturgicalWeek),
    /// Included if the given day is this [Weekday](calendar::Weekday).
    /// ```
    /// # use crate::liturgy::{Condition, PreferenceKey, PreferenceValue};
    /// # use calendar::{Date, LiturgicalDayId, Feast, Weekday, BCP1979_CALENDAR};
    /// # let prefs : [(PreferenceKey, PreferenceValue); 0] = [];
    /// let good_friday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 15), false);
    /// let condition = Condition::Weekday(Weekday::Fri);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &good_friday, &prefs), true);
    /// let condition = Condition::Weekday(Weekday::Sat);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &good_friday, &prefs), false);
    /// ```
    Weekday(Weekday),
    /// Included if the day has a rank >= the [Rank](calendar::Rank) given.
    /// ```
    /// # use crate::liturgy::{Condition, PreferenceKey, PreferenceValue};
    /// # use calendar::{Date, LiturgicalDayId, Feast, Rank, BCP1979_CALENDAR};
    /// # let prefs : [(PreferenceKey, PreferenceValue); 0] = [];
    /// let annunciation = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 3, 25), false);
    /// let next_day = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 3, 26), false);
    /// let condition = Condition::RankGte(Rank::HolyDay);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &annunciation, &prefs), true);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &next_day, &prefs), false);
    /// let condition = Condition::RankGte(Rank::PrincipalFeast);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &annunciation, &prefs), false);
    /// ```
    RankGte(Rank),
    /// Included only on dates earlier than the given (month, day) pair in the current calendar year.
    /// ```
    /// # use crate::liturgy::{Condition, PreferenceKey, PreferenceValue};
    /// # use calendar::{Date, LiturgicalDayId, Feast, BCP1979_CALENDAR};
    /// # let prefs : [(PreferenceKey, PreferenceValue); 0] = [];
    /// let annunciation = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 3, 25), false);
    /// let epiphany = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 1, 6), false);
    /// let presentation = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 2, 2), false);
    /// let christmas = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2021, 12, 25), false);
    /// let condition = Condition::DateLt(2, 2);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &annunciation, &prefs), false);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &presentation, &prefs), false);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &epiphany, &prefs), true);
    /// // note that this doesn't wrap around years; Christmas 2021 appears to be "later" than 2/2
    /// // because we only compare dates in the same year, and 12/25/2022 is later than 2/2/2022
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &christmas, &prefs), false);
    /// ```
    DateLt(u8, u8),
    /// Included only on dates earlier than or on the given (month, day) pair in the current calendar year.
    /// ```
    /// # use crate::liturgy::{Condition, PreferenceKey, PreferenceValue};
    /// # use calendar::{Date, LiturgicalDayId, Feast, BCP1979_CALENDAR};
    /// # let prefs : [(PreferenceKey, PreferenceValue); 0] = [];
    /// let annunciation = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 3, 25), false);
    /// let epiphany = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 1, 6), false);
    /// let presentation = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 2, 2), false);
    /// let christmas = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2021, 12, 25), false);
    /// let condition = Condition::DateLte(2, 2);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &annunciation, &prefs), false);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &presentation, &prefs), true);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &epiphany, &prefs), true);
    /// // note that this doesn't wrap around years; Christmas 2021 appears to be "later" than 2/2
    /// // because we only compare dates in the same year, and 12/25/2022 is later than 2/2/2022
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &christmas, &prefs), false);
    /// ```
    DateLte(u8, u8),
    /// Included only on dates later than the given (month, day) pair in the current calendar year.
    /// ```
    /// # use crate::liturgy::{Condition, PreferenceKey, PreferenceValue};
    /// # use calendar::{Date, LiturgicalDayId, Feast, BCP1979_CALENDAR};
    /// # let prefs : [(PreferenceKey, PreferenceValue); 0] = [];
    /// let annunciation = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 3, 25), false);
    /// let epiphany = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 1, 6), false);
    /// let presentation = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 2, 2), false);
    /// let christmas = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2021, 12, 25), false);
    /// let condition = Condition::DateGt(2, 2);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &annunciation, &prefs), true);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &presentation, &prefs), false);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &epiphany, &prefs), false);
    /// // note that this doesn't wrap around years; Christmas 2021 appears to be "later" than 2/2
    /// // because we only compare dates in the same year, and 12/25/2022 is later than 2/2/2022
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &christmas, &prefs), true);
    /// ```
    DateGt(u8, u8),
    /// Included only on dates later than or on the given (month, day) pair in the current calendar year.
    /// ```
    /// # use crate::liturgy::{Condition, PreferenceKey, PreferenceValue};
    /// # use calendar::{Date, LiturgicalDayId, Feast, BCP1979_CALENDAR};
    /// # let prefs : [(PreferenceKey, PreferenceValue); 0] = [];
    /// let annunciation = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 3, 25), false);
    /// let epiphany = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 1, 6), false);
    /// let presentation = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 2, 2), false);
    /// let christmas = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2021, 12, 25), false);
    /// let condition = Condition::DateGte(2, 2);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &annunciation, &prefs), true);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &presentation, &prefs), true);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &epiphany, &prefs), false);
    /// // note that this doesn't wrap around years; Christmas 2021 appears to be "later" than 2/2
    /// // because we only compare dates in the same year, and 12/25/2022 is later than 2/2/2022
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &christmas, &prefs), true);
    /// ```
    DateGte(u8, u8),
    /// Included only on the given day of the month.
    /// ```
    /// # use crate::liturgy::{Condition, PreferenceKey, PreferenceValue};
    /// # use calendar::{Date, LiturgicalDayId, Feast, BCP1979_CALENDAR};
    /// # let prefs : [(PreferenceKey, PreferenceValue); 0] = [];
    /// let annunciation = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 3, 25), false);
    /// let epiphany = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 1, 6), false);
    /// let condition = Condition::DayOfMonth(25);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &annunciation, &prefs), true);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &epiphany, &prefs), false);
    /// ```
    DayOfMonth(u8),
    /// Included only if the user's chosen preferences include the given key-value pair.
    /// ```
    /// # use crate::liturgy::{Condition, PreferenceKey, PreferenceValue};
    /// # use calendar::{Date, LiturgicalDayId, Feast, BCP1979_CALENDAR};
    /// # let day = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 3, 25), false);
    /// # use std::collections::HashMap;
    /// let mut prefs = HashMap::new();
    /// prefs.insert(String::from("bibleVersion"), String::from("NRSV"));
    /// let condition = Condition::Preference(String::from("bibleVersion"), String::from("NRSV"));
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &day, &prefs), true);
    /// prefs.insert(String::from("bibleVersion"), String::from("ESV"));
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &day, &prefs), false);
    /// ```
    Preference(PreferenceKey, PreferenceValue),

    /// # Combining conditions — used for basic logic

    /// Included only if the child [Condition] is not included.
    /// ```
    /// # use crate::liturgy::{Condition, PreferenceKey, PreferenceValue};
    /// # use calendar::{Date, LiturgicalDayId, Feast, BCP1979_CALENDAR};
    /// # let prefs : [(PreferenceKey, PreferenceValue); 0] = [];
    /// let annunciation = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 3, 25), false);
    /// let epiphany = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 1, 6), false);
    /// let condition = Condition::Not(Box::new(Condition::DayOfMonth(6)));
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &annunciation, &prefs), true);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &epiphany, &prefs), false);
    /// ```
    Not(Box<Condition>),
    /// Included only if both branch [Condition]s are included.
    /// ```
    /// # use crate::liturgy::{Condition, PreferenceKey, PreferenceValue};
    /// # use calendar::{Date, LiturgicalDayId, Feast, Season, Weekday, BCP1979_CALENDAR};
    /// # let prefs : [(PreferenceKey, PreferenceValue); 0] = [];
    /// let good_friday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 15), false);
    /// let holy_saturday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 16), false);
    /// let condition = Condition::And(Box::new(Condition::Weekday(Weekday::Fri)), Box::new(Condition::Season(Season::HolyWeek)));
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &good_friday, &prefs), true);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &holy_saturday, &prefs), false);
    /// ```
    And(Box<Condition>, Box<Condition>),
    /// Included if either branch [Condition] is included.
    ///  ```
    /// # use crate::liturgy::{Condition, PreferenceKey, PreferenceValue};
    /// # use calendar::{Date, LiturgicalDayId, Feast, Season, Weekday, BCP1979_CALENDAR};
    /// # let prefs : [(PreferenceKey, PreferenceValue); 0] = [];
    /// let good_friday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 15), false);
    /// let holy_saturday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 16), false);
    /// let easter_sunday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 17), false);
    /// let easter_friday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 22), false);
    /// let condition = Condition::Or(Box::new(Condition::Weekday(Weekday::Fri)), Box::new(Condition::Season(Season::HolyWeek)));
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &good_friday, &prefs), true);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &holy_saturday, &prefs), true);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &easter_sunday, &prefs), false);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &easter_friday, &prefs), true);
    /// ```
    Or(Box<Condition>, Box<Condition>),
    /// Included if any child [Condition]s are included.
    /// ```
    /// # use crate::liturgy::{Condition, PreferenceKey, PreferenceValue};
    /// # use calendar::{Date, LiturgicalDayId, Feast, Season, Weekday, BCP1979_CALENDAR};
    /// # let prefs : [(PreferenceKey, PreferenceValue); 0] = [];
    /// let good_friday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 15), false);
    /// let holy_saturday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 16), false);
    /// let easter_sunday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 17), false);
    /// let easter_friday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 22), false);
    /// let condition = Condition::Any(vec![Condition::Weekday(Weekday::Fri), Condition::Season(Season::HolyWeek)]);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &good_friday, &prefs), true);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &holy_saturday, &prefs), true);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &easter_sunday, &prefs), false);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &easter_friday, &prefs), true);
    /// ```
    Any(Vec<Condition>),
    /// Included only if all child [Condition]s are included.
    /// ```
    /// # use crate::liturgy::{Condition, PreferenceKey, PreferenceValue};
    /// # use calendar::{Date, LiturgicalDayId, Feast, Season, Weekday, BCP1979_CALENDAR};
    /// # let prefs : [(PreferenceKey, PreferenceValue); 0] = [];
    /// let good_friday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 15), false);
    /// let holy_saturday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 16), false);
    /// let easter_sunday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 17), false);
    /// let easter_friday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 22), false);
    /// let condition = Condition::All(vec![Condition::Weekday(Weekday::Fri), Condition::Season(Season::HolyWeek)]);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &good_friday, &prefs), true);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &holy_saturday, &prefs), false);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &easter_sunday, &prefs), false);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &easter_friday, &prefs), false);
    /// ```
    All(Vec<Condition>),
    /// Included only if none of the child [Condition]s are included.
    /// ```
    /// # use crate::liturgy::{Condition, PreferenceKey, PreferenceValue};
    /// # use calendar::{Date, LiturgicalDayId, Feast, Season, Weekday, BCP1979_CALENDAR};
    /// # let prefs : [(PreferenceKey, PreferenceValue); 0] = [];
    /// let good_friday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 15), false);
    /// let holy_saturday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 16), false);
    /// let easter_sunday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 17), false);
    /// let easter_friday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 4, 22), false);
    /// let condition = Condition::None(vec![Condition::Weekday(Weekday::Fri), Condition::Season(Season::HolyWeek)]);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &good_friday, &prefs), false);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &holy_saturday, &prefs), false);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &easter_sunday, &prefs), true);
    /// assert_eq!(condition.include(&BCP1979_CALENDAR, &easter_friday, &prefs), false);
    /// ```
    None(Vec<Condition>),
}

impl Condition {
    pub fn include(
        &self,
        calendar: &Calendar,
        day: &LiturgicalDay,
        prefs: &impl ClientPreferences,
    ) -> bool {
        match self {
            Condition::Day(id) => day.observed == *id,
            Condition::Season(season) => {
                calendar.season(day) == *season || calendar.base_season(day) == *season
            }
            Condition::Week(week) => day.week == *week,
            Condition::Weekday(weekday) => day.date.weekday() == *weekday,
            Condition::RankGte(rank) => calendar.rank(day) >= *rank,
            Condition::DateLt(m, d) => day.date < Date::from_ymd(day.date.year(), *m, *d),
            Condition::DateLte(m, d) => day.date <= Date::from_ymd(day.date.year(), *m, *d),
            Condition::DateGt(m, d) => day.date > Date::from_ymd(day.date.year(), *m, *d),
            Condition::DateGte(m, d) => day.date >= Date::from_ymd(day.date.year(), *m, *d),
            Condition::DayOfMonth(d) => day.date.day() == *d,
            Condition::Preference(key, value) => prefs.value(key) == *value,
            Condition::Not(cond) => !cond.include(calendar, day, prefs),
            Condition::And(a, b) => {
                a.include(calendar, day, prefs) && b.include(calendar, day, prefs)
            }
            Condition::Or(a, b) => {
                a.include(calendar, day, prefs) || b.include(calendar, day, prefs)
            }
            Condition::Any(conds) => {
                if conds.is_empty() {
                    true
                } else {
                    conds.iter().any(|cond| cond.include(calendar, day, prefs))
                }
            }
            Condition::All(conds) => {
                if conds.is_empty() {
                    true
                } else {
                    conds.iter().all(|cond| cond.include(calendar, day, prefs))
                }
            }
            Condition::None(conds) => {
                if conds.is_empty() {
                    true
                } else {
                    !conds.iter().any(|cond| cond.include(calendar, day, prefs))
                }
            }
        }
    }
}
