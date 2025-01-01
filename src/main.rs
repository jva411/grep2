use std::io::{stdin, stdout, BufRead, Write};

use clap::{ArgAction, Parser};
use regex::Regex;


fn main() {
    let args = Args::parse();
    filter_stdin(args);
}

#[derive(Parser)]
struct Args {
    /// The pattern to match
    pattern: String,

    /// Use regex to match patterns
    #[arg(long, action = ArgAction::SetTrue)]
    regex: bool,

    /// Filter input if the pattern NOT matches
    #[arg(short, long, action = ArgAction::SetTrue)]
    reverse: bool,

    /// Number of lines before the match
    #[arg(short, long)]
    before: Option<usize>,

    /// Number of lines after the match
    #[arg(short, long)]
    after: Option<usize>,
}

enum PatternType {
    Includes(String),
    Regex(Regex),
}

/// filter the stdin lines according to the selected pattern type
fn filter_stdin(args: Args) {
    let stdin = stdin().lock();
    let pattern = match args.regex {
        true => PatternType::Regex(Regex::new(&args.pattern).unwrap()),
        false => PatternType::Includes(args.pattern),
    };

    for line in stdin.lines() {
        let line = line.unwrap();
        let matched = match pattern {
            PatternType::Includes(ref pattern) => line.contains(pattern),
            PatternType::Regex(ref re) => re.is_match(&line),
        };

        let should_print = matched && !args.reverse || !matched && matched;
        if should_print {
            stdout().write_all(line.as_bytes()).unwrap();
        }
    }
}