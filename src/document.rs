use chrono::{NaiveDate, NaiveTime, Datelike, Duration};
use regex::{Regex, Captures};
use crate::document::Line::{Blank, ClosedShift, Comment, DayHeader, OpenShift, DurationShift, SpecialDay, SpecialShift};

#[derive(PartialEq, Debug, Clone)]
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
    DurationShift {
        text: String,
        duration: Duration
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

impl Line {
    fn is_shift(&self) -> bool {
        return matches!(self, OpenShift {..}) || matches!(self, ClosedShift {..}) || matches!(self, SpecialShift {..})
    }
}

impl ToString for Line {
    fn to_string(&self) -> String {
        match self {
            Comment { text } => format!("# {}", text),
            DayHeader { date } => format!("[{} {}]", "foobar", date),
            OpenShift { start_time } => format!("* {}-{}", start_time.format("%H:%M"), ""),
            ClosedShift { start_time, stop_time } => format!("* {}-{}", start_time.format("%H:%M"), stop_time.format("%H:%M")),
            DurationShift { text, duration } => format!("* {} {}h {}m", text, duration.num_hours(), (duration.num_minutes() - duration.num_hours() * 60).abs()),
            SpecialDay { text } => format!("* {}", text),
            SpecialShift { text, start_time, stop_time } => format!("* {} {}-{}", text, start_time.format("%H:%M"), stop_time.format("%H:%M")),
            Blank => String::from("")
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Day {
    pub date: NaiveDate,
    pub lines: Vec<Line>,
}

impl Day {
    pub fn has_open_shift(&self) -> bool {
        return self.lines.iter().any(|line| matches!(line, OpenShift {..}))
    }

    pub fn adding_shift(&self, line: Line) -> Self {
        let before = self.lines.clone().into_iter().take_while(|line| line.is_shift());
        let after = self.lines.clone().into_iter().skip_while(|line| line.is_shift());
        let lines: Vec<Line> = before
            .chain(vec![line].into_iter())
            .chain(after)
            .collect();
        Day {
            date: self.date.clone(),
            lines: lines
        }
    }

    pub fn closing_shift(&self, closing_time: NaiveTime) -> Self {
        let open_shift_count = self.lines.iter().filter(|line| matches!(line, OpenShift {..})).count();
        if open_shift_count == 0 {
            panic!("No open shift to close!");
        }
        if open_shift_count > 1 {
            panic!("More than one open shift to close!");
        }
        let lines: Vec<Line> = self.lines.iter().map(|line| {
            match line {
                OpenShift { start_time } => ClosedShift { start_time: *start_time, stop_time: closing_time },
                _ => line.clone()
            }
        }).collect();

        Day {
            date: self.date.clone(),
            lines: lines
        }
    }

    pub fn create(date: NaiveDate, lines: Vec<Line>) -> Self {
        Day {
            date: date,
            lines: lines
        }
    }
}

fn format_weekday(date: NaiveDate) -> String {
    match date.weekday() {
        chrono::Weekday::Mon => String::from("monday"),
        chrono::Weekday::Tue => String::from("tuesday"),
        chrono::Weekday::Wed => String::from("wednesday"),
        chrono::Weekday::Thu => String::from("thursday"),
        chrono::Weekday::Fri => String::from("friday"),
        chrono::Weekday::Sat => String::from("saturday"),
        chrono::Weekday::Sun => String::from("sunday"),
    }
}

impl ToString for Day {
    fn to_string(&self) -> String {
        let mut string = String::new();
        string.push_str(&format!("[{} {}]", format_weekday(self.date), self.date));
        string.push_str("\n");
        for line in &self.lines {
            string.push_str(&line.to_string());
            string.push_str("\n");
        }
        string
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Document {
    pub preamble: Vec<Line>,
    pub days: Vec<Day>,
}

impl Document {
    pub fn new(preamble: Vec<Line>, days: Vec<Day>) -> Self {
        return Document {
            preamble: preamble,
            days: days
        }
    }

    pub fn empty() -> Self {
        return Document::new(vec![], vec![]);
    }

    pub fn has_open_shift(&self) -> bool {
        return self.days.iter().any(|day| day.has_open_shift())
    }

    /// Returns the same document but with a certain day replaced
    pub fn replacing_day(&self, date: NaiveDate, day: Day) -> Self {
        Document {
            preamble: self.preamble.clone(),
            days: self.days
                .iter()
                .cloned()
                .map(|d| if d.date.eq(&date) { day.clone() } else { d })
                .collect()
        }
    }

    /// Returns the same document but with a certain day inserted in the right place.
    /// And with a blank line before it if needed.
    pub fn inserting_day(&self, day: Day) -> Self {
        let mut days_before: Vec<Day> = self.days
            .iter()
            .cloned()
            .filter(|d| d.date < day.date)
            .collect::<Vec<Day>>();
        let number_of_days = days_before.len();
        if number_of_days > 0 {
            days_before[number_of_days - 1].lines.push(Blank);
        }
        let days_inbetween: Vec<Day> = vec!(day.clone());
        let days_after: Vec<Day> = self.days
            .iter()
            .cloned()
            .filter(|d| d.date > day.date)
            .collect::<Vec<Day>>();
        Document {
            preamble: self.preamble.clone(),
            days: days_before
                .into_iter()
                .chain(days_inbetween.into_iter())
                .chain(days_after.into_iter())
                .collect()
        }
    }
}

impl ToString for Document {
    fn to_string(&self) -> String {
        let mut string = String::new();
        for line in &self.preamble {
            string.push_str(&line.to_string());
            string.push_str("\n");
        }
        for day in &self.days {
            string.push_str(&day.to_string());
        }
        string
    }
}

pub struct Parser {
    comment_regex: Regex,
    day_header_regex: Regex,
    open_shift_regex: Regex,
    closed_shift_regex: Regex,
    duration_shift_regex: Regex,
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

fn get_i64(m: &Captures, name: &str) -> i64 {
    m.name(name).unwrap().as_str().parse::<i64>().unwrap()
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            comment_regex: Regex::new(r"^# (?P<text>.*)$").unwrap(),
            day_header_regex: Regex::new(r"^\[[a-z]+\s+(?P<year>[0-9]{4})-(?P<month>[0-9]{2})-(?P<day>[0-9]{2})]$").unwrap(),
            open_shift_regex: Regex::new(r"^\* (?P<hour>[0-9]{2}):(?P<minute>[0-9]{2})-$").unwrap(),
            closed_shift_regex: Regex::new(r"^\* (?P<startHour>[0-9]{2}):(?P<startMinute>[0-9]{2})-(?P<stopHour>[0-9]{2}):(?P<stopMinute>[0-9]{2})$").unwrap(),
            duration_shift_regex: Regex::new(r"^\* (?P<text>[A-Za-z]+)\s+(?P<hours>-?[0-9])+\s*h\s+(?P<minutes>[0-9]+)\s*m$").unwrap(),
            special_shift_regex: Regex::new(r"^\* (?P<text>[A-Za-z]+) (?P<startHour>[0-9]{2}):(?P<startMinute>[0-9]{2})-(?P<stopHour>[0-9]{2}):(?P<stopMinute>[0-9]{2})$").unwrap(),
            special_day_regex: Regex::new(r"^\* (?P<text>[A-Za-z]+)$").unwrap(),
            blank_regex: Regex::new(r"^\s*$").unwrap(),
        }
    }

    fn parse_line(self: &Self, string: &str) -> Option<Line> {
        self.parse_comment(string)
            .or_else(|| self.parse_day_header(string))
            .or_else(|| self.parse_open_shift(string))
            .or_else(|| self.parse_closed_shift(string))
            .or_else(|| self.parse_special_shift(string))
            .or_else(|| self.parse_duration_shift(string))
            .or_else(|| self.parse_special_day(string))
            .or_else(|| self.parse_blank(string))
            .or_else(|| None)
    }

    fn parse_comment(self: &Self, string: &str) -> Option<Line> {
        self.comment_regex.captures(string).map(|m| Comment {
            text: String::from(m.name("text").unwrap().as_str())
        })
    }

    fn parse_day_header(self: &Self, string: &str) -> Option<Line> {
        self.day_header_regex.captures(string).map(|m| DayHeader {
            date: NaiveDate::from_ymd_opt(
                get_i32(&m, "year"),
                get_u32(&m, "month"),
                get_u32(&m, "day")
            ).unwrap()
        })
    }

    fn parse_open_shift(self: &Self, string: &str) -> Option<Line> {
        self.open_shift_regex.captures(string).map(|m| OpenShift {
            start_time: NaiveTime::from_hms_opt(
                get_u32(&m, "hour"),
                get_u32(&m, "minute"),
                0
            ).unwrap()
        })
    }

    fn parse_closed_shift(self: &Self, string: &str) -> Option<Line> {
        self.closed_shift_regex.captures(string).map(|m| ClosedShift {
            start_time: NaiveTime::from_hms_opt(
                get_u32(&m, "startHour"),
                get_u32(&m, "startMinute"),
                0
            ).unwrap(),
            stop_time: NaiveTime:: from_hms_opt(
                get_u32(&m, "stopHour"),
                get_u32(&m, "stopMinute"),
                0
            ).unwrap()
        })
    }

    fn parse_duration_shift(self: &Self, string: &str) -> Option<Line> {
        self.duration_shift_regex.captures(string).map(|m| 
            DurationShift {
            text: String::from(m.name("text").unwrap().as_str()),
            duration: Duration::minutes(
                get_i64(&m, "hours") * 60 + 
                get_i64(&m, "minutes") * get_i64(&m, "hours").signum()
            )
        })
    }

    fn parse_special_shift(self: &Self, string: &str) -> Option<Line> {
        self.special_shift_regex.captures(string).map(|m| SpecialShift {
            text: String::from(m.name("text").unwrap().as_str()),
            start_time: NaiveTime::from_hms_opt(
                get_u32(&m, "startHour"),
                get_u32(&m, "startMinute"),
                0
            ).unwrap(),
            stop_time: NaiveTime:: from_hms_opt(
                get_u32(&m, "stopHour"),
                get_u32(&m, "stopMinute"),
                0
            ).unwrap()
        })
    }

    fn parse_special_day(self: &Self, string: &str) -> Option<Line> {
        self.special_day_regex.captures(string).map(|m| SpecialDay {
            text: String::from(m.name("text").unwrap().as_str())
        })
    }

    fn parse_blank(self: &Self, string: &str) -> Option<Line> {
        self.blank_regex.captures(string).map(|_| Blank)
    }

    pub fn parse_document(self: &Self, string: &str) -> Document {
        // Far from pretty, but works..
        
        let mut preamble: Vec<Line> = Vec::new();
        let mut days: Vec<Day> = Vec::new();
        let mut current_date: Option<NaiveDate> = None;
        let mut current_day_lines: Vec<Line> = Vec::new();

        let lines = string.lines().map(|l| self.parse_line(l).unwrap());
        for line in lines {
            match current_date {
                Some(date) => match line {
                    DayHeader {date: new_date } => {
                        let days_lines = current_day_lines.clone();
                        current_day_lines.clear();
                        days.push(Day { date: date, lines: days_lines });
                        current_date = Some(new_date);
                    }
                    _ => current_day_lines.push(line)
                }
                None => match line {
                    DayHeader {date: new_date } => {
                        current_date = Some(new_date)
                    }
                    _ => preamble.push(line)
                }
            }
        }
        match current_date {
            Some(date) => days.push(Day { date: date, lines: current_day_lines }),
            None => ()
        }
        Document {
            preamble: preamble,
            days: days
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::document::{Parser, Document, Day};
    use crate::document::Line::{Comment, SpecialDay, Blank, ClosedShift, SpecialShift, OpenShift, DurationShift, DayHeader};
    use chrono::{NaiveDate, NaiveTime, Duration};

    #[test]
    fn read_line() {
        let parser = Parser::new();

        assert_eq!(
            Option::Some(Comment { text: String::from("hello") }),
            parser.parse_line("# hello")
        );

        assert_eq!(
            Option::Some(DayHeader { date: NaiveDate::from_ymd_opt(2021, 9, 13).unwrap()}),
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
    fn deserialize() {
        let serialized_document = example_1_text();
        let document = example_1_document();

        let parser = Parser::new();
        let parsed = parser.parse_document(&serialized_document);
        assert_eq!(document, parsed)
    }

    #[test]
    fn serialize() {
        let serialized_document = example_1_text();
        let document = example_1_document();

        assert_eq!(serialized_document, document.to_string())
    }

    #[test]
    fn replacing_day_that_does_not_exist() {
        let document = Document {
            preamble: vec![],
            days: vec![]
        };
        let new_document = document.replacing_day(
            NaiveDate::from_ymd_opt(2020, 7, 13).unwrap(),
            Day {
                date: NaiveDate::from_ymd_opt(2020, 7, 13).unwrap(),
                lines: vec![]
            }
        );
        assert_eq!(document, new_document)
    }

    // Helpers

    fn time_hm(hour: u32, minute: u32) -> NaiveTime {
        NaiveTime::from_hms_opt(hour, minute, 0).unwrap()
    }

    // Test data

    fn example_1_document() -> Document {
        Document {
            preamble: vec![
                Comment { text: String::from("Preamble") },
                DurationShift { 
                    text: String::from("carry"),
                    duration: Duration::minutes(70)
                },
                Blank
            ],
            days: vec![
                Day {
                    date: NaiveDate::from_ymd_opt(2020, 7, 13).unwrap(),
                    lines: vec![
                        SpecialDay { text: String::from("Vacation") },
                        Comment { text: String::from("Came back from Jämtland")},
                        Blank
                    ]
                },
                Day {
                    date: NaiveDate::from_ymd_opt(2020, 7, 14).unwrap(),
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
                    date: NaiveDate::from_ymd_opt(2020, 7, 15).unwrap(),
                    lines: vec![
                        ClosedShift {
                            start_time: time_hm(11, 0),
                            stop_time: time_hm(18, 0)
                        },
                        Blank
                    ]
                },
                Day {
                    date: NaiveDate::from_ymd_opt(2020, 7, 16).unwrap(),
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
                    date: NaiveDate::from_ymd_opt(2020, 7, 17).unwrap(),
                    lines: vec![
                        OpenShift {
                            start_time: time_hm(8, 12)
                        }
                    ]
                }
            ]
        }
    }

    fn example_1_text() -> String {
        String::from("# Preamble
* carry 1h 10m

[monday 2020-07-13]
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
")
    }


}