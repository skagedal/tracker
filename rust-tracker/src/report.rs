use chrono::{Duration, NaiveDate};

use crate::document::Document;

#[derive(PartialEq, Debug, Clone)]
pub struct Report {
    pub duration_today: Duration,
    pub duration_week: Duration,
    pub is_ongoing: bool
}

impl Report {
    pub fn from_document(document: &Document, today: &NaiveDate) -> Report {
        Report {
            duration_today: Duration::hours(0),
            duration_week: Duration::hours(0),
            is_ongoing: false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{document::{Document, Day, Line}, report::Report, testutils::utils::{naive_date, naive_time}};



    #[test]
    fn empty_report() {
        let document = Document::new(
            vec![], 
            vec![]
        );
        let today = naive_date(2021, 1, 1);
        assert_eq!(
            Report {
                duration_today: chrono::Duration::hours(0),
                duration_week: chrono::Duration::hours(0),
                is_ongoing: false
            },
            Report::from_document(&document, &today)
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
        let today = naive_date(2021, 1, 1);
        assert_eq!(
            Report {
                duration_today: chrono::Duration::hours(4),
                duration_week: chrono::Duration::hours(4),
                is_ongoing: false
            },
            Report::from_document(&document, &today)
        )
    }
}