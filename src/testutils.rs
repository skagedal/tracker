/// Helpers to be used in the test suite only.
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

pub fn naive_date_time(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> NaiveDateTime {
    NaiveDateTime::new(naive_date(year, month, day), naive_time(hour, minute))
}

pub fn naive_date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}

pub fn naive_time(hour: u32, minute: u32) -> NaiveTime {
    NaiveTime::from_hms_opt(hour, minute, 0).unwrap()
}
