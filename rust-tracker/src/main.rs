mod document;
mod tracker;
mod report;

use std::path::PathBuf;

use chrono::{Local};
use clap::{Parser, Subcommand};
use crate::tracker::Tracker;

/// Track work time
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Set a custom week file 
    #[arg(short, long, value_name = "WEEKFILE")]
    weekfile: Option<PathBuf>,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Start tracking
    Start,
    /// Stop tracking
    Stop,
    /// Edit tracking file
    Edit,
    /// Show a report
    Report
}

fn main() {
    let args = Args::parse();
    let tracker = Tracker::new_with_weekfile(args.weekfile);
    match args.command {
        Some(Commands::Start) => start_tracking(tracker),
        Some(Commands::Stop) => stop_tracking(tracker),
        Some(Commands::Edit) => edit_file(tracker),
        Some(Commands::Report) => show_report(tracker),
        None => println!("No commmand!")
    }
}

// Commands

fn start_tracking(tracker: Tracker) {
    let now = Local::now();
    let date = now.naive_local().date();
    let time = now.naive_local().time();

    tracker.start_tracking(date, time);
}

fn stop_tracking(tracker: Tracker) {
    let now = Local::now();
    let date = now.naive_local().date();
    let time = now.naive_local().time();

    tracker.stop_tracking(date, time);
}

fn edit_file(tracker: Tracker) {
    let now = Local::now();
    let date = now.naive_local().date();

    tracker.edit_file(date);
}

fn show_report(tracker: Tracker) {
    let now = Local::now();
    let date = now.naive_local().date();

    tracker.show_report(date);
}

