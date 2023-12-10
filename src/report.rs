use std::ops::Add;

use chrono::{Duration, NaiveDateTime};

use crate::document::{Document, Line, Day};

#[derive(PartialEq, Debug, Clone)]
pub struct Report {
    pub duration_today: Duration,
    pub duration_week: Duration,
    pub is_ongoing: bool
}

fn duration_for_line(line: &Line, now: Option<NaiveDateTime>) -> Duration {
    match line {
        Line::ClosedShift { start_time, stop_time } => {
            stop_time.signed_duration_since(*start_time)
        },
        Line::OpenShift { start_time } => {
            now.map(|now| now.time().signed_duration_since(*start_time)).unwrap_or_else(|| Duration::zero())
        },
        Line::SpecialShift { start_time, stop_time, .. } => {
            stop_time.signed_duration_since(*start_time)
        },
        Line::SpecialDay { .. } => Duration::hours(8),
        _ => Duration::zero(),        
    }
}

fn duration_for_day(day: &Day) -> Duration {
    day.lines.iter().fold(Duration::hours(0), |acc, line| {
        acc + duration_for_line(line, None)
    })
}

fn duration_for_today(day: &Day, now: &NaiveDateTime) -> Duration {
    day.lines.iter().fold(Duration::hours(0), |acc, line| {
        acc + duration_for_line(line, Some(*now))
    })
}

impl Report {
    pub fn from_document(document: &Document, now: &NaiveDateTime) -> Report {
        let this_day = document.days.iter().find(|day| day.date == now.date());
        let duration_today = this_day.map(|day| duration_for_today(day, now)).unwrap_or_else(|| Duration::zero());
        let duration_week = document.days.iter()
            .filter(|day| day.date != now.date())
            .fold(Duration::hours(0), |acc, day| {
                acc + duration_for_day(day)
            })
            .add(duration_today);
        Report {
            duration_today,
            duration_week,
            is_ongoing: this_day.map(Day::has_open_shift).unwrap_or_else(|| false)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{document::{Document, Day, Line}, report::Report, testutils::utils::{naive_date, naive_time, naive_date_time}};

    #[test]
    fn empty_report() {
        let document = Document::new(
            vec![], 
            vec![]
        );
        let now = naive_date_time(2021, 1, 1, 12, 0);
        assert_eq!(
            Report {
                duration_today: chrono::Duration::hours(0),
                duration_week: chrono::Duration::hours(0),
                is_ongoing: false
            },
            Report::from_document(&document, &now)
        )
    }

    #[test]
    fn simple_report() {
        let document = Document::new(
            vec![],
            vec![
                Day {
                    date: naive_date(2021, 1, 1),
                    lines: vec![
                        Line::ClosedShift {
                            start_time: naive_time(8, 0),
                            stop_time: naive_time(12, 0)
                        },
                    ]
                }
            ]
        );
        let now = naive_date_time(2021, 1, 1, 12, 0);
        assert_eq!(
            Report {
                duration_today: chrono::Duration::hours(4),
                duration_week: chrono::Duration::hours(4),
                is_ongoing: false
            },
            Report::from_document(&document, &now)
        )
    }

    #[test]
    fn special_days_are_counted() {
        let document = Document::new(
            vec![],
            vec![
                Day {
                    date: naive_date(2021, 1, 1),
                    lines: vec![
                        Line::SpecialDay { text: String::from("vacation") },
                    ]
                }
            ]
        );
        let now = naive_date_time(2021, 1, 1, 12, 0);
        assert_eq!(
            Report {
                duration_today: chrono::Duration::hours(8),
                duration_week: chrono::Duration::hours(8),
                is_ongoing: false
            },
            Report::from_document(&document, &now)
        )
    }

    #[test]
    fn special_shifts_are_counted() {
        let document = Document::new(
            vec![],
            vec![
                Day {
                    date: naive_date(2021, 1, 1),
                    lines: vec![
                        Line::SpecialShift { 
                            text: String::from("vacation"), 
                            start_time: naive_time(11, 10), 
                            stop_time: naive_time(11, 50)
                         }
                    ]
                }
            ]
        );
        let now = naive_date_time(2021, 1, 1, 12, 0);
        assert_eq!(
            Report {
                duration_today: chrono::Duration::minutes(40),
                duration_week: chrono::Duration::minutes(40),
                is_ongoing: false
            },
            Report::from_document(&document, &now)
        )
    }

    #[test]
    fn shifts_are_summed_correctly() {
        let document = Document::new(
            vec![],
            vec![
                Day {
                    date: naive_date(2021, 1, 1),
                    lines: vec![
                        Line::SpecialShift { 
                            text: String::from("vacation"), 
                            start_time: naive_time(11, 10), 
                            stop_time: naive_time(11, 50)
                         },
                         Line::ClosedShift { 
                            start_time: naive_time(13, 5), 
                            stop_time: naive_time(13, 10) 
                        }
                    ]
                },
                Day {
                    date: naive_date(2021, 1, 2),
                    lines: vec![
                        Line::ClosedShift { 
                            start_time: naive_time(8, 0), 
                            stop_time: naive_time(12, 0) 
                        }
                    ]
                }
            ]
        );
        let now = naive_date_time(2021, 1, 2, 12, 0);
        assert_eq!(
            Report {
                duration_today: chrono::Duration::hours(4),
                duration_week: chrono::Duration::minutes(285),
                is_ongoing: false
            },
            Report::from_document(&document, &now)
        )
    }
}