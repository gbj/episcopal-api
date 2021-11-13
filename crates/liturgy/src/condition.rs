use calendar::{
    Calendar, Date, LiturgicalDay, LiturgicalDayId, LiturgicalWeek, Rank, Season, Weekday,
};

use crate::{ClientPreferences, PreferenceKey, PreferenceValue};

pub enum Condition {
    /// Occurs on all days, unless they match the given [LiturgicalDayId](calendar::LiturgicalDayId)
    DayExcept(Vec<LiturgicalDayId>),
    /// Occurs only on the day matching the given [LiturgicalDayId](calendar::LiturgicalDayId)
    DayOnly(Vec<LiturgicalDayId>),
    /// Occurs on all days, except during the given [Season](calendar::Season)
    SeasonExcept(Vec<Season>),
    /// Occurs only on days during the given [Season](calendar::Season)
    SeasonOnly(Vec<Season>),
    /// Day has a rank >= the [Rank](calendar::Rank) given
    RankAbove(Rank),
    /// Occurs on all days, unless they are one of the given [Weekday](calendar::Weekday)s
    WeekdayExcept(Vec<Weekday>),
    /// Occurs only on days that are one of the given [Weekday](calendar::Weekday)s
    WeekdayOnly(Vec<Weekday>),
    /// Occurs on all days, unless they are during one of the given [LiturgicalWeek](calendar::LiturgicalWeek)s
    WeekExcept(Vec<LiturgicalWeek>),
    /// Occurs only on days that are one of the given [LiturgicalWeek](calendar::LiturgicalWeek)s
    WeekOnly(Vec<LiturgicalWeek>),
    /// Occurs only on dates earlier than the given (month, day) pair in the current calendar year
    DateLt(u8, u8),
    /// Occurs only on dates earlier than or on the given (month, day) pair in the current calendar year
    DateLte(u8, u8),
    /// Occurs only on dates later than the given (month, day) pair in the current calendar year
    DateGt(u8, u8),
    /// Occurs only on dates later than or on the given (month, day) pair in the current calendar year
    DateGte(u8, u8),
    /// Occurs only on the given day of the month
    DayOfMonthEq(u8),
    /// Occurs only on days that are not the given day of the month
    DayOfMonthNeq(u8),
    /// Occurs only if the user's chosen preferences include the given key-value pair
    PreferenceEq(PreferenceKey, PreferenceValue),
    /// Occurs only if the user's chosen preferences do not include the given key-value pair
    PreferenceNeq(PreferenceKey, PreferenceValue),
}

impl Condition {
    pub fn include(
        &self,
        calendar: &Calendar,
        day: &LiturgicalDay,
        prefs: impl ClientPreferences,
    ) -> bool {
        let observed = day.observed;
        match self {
            Condition::DayExcept(ids) => !ids.contains(&observed),
            Condition::DayOnly(ids) => ids.contains(&observed),
            Condition::SeasonExcept(seasons) => !seasons.contains(&calendar.season(day)),
            Condition::SeasonOnly(seasons) => !seasons.contains(&calendar.season(day)),
            Condition::RankAbove(rank) => calendar.rank(day) >= *rank,
            Condition::WeekdayExcept(weekdays) => !weekdays.contains(&day.weekday),
            Condition::WeekdayOnly(weekdays) => weekdays.contains(&day.weekday),
            Condition::WeekExcept(weeks) => !weeks.contains(&day.week),
            Condition::WeekOnly(weeks) => weeks.contains(&day.week),
            Condition::DateLt(m, d) => day.date < Date::from_ymd(day.date.year(), *m, *d),
            Condition::DateLte(m, d) => day.date <= Date::from_ymd(day.date.year(), *m, *d),
            Condition::DateGt(m, d) => day.date > Date::from_ymd(day.date.year(), *m, *d),
            Condition::DateGte(m, d) => day.date >= Date::from_ymd(day.date.year(), *m, *d),
            Condition::DayOfMonthEq(d) => day.date.day() == *d,
            Condition::DayOfMonthNeq(d) => day.date.day() != *d,
            Condition::PreferenceEq(key, value) => prefs.value(key) == value,
            Condition::PreferenceNeq(key, value) => prefs.value(key) != value,
        }
    }
}
