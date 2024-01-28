use chrono::Datelike;

use crate::document::{Day, Document, Line};
use crate::testutils::{naive_date, naive_time};
use crate::tracker::Tracker;

#[test]
fn start_a_new_shift_in_empty_document() {
    let tracker = Tracker::builder().build();
    let document = Document::empty(naive_date(2019, 12, 3).iso_week());
    let new_document = tracker
        .document_with_tracking_started(&document, naive_date(2019, 12, 3), naive_time(8, 0))
        .unwrap();
    assert_eq!(
        Document::new(
            naive_date(2019, 12, 3).iso_week(),
            vec![],
            vec![Day {
                date: naive_date(2019, 12, 3),
                lines: vec![Line::OpenShift {
                    start_time: naive_time(8, 0)
                }]
            }]
        ),
        new_document
    )
}

#[test]
fn blank_line_is_created_before_inserted_date() {
    let tracker = Tracker::builder().build();
    let document = Document::new(
        naive_date(2019, 12, 2).iso_week(),
        vec![],
        vec![Day {
            date: naive_date(2019, 12, 2),
            lines: vec![Line::ClosedShift {
                start_time: naive_time(10, 0),
                stop_time: naive_time(10, 30),
            }],
        }],
    );
    let new_document = tracker
        .document_with_tracking_started(&document, naive_date(2019, 12, 3), naive_time(8, 0))
        .unwrap();
    assert_eq!(
        Document::new(
            naive_date(2019, 12, 2).iso_week(),
            vec![],
            vec![
                Day {
                    date: naive_date(2019, 12, 2),
                    lines: vec![
                        Line::ClosedShift {
                            start_time: naive_time(10, 0),
                            stop_time: naive_time(10, 30)
                        },
                        Line::Blank
                    ]
                },
                Day {
                    date: naive_date(2019, 12, 3),
                    lines: vec![Line::OpenShift {
                        start_time: naive_time(8, 0)
                    }]
                }
            ]
        ),
        new_document
    )
}

#[test]
fn can_start_a_shift_on_an_already_existing_date() {
    let tracker = Tracker::builder().build();
    let document = Document::new(
        naive_date(2019, 12, 2).iso_week(),
        vec![],
        vec![
            Day {
                date: naive_date(2019, 12, 2),
                lines: vec![Line::ClosedShift {
                    start_time: naive_time(10, 0),
                    stop_time: naive_time(10, 30),
                }],
            },
            Day {
                date: naive_date(2019, 12, 3),
                lines: vec![Line::ClosedShift {
                    start_time: naive_time(11, 0),
                    stop_time: naive_time(11, 40),
                }],
            },
        ],
    );
    let new_document = tracker
        .document_with_tracking_started(&document, naive_date(2019, 12, 3), naive_time(12, 0))
        .unwrap();
    assert_eq!(
        Document::new(
            naive_date(2019, 12, 2).iso_week(),
            vec![],
            vec![
                Day {
                    date: naive_date(2019, 12, 2),
                    lines: vec![Line::ClosedShift {
                        start_time: naive_time(10, 0),
                        stop_time: naive_time(10, 30)
                    }]
                },
                Day {
                    date: naive_date(2019, 12, 3),
                    lines: vec![
                        Line::ClosedShift {
                            start_time: naive_time(11, 0),
                            stop_time: naive_time(11, 40)
                        },
                        Line::OpenShift {
                            start_time: naive_time(12, 0)
                        }
                    ]
                }
            ]
        ),
        new_document
    )
}

#[test]
fn new_open_shifts_are_added_right_after_last_existing_shift() {
    let tracker = Tracker::builder().build();
    let document = Document::new(
        naive_date(2019, 12, 2).iso_week(),
        vec![],
        vec![Day {
            date: naive_date(2019, 12, 2),
            lines: vec![
                Line::ClosedShift {
                    start_time: naive_time(10, 0),
                    stop_time: naive_time(10, 30),
                },
                Line::Blank,
            ],
        }],
    );
    let new_document = tracker
        .document_with_tracking_started(&document, naive_date(2019, 12, 2), naive_time(12, 0))
        .unwrap();

    assert_eq!(
        Document::new(
            naive_date(2019, 12, 2).iso_week(),
            vec![],
            vec![Day {
                date: naive_date(2019, 12, 2),
                lines: vec![
                    Line::ClosedShift {
                        start_time: naive_time(10, 0),
                        stop_time: naive_time(10, 30)
                    },
                    Line::OpenShift {
                        start_time: naive_time(12, 0)
                    },
                    Line::Blank
                ]
            },]
        ),
        new_document
    );
}

#[test]
fn we_can_not_start_a_shift_if_one_is_already_started() {
    let tracker = Tracker::builder().build();
    let result = tracker.document_with_tracking_started(
        &Document::new(
            naive_date(2019, 12, 2).iso_week(),
            vec![],
            vec![Day {
                date: naive_date(2019, 12, 2),
                lines: vec![Line::OpenShift {
                    start_time: naive_time(10, 0),
                }],
            }],
        ),
        naive_date(2019, 12, 2),
        naive_time(12, 0),
    );
    assert!(result.is_err());
}

#[test]
fn we_can_stop_a_shift() {
    let tracker = Tracker::builder().build();
    let document = Document::new(
        naive_date(2019, 12, 2).iso_week(),
        vec![],
        vec![Day {
            date: naive_date(2019, 12, 2),
            lines: vec![Line::OpenShift {
                start_time: naive_time(10, 0),
            }],
        }],
    );
    let new_document = tracker
        .document_with_tracking_stopped(&document, naive_date(2019, 12, 2), naive_time(12, 0))
        .unwrap();

    assert_eq!(
        Document::new(
            naive_date(2019, 12, 2).iso_week(),
            vec![],
            vec![Day {
                date: naive_date(2019, 12, 2),
                lines: vec![Line::ClosedShift {
                    start_time: naive_time(10, 0),
                    stop_time: naive_time(12, 0)
                }]
            },]
        ),
        new_document
    );
}
