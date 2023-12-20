use std::{
    cmp::min,
    ops::{Add, Sub},
};

use chrono::{Datelike, Duration, NaiveDateTime};

use crate::constants;
use crate::document::{Day, Document, Line};

#[derive(PartialEq, Debug, Clone)]
pub struct Report {
    pub duration_today: Duration,
    pub duration_week: Duration,
    pub is_ongoing: bool,
    pub balance: Duration,
}

fn duration_for_line(line: &Line, now: Option<NaiveDateTime>) -> Duration {
    match line {
        Line::ClosedShift {
            start_time,
            stop_time,
        } => stop_time.signed_duration_since(*start_time),
        Line::OpenShift { start_time } => now
            .map(|now| now.time().signed_duration_since(*start_time))
            .unwrap_or_else(|| Duration::zero()),
        Line::SpecialShift {
            start_time,
            stop_time,
            ..
        } => stop_time.signed_duration_since(*start_time),
        Line::SpecialDay { .. } => Duration::hours(constants::WORK_HOURS_PER_DAY.into()),
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
        let duration_today = this_day
            .map(|day| duration_for_today(day, now))
            .unwrap_or_else(|| Duration::zero());
        let duration_week = document
            .days
            .iter()
            .filter(|day| day.date != now.date())
            .fold(Duration::hours(0), |acc, day| acc + duration_for_day(day))
            .add(duration_today);
        let expected_days_so_far = min(
            now.weekday().num_days_from_monday() + 1,
            constants::WORK_DAYS_PER_WEEK,
        );
        let expected_duration_so_far_week =
            Duration::hours((expected_days_so_far * constants::WORK_HOURS_PER_DAY).into());
        let incoming_balance: Duration = document
            .preamble
            .iter()
            .filter_map(|d| match d {
                Line::DurationShift { text: _, duration } => Some(duration),
                _ => None,
            })
            .sum();
        Report {
            duration_today,
            duration_week,
            is_ongoing: this_day.map(Day::has_open_shift).unwrap_or_else(|| false),
            balance: duration_week
                .sub(expected_duration_so_far_week)
                .add(incoming_balance),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        document::{Day, Document, Line},
        report::Report,
        testutils::utils::{naive_date, naive_date_time, naive_time},
    };

    #[test]
    fn empty_report() {
        let document = Document::new(vec![], vec![]);
        let now = naive_date_time(2023, 12, 18, 12, 0);
        assert_eq!(
            Report {
                duration_today: chrono::Duration::hours(0),
                duration_week: chrono::Duration::hours(0),
                is_ongoing: false,
                balance: chrono::Duration::hours(-8)
            },
            Report::from_document(&document, &now)
        )
    }

    #[test]
    fn simple_report() {
        let document = Document::new(
            vec![],
            vec![Day {
                date: naive_date(2023, 12, 18), // a monday
                lines: vec![Line::ClosedShift {
                    start_time: naive_time(8, 0),
                    stop_time: naive_time(12, 0),
                }],
            }],
        );
        let now = naive_date_time(2023, 12, 18, 12, 0);
        assert_eq!(
            Report {
                duration_today: chrono::Duration::hours(4),
                duration_week: chrono::Duration::hours(4),
                is_ongoing: false,
                balance: chrono::Duration::hours(-4)
            },
            Report::from_document(&document, &now)
        )
    }

    #[test]
    fn special_days_are_counted() {
        let document = Document::new(
            vec![],
            vec![Day {
                date: naive_date(2023, 12, 18), // a monday
                lines: vec![Line::SpecialDay {
                    text: String::from("vacation"),
                }],
            }],
        );
        let now = naive_date_time(2023, 12, 18, 12, 0);
        assert_eq!(
            Report {
                duration_today: chrono::Duration::hours(8),
                duration_week: chrono::Duration::hours(8),
                is_ongoing: false,
                balance: chrono::Duration::hours(0)
            },
            Report::from_document(&document, &now)
        )
    }

    #[test]
    fn special_shifts_are_counted() {
        let document = Document::new(
            vec![],
            vec![Day {
                date: naive_date(2023, 12, 18), // a monday
                lines: vec![Line::SpecialShift {
                    text: String::from("vacation"),
                    start_time: naive_time(11, 10),
                    stop_time: naive_time(11, 50),
                }],
            }],
        );
        let now = naive_date_time(2023, 12, 18, 12, 0);
        assert_eq!(
            Report {
                duration_today: chrono::Duration::minutes(40),
                duration_week: chrono::Duration::minutes(40),
                is_ongoing: false,
                balance: chrono::Duration::minutes(40 - 8 * 60)
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
                    date: naive_date(2023, 12, 18), // a monday
                    lines: vec![
                        Line::SpecialShift {
                            text: String::from("vacation"),
                            start_time: naive_time(11, 10),
                            stop_time: naive_time(11, 50),
                        },
                        Line::ClosedShift {
                            start_time: naive_time(13, 5),
                            stop_time: naive_time(13, 10),
                        },
                    ],
                },
                Day {
                    date: naive_date(2023, 12, 19), // a tuesday
                    lines: vec![Line::ClosedShift {
                        start_time: naive_time(8, 0),
                        stop_time: naive_time(12, 0),
                    }],
                },
            ],
        );
        let now = naive_date_time(2023, 12, 19, 12, 0);
        assert_eq!(
            Report {
                duration_today: chrono::Duration::hours(4),
                duration_week: chrono::Duration::minutes(285),
                is_ongoing: false,
                balance: chrono::Duration::minutes(285 - 2 * 8 * 60)
            },
            Report::from_document(&document, &now)
        )
    }
}
