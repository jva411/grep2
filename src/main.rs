use std::io::BufRead;

use clap::{ArgAction, Parser};
use regex::Regex;


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

fn main() {
    let args = Args::parse();

    if args.regex {
        filter_stdin_by_regex(args);
    } else {
        filter_stdin_by_includes(args);
    }
}

/// filter the stdin lines according to the pattern and send to stdout
fn filter_stdin_by_includes(args: Args) {
    let stdin = std::io::stdin().lock();

    for line in stdin.lines() {
        let line = line.unwrap();
        let matched = line.contains(&args.pattern);

        if matched && !args.reverse {
            println!("{}", line);
        } else if !matched && args.reverse {
            println!("{}", line);
        }
    }
}

/// filter the stdin lines according to the pattern and send to stdout
fn filter_stdin_by_regex(args: Args) {
    let stdin = std::io::stdin().lock();
    let re = Regex::new(&args.pattern).unwrap();

    for line in stdin.lines() {
        let line = line.unwrap();
        let matched = re.is_match(&line);

        if matched && !args.reverse {
            println!("{}", line);
        } else if !matched && args.reverse {
            println!("{}", line);
        }
    }
}
