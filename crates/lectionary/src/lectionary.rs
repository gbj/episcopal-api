use serde::{Deserialize, Serialize};

use calendar::{DailyOfficeYear, LiturgicalDay, LiturgicalDayId, RCLYear};

use crate::{Reading, ReadingType};

/// Represents a given lectionary cycle of readings, e.g., the Revised Common Lectionary
/// or the 1979 Book of Common Prayer Daily Office Lectionary.
/// Given a [LiturgicalDay](crate::calendar::LiturgicalDay), it can give either all of the readings,
/// or a specific reading.
pub struct Lectionary {
    pub year_type: YearType,
    pub readings: &'static [(LiturgicalDayId, Year, ReadingType, &'static str)],
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum YearType {
    Rcl,
    DailyOffice,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Year {
    DailyOffice(DailyOfficeYear),
    Rcl(RCLYear),
    Any,
}

impl Lectionary {
    pub fn readings_by_day(&'static self, day: &LiturgicalDay) -> impl Iterator<Item = Reading> {
        let year = match self.year_type {
            YearType::Rcl => Year::Rcl(day.rcl_year),
            YearType::DailyOffice => Year::DailyOffice(day.daily_office_year),
        };

        let observed = if let LiturgicalDayId::TransferredFeast(feast) = day.observed {
            LiturgicalDayId::Feast(feast)
        } else {
            day.observed.clone()
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
        day: &LiturgicalDay,
        reading_type: ReadingType,
    ) -> impl Iterator<Item = Reading> {
        self.readings_by_day(day)
            .filter(move |reading| reading.reading_type == reading_type)
    }
}
