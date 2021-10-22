use chrono::{Datelike, NaiveDate};
use std::convert::TryInto;

mod bcp1979;
mod calendar;
mod date;
mod error;
pub mod feasts;
mod holy_day;
pub mod lff2018;
mod liturgical_color;
mod liturgical_day;
mod liturgical_week;
pub mod propers;
mod rank;
mod sanctoral;
mod season;
mod year;
pub use self::calendar::Calendar;
pub use bcp1979::BCP1979_CALENDAR;
pub use date::{Date, Weekday};
pub use error::CalendarError;
pub use feasts::Feast;
pub use holy_day::HolyDay;
pub use liturgical_color::Color;
pub use liturgical_day::{LiturgicalDay, LiturgicalDayId};
pub use liturgical_week::LiturgicalWeek;
pub use propers::Proper;
pub use rank::Rank;
pub use sanctoral::Sanctoral;
pub use season::Season;
pub use year::{DailyOfficeYear, RCLYear};

/// Calculates the date of Easter as a [Date](Date) in any given year.
/// ```
/// # use calendar::easter_in_year;
/// use chrono::Datelike;
/// let year1 = 2020;
/// let year2 = 1983; // April 3
/// let year3 = 2027; // March 28
/// // Easter 2020: April 12
/// assert_eq!(easter_in_year(2020).month(), 4);
/// assert_eq!(easter_in_year(2020).day(), 12);
/// // Easter 1983: April 3
/// assert_eq!(easter_in_year(1983).month(), 4);
/// assert_eq!(easter_in_year(1983).day(), 3);
/// // Easter 2027: March 28
/// assert_eq!(easter_in_year(2027).month(), 3);
/// assert_eq!(easter_in_year(2027).day(), 28);
/// ```
#[allow(clippy::many_single_char_names)]
pub fn easter_in_year(year: u32) -> Date {
    // Computus: Meeus/Jones/Butcher algorithm
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let month = (h + l - 7 * m + 114) / 31;
    let day = ((h + l - 7 * m + 114) % 31) + 1;
    let naive_date = NaiveDate::from_ymd(year.try_into().unwrap(), month, day);
    naive_date.into()
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
pub fn sunday_before(date: Date) -> Date {
    let date = date.naive_date;
    let nth_weekday_from_sunday = date.weekday().num_days_from_sunday();
    let naive_date = date - chrono::Duration::days(nth_weekday_from_sunday.into());
    naive_date.into()
}
