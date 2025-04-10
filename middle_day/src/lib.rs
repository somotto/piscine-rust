use chrono::{Datelike, NaiveDate, Weekday as wd};

pub fn middle_day(year: i32) -> Option<wd> {
    let is_leap_year = chrono::NaiveDate::from_ymd_opt(year, 2, 29).is_some();
    let days_in_year = if is_leap_year { 366 } else { 365 };
    
    if days_in_year % 2 == 0 {
        return None;
    }
    
    let middle_day_of_year = (days_in_year + 1) / 2;
    
    let jan1 = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
    let middle_date = jan1.with_ordinal(middle_day_of_year as u32).unwrap();
    
    Some(middle_date.weekday())
}