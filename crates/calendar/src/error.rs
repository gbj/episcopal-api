use thiserror::Error;

#[derive(Error, Debug)]
pub enum CalendarError {
    #[error("cannot calculate Christian liturgical dates for years Before Christ")]
    YearBeforeAD,
}
