use std::{
    error::Error,
    fs::File,
    io::{self, BufReader},
};

use clap::Parser;
use logy::*;

fn main() {
    let args = CliArgs::parse();

    let opts = match CliOptions::try_from(args) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("error: {}", e);
            debug_error(&e);
            std::process::exit(1);
        }
    };

    if let Err(e) = run(opts) {
        eprintln!("error: {}", e);
        debug_error(&e);
        std::process::exit(1);
    }
}

fn run(opts: CliOptions) -> Result<(), LogyError> {
    let reader = make_reader(opts.filename)?;

    if opts.stream {
        reader::stream_logs(reader, &opts.filters, opts.strict, |entry| {
            print_entry(&entry, opts.json);
        })?;
    } else {
        let mut entries = reader::extract_log_entries(reader, &opts.filters, opts.strict)?;

        sort_entries(&mut entries, opts.filters.sort);

        if opts.json {
            println!("{}", serde_json::to_string_pretty(&entries)?);
        } else {
            for entry in &entries {
                println!("{}", entry);
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

fn sort_entries(entries: &mut Vec<LogEntry>, sort_mode: SortMode) {
    match sort_mode {
        SortMode::Desc => entries.sort_by_key(|e| std::cmp::Reverse(e.timestamp)),
        SortMode::Asc => entries.sort_by_key(|e| e.timestamp),
    }
}

fn debug_error(err: &LogyError) {
    eprintln!("DISPLAY: {}", err);

    let mut current = err.source();
    let mut depth = 1;

    while let Some(src) = current {
        eprintln!("SOURCE {}: {}", depth, src);
        current = src.source();
        depth += 1;
    }
}
