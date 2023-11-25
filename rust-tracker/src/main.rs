// We shall read the file

mod document;
mod tracker;

use std::path::PathBuf;

use chrono::{Local};
use clap::{Parser, Subcommand};
use crate::tracker::Tracker;

/// Track work time
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Sets a custom week file 
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
    match args.command {
        Some(Commands::Start) => start_tracking(Tracker::new_with_weekfile(args.weekfile)),
        Some(Commands::Stop) => stop_tracking(),
        Some(Commands::Edit) => edit_file(),
        Some(Commands::Report) => show_report(Tracker::new()),
        None => println!("No commmand!")
    }
}

fn edit_file() {
    println!("Let's edit file!");
    todo!()
}

fn stop_tracking() {
    println!("Let's stop tracking!");
    todo!();
}

// Commands

fn start_tracking(tracker: Tracker) {
    let now = Local::now();
    let date = now.naive_local().date();
    let time = now.naive_local().time();

    tracker.start_tracking(date, time);
}

fn show_report(tracker: Tracker) {
    let now = Local::now();
    let date = now.naive_local().date();

    tracker.show_report(date);
}

