use std::{io, path::PathBuf};

use ::tracker::paths::TrackerDirs;
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
    #[arg(short('f'), long("weekfile"), value_name = "WEEKFILE")]
    explicit_weekfile: Option<PathBuf>,

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
    let now = Local::now().naive_local();
    let dirs = TrackerDirs::real();
    let tracker = Tracker::builder(now, dirs)
        .explicit_weekfile(args.explicit_weekfile)
        .weekdiff(args.week)
        .build();

    match args.command {
        Some(Commands::Start) => tracker.start_tracking(),
        Some(Commands::Stop) => tracker.stop_tracking(),
        Some(Commands::Edit { show_path: true }) => tracker.show_weekfile_path(),
        Some(Commands::Edit { show_path: false }) => tracker.edit_file(),
        Some(Commands::Report { is_working }) => tracker.show_report(is_working),
        Some(Commands::Completions { shell }) => generate_completions(shell),
        None => tracker.show_report(false),
    }
}

fn generate_completions(shell: Shell) {
    let mut cmd = Args::command();
    generate(shell, &mut cmd, "tracker", &mut io::stdout());
}
