use chrono::Duration;

use crate::document::Document;

#[derive(PartialEq, Debug, Clone)]
pub struct Report {
    pub duration_today: Duration,
    pub duration_week: Duration,
    pub is_ongoing: bool
}

impl Report {
    pub fn from_document(document: &Document) -> Report {
        Report {
            duration_today: Duration::hours(0),
            duration_week: Duration::hours(0),
            is_ongoing: false
        }
    }
}