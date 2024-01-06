use crate::document::Line::{
    Blank, ClosedShift, Comment, DayHeader, DurationShift, OpenShift, SpecialDay, SpecialShift,
};
use chrono::{Datelike, Duration, IsoWeek, NaiveDate, NaiveTime};
use regex::{Captures, Regex};

#[derive(PartialEq, Debug, Clone)]
pub enum Line {
    Comment {
        text: String,
    },
    DayHeader {
        date: NaiveDate,
    },
    OpenShift {
        start_time: NaiveTime,
    },
    ClosedShift {
        start_time: NaiveTime,
        stop_time: NaiveTime,
    },
    DurationShift {
        text: String,
        duration: Duration,
    },
    SpecialDay {
        text: String,
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
        matches!(self, OpenShift { .. })
            || matches!(self, ClosedShift { .. })
            || matches!(self, SpecialShift { .. })
    }
}

impl ToString for Line {
    fn to_string(&self) -> String {
        match self {
            Comment { text } => format!("# {}", text),
            DayHeader { date } => format!("[{} {}]", "foobar", date),
            OpenShift { start_time } => format!("* {}-{}", start_time.format("%H:%M"), ""),
            ClosedShift {
                start_time,
                stop_time,
            } => format!(
                "* {}-{}",
                start_time.format("%H:%M"),
                stop_time.format("%H:%M")
            ),
            DurationShift { text, duration } => format!(
                "* {} {}h {}m",
                text,
                duration.num_hours(),
                (duration.num_minutes() - duration.num_hours() * 60).abs()
            ),
            SpecialDay { text } => format!("* {}", text),
            SpecialShift {
                text,
                start_time,
                stop_time,
            } => format!(
                "* {} {}-{}",
                text,
                start_time.format("%H:%M"),
                stop_time.format("%H:%M")
            ),
            Blank => String::from(""),
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
        return self
            .lines
            .iter()
            .any(|line| matches!(line, OpenShift { .. }));
    }

    pub fn adding_shift(&self, line: Line) -> Self {
        let before = self
            .lines
            .clone()
            .into_iter()
            .take_while(|line| line.is_shift());
        let after = self
            .lines
            .clone()
            .into_iter()
            .skip_while(|line| line.is_shift());
        let lines: Vec<Line> = before.chain(vec![line]).chain(after).collect();
        Day {
            date: self.date,
            lines,
        }
    }

    pub fn closing_shift(&self, closing_time: NaiveTime) -> Self {
        let open_shift_count = self
            .lines
            .iter()
            .filter(|line| matches!(line, OpenShift { .. }))
            .count();
        if open_shift_count == 0 {
            panic!("No open shift to close!");
        }
        if open_shift_count > 1 {
            panic!("More than one open shift to close!");
        }
        let lines: Vec<Line> = self
            .lines
            .iter()
            .map(|line| match line {
                OpenShift { start_time } => ClosedShift {
                    start_time: *start_time,
                    stop_time: closing_time,
                },
                _ => line.clone(),
            })
            .collect();

        Day {
            date: self.date,
            lines,
        }
    }

    pub fn create(date: NaiveDate, lines: Vec<Line>) -> Self {
        Day { date, lines }
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
        string.push('\n');
        for line in &self.lines {
            string.push_str(&line.to_string());
            string.push('\n');
        }
        string
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Document {
    pub week: IsoWeek,
    pub preamble: Vec<Line>,
    pub days: Vec<Day>,
}

impl Document {
    pub fn new(week: IsoWeek, preamble: Vec<Line>, days: Vec<Day>) -> Self {
        Document {
            week,
            preamble,
            days,
        }
    }

    pub fn empty(week: IsoWeek) -> Self {
        Document::new(week, vec![], vec![])
    }

    pub fn has_open_shift(&self) -> bool {
        return self.days.iter().any(|day| day.has_open_shift());
    }

    /// Find a day
    pub fn get_day(&self, date: NaiveDate) -> Option<&Day> {
        return self.days.iter().find(|d| d.date == date);
    }

    /// Returns the same document but with a certain day replaced
    pub fn replacing_day(&self, date: NaiveDate, day: Day) -> Self {
        Document {
            week: self.week,
            preamble: self.preamble.clone(),
            days: self
                .days
                .iter()
                .cloned()
                .map(|d| if d.date.eq(&date) { day.clone() } else { d })
                .collect(),
        }
    }

    /// Returns the same document but with a certain day inserted in the right place.
    /// And with a blank line before it if needed.
    pub fn inserting_day(&self, day: Day) -> Self {
        let mut days_before: Vec<Day> = self
            .days
            .iter()
            .filter(|&d| d.date < day.date)
            .cloned()
            .collect::<Vec<Day>>();
        let number_of_days = days_before.len();
        if number_of_days > 0 {
            days_before[number_of_days - 1].lines.push(Blank);
        }
        let days_inbetween: Vec<Day> = vec![day.clone()];
        let days_after: Vec<Day> = self
            .days
            .iter()
            .filter(|&d| d.date > day.date)
            .cloned()
            .collect::<Vec<Day>>();
        Document {
            week: self.week,
            preamble: self.preamble.clone(),
            days: days_before
                .into_iter()
                .chain(days_inbetween)
                .chain(days_after)
                .collect(),
        }
    }
}

impl ToString for Document {
    fn to_string(&self) -> String {
        let mut string = String::new();
        for line in &self.preamble {
            string.push_str(&line.to_string());
            string.push('\n');
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
            day_header_regex: Regex::new(r"^\[[a-z]+\s+(?P<year>[0-9]{4})-(?P<month>[0-9]{2})-(?P<day>[0-9]{2})]\s*$").unwrap(),
            open_shift_regex: Regex::new(r"^\* (?P<hour>[0-9]{2}):(?P<minute>[0-9]{2})-\s*$").unwrap(),
            closed_shift_regex: Regex::new(r"^\* (?P<startHour>[0-9]{2}):(?P<startMinute>[0-9]{2})-(?P<stopHour>[0-9]{2}):(?P<stopMinute>[0-9]{2})\s*$").unwrap(),
            duration_shift_regex: Regex::new(r"^\* (?P<text>[A-Za-z]+)\s+(?P<hours>-?[0-9])+\s*h\s+(?P<minutes>[0-9]+)\s*m\s*$").unwrap(),
            special_shift_regex: Regex::new(r"^\* (?P<text>[A-Za-z]+) (?P<startHour>[0-9]{2}):(?P<startMinute>[0-9]{2})-(?P<stopHour>[0-9]{2}):(?P<stopMinute>[0-9]{2})\s*$").unwrap(),
            special_day_regex: Regex::new(r"^\* (?P<text>[A-Za-z]+)\s*$").unwrap(),
            blank_regex: Regex::new(r"^\s*$").unwrap(),
        }
    }

    fn parse_line(&self, string: &str) -> Option<Line> {
        self.parse_comment(string)
            .or_else(|| self.parse_day_header(string))
            .or_else(|| self.parse_open_shift(string))
            .or_else(|| self.parse_closed_shift(string))
            .or_else(|| self.parse_special_shift(string))
            .or_else(|| self.parse_duration_shift(string))
            .or_else(|| self.parse_special_day(string))
            .or_else(|| self.parse_blank(string))
            .or(None)
    }

    fn parse_comment(&self, string: &str) -> Option<Line> {
        self.comment_regex.captures(string).map(|m| Comment {
            text: String::from(m.name("text").unwrap().as_str()),
        })
    }

    fn parse_day_header(&self, string: &str) -> Option<Line> {
        self.day_header_regex.captures(string).map(|m| DayHeader {
            date: NaiveDate::from_ymd_opt(
                get_i32(&m, "year"),
                get_u32(&m, "month"),
                get_u32(&m, "day"),
            )
            .unwrap(),
        })
    }

    fn parse_open_shift(&self, string: &str) -> Option<Line> {
        self.open_shift_regex.captures(string).map(|m| OpenShift {
            start_time: NaiveTime::from_hms_opt(get_u32(&m, "hour"), get_u32(&m, "minute"), 0)
                .unwrap(),
        })
    }

    fn parse_closed_shift(&self, string: &str) -> Option<Line> {
        self.closed_shift_regex
            .captures(string)
            .map(|m| ClosedShift {
                start_time: NaiveTime::from_hms_opt(
                    get_u32(&m, "startHour"),
                    get_u32(&m, "startMinute"),
                    0,
                )
                .unwrap(),
                stop_time: NaiveTime::from_hms_opt(
                    get_u32(&m, "stopHour"),
                    get_u32(&m, "stopMinute"),
                    0,
                )
                .unwrap(),
            })
    }

    fn parse_duration_shift(&self, string: &str) -> Option<Line> {
        self.duration_shift_regex
            .captures(string)
            .map(|m| DurationShift {
                text: String::from(m.name("text").unwrap().as_str()),
                duration: Duration::minutes(
                    get_i64(&m, "hours") * 60
                        + get_i64(&m, "minutes") * get_i64(&m, "hours").signum(),
                ),
            })
    }

    fn parse_special_shift(&self, string: &str) -> Option<Line> {
        self.special_shift_regex
            .captures(string)
            .map(|m| SpecialShift {
                text: String::from(m.name("text").unwrap().as_str()),
                start_time: NaiveTime::from_hms_opt(
                    get_u32(&m, "startHour"),
                    get_u32(&m, "startMinute"),
                    0,
                )
                .unwrap(),
                stop_time: NaiveTime::from_hms_opt(
                    get_u32(&m, "stopHour"),
                    get_u32(&m, "stopMinute"),
                    0,
                )
                .unwrap(),
            })
    }

    fn parse_special_day(&self, string: &str) -> Option<Line> {
        self.special_day_regex.captures(string).map(|m| SpecialDay {
            text: String::from(m.name("text").unwrap().as_str()),
        })
    }

    fn parse_blank(&self, string: &str) -> Option<Line> {
        self.blank_regex.captures(string).map(|_| Blank)
    }

    pub fn parse_document(&self, week: IsoWeek, string: &str) -> Document {
        // Far from pretty, but works..

        let mut preamble: Vec<Line> = Vec::new();
        let mut days: Vec<Day> = Vec::new();
        let mut current_date: Option<NaiveDate> = None;
        let mut current_day_lines: Vec<Line> = Vec::new();

        let lines = string.lines().enumerate().map(|(line_num, l)| {
            self.parse_line(l)
                .unwrap_or_else(|| panic!("line {} could not be parsed: {}", line_num, l))
        });
        for line in lines {
            match current_date {
                Some(date) => match line {
                    DayHeader { date: new_date } => {
                        let days_lines = current_day_lines.clone();
                        current_day_lines.clear();
                        days.push(Day {
                            date,
                            lines: days_lines,
                        });
                        current_date = Some(new_date);
                    }
                    _ => current_day_lines.push(line),
                },
                None => match line {
                    DayHeader { date: new_date } => current_date = Some(new_date),
                    _ => preamble.push(line),
                },
            }
        }
        if let Some(date) = current_date {
            days.push(Day {
                date,
                lines: current_day_lines,
            })
        }
        Document {
            week,
            preamble,
            days,
        }
    }
}

#[cfg(test)]
mod tests;
