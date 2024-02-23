use std::{io, path::PathBuf};

use ::tracker::tracker::Tracker;
use chrono::Local;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};

/// Track work time
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Use a custom week relative to the current
    #[arg(short, long, value_name = "WEEK")]
    week: Option<i32>,

    /// Use a custom week file (takes precedence over week)
    #[arg(short('f'), long, value_name = "WEEKFILE")]
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
    Edit {
        /// Only show path
        #[arg(short, long)]
        show_path: bool,
    },
    /// Show a report
    Report {
        /// Only report with status code whether work is ongoing
        #[arg(short, long)]
        is_working: bool,
    },
    /// Generate command-line completions
    Completions { shell: Shell },
}

fn main() {
    let args = Args::parse();
    let tracker = Tracker::builder()
        .weekfile(args.weekfile)
        .weekdiff(args.week)
        .build();

    match args.command {
        Some(Commands::Start) => start_tracking(tracker),
        Some(Commands::Stop) => stop_tracking(tracker),
        Some(Commands::Edit { show_path: true }) => show_weekfile_path(tracker),
        Some(Commands::Edit { show_path: false }) => edit_file(tracker),
        Some(Commands::Report { is_working }) => show_report(tracker, is_working),
        Some(Commands::Completions { shell }) => generate_completions(shell),
        None => show_report(tracker, false),
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

fn show_weekfile_path(tracker: Tracker) {
    let now = Local::now();
    let date = now.naive_local().date();

    tracker.show_weekfile_path(date);
}

fn edit_file(tracker: Tracker) {
    let now = Local::now();
    let date = now.naive_local().date();

    tracker.edit_file(date);
}

fn show_report(tracker: Tracker, is_working: bool) {
    let now = Local::now().naive_local();

    tracker.show_report(now, is_working);
}

fn generate_completions(shell: Shell) {
    let mut cmd = Args::command();
    generate(shell, &mut cmd, "tracker", &mut io::stdout());
}
