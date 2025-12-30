use std::{env, fs::File, io::BufReader};

mod analyzer;
mod log;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let level_filter = parser::determine_level_filter(&args);

    let file = File::open(filename).expect("failed to open file");
    let reader = BufReader::new(file);
    let entries = parser::extract_log_entries(reader, level_filter);
    let counts = analyzer::count_by_level(&entries);

    for (level, count) in counts {
        println!("{:?}: {}", level, count);
    }
}
