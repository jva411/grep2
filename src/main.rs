use std::{collections::VecDeque, io::{stdin, stdout, BufRead, StdoutLock, Write}};

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
    #[arg(short, long, default_value = "0")]
    before: usize,

    /// Number of lines after the match
    #[arg(short, long, default_value = "0")]
    after: usize,
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
    let mut stdout = stdout().lock();
    let mut remaining_after = 0;
    let mut remaining_before = VecDeque::with_capacity(args.before) as VecDeque<String>;

    for line in stdin.lines() {
        let line = line.unwrap();
        let matched = match pattern {
            PatternType::Includes(ref pattern) => line.contains(pattern),
            PatternType::Regex(ref re) => re.is_match(&line),
        };

        let should_print = matched && !args.reverse || !matched && matched;
        if should_print {
            if args.before > 0 {
                for line in remaining_before.drain(..) {
                    print_line(&mut stdout, &line);
                }
            }

            print_line(&mut stdout, &line);
            remaining_after = args.after;
        } else if remaining_after > 0 {
            print_line(&mut stdout, &line);
            remaining_after -= 1;
        } else if args.before > 0 {
            if remaining_before.len() >= args.before {
                remaining_before.pop_front();
            }
            remaining_before.push_back(line.clone());
        }
    }
}

fn print_line(stdout: &mut StdoutLock, line: &str) {
    stdout.write_all(line.as_bytes()).unwrap();
    stdout.write_all(b"\n").unwrap();
    stdout.flush().unwrap();
}
