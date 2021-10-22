use serde::{Deserialize, Serialize};

use crate::{Date, LiturgicalWeek};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub enum DailyOfficeYear {
    One,
    Two,
}

impl DailyOfficeYear {
    /// Calculates the year in the Daily Office Lectionary.
    /// ```
    /// # use crate::calendar::{Date, LiturgicalWeek, DailyOfficeYear};
    /// // see BCP p. 934
    /// let christmas_1976 = Date::from_ymd(1976, 12, 25);
    /// assert_eq!(DailyOfficeYear::new(christmas_1976, LiturgicalWeek::Christmas), DailyOfficeYear::One);
    /// ```
    pub fn new(date: Date, week: LiturgicalWeek) -> Self {
        let base_year = if is_advent(week) || date.month() == 12 {
            date.year()
        } else {
            date.year() - 1
        };

        if base_year % 2 == 0 {
            DailyOfficeYear::One
        } else {
            DailyOfficeYear::Two
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub enum RCLYear {
    A,
    B,
    C,
}

impl RCLYear {
    /// Calculates the year in the Revised Common Lectionary.
    /// ```
    /// # use crate::calendar::{Date, LiturgicalWeek, RCLYear};
    /// let year_a_example = Date::from_ymd(2019, 12, 8);
    /// let year_b_example = Date::from_ymd(2021, 10, 24);
    /// assert_eq!(RCLYear::new(year_a_example, LiturgicalWeek::Advent2), RCLYear::A);
    /// assert_eq!(RCLYear::new(year_b_example, LiturgicalWeek::Pentecost22), RCLYear::B);
    /// ```
    pub fn new(date: Date, week: LiturgicalWeek) -> Self {
        let base_year = if is_advent(week) || date.month() == 12 {
            date.year()
        } else {
            date.year() - 1
        };

        let offset = base_year % 3;

        if offset == 0 {
            RCLYear::A
        } else if offset == 1 {
            RCLYear::B
        } else {
            RCLYear::C
        }
    }
}

fn is_advent(week: LiturgicalWeek) -> bool {
    matches!(
        week,
        LiturgicalWeek::Advent1
            | LiturgicalWeek::Advent2
            | LiturgicalWeek::Advent3
            | LiturgicalWeek::Advent4
    )
}
