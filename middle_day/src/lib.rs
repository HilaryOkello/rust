use chrono::{Datelike, NaiveDate, Weekday};

pub fn middle_day(year: i32) -> Option<Weekday> {
    let days_in_year = if chrono::NaiveDate::from_ymd_opt(year + 1, 1, 1).is_some() {
        chrono::NaiveDate::from_ymd(year + 1, 1, 1)
            .signed_duration_since(chrono::NaiveDate::from_ymd(year, 1, 1))
            .num_days() as u32
    } else {
        chrono::NaiveDate::from_ymd(year, 12, 31)
            .signed_duration_since(chrono::NaiveDate::from_ymd(year, 1, 1))
            .num_days() as u32 + 1
    };

    if days_in_year % 2 == 0 {
        None
    } else {
        let middle_day_number = (days_in_year + 1) / 2;
        let first_day_of_year = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
        let middle_date = first_day_of_year + chrono::Duration::days((middle_day_number - 1) as i64);
        Some(middle_date.weekday())
    }
}