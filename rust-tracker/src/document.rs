use chrono::{NaiveDate, NaiveTime};
use regex::{Regex, Captures};
use crate::document::Line::{Comment, DayHeader, OpenShift};

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

#[derive(PartialEq, Debug)]
pub struct Day {
    date: NaiveDate,
    lines: Vec<Line>,
}

#[derive(PartialEq, Debug)]
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

fn get_i32(m: &Captures, name: &str) -> i32 {
    m.name(name).unwrap().as_str().parse::<i32>().unwrap()
}

fn get_u32(m: &Captures, name: &str) -> u32 {
    m.name(name).unwrap().as_str().parse::<u32>().unwrap()
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
            .or_else(|| self.parse_day_header(string))
            .or_else(|| self.parse_open_shift(string))
            // TODO: Here, other parsers
            .or_else(|| None)
    }

    fn parse_comment(self: &Self, string: &str) -> Option<Line> {
        self.comment_regex.captures(string).map(|m| Comment {
            text: String::from(m.name("text").unwrap().as_str())
        })
    }

    fn parse_day_header(self: &Self, string: &str) -> Option<Line> {
        self.day_header_regex.captures(string).map(|m| DayHeader {
            date: NaiveDate::from_ymd(
                get_i32(&m, "year"),
                get_u32(&m, "month"),
                get_u32(&m, "day")
            )
        })
    }

    fn parse_open_shift(self: &Self, string: &str) -> Option<Line> {
        self.open_shift_regex.captures(string).map(|m| OpenShift {
            start_time: NaiveTime::from_hms(
                get_u32(&m, "hour"),
                get_u32(&m, "minute"),
                0
            )
        })
    }

    fn parse_document(self: &Self, string: &str) -> Document {
        Document {
            preamble: vec![],
            days: vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::document::{Parser, Document, Day};
    use crate::document::Line::{Comment, SpecialDay, Blank, ClosedShift, SpecialShift, OpenShift, DayHeader};
    use chrono::{NaiveDate, NaiveTime};

    #[test]
    fn read_line() {
        let parser = Parser::new();

        assert_eq!(
            Option::Some(Comment { text: String::from("hello") }),
            parser.parse_line("# hello")
        );

        assert_eq!(
            Option::Some(DayHeader { date: NaiveDate::from_ymd(2021, 9, 13)}),
            parser.parse_line("[monday 2021-09-13]")
        );

        assert_eq!(
            Option::Some(OpenShift { start_time: time_hm(8, 12) }),
            parser.parse_line("* 08:12-")
        );

        assert_eq!(
            Option::Some(ClosedShift { start_time: time_hm(8, 24), stop_time: time_hm(9, 12)}),
            parser.parse_line("* 08:24-09:12")
        );

        assert_eq!(
            Option::Some(SpecialDay { text: String::from("hello")}),
            parser.parse_line("* hello")
        );

        assert_eq!(
            Option::Some(SpecialShift { text: String::from("VAB"), start_time: time_hm(13, 5), stop_time: time_hm(20, 2)}),
            parser.parse_line("* VAB 13:05-20:02")
        );

        assert_eq!(
            Option::Some(Blank),
            parser.parse_line("")
        );

    }

    #[test]
    fn serialize_deserialize() {
        let serialized_form = "[monday 2020-07-13]
* Vacation
# Came back from Jämtland

[tuesday 2020-07-14]
* 08:32-12:02
* 12:30-13:01
* 13:45-18:03

[wednesday 2020-07-15]
* 11:00-18:00

[thursday 2020-07-16]
* 08:00-12:00
* VAB 13:00-17:00

[friday 2020-07-17]
* 08:12-
";
        let document = Document {
            preamble: vec![],
            days: vec![
                Day {
                    date: NaiveDate::from_ymd(2020, 7, 13),
                    lines: vec![
                        SpecialDay { text: String::from("Vacation") },
                        Comment { text: String::from("Came back from Jämtland")},
                        Blank
                    ]
                },
                Day {
                    date: NaiveDate::from_ymd(2020, 7, 14),
                    lines: vec![
                        ClosedShift {
                            start_time: time_hm(8, 32),
                            stop_time: time_hm(12, 2)
                        },
                        ClosedShift {
                            start_time: time_hm(12, 30),
                            stop_time: time_hm(13, 1)
                        },
                        ClosedShift {
                            start_time: time_hm(13, 45),
                            stop_time: time_hm(18, 3)
                        },
                        Blank
                    ]
                },
                Day {
                    date: NaiveDate::from_ymd(2020, 7, 15),
                    lines: vec![
                        ClosedShift {
                            start_time: time_hm(11, 0),
                            stop_time: time_hm(18, 0)
                        },
                        Blank
                    ]
                },
                Day {
                    date: NaiveDate::from_ymd(2020, 7, 16),
                    lines: vec![
                        ClosedShift {
                            start_time: time_hm(8, 0),
                            stop_time: time_hm(12, 0)
                        },
                        SpecialShift {
                            text: String::from("VAB"),
                            start_time: time_hm(13, 0),
                            stop_time: time_hm(17, 0)
                        },
                        Blank
                    ]
                },
                Day {
                    date: NaiveDate::from_ymd(2020, 7, 17),
                    lines: vec![
                        OpenShift {
                            start_time: time_hm(8, 12)
                        }
                    ]
                }
            ]
        };

        let parser = Parser::new();
        let parsed = parser.parse_document(serialized_form);
        assert_eq!(
            document,
            parsed
        )
    }

    fn time_hm(hour: u32, minute: u32) -> NaiveTime {
        NaiveTime::from_hms(hour, minute, 0)
    }
}
