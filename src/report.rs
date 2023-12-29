use std::{
    cmp::min,
    ops::{Add, Sub},
};

use chrono::{Datelike, Duration, NaiveDateTime};

use crate::constants;
use crate::document::{Day, Document, Line};

#[derive(PartialEq, Debug, Clone)]
pub struct Report {
    pub duration_today: Duration,
    pub duration_week: Duration,
    pub is_ongoing: bool,
    pub balance: Duration,
}

fn duration_for_line(line: &Line, now: Option<NaiveDateTime>) -> Duration {
    match line {
        Line::ClosedShift {
            start_time,
            stop_time,
        } => stop_time.signed_duration_since(*start_time),
        Line::OpenShift { start_time } => now
            .map(|now| now.time().signed_duration_since(*start_time))
            .unwrap_or_else(|| Duration::zero()),
        Line::SpecialShift {
            start_time,
            stop_time,
            ..
        } => stop_time.signed_duration_since(*start_time),
        Line::SpecialDay { .. } => Duration::hours(constants::WORK_HOURS_PER_DAY.into()),
        _ => Duration::zero(),
    }
}

fn duration_for_day(day: &Day) -> Duration {
    day.lines.iter().fold(Duration::hours(0), |acc, line| {
        acc + duration_for_line(line, None)
    })
}

fn duration_for_today(day: &Day, now: &NaiveDateTime) -> Duration {
    day.lines.iter().fold(Duration::hours(0), |acc, line| {
        acc + duration_for_line(line, Some(*now))
    })
}

impl Report {
    pub fn from_document(document: &Document, now: &NaiveDateTime) -> Report {
        let this_day = document.days.iter().find(|day| day.date == now.date());
        let duration_today = this_day
            .map(|day| duration_for_today(day, now))
            .unwrap_or_else(|| Duration::zero());
        let duration_week = document
            .days
            .iter()
            .filter(|day| day.date != now.date())
            .fold(Duration::hours(0), |acc, day| acc + duration_for_day(day))
            .add(duration_today);
        let expected_days_so_far = min(
            now.weekday().num_days_from_monday() + 1,
            constants::WORK_DAYS_PER_WEEK,
        );
        let expected_duration_so_far_week =
            Duration::hours((expected_days_so_far * constants::WORK_HOURS_PER_DAY).into());
        let incoming_balance: Duration = document
            .preamble
            .iter()
            .filter_map(|d| match d {
                Line::DurationShift { text: _, duration } => Some(duration),
                _ => None,
            })
            .sum();
        Report {
            duration_today,
            duration_week,
            is_ongoing: this_day.map(Day::has_open_shift).unwrap_or_else(|| false),
            balance: duration_week
                .sub(expected_duration_so_far_week)
                .add(incoming_balance),
        }
    }
}

#[cfg(test)]
mod tests;
