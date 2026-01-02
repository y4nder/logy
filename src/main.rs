use std::{
    env,
    fs::File,
    io::{self, BufReader},
};

use crate::{cli::parse_args, error::LogyError, log::LogEntry, reader::extract_log_entries};

mod cli;
mod error;
mod filters;
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
        reader::stream_logs(reader, &opts.filters, opts.strict, |entry| {
            print_entry(&entry, opts.json);
        })?;
    } else {
        let mut entries = extract_log_entries(reader, &opts.filters, opts.strict)?;

        sort_entries(&mut entries, opts.desc);

        if opts.json {
            println!("{}", serde_json::to_string_pretty(&entries)?);
        } else {
            for entry in &entries {
                println!("{:?}", entry);
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

fn sort_entries(entries: &mut Vec<LogEntry>, desc: bool) {
    if desc {
        entries.sort_by_key(|e| std::cmp::Reverse(e.date));
    } else {
        entries.sort_by_key(|e| e.date);
    }
}
