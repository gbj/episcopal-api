use crate::{holy_day::HolyDayId, Calendar, Feast, LiturgicalDay, LiturgicalDayId, Rank, Weekday};

impl Calendar {
    /// Checks whether any feasts that would have occurred on a Sunday, during Holy Week,
    /// or during the week of Easter should be transferred forward to the next open day,
    /// and therefore override the propers a particular liturgical day:
    /// "Feasts of our Lord, and all other Major Feasts appointed on fixed days
    /// in the Calendar, when they occur on a Sunday, are normally transferred
    /// to the first convenient open day within the week." (BCP p. 16)
    /// ```
    /// # use crate::calendar::{BCP1979_CALENDAR, Date, Feast, LiturgicalDayId, Proper, Weekday};
    /// // e.g., transfer Visitation to Monday when it overlaps with Pentecost the day before
    /// let date = Date::from_ymd(2020, 6, 1);
    /// let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
    /// assert_eq!(original_day.observed, LiturgicalDayId::ProperAndDay(Proper::Proper4, Weekday::Mon));
    /// let transfer = BCP1979_CALENDAR.transferred_feast(&original_day);
    /// assert_eq!(transfer, Some(Feast::TheVisitation));
    ///
    /// ```
    pub fn transferred_feast(&self, day: &LiturgicalDay) -> Option<Feast> {
        self.transferred_feast_with_open_days(day, day, &mut Vec::new(), &mut Vec::new())
    }

    fn transferred_feast_with_open_days(
        &self,
        day: &LiturgicalDay,
        original_day: &LiturgicalDay,
        acc: &mut Vec<Feast>,
        open_days: &mut Vec<LiturgicalDay>,
    ) -> Option<Feast> {
        // TODO: this is a port of the old, mostly-working JS version
        // it needs more test cases (see below) to capture more-obscure patterns of transfer
        // I imagine it can also be cleaned up or rustified significantly

        // today
        let date = day.date;
        let evening = day.evening;
        let yesterday =
            self.liturgical_day_without_transferred_feasts(date.subtract_days(1), evening);
        let day_before_yesterday =
            self.liturgical_day_without_transferred_feasts(date.subtract_days(2), evening);

        let for_rank = acc
            .get(0)
            .map(|feast| self.feast_day_rank(feast))
            .unwrap_or(Rank::HolyDay);
        let today_is_empty = self.is_empty_for_rank(day, for_rank);
        let yesterday_is_empty = self.is_empty_for_rank(&yesterday, for_rank);

        // which feast, if any, would be observed today
        let today_feast = if let LiturgicalDayId::Feast(observed) = day.observed {
            day.holy_days.iter().find(|feast| {
                observed != **feast
                    && self.feast_day_rank(feast) >= Rank::HolyDay
                    && !self.feast_is_eve(feast)
            })
        } else if let LiturgicalDayId::WeekAndDay(_, _) = day.observed {
            day.holy_days.iter().find(|feast| {
                self.feast_day_rank(feast) >= Rank::HolyDay && !self.feast_is_eve(feast)
            })
        } else if let LiturgicalDayId::ProperAndDay(_, _) = day.observed {
            day.holy_days.iter().find(|feast| {
                self.feast_day_rank(feast) >= Rank::HolyDay && !self.feast_is_eve(feast)
            })
        } else {
            None
        };

        println!("\n\n[NEW ITERATION] {}\n\n", date.naive_date);

        if today_is_empty && yesterday_is_empty {
            println!("today is empty and yesterday is empty");
            // check ONE more day -- we will rarely need to transfer more than two days, but it does happen around Easter Week sometimes
            if self.is_empty_for_rank(&day_before_yesterday, for_rank) {
                println!("and day before is empty");
                println!("accumulated feasts = \t {:#?}", acc);
                println!("open days = \t{:#?}", open_days);
                // find index of original day
                open_days.reverse();
                let original_day_index = open_days
                    .iter()
                    .enumerate()
                    .find(|(_, day)| *day == original_day)
                    .map(|(idx, _)| idx)
                    .unwrap_or(0);
                // reverse because accumulate open days moving backwards
                acc.reverse();
                acc.get(original_day_index).copied()
            } else {
                println!("and day before is not empty.");
                open_days.push(day.clone());
                self.transferred_feast_with_open_days(&yesterday, original_day, acc, open_days)
            }
        }
        // if today is empty and yesterday is not empty, recurse back one more day
        else if today_is_empty && !yesterday_is_empty {
            println!("today is empty, yesterday is not empty {:#?}", yesterday);
            // add today to the open days
            open_days.push(day.clone());

            println!("open_days = {:#?}", open_days);

            self.transferred_feast_with_open_days(&yesterday, original_day, acc, open_days)
        }
        // if today is not empty and today's feast is not observed...
        else if yesterday_is_empty {
            println!("yesterday is empty ( today is not)\n\ntoday is {:#?}", day);

            let feast_is_observed_today = match (today_feast, &day.observed) {
                (Some(today_feast), LiturgicalDayId::Feast(observed_feast)) => {
                    *today_feast == *observed_feast
                }
                _ => false,
            };

            println!("feast_is_observed_today = {:#?}", feast_is_observed_today);
            println!("today_feast = {:#?}", today_feast);

            //if !feast_is_observed_today {
            if let Some(feast) = today_feast {
                acc.push(*feast);
            }
            //}

            println!("acc now = {:#?}", acc);

            self.transferred_feast_with_open_days(&yesterday, original_day, acc, open_days)
        } else {
            println!("neither yesterday nor today is empty");
            if let Some(feast) = today_feast {
                acc.push(*feast);
            }
            println!("acc = {:#?}", acc);
            self.transferred_feast_with_open_days(&yesterday, original_day, acc, open_days)
        }
    }

    pub fn is_empty_for_rank(&self, day: &LiturgicalDay, rank: Rank) -> bool {
        day.weekday != Weekday::Sun
            && !day
                .holy_days
                .iter()
                .any(|feast| self.feast_day_rank(feast) > rank && !self.feast_is_eve(feast))
    }
}

#[cfg(test)]
mod tests {
    use crate::{LiturgicalWeek, Rank};

    use super::super::{Date, Feast, LiturgicalDayId, Proper, Weekday, BCP1979_CALENDAR};

    #[test]
    fn is_empty_works() {
        // 6/1 has no feast before transfers
        let date = Date::from_ymd(2020, 6, 1);
        let day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        assert!(BCP1979_CALENDAR.is_empty_for_rank(&day, Rank::HolyDay));
        // 5/31 has an observed feast (Pentecost)
        let date = Date::from_ymd(2020, 5, 31);
        let day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        assert!(!BCP1979_CALENDAR.is_empty_for_rank(&day, Rank::HolyDay));

        // 5/30 has "Eve of the Visitation" but we ignore eves when transferring
        let date = Date::from_ymd(2020, 5, 30);
        let day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        assert!(BCP1979_CALENDAR.is_empty_for_rank(&day, Rank::HolyDay));

        // ignore black-letter days
        let date = Date::from_ymd(2008, 2, 18);
        let day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        assert!(BCP1979_CALENDAR.is_empty_for_rank(&day, Rank::HolyDay));
    }

    #[test]
    fn empty_if_no_feasts() {
        let date = Date::from_ymd(2020, 6, 3);
        let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        assert_eq!(
            original_day.observed,
            LiturgicalDayId::ProperAndDay(Proper::Proper4, Weekday::Wed)
        );
        let transfer = BCP1979_CALENDAR.transferred_feast(&original_day);
        assert_eq!(transfer, None);
    }

    #[test]
    fn does_not_transfer_observed_feasts_forward() {
        // Transfiguration observed 8/6
        let date = Date::from_ymd(2020, 8, 6);
        let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        assert_eq!(
            original_day.observed,
            LiturgicalDayId::Feast(Feast::TheTransfiguration)
        );

        // So Transfiguration not transferred to 8/7
        let date = Date::from_ymd(2020, 8, 7);
        let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        assert_eq!(
            original_day.observed,
            LiturgicalDayId::ProperAndDay(Proper::Proper13, Weekday::Fri)
        );
        let transfer = BCP1979_CALENDAR.transferred_feast(&original_day);
        assert_eq!(transfer, None);
    }

    #[test]
    fn transfers_forward_from_a_sunday() {
        // 4/25 is a Sunday
        let date = Date::from_ymd(2021, 4, 25);
        let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        assert_eq!(
            original_day.observed,
            LiturgicalDayId::WeekAndDay(LiturgicalWeek::Easter4, Weekday::Sun)
        );

        // So St. Mark transfers to 4/26
        let date = Date::from_ymd(2021, 4, 26);
        let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        let transfer = BCP1979_CALENDAR.transferred_feast(&original_day);
        assert_eq!(transfer, Some(Feast::Mark));
    }

    #[test]
    fn transfers_visitation_forward_from_pentecost() {
        // Pentecost is observed 5/31 (ordinary date for Visitation)
        let date = Date::from_ymd(2020, 5, 31);
        let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        assert_eq!(
            original_day.observed,
            LiturgicalDayId::Feast(Feast::Pentecost)
        );

        // So Visitation transfers to 6/1
        let date = Date::from_ymd(2020, 6, 1);
        let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        let transfer = BCP1979_CALENDAR.transferred_feast(&original_day);
        assert_eq!(transfer, Some(Feast::TheVisitation));
    }

    #[test]
    fn does_not_transfer_monday_feasts_observed_on_monday() {
        let date = Date::from_ymd(2020, 8, 24);
        let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        assert_eq!(
            original_day.observed,
            LiturgicalDayId::Feast(Feast::Bartholomew)
        );

        let date = Date::from_ymd(2020, 8, 25);
        let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        let transfer = BCP1979_CALENDAR.transferred_feast(&original_day);
        assert_eq!(transfer, None);
    }

    #[test]
    fn transfers_joseph_and_annunciation() {
        let date_1 = Date::from_ymd(2008, 3, 31);
        let day_1 = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date_1, false);
        let date_2 = Date::from_ymd(2008, 4, 1);
        let day_2 = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date_2, false);

        let transfer_1 = BCP1979_CALENDAR.transferred_feast(&day_1);
        let transfer_2 = BCP1979_CALENDAR.transferred_feast(&day_2);

        assert_eq!(transfer_1, Some(Feast::Joseph));
        assert_eq!(transfer_2, Some(Feast::Annunciation));
    }

    #[test]
    fn transfers_days_within_christmastide_properly() {
        // in 2021, Christmas is Saturday, which means 12/26 is Christmas 1 (not St. Stephen)
        // this means the whole week of feasts is displaced by a day
        // let's try it out
        let dec_25 = BCP1979_CALENDAR
            .liturgical_day_without_transferred_feasts(Date::from_ymd(2021, 12, 25), false);
        let dec_26 = BCP1979_CALENDAR
            .liturgical_day_without_transferred_feasts(Date::from_ymd(2021, 12, 26), false);
        let dec_27 = BCP1979_CALENDAR
            .liturgical_day_without_transferred_feasts(Date::from_ymd(2021, 12, 27), false);
        let dec_28 = BCP1979_CALENDAR
            .liturgical_day_without_transferred_feasts(Date::from_ymd(2021, 12, 28), false);
        let dec_29 = BCP1979_CALENDAR
            .liturgical_day_without_transferred_feasts(Date::from_ymd(2021, 12, 29), false);
        let dec_30 = BCP1979_CALENDAR
            .liturgical_day_without_transferred_feasts(Date::from_ymd(2021, 12, 30), false);
        //assert_eq!(BCP1979_CALENDAR.transferred_feast(&dec_30), None);
        /* assert_eq!(
            BCP1979_CALENDAR.transferred_feast(&dec_29),
            Some(Feast::HolyInnocents)
        ); */
        assert_eq!(
            BCP1979_CALENDAR.transferred_feast(&dec_28),
            Some(Feast::John)
        );
        /* assert_eq!(
            BCP1979_CALENDAR.transferred_feast(&dec_27),
            Some(Feast::Stephen)
        ); */
        //assert_eq!(BCP1979_CALENDAR.transferred_feast(&dec_26), None);
        /* assert_eq!(
            dec_26.observed,
            LiturgicalDayId::WeekAndDay(LiturgicalWeek::Christmas1, Weekday::Sun)
        ); */
    }
}
