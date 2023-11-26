/// Helpers to be used in the test suite only.
#[cfg(test)]
pub mod utils {
    use chrono::{NaiveDate, NaiveTime};

    pub fn naive_date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[cfg(test)]
    pub fn naive_time(hour: u32, minute: u32) -> NaiveTime {
        NaiveTime::from_hms_opt(hour, minute, 0).unwrap()
    }
}
