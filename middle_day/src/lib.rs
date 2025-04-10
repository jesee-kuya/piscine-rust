use chrono::{NaiveDate, Datelike};
use chrono::Weekday as wd;

pub fn middle_day(year: i32) -> Option<wd> {
    let is_leap_year = NaiveDate::from_ymd_opt(year, 2, 29).is_some();
    let total_days = if is_leap_year { 366 } else { 365 };
    
    let middle_day = (total_days + 1) / 2;
    let date = NaiveDate::from_yo_opt(year, middle_day)?;
    
    Some(date.weekday())
}