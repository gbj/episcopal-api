use std::{convert::TryInto, ops::Sub};

use chrono::Datelike;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    pub(crate) naive_date: chrono::NaiveDate,
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.naive_date.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let naive_date = chrono::NaiveDate::deserialize(deserializer)?;
        Ok(Self { naive_date })
    }
}

impl Date {
    /// Creates Date from a year, month, and day.
    pub fn from_ymd(year: u16, month: u8, day: u8) -> Date {
        let naive_date = chrono::NaiveDate::from_ymd(year.into(), month.into(), day.into());
        Self { naive_date }
    }

    pub fn year(&self) -> u16 {
        // relatively because we only construct dates with a u16 year
        self.naive_date.year().try_into().unwrap()
    }

    pub fn month(&self) -> u8 {
        // relatively because we only construct dates with a u8 month
        self.naive_date.month().try_into().unwrap()
    }

    pub fn day(&self) -> u8 {
        // relatively because we only construct dates with a u8 day
        self.naive_date.day().try_into().unwrap()
    }

    pub fn weekday(&self) -> Weekday {
        self.naive_date.weekday()
    }

    pub fn add_weeks(&self, weeks: impl Into<i64>) -> Self {
        let naive_date = self.naive_date + chrono::Duration::weeks(weeks.into());
        Self { naive_date }
    }

    pub fn subtract_weeks(&self, weeks: impl Into<i64>) -> Self {
        let naive_date = self.naive_date - chrono::Duration::weeks(weeks.into());
        Self { naive_date }
    }

    pub fn add_days(&self, weeks: impl Into<i64>) -> Self {
        let naive_date = self.naive_date + chrono::Duration::days(weeks.into());
        Self { naive_date }
    }

    pub fn subtract_days(&self, weeks: impl Into<i64>) -> Self {
        let naive_date = self.naive_date - chrono::Duration::days(weeks.into());
        Self { naive_date }
    }

    /// Calculates the [Date](Date) of the Sunday before the given date.
    /// ```
    /// # use calendar::sunday_before;
    /// use chrono::{NaiveDate, Datelike};
    /// let test_1 = sunday_before(NaiveDate::from_ymd(2020, 5, 21));
    /// assert_eq!(test_1.month(), 5);
    /// assert_eq!(test_1.day(), 17);
    /// // Wraps to previous month
    /// let test_2 = sunday_before(NaiveDate::from_ymd(2020, 4, 1));
    /// assert_eq!(test_2.month(), 3);
    /// assert_eq!(test_2.day(), 29);
    /// // Wraps to previous year
    /// let test_3 = sunday_before(NaiveDate::from_ymd(2020, 1, 4));
    /// assert_eq!(test_3.month(), 12);
    /// assert_eq!(test_3.day(), 29);
    /// // On Sundays, returns the same day
    /// let test_4 = sunday_before(NaiveDate::from_ymd(2021, 10, 3));
    /// assert_eq!(test_4.month(), 10);
    /// assert_eq!(test_4.day(), 3);
    /// ```
    pub fn sunday_before(&self) -> Date {
        let date = self.naive_date;
        let nth_weekday_from_sunday = date.weekday().num_days_from_sunday();
        let naive_date = date - chrono::Duration::days(nth_weekday_from_sunday.into());
        naive_date.into()
    }
}

pub type Weekday = chrono::Weekday;
pub type Duration = chrono::Duration;

impl From<chrono::NaiveDate> for Date {
    fn from(naive_date: chrono::NaiveDate) -> Self {
        Self { naive_date }
    }
}

impl From<Date> for chrono::NaiveDate {
    fn from(date: Date) -> Self {
        date.naive_date
    }
}

impl Sub for Date {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.naive_date - rhs.naive_date
    }
}
