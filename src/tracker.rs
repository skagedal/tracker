use crate::document::Line::OpenShift;
use crate::document::{Day, Document, Parser};
use crate::report::Report;
use chrono::{Datelike, Duration, IsoWeek, NaiveDate, NaiveDateTime, NaiveTime};
use directories::ProjectDirs;
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

        let document = self
            .document_with_tracking_started(&document, date, time)
            .expect("Start tracking failed");

        self.write_day_stdout(&document, date);

        fs::write(path_buf.as_path(), document.to_string())
            .expect("Could not write document to file");
    }

    pub fn stop_tracking(&self, date: NaiveDate, time: NaiveTime) {
        let path_buf = self.week_tracker_file(date);
        let document = self
            .read_document(date.iso_week(), path_buf.as_path())
            .unwrap_or_else(|err| {
                panic!("Unexpected error reading document: {}", err);
            });

        let document = self
            .document_with_tracking_stopped(&document, date, time)
            .expect("Stop tracking failed");

        self.write_day_stdout(&document, date);

        fs::write(path_buf.as_path(), document.to_string())
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
        let path = week_tracker_file_create_if_needed(
            self.active_week(now.date()),
            self.week_tracker_file(now.date()),
        );
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
        match fs::read_to_string(path) {
            Ok(content) => Result::Ok(self.parser.parse_document(week, &content)),
            Err(err) => Result::Err(err),
        }
    }

    fn get_report(&self, content: String, now: NaiveDateTime) -> Report {
        let document = self
            .parser
            .parse_document(self.active_week(now.date()), &content);
        Report::from_document(&document, &now)
    }

    fn process_report_of_content(&self, content: String, now: NaiveDateTime, is_working: bool) {
        let report = self.get_report(content, now);
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
        Ok(document.inserting_day(Day::create(date, vec![OpenShift { start_time: time }])))
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
            return Ok(document.replacing_day(date, day.closing_shift(time)));
        }
        Err(DocumentError::TrackerFileDoesNotHaveOpenShift)
    }

    fn write_day_stdout(&self, document: &Document, date: NaiveDate) {
        let day = document
            .get_day(date)
            .expect("this should be called right after day is modified");
        print!("{}", day.to_string())
    }

    fn active_week(&self, date: NaiveDate) -> IsoWeek {
        self.weekdiff
            .map(|d| date + Duration::days(d as i64 * 7))
            .unwrap_or(date)
            .iso_week()
    }
}

#[derive(Debug, Clone)]
pub enum DocumentError {
    TrackerFileAlreadyHasOpenShift,
    TrackerFileDoesNotHaveOpenShift,
}

impl Tracker {
    pub fn builder() -> TrackerBuilder {
        TrackerBuilder::default()
    }
}

#[derive(Default)]
pub struct TrackerBuilder {
    weekfile: Option<PathBuf>,
    weekdiff: Option<i32>,
}

impl TrackerBuilder {
    pub fn weekfile(mut self, weekfile: Option<PathBuf>) -> Self {
        self.weekfile = weekfile;
        self
    }

    pub fn weekdiff(mut self, weekdiff: Option<i32>) -> Self {
        self.weekdiff = weekdiff;
        self
    }

    pub fn build(self) -> Tracker {
        Tracker {
            weekfile: self.weekfile,
            weekdiff: self.weekdiff,
            parser: Parser::new(),
        }
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

    path
}

fn week_tracker_file_for_date(date: NaiveDate, weekdiff: Option<i32>) -> PathBuf {
    let proj_dirs = ProjectDirs::from("tech", "skagedal", "tracker").unwrap();

    let date = weekdiff
        .map(|d| date + Duration::days(d as i64 * 7))
        .unwrap_or(date);

    proj_dirs
        .data_dir()
        .join("week-files")
        .join(date.format("%Y-W%W.txt").to_string())
}

#[cfg(test)]
mod tests;
