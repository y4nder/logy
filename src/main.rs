use std::{env, fs::File, io::BufReader};

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

    let file = File::open(&opts.filename)?;
    let reader = BufReader::new(file);
    let entries = extract_log_entries(reader, opts.level_filter);
    let counts = analyzer::count_by_level(&entries);

    for (level, count) in counts {
        println!("{:?}: {}", level, count);
    }

    Ok(())
}
