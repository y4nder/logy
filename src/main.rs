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

    let reader: Box<dyn std::io::BufRead> = match opts.filename {
        Some(path) => {
            let file = File::open(path)?;
            Box::new(BufReader::new(file))
        }
        None => Box::new(BufReader::new(io::stdin())),
    };

    let entries = extract_log_entries(reader, opts.level_filter);
    let counts = analyzer::count_by_level(&entries);

    if opts.json {
        println!("{}", serde_json::to_string_pretty(&entries)?);
    } else {
        for (level, count) in counts {
            println!("{:?}: {}", level, count);
        }
    }

    Ok(())
}
