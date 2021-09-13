use chrono::{NaiveDate, NaiveTime};
use regex::Regex;
use crate::document::Line::Comment;

#[derive(PartialEq, Debug)]
pub enum Line {
    Comment {
        text: String
    },
    DayHeader {
        date: NaiveDate
    },
    OpenShift {
        start_time: NaiveTime
    },
    ClosedShift {
        start_time: NaiveTime,
        stop_time: NaiveTime,
    },
    SpecialDay {
        text: String
    },
    SpecialShift {
        text: String,
        start_time: NaiveTime,
        stop_time: NaiveTime,
    },
    Blank,
}

pub struct Day {
    date: NaiveDate,
    lines: Vec<Line>,
}

pub struct Document {
    preamble: Vec<Line>,
    days: Vec<Day>,
}

pub struct Parser {
    comment_regex: Regex,
    day_header_regex: Regex,
    open_shift_regex: Regex,
    closed_shift_regex: Regex,
    special_shift_regex: Regex,
    special_day_regex: Regex,
    blank_regex: Regex,
}

impl Parser {
    fn new() -> Self {
        Parser {
            comment_regex: Regex::new(r"^# (?P<text>.*)$").unwrap(),
            day_header_regex: Regex::new(r"^\[[a-z]+\s+(?P<year>[0-9]{4})-(?P<month>[0-9]{2})-(?P<day>[0-9]{2})]$").unwrap(),
            open_shift_regex: Regex::new(r"^\* (?P<hour>[0-9]{2}):(?P<minute>[0-9]{2})-$").unwrap(),
            closed_shift_regex: Regex::new(r"^\* (?P<startHour>[0-9]{2}):(?P<startMinute>[0-9]{2})-(?P<stopHour>[0-9]{2}):(?P<stopMinute>[0-9]{2})$").unwrap(),
            special_shift_regex: Regex::new(r"^\* (?P<text>[A-Za-z]+) (?P<startHour>[0-9]{2}):(?P<startMinute>[0-9]{2})-(?P<stopHour>[0-9]{2}):(?P<stopMinute>[0-9]{2})$").unwrap(),
            special_day_regex: Regex::new(r"^\* (?P<text>[A-Za-z]+)$").unwrap(),
            blank_regex: Regex::new(r"^\s*$").unwrap(),
        }
    }

    fn parse_line(self: &Self, string: &str) -> Option<Line> {
        self.parse_comment(string)
            // TODO: Here, other parsers
            .or_else(|| Option::None)
    }

    fn parse_comment(self: &Self, string: &str) -> Option<Line> {
        self.comment_regex.captures(string).map(|m| Comment {
            text: String::from(m.name("text").unwrap().as_str())
        })
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use crate::document::Parser;
    use crate::document::Line::Comment;

    #[test]
    fn read_line() {
        let parser = Parser::new();

        assert_eq!(
            Option::Some(Comment { text: String::from("hello") }),
            parser.parse_line("# hello")
        )
    }
}
