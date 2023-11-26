use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::{fs, io};
use std::path::{Path, PathBuf};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime, Duration};
use crate::document::{Document, Parser, Day};
use crate::document::Line::OpenShift;
use crate::report::Report;

pub struct Tracker {
    weekfile: Option<PathBuf>,
    parser: Parser,
}

fn format_duration(duration: &Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() - (hours * 60);
    format!("{} hours {} minutes", hours, minutes)
}

impl Tracker {
    pub fn start_tracking(&self, date: NaiveDate, time: NaiveTime) {
        let path_buf = week_tracker_file_create_if_needed(self.week_tracker_file(date));
        let document = self.read_document(path_buf.as_path()).unwrap_or_else(|err| {
            panic!("Unexpected error reading document: {}", err);
        });

        let new_document = self
            .document_with_tracking_started(&document, date, time)
            .expect("Start tracking failed");
        fs::write(path_buf.as_path(), new_document.to_string())
            .expect("Could not write document to file");
    }

    pub fn stop_tracking(&self, date: NaiveDate, time: NaiveTime) {
        let path_buf = self.week_tracker_file(date);
        let document = self.read_document(path_buf.as_path()).unwrap_or_else(|err| {
            panic!("Unexpected error reading document: {}", err);
        });

        let new_document = self
            .document_with_tracking_stopped(&document, date, time)
            .expect("Stop tracking failed");
        fs::write(path_buf.as_path(), new_document.to_string())
            .expect("Could not write document to file");
    }

    pub fn edit_file(&self, date: NaiveDate) {
        let path = week_tracker_file_create_if_needed(self.week_tracker_file(date));

        let editor = env::var("EDITOR").unwrap();   
        Command::new(editor)
            .arg(&path)
            .status()
            .expect("Could not open editor");
    }

    pub fn show_report(&self, now: NaiveDateTime) {
        let path = week_tracker_file_create_if_needed(self.week_tracker_file(now.date()));
        let result = fs::read_to_string(path);
        match result {
            Ok(content) => self.show_report_of_content(content, now),
            Err(err) => eprintln!("Error: {}", err)
        }
    }

    fn week_tracker_file(&self, date: NaiveDate) -> PathBuf {
        self.weekfile.clone().unwrap_or_else(|| week_tracker_file_for_date(date))
    }

    fn read_document(&self, path: &Path) -> io::Result<Document> {
        return match fs::read_to_string(path) {
            Ok(content) => Result::Ok(self.parser.parse_document(&content)),
            Err(err) => Result::Err(err)
        }
    }

    fn show_report_of_content(&self, content: String, now: NaiveDateTime) {
        let document = self.parser.parse_document(&content);
        let report = Report::from_document(&document, &now);
        println!("You have worked {} today.", format_duration(&report.duration_today));
        println!("You have worked {} this week.", format_duration(&report.duration_week));
    }

    pub fn document_with_tracking_started(&self, document: &Document, date: NaiveDate, time: NaiveTime) -> Result<Document, DocumentError> {
        if document.has_open_shift() {
            return Err(DocumentError::TrackerFileAlreadyHasOpenShift)
        }
        if let Some(day) = document.days.iter().find(|day| day.date.eq(&date)) {
            return Ok(document.replacing_day(date, day.adding_shift(OpenShift {start_time: time})))
        }
        return Ok(document.inserting_day(Day::create(date, vec![OpenShift {start_time: time}])));
    }

    pub fn document_with_tracking_stopped(&self, document: &Document, date: NaiveDate, time: NaiveTime) -> Result<Document, DocumentError> {
        if !document.has_open_shift() {
            return Err(DocumentError::TrackerFileDoesNotHaveOpenShift)
        }
        if let Some(day) = document.days.iter().find(|day| day.date.eq(&date)) {
            println!("Found day: {:?}", day);
            return Ok(document
                .replacing_day(date, day.closing_shift(time)))
        }
        return Err(DocumentError::TrackerFileDoesNotHaveOpenShift);
    }
}

#[derive(Debug, Clone)]
pub enum DocumentError {
    TrackerFileAlreadyHasOpenShift,
    TrackerFileDoesNotHaveOpenShift
}

impl Tracker {
    #[cfg(test)]
    pub fn new() -> Self {
        return Tracker {
            weekfile: None,
            parser: Parser::new()
        }
    }

    pub fn new_with_weekfile(weekfile: Option<PathBuf>) -> Self {
        return Tracker {
            weekfile,
            parser: Parser::new()
        }
    }
}

// Week tracker file

fn week_tracker_file_create_if_needed(path: PathBuf) -> PathBuf {
    // Create parents if needed
    if let Some(parent_path) = path.parent() {
        fs::create_dir_all(parent_path).unwrap_or_else(|err| eprintln!("Error: {}", err));
    }

    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path) {
        Ok(mut file) => {
            let empty = Document::empty();
            file.write_all(empty.to_string().as_bytes()).expect("Could not write example document to file");
        },
        Err(err) => {
            if err.kind() != io::ErrorKind::AlreadyExists {
                eprintln!("Error: {}", err);
            }
        }
    }
    
    return path;
}

fn week_tracker_file_for_date(date: NaiveDate) -> PathBuf {
    dirs::home_dir().unwrap()
        .join(".simons-assistant")
        .join("data")
        .join("tracker")
        .join(date.format("%Y-W%W.txt").to_string())
}

#[cfg(test)]
mod tests {
    use crate::document::{Day, Document, Line};
    use crate::Tracker;
    use crate::testutils::utils::{naive_date, naive_time};

    #[test]
    fn start_a_new_shift_in_empty_document() {
        let tracker = Tracker::new();
        let document = Document::empty();
        let new_document = tracker.document_with_tracking_started(
            &document,
            naive_date(2019, 12, 3),
            naive_time(8, 0)
        ).unwrap();
        assert_eq!(
            Document::new(
                vec![],
                vec![
                    Day {
                        date: naive_date(2019, 12, 3),
                        lines: vec![
                            Line::OpenShift {
                                start_time: naive_time(8, 0)
                            }
                        ]
                    }
                ]
            ),
            new_document
        )
    }

    #[test]
    fn blank_line_is_created_before_inserted_date() {
        let tracker = Tracker::new();
        let document = Document::new(
            vec![],
            vec![
                Day {
                    date: naive_date(2019, 12, 2),
                    lines: vec![
                        Line::ClosedShift { 
                            start_time: naive_time(10, 0), 
                            stop_time: naive_time(10, 30)
                        }
                    ]
                }
            ]
        );
        let new_document = tracker.document_with_tracking_started(
            &document,
            naive_date(2019, 12, 3),
            naive_time(8, 0)
        ).unwrap();
        assert_eq!(
            Document::new(
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
                        lines: vec![
                            Line::OpenShift {
                                start_time: naive_time(8, 0)
                            }
                        ]
                    }
                ]
            ),
            new_document
        )
    }

    #[test]
    fn can_start_a_shift_on_an_already_existing_date() {
        let tracker = Tracker::new();
        let document = Document::new(
            vec![],
            vec![
                Day {
                    date: naive_date(2019, 12, 2),
                    lines: vec![
                        Line::ClosedShift { 
                            start_time: naive_time(10, 0), 
                            stop_time: naive_time(10, 30)
                        }
                    ]
                },
                Day {
                    date: naive_date(2019, 12, 3),
                    lines: vec![
                        Line::ClosedShift { 
                            start_time: naive_time(11, 0), 
                            stop_time: naive_time(11, 40) 
                        }
                    ]
                }
            ]
        );
        let new_document = tracker.document_with_tracking_started(
            &document,
            naive_date(2019, 12, 3),
            naive_time(12, 0)
        ).unwrap();
        assert_eq!(
            Document::new(
                vec![],
                vec![
                    Day {
                        date: naive_date(2019, 12, 2),
                        lines: vec![
                            Line::ClosedShift { 
                                start_time: naive_time(10, 0), 
                                stop_time: naive_time(10, 30)
                            }
                        ]
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
        let tracker = Tracker::new();
        let document = Document::new(
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
            ]
        );
        let new_document = tracker.document_with_tracking_started(
            &document,
            naive_date(2019, 12, 2),
            naive_time(12, 0)
        ).unwrap();

        assert_eq!(
            Document::new(
                vec![], 
                vec![
                    Day {
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
                    },
                ]
            ),
            new_document
        );
    }

    #[test]
    fn we_can_not_start_a_shift_if_one_is_already_started() {
        let tracker = Tracker::new();
        let result = tracker.document_with_tracking_started(
            &Document::new(
                vec![],
                vec![
                    Day {
                        date: naive_date(2019, 12, 2),
                        lines: vec![
                            Line::OpenShift { 
                                start_time: naive_time(10, 0)
                            }
                        ]
                    },
                ]
            ),
            naive_date(2019, 12, 2),
            naive_time(12, 0)
        );
        assert!(result.is_err());
    }

    #[test]
    fn we_can_stop_a_shift() {
        let tracker = Tracker::new();
        let document = Document::new(
            vec![],
            vec![
                Day {
                    date: naive_date(2019, 12, 2),
                    lines: vec![
                        Line::OpenShift { 
                            start_time: naive_time(10, 0)
                        }
                    ]
                },
            ]
        );
        let new_document = tracker.document_with_tracking_stopped(
            &document,
            naive_date(2019, 12, 2),
            naive_time(12, 0)
        ).unwrap();

        assert_eq!(
            Document::new(
                vec![],
                vec![
                    Day {
                        date: naive_date(2019, 12, 2),
                        lines: vec![
                            Line::ClosedShift { 
                                start_time: naive_time(10, 0),
                                stop_time: naive_time(12, 0)
                            }
                        ]
                    },
                ]
            ),
            new_document
        );
    }

}