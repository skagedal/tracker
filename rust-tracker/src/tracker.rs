use std::{fs, io};
use std::path::{Path, PathBuf};
use chrono::{NaiveDate, NaiveTime};
use crate::document::{Document, Parser};

pub struct Tracker {
    parser: Parser,
}

impl Tracker {
    pub fn start_tracking(&self, date: NaiveDate, time: NaiveTime) {
        let path_buf = week_tracker_file_create_if_needed(date);
        let document = self.read_document(path_buf.as_path());

        println!("Got a document: {:?}", document);
        println!("Now we should start tracking at {:?}", time);

        todo!()
    }

    pub fn show_report(&self, date: NaiveDate) {
        let path = week_tracker_file_create_if_needed(date);
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

    pub fn document_with_tracking_started(&self, document: Document, date: NaiveDate, time: NaiveTime) -> Document {
        return document
    }
}

impl Tracker {
    pub fn new() -> Self {
        return Tracker {
            parser: Parser::new()
        }
    }
}

// Week tracker file

fn week_tracker_file_create_if_needed(date: NaiveDate) -> PathBuf {
    let path = week_tracker_file(date);
    // Create parents if needed
    if let Some(parent_path) = path.parent() {
        fs::create_dir_all(parent_path).unwrap_or_else(|err| eprintln!("Error: {}", err));
    }
    // TODO: create default document if needed
    return path;
}

fn week_tracker_file(date: NaiveDate) -> PathBuf {
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

    #[test]
    fn start_a_new_shift_in_empty_document() {
        let tracker = Tracker::new();
        let document = Document::empty();
        let new_document = tracker.document_with_tracking_started(
            document,
            NaiveDate::from_ymd(2019, 12, 3),
            NaiveTime::from_hms(8, 0, 0)
        );
        assert_eq!(
            Document::new(
                vec![],
                vec![
                    Day {
                        date: NaiveDate::from_ymd(2019, 12, 3),
                        lines: vec![
                            Line::OpenShift {
                                start_time: NaiveTime::from_hms(8, 0, 0)
                            }
                        ]
                    }
                ]
            ),
            new_document
        )
    }
}