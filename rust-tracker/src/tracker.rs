use std::fs::{OpenOptions};
use std::io::Write;
use std::{fs, io};
use std::path::{Path, PathBuf};
use chrono::{NaiveDate, NaiveTime};
use crate::document::{Document, Parser, Day};
use crate::document::Line::OpenShift;

pub struct Tracker {
    weekfile: Option<PathBuf>,
    parser: Parser,
}

impl Tracker {
    pub fn start_tracking(&self, date: NaiveDate, time: NaiveTime) {
        let path_buf = week_tracker_file_create_if_needed(self.weekfile.clone().unwrap_or_else(|| week_tracker_file_for_date(date)));
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
        let path_buf = self.weekfile.clone().unwrap_or_else(|| week_tracker_file_for_date(date));
        let document = self.read_document(path_buf.as_path()).unwrap_or_else(|err| {
            panic!("Unexpected error reading document: {}", err);
        });

        let new_document = self
            .document_with_tracking_stopped(&document, date, time)
            .expect("Stop tracking failed");
        fs::write(path_buf.as_path(), new_document.to_string())
            .expect("Could not write document to file");
    }

    pub fn show_report(&self, date: NaiveDate) {
        let path = week_tracker_file_for_date_create_if_needed(date);
        let result = fs::read_to_string(path);
        match result {
            Ok(content) => self.show_report_of_content(content),
            Err(err) => eprintln!("Error: {}", err)
        }

        todo!()
    }

    fn read_document(&self, path: &Path) -> io::Result<Document> {
        return match fs::read_to_string(path) {
            Ok(content) => Result::Ok(self.parser.parse_document(&content)),
            Err(err) => Result::Err(err)
        }
    }

    fn show_report_of_content(&self, content: String) {
        let document = self.parser.parse_document(&content);
        println!("{:?}", document);
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
            return Ok(document.clone())
        }
        return Ok(document.inserting_day(Day::create(date, vec![OpenShift {start_time: time}])));
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

fn week_tracker_file_for_date_create_if_needed(date: NaiveDate) -> PathBuf {
    let path = week_tracker_file_for_date(date);
    return week_tracker_file_create_if_needed(path);
}

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
    use chrono::{NaiveDate, NaiveTime};
    use crate::document::{Day, Document, Line};
    use crate::Tracker;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    fn time(hour: u32, minute: u32, second: u32) -> NaiveTime {
        NaiveTime::from_hms_opt(hour, minute, second).unwrap()
    }

    #[test]
    fn start_a_new_shift_in_empty_document() {
        let tracker = Tracker::new();
        let document = Document::empty();
        let new_document = tracker.document_with_tracking_started(
            &document,
            date(2019, 12, 3),
            time(8, 0, 0)
        ).unwrap();
        assert_eq!(
            Document::new(
                vec![],
                vec![
                    Day {
                        date: date(2019, 12, 3),
                        lines: vec![
                            Line::OpenShift {
                                start_time: time(8, 0, 0)
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
                    date: date(2019, 12, 2),
                    lines: vec![
                        Line::ClosedShift { 
                            start_time: time(10, 0, 0), 
                            stop_time: time(10, 30, 0)
                        }
                    ]
                }
            ]
        );
        let new_document = tracker.document_with_tracking_started(
            &document,
            date(2019, 12, 3),
            time(8, 0, 0)
        ).unwrap();
        assert_eq!(
            Document::new(
                vec![],
                vec![
                    Day {
                        date: date(2019, 12, 2),
                        lines: vec![
                            Line::ClosedShift { 
                                start_time: time(10, 0, 0), 
                                stop_time: time(10, 30, 0)
                            },
                            Line::Blank
                        ]
                    },
                    Day {
                        date: date(2019, 12, 3),
                        lines: vec![
                            Line::OpenShift {
                                start_time: time(8, 0, 0)
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
                    date: date(2019, 12, 2),
                    lines: vec![
                        Line::ClosedShift { 
                            start_time: time(10, 0, 0), 
                            stop_time: time(10, 30, 0)
                        }
                    ]
                },
                Day {
                    date: date(2019, 12, 3),
                    lines: vec![
                        Line::ClosedShift { 
                            start_time: time(11, 0, 0), 
                            stop_time: time(11, 40, 0) 
                        }
                    ]
                }
            ]
        );
        let new_document = tracker.document_with_tracking_started(
            &document,
            date(2019, 12, 3),
            time(12, 0, 0)
        ).unwrap();
        assert_eq!(
            Document::new(
                vec![],
                vec![
                    Day {
                        date: date(2019, 12, 2),
                        lines: vec![
                            Line::ClosedShift { 
                                start_time: time(10, 0, 0), 
                                stop_time: time(10, 30, 0)
                            }
                        ]
                    },    
                    Day {
                        date: date(2019, 12, 3),
                        lines: vec![
                            Line::ClosedShift { 
                                start_time: time(11, 0, 0), 
                                stop_time: time(11, 40, 0) 
                            },
                            Line::OpenShift {
                                start_time: time(12, 0, 0)
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
                    date: date(2019, 12, 2),
                    lines: vec![
                        Line::ClosedShift { 
                            start_time: time(10, 0, 0), 
                            stop_time: time(10, 30, 0)
                        },
                        Line::Blank
                    ]
                },
            ]
        );
        let new_document = tracker.document_with_tracking_started(
            &document,
            date(2019, 12, 2),
            time(12, 0, 0)
        ).unwrap();

        assert_eq!(
            Document::new(
                vec![], 
                vec![
                    Day {
                        date: date(2019, 12, 2),
                        lines: vec![
                            Line::ClosedShift { 
                                start_time: time(10, 0, 0), 
                                stop_time: time(10, 30, 0)
                            },
                            Line::OpenShift {
                                start_time: time(12, 0, 0)
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
                        date: date(2019, 12, 2),
                        lines: vec![
                            Line::OpenShift { 
                                start_time: time(10, 0, 0)
                            }
                        ]
                    },
                ]
            ),
            date(2019, 12, 2),
            time(12, 0, 0)
        );
        assert!(result.is_err());
    }

}