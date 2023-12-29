use chrono::Datelike;

use crate::{
    document::{Day, Document, Line},
    report::Report,
    testutils::{naive_date, naive_date_time, naive_time},
};

#[test]
fn empty_report() {
    let now = naive_date_time(2023, 12, 18, 12, 0);
    let week = now.iso_week();
    let document = Document::new(week, vec![], vec![]);
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
        naive_date(2023, 12, 18).iso_week(),
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
        naive_date(2023, 12, 18).iso_week(),
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
        naive_date(2023, 12, 18).iso_week(),
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
        naive_date(2023, 12, 18).iso_week(),
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
