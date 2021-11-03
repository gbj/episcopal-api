mod lectionaries;
mod reading;
mod reading_type;

pub use lectionaries::{BCP1979_30_DAY_PSALTER, BCP1979_DAILY_OFFICE_LECTIONARY};
pub use reading::Reading;
pub use reading_type::ReadingType;

use calendar::{LiturgicalDay, LiturgicalDayId, Year, YearType};

/// Represents a given lectionary cycle of readings, e.g., the Revised Common Lectionary
/// or the 1979 Book of Common Prayer Daily Office Lectionary.
/// Given a [LiturgicalDay](crate::calendar::LiturgicalDay), it can give either all of the readings,
/// or a specific reading.
pub struct Lectionary {
    pub year_type: YearType,
    pub readings: &'static [(LiturgicalDayId, Year, ReadingType, &'static str)],
}

impl Lectionary {
    pub fn readings_by_day(
        &'static self,
        observed: &LiturgicalDayId,
        day: &LiturgicalDay,
    ) -> impl Iterator<Item = Reading> {
        let year = match self.year_type {
            YearType::Rcl => Year::Rcl(day.rcl_year),
            YearType::DailyOffice => Year::DailyOffice(day.daily_office_year),
            YearType::None => Year::Any,
        };

        let observed = if let LiturgicalDayId::TransferredFeast(feast) = observed {
            LiturgicalDayId::Feast(*feast)
        } else {
            *observed
        };

        self.readings
            .iter()
            .filter(move |(search_id, search_year, _, _)| {
                *search_id == observed && (*search_year == year || *search_year == Year::Any)
            })
            .map(|(_, _, reading_type, citation)| Reading::new(*reading_type, citation.to_string()))
    }

    pub fn reading_by_type(
        &'static self,
        observed: &LiturgicalDayId,
        day: &LiturgicalDay,
        reading_type: ReadingType,
    ) -> impl Iterator<Item = Reading> {
        self.readings_by_day(observed, day)
            .filter(move |reading| reading.reading_type == reading_type)
    }
}
