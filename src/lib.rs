use clap::{command, Arg, ArgAction};
use std::io::{ self, BufRead, BufReader };
use std::fmt;
use std::fs::File;
pub struct Args {
    pub files: Vec<Input>,
    pub count: bool,
    pub unique: bool,
    pub ignore_case: bool
}

pub enum Input {
    File(String),
    Stdin
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
        files: files,
        count: *matches.get_one::<bool>("count").unwrap(),
        unique: *matches.get_one::<bool>("unique").unwrap(),
        ignore_case: *matches.get_one::<bool>("ignore-case").unwrap(),
    }
}

pub fn execute(file: &Input, args: &Args) {
    match file {
        Input::Stdin => read_from_stdin(args),
        Input::File(file_path) => read_from_file(file_path, args),
    }
}

fn read_from_file(file: &str, args: &Args) {
    if let Err(error) = File::open(file) { return eprintln!("runiq: {}: {}", file, error) }

    let content = File::open(file).unwrap();
    let buffer = BufReader::new(content);

    let mut results: Vec<(String, u32)> = Vec::new();

    for line in buffer.lines() {
        match line {
            Err(error) => eprintln!("{error}"),
            Ok(content) => {
                if results.len() == 0 {
                    results.push((content, 1));
                } else if results.last().unwrap().0 == content {
                    let counter = results.last().unwrap().1 + 1;
                    results.pop();
                    results.push((content, counter));
                } else {
                    results.push((content, 1));
                }
            },
        }
    }
    for (content, count) in results {
        if args.count {
            println!("{:7} {content}", count);
        } else {
            println!("{content}");
        }
    }
}

fn read_from_stdin(args: &Args) {
    let stdin = io::stdin();


    let mut results: Vec<(String, u32)> = Vec::new();

    for line in stdin.lines() {
        match line {
            Err(error) => eprintln!("{error}"),
            Ok(content) => {
                if results.len() == 0 {
                    results.push((content, 1));
                } else if results.last().unwrap().0 == content {
                    let counter = results.last().unwrap().1 + 1;
                    results.pop();
                    results.push((content, counter));
                } else {
                    results.push((content, 1));
                }
            },
        }
    }

    for (content, count) in results {
        if args.count {
            println!("{:7} {content}", count);
        } else {
            println!("{content}");
        }
    }
}
