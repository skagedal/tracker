// We shall read the file

mod document;

use std::fs;
use clap::{Parser, Subcommand};

/// Track work time
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
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
        Some(Commands::Start) => println!("Let's track!"),
        Some(Commands::Stop) => println!("Let's stop tracking!"),
        Some(Commands::Edit) => println!("Let's edit file!"),
        Some(Commands::Report) => show_report(),
        None => println!("No commmand!")
    }
}

fn show_report() {
    println!("Let's show a report!");
    let result = fs::read_to_string("/Users/simon/.simons-assistant/data/tracker/2022-W18.txt");
    match result {
        Ok(content) => show_report_of_content(content),
        Err(err) => eprintln!("Error: {}", err)
    }
}

fn show_report_of_content(content: String) {
    let parser = document::Parser::new();
    let document =  parser.parse_document(&content);
    println!("{:?}", document);
}