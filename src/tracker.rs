use crate::document::Line::OpenShift;
use crate::document::{Day, Document, Parser};
use crate::report::Report;
use chrono::{Datelike, Duration, IsoWeek, NaiveDate, NaiveDateTime, NaiveTime};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};

pub struct Tracker {
    weekfile: Option<PathBuf>,
    weekdiff: Option<i32>,
    parser: Parser,
}

fn format_duration(duration: &Duration) -> String {
    let hours = duration.num_hours();
    let minutes = (duration.num_minutes() - (hours * 60)).abs();
    format!("{} h {} m", hours, minutes)
}

impl Tracker {
    pub fn start_tracking(&self, date: NaiveDate, time: NaiveTime) {
        let path_buf =
            week_tracker_file_create_if_needed(date.iso_week(), self.week_tracker_file(date));
        let document = self
            .read_document(date.iso_week(), path_buf.as_path())
            .unwrap_or_else(|err| {
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
        let document = self
            .read_document(date.iso_week(), path_buf.as_path())
            .unwrap_or_else(|err| {
                panic!("Unexpected error reading document: {}", err);
            });

        let new_document = self
            .document_with_tracking_stopped(&document, date, time)
            .expect("Stop tracking failed");
        fs::write(path_buf.as_path(), new_document.to_string())
            .expect("Could not write document to file");
    }

    pub fn edit_file(&self, date: NaiveDate) {
        let path =
            week_tracker_file_create_if_needed(date.iso_week(), self.week_tracker_file(date));

        let editor = env::var("EDITOR").unwrap();
        Command::new(editor)
            .arg(&path)
            .status()
            .expect("Could not open editor");
    }

    pub fn show_report(&self, now: NaiveDateTime, is_working: bool) {
        let path =
            week_tracker_file_create_if_needed(now.iso_week(), self.week_tracker_file(now.date()));
        let result = fs::read_to_string(path);
        match result {
            Ok(content) => self.process_report_of_content(content, now, is_working),
            Err(err) => eprintln!("Error: {}", err),
        }
    }

    fn week_tracker_file(&self, date: NaiveDate) -> PathBuf {
        self.weekfile
            .clone()
            .unwrap_or_else(|| week_tracker_file_for_date(date, self.weekdiff))
    }

    fn read_document(&self, week: IsoWeek, path: &Path) -> io::Result<Document> {
        return match fs::read_to_string(path) {
            Ok(content) => Result::Ok(self.parser.parse_document(week, &content)),
            Err(err) => Result::Err(err),
        };
    }

    fn process_report_of_content(&self, content: String, now: NaiveDateTime, is_working: bool) {
        let document = self.parser.parse_document(now.iso_week(), &content);
        let report = Report::from_document(&document, &now);
        if is_working {
            let code = match report.is_ongoing {
                true => 0,
                false => 1,
            };
            std::process::exit(code);
        }

        print!(
            "You have worked {} today",
            format_duration(&report.duration_today)
        );
        if report.is_ongoing {
            println!(", ongoing.")
        } else {
            println!(".")
        }
        println!(
            "You have worked {} this week.",
            format_duration(&report.duration_week)
        );
        println!("Balance: {}", format_duration(&report.balance))
    }

    pub fn document_with_tracking_started(
        &self,
        document: &Document,
        date: NaiveDate,
        time: NaiveTime,
    ) -> Result<Document, DocumentError> {
        if document.has_open_shift() {
            return Err(DocumentError::TrackerFileAlreadyHasOpenShift);
        }
        if let Some(day) = document.days.iter().find(|day| day.date.eq(&date)) {
            return Ok(
                document.replacing_day(date, day.adding_shift(OpenShift { start_time: time }))
            );
        }
        return Ok(document.inserting_day(Day::create(date, vec![OpenShift { start_time: time }])));
    }

    pub fn document_with_tracking_stopped(
        &self,
        document: &Document,
        date: NaiveDate,
        time: NaiveTime,
    ) -> Result<Document, DocumentError> {
        if !document.has_open_shift() {
            return Err(DocumentError::TrackerFileDoesNotHaveOpenShift);
        }
        if let Some(day) = document.days.iter().find(|day| day.date.eq(&date)) {
            println!("Found day: {:?}", day);
            return Ok(document.replacing_day(date, day.closing_shift(time)));
        }
        return Err(DocumentError::TrackerFileDoesNotHaveOpenShift);
    }
}

#[derive(Debug, Clone)]
pub enum DocumentError {
    TrackerFileAlreadyHasOpenShift,
    TrackerFileDoesNotHaveOpenShift,
}

impl Tracker {
    #[cfg(test)]
    pub fn new() -> Self {
        return Tracker {
            weekfile: None,
            weekdiff: None,
            parser: Parser::new(),
        };
    }

    pub fn new_with_options(weekfile: Option<PathBuf>, week: Option<i32>) -> Self {
        return Tracker {
            weekfile,
            weekdiff: week,
            parser: Parser::new(),
        };
    }
}

// Week tracker file

fn week_tracker_file_create_if_needed(week: IsoWeek, path: PathBuf) -> PathBuf {
    // Create parents if needed
    if let Some(parent_path) = path.parent() {
        fs::create_dir_all(parent_path).unwrap_or_else(|err| eprintln!("Error: {}", err));
    }

    match OpenOptions::new().write(true).create_new(true).open(&path) {
        Ok(mut file) => {
            let empty = Document::empty(week);
            file.write_all(empty.to_string().as_bytes())
                .expect("Could not write example document to file");
        }
        Err(err) => {
            if err.kind() != io::ErrorKind::AlreadyExists {
                eprintln!("Error: {}", err);
            }
        }
    }

    return path;
}

fn week_tracker_file_for_date(date: NaiveDate, weekdiff: Option<i32>) -> PathBuf {
    let date = weekdiff
        .map(|d| date + Duration::days(d as i64 * 7))
        .unwrap_or(date);
    dirs::home_dir()
        .unwrap()
        .join(".simons-assistant")
        .join("data")
        .join("tracker")
        .join(date.format("%Y-W%W.txt").to_string())
}

#[cfg(test)]
mod tests;
