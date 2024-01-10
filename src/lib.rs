use clap::{command, Arg, ArgAction};
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
pub struct Args {
    pub files: Vec<Input>,
    count: bool,
    unique: bool,
    repeated: bool,
    ignore_case: bool,
}

pub enum Input {
    File(String),
    Stdin,
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Input::File(file_path) => write!(f, "{}", file_path),
            Input::Stdin => write!(f, "this is stdin"),
        }
    }
}

pub fn get_args() -> Args {
    let matches = command!()
        .arg(
            Arg::new("count")
                .help("prefix lines by the number of occurrences")
                .short('c')
                .long("count")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("unique")
                .help("only print unique lines")
                .short('u')
                .long("unique")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("repeated")
                .help("only print duplicate lines, one for each group")
                .short('d')
                .long("repeated")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ignore-case")
                .help("ignore differences in case when comparing")
                .short('i')
                .long("ignore-case")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("files")
                .action(ArgAction::Append)
                .action(ArgAction::Append)
                .default_value("-"),
        )
        .get_matches();

    let files = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|f| {
            if f == "-" {
                Input::Stdin
            } else {
                Input::File(f.to_string())
            }
        })
        .collect::<Vec<Input>>();

    Args {
        files,
        count: *matches.get_one::<bool>("count").unwrap(),
        unique: *matches.get_one::<bool>("unique").unwrap(),
        repeated: *matches.get_one::<bool>("repeated").unwrap(),
        ignore_case: *matches.get_one::<bool>("ignore-case").unwrap(),
    }
}

pub fn execute(file: &Input, args: &Args) {
    match file {
        Input::Stdin => read_from_stdin(args),
        Input::File(file_path) => read_from_file(file_path, args),
    }
}

fn read_from_stdin(args: &Args) {
    let stdin = BufReader::new(io::stdin());
    let result = process(stdin, args);

    display_result(result, args);
}

fn read_from_file(file: &str, args: &Args) {
    if let Err(error) = File::open(file) {
        return eprintln!("runiq: {}: {}", file, error);
    }

    let content = File::open(file).unwrap();
    let buffer = BufReader::new(content);
    let result = process(buffer, args);

    display_result(result, args);
}

fn process(buffer: impl BufRead, args: &Args) -> Vec<(String, u32)> {
    let mut results: Vec<(String, u32)> = Vec::new();

    for line in buffer.lines() {
        match line {
            Err(error) => eprintln!("{error}"),
            Ok(content) => {
                if results.is_empty() {
                    results.push((content, 1));
                } else if args.ignore_case
                    && results.last().unwrap().0.to_lowercase() == content.to_lowercase()
                {
                    results.last_mut().unwrap().1 += 1;
                } else if results.last().unwrap().0 == content {
                    results.last_mut().unwrap().1 += 1;
                } else {
                    results.push((content, 1));
                }
            }
        }
    }

    results
}

fn display_result(result: Vec<(String, u32)>, args: &Args) {
    for (content, count) in result {
        if args.unique && count != 1 {
            continue;
        }
        if args.repeated && count == 1 {
            continue;
        }
        if args.count {
            println!("{:7} {content}", count);
        } else {
            println!("{content}");
        }
    }
}
