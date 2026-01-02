use std::{
    env,
    fs::File,
    io::{self, BufReader},
};

use crate::{cli::parse_args, error::LogyError, reader::extract_log_entries};

mod analyzer;
mod cli;
mod error;
mod log;
mod parser;
mod reader;

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), LogyError> {
    let args: Vec<String> = env::args().collect();
    let opts = parse_args(&args)?;

    let reader = make_reader(opts.filename)?;
    if opts.stream {
        reader::stream_logs(reader, opts.level_filter, opts.strict, |entry| {
            print_entry(&entry, opts.json);
        })?;
    } else {
        let entries = extract_log_entries(reader, opts.level_filter, opts.strict)?;
        let counts = analyzer::count_by_level(&entries);
        if opts.json {
            println!("{}", serde_json::to_string_pretty(&entries)?);
        } else {
            for entry in &entries {
                println!("{:?}", entry);
            }

            for (level, count) in counts {
                println!("{:?}: {}", level, count);
            }
        }
    }

    Ok(())
}

fn make_reader(filename: Option<String>) -> Result<Box<dyn std::io::BufRead>, LogyError> {
    Ok(match filename {
        Some(path) => Box::new(BufReader::new(File::open(path)?)),
        None => Box::new(BufReader::new(io::stdin())),
    })
}

fn print_entry(entry: &log::LogEntry, json: bool) {
    if json {
        println!("{}", serde_json::to_string(entry).unwrap());
    } else {
        println!("{:?}", entry);
    }
}
