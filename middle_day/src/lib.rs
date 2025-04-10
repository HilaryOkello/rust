use chrono::{Datelike, NaiveDate, Weekday};

pub fn middle_day(year: i32) -> Option<Weekday> {
    let first_day = NaiveDate::from_ymd_opt(year, 1, 1)?;
    let last_day = NaiveDate::from_ymd_opt(year, 12, 31)?;
    let days_in_year = last_day.signed_duration_since(first_day).num_days() as u32 + 1;

    if days_in_year % 2 == 0 {
        None
    } else {
        let middle_day_number = (days_in_year + 1) / 2;
        let middle_date = first_day + chrono::Duration::days((middle_day_number - 1) as i64);
        Some(middle_date.weekday())
    }
}