use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::log::LogLevel;

mod analyzer;
mod log;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let level_filter = args.iter().find_map(|arg| match arg.as_str() {
        "--level=INFO" => Some(LogLevel::Info),
        "--level=WARN" => Some(LogLevel::Warn),
        "--level=ERROR" => Some(LogLevel::Error),
        _ => None,
    });

    let file = File::open(filename).expect("failed to open file");

    let reader = BufReader::new(file);

    let entries: Vec<_> = reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| parser::parse_line(&line))
        .filter(|e| level_filter.map_or(true, |lvl| e.0 == lvl))
        .collect();

    let counts = analyzer::count_by_level(&entries);

    for (level, count) in counts {
        println!("{:?}: {}", level, count);
    }
}
