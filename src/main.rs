mod document;
mod tracker;
mod report;
mod constants;
mod testutils;

use std::{path::PathBuf, io};

use chrono::Local;
use clap::{Parser, Subcommand, CommandFactory};
use clap_complete::{Shell, generate};
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
    Report,
    /// Generate command-line completions
    Completions {
        shell: Shell
    }
}

fn main() {
    let args = Args::parse();
    let tracker = Tracker::new_with_weekfile(args.weekfile);
    match args.command {
        Some(Commands::Start) => start_tracking(tracker),
        Some(Commands::Stop) => stop_tracking(tracker),
        Some(Commands::Edit) => edit_file(tracker),
        Some(Commands::Report) => show_report(tracker),
        Some(Commands::Completions { shell }) => generate_completions(shell),
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
    let now = Local::now().naive_local();

    tracker.show_report(now);
}

fn generate_completions(shell: Shell) {
    let mut cmd = Args::command();
    generate(shell, &mut cmd, "tracker", &mut io::stdout());
}