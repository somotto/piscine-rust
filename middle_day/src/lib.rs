use chrono::{Datelike, NaiveDate, Weekday as wd};

pub fn middle_day(year: i32) -> Option<wd> {
    let is_leap = NaiveDate::from_ymd_opt(year, 2, 29).is_some();

    if is_leap {
        return None;
    }

    let middle = NaiveDate::from_ymd_opt(year, 1, 1)?.succ().checked_add_signed(chrono::Duration::days(181))?;
    Some(middle.weekday())
}
