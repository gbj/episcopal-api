use std::convert::TryInto;

use crate::{
    easter_in_year, feasts::KalendarEntry, holy_day::HolyDayId, liturgical_day::LiturgicalDayId,
    liturgical_week::Cycle, propers::calculate_proper, sunday_before, DailyOfficeYear, Date, Feast,
    LiturgicalDay, LiturgicalWeek, Proper, RCLYear, Rank, Weekday,
};

/// The settings for a particular calendar. Different calendars vary slightly
/// in the way their liturgical cycles are set up relative to Christmas and Easter.
/// Based on this structure, we can generate a [LiturgicalWeek](LiturgicalWeek)
/// and [LiturgicalDay](LiturgicalDay) from any date.
pub struct Calendar {
    /// How many weeks before Easter the Easter cycle begins in the calendar
    pub easter_cycle_begins: u8,
    /// How many weeks before Christmas the Christmas cycle begins in the calendar
    pub christmas_cycle_begins: u8,
    /// Whether to use `Proper 1`, `Proper 2`, etc. for weeks after Pentecost
    pub has_propers: bool,
    /// Maps nth week of cycle onto the liturgical week identifier
    /// i.e., in the Episcopal Church calendar the 1st week of the Christmas cycle is Christ the King
    // TODO: benchmark against a HashMap — I'm making the assumption that O(n) for small n is better than O(1) given the cost of hashing
    pub weeks: &'static [(Cycle, u8, LiturgicalWeek)],
    /// All holy days in the calendar
    pub holy_days: &'static [KalendarEntry],
    /// Ranks of holy days in this calendar
    pub holy_day_ranks: &'static [(Feast, Rank)],
}

impl Calendar {
    /// The [LiturgicalDay](LiturgicalDay) that is observed on a given date,
    /// including any feasts or special observances.
    /// ```
    /// # use crate::calendar::{BCP1979_CALENDAR, Date, Weekday, LiturgicalWeek, DailyOfficeYear, RCLYear, Feast, LiturgicalDayId};
    /// let date = Date::from_ymd(2020, 5, 21);
    /// let thursday_easter_6 = BCP1979_CALENDAR.liturgical_day(date, false);
    /// assert_eq!(thursday_easter_6.week, LiturgicalWeek::Easter6);
    /// assert_eq!(thursday_easter_6.weekday, Weekday::Thu);
    /// assert_eq!(thursday_easter_6.daily_office_year, DailyOfficeYear::Two);
    /// assert_eq!(thursday_easter_6.rcl_year, RCLYear::A);
    /// assert_eq!(thursday_easter_6.holy_days, vec![Feast::AscensionDay]);
    /// assert_eq!(thursday_easter_6.proper, None);
    /// assert_eq!(thursday_easter_6.observed, LiturgicalDayId::Feast(Feast::AscensionDay));
    /// ```
    pub fn liturgical_day(&self, date: Date, evening: bool) -> LiturgicalDay {
        let weekday = date.weekday();
        let week = self.liturgical_week(date);
        let proper = self.proper(date, week);
        let holy_days = self.holy_days(date, week, evening).collect::<Vec<_>>();
        let observed = self.observed_day(week, proper, weekday, &holy_days);
        LiturgicalDay {
            date,
            evening,
            week,
            weekday,
            daily_office_year: DailyOfficeYear::new(date, week),
            rcl_year: RCLYear::new(date, week),
            holy_days,
            proper,
            observed,
        }
    }

    /// The [LiturgicalWeek](LiturgicalWeek) within which a given date falls,
    /// ignoring any feasts or special observances.
    fn liturgical_week(&self, date: Date) -> LiturgicalWeek {
        let index = self.liturgical_week_index(date);
        self.weeks
            .iter()
            .find(|(cycle, offset, _)| index.cycle == *cycle && index.week == *offset)
            .map(|(_, _, week)| *week)
            .unwrap_or(LiturgicalWeek::None)
    }

    /// For calendars that use the Proper ____ system, gives the [Proper](Proper)
    /// within which the date falls, if any.
    fn proper(&self, date: Date, week: LiturgicalWeek) -> Option<Proper> {
        if self.has_propers && week >= LiturgicalWeek::Pentecost {
            calculate_proper(date)
        } else {
            None
        }
    }

    /// The rank of the given feast day in this calendar
    fn feast_day_rank(&self, feast: &Feast) -> Rank {
        self.holy_day_ranks
            .iter()
            .find(|(search_feast, _)| search_feast == feast)
            .map(|(_, rank)| *rank)
            .unwrap_or(Rank::OptionalObservance)
    }

    fn holy_days(
        &self,
        date: Date,
        week: LiturgicalWeek,
        evening: bool,
    ) -> impl Iterator<Item = Feast> {
        let month = date.month();
        let day = date.day();
        let weekday = date.weekday();
        self.holy_days
            .iter()
            .filter_map(move |(id, feast, f_evening)| match id {
                HolyDayId::Date(f_month, f_day) => {
                    if *f_month == month && *f_day == day && (!evening || *f_evening == evening) {
                        Some(*feast)
                    } else {
                        None
                    }
                }
                HolyDayId::SpecialDay(f_week, f_weekday) => {
                    if *f_week == week && *f_weekday == weekday {
                        Some(*feast)
                    } else {
                        None
                    }
                }
                // TODO
                HolyDayId::DayOfMonth { month, week, day } => todo!(),
            })
    }

    fn observed_day(
        &self,
        week: LiturgicalWeek,
        proper: Option<Proper>,
        weekday: Weekday,
        holy_days: &Vec<Feast>,
    ) -> LiturgicalDayId {
        if holy_days.is_empty() {
            self.observed_day_from_week_or_proper(week, proper, weekday)
        } else {
            let mut observable_feasts = holy_days
                .iter()
                .filter(|feast| self.feast_day_rank(feast) >= Rank::HolyDay)
                .collect::<Vec<_>>();
            observable_feasts.sort_by_key(|feast| self.feast_day_rank(feast));
            if observable_feasts.is_empty() {
                self.observed_day_from_week_or_proper(week, proper, weekday)
            } else {
                let highest_ranking_feast = observable_feasts[0];
                LiturgicalDayId::Feast(*highest_ranking_feast)
            }
        }
    }

    fn observed_day_from_week_or_proper(
        &self,
        week: LiturgicalWeek,
        proper: Option<Proper>,
        weekday: Weekday,
    ) -> LiturgicalDayId {
        if let Some(proper) = proper {
            LiturgicalDayId::ProperAndDay(proper, weekday)
        } else {
            LiturgicalDayId::WeekAndDay(week, weekday)
        }
    }

    fn liturgical_week_index(&self, date: Date) -> LiturgicalWeekIndex {
        let year = date.year();
        let easter = easter_in_year(year.into());
        let christmas_eve = Date::from_ymd(year, 12, 24);
        let last_epiphany = sunday_before(easter.subtract_weeks(self.easter_cycle_begins));
        let fourth_advent = sunday_before(christmas_eve);
        let last_pentecost =
            sunday_before(fourth_advent.subtract_weeks(self.christmas_cycle_begins));
        if date >= last_pentecost || date < last_epiphany {
            self.christmas_cycle_week(date)
        } else {
            self.easter_cycle_week(date, easter)
        }
    }

    fn christmas_cycle_week(&self, date: Date) -> LiturgicalWeekIndex {
        // year of Christmas is either the current year or, if we're in January/February, the previous year
        let christmas_year = if date.month() >= 10 {
            date.year()
        } else {
            date.year() - 1
        };

        // date of Christmas in this liturgical year
        let christmas = Date::from_ymd(christmas_year, 12, 25);
        let christmas_eve = Date::from_ymd(christmas_year, 12, 24);
        let epiphany = Date::from_ymd(christmas_year + 1, 1, 6);

        // If in Advent...
        if date <= christmas_eve {
            let advent_4 = sunday_before(christmas_eve);
            let weeks_from_advent_4 = sunday_before(date) - advent_4;
            let week = weeks_from_advent_4.num_weeks() + 4;
            LiturgicalWeekIndex {
                cycle: Cycle::Advent,
                week: week.try_into().unwrap(),
                proper: None,
            }
        }
        // Christmas
        else if date < epiphany {
            let week = date - sunday_before(christmas);
            LiturgicalWeekIndex {
                cycle: Cycle::Christmas,
                week: week.num_weeks().try_into().unwrap(),
                proper: None,
            }
        }
        // Epiphany
        else {
            let week = date - sunday_before(epiphany);
            LiturgicalWeekIndex {
                cycle: Cycle::Epiphany,
                week: week.num_weeks().try_into().unwrap(),
                proper: None,
            }
        }
    }

    fn easter_cycle_week(&self, date: Date, easter: Date) -> LiturgicalWeekIndex {
        let weeks_from_easter: u8 = (date - easter).num_weeks().try_into().unwrap();
        let week = weeks_from_easter + self.easter_cycle_begins;
        let proper = if week >= 14 && self.has_propers {
            calculate_proper(date)
        } else {
            None
        };
        LiturgicalWeekIndex {
            cycle: Cycle::Easter,
            week,
            proper,
        }
    }
}
struct LiturgicalWeekIndex {
    cycle: Cycle,
    week: u8,
    proper: Option<Proper>,
}
