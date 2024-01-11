use clap::{command, Arg, ArgAction};
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
pub struct Args {
    pub files: Vec<String>,
    count: bool,
    unique: bool,
    repeated: bool,
    ignore_case: bool,
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
        .map(|f| f.to_string())
        .collect::<Vec<String>>();

    Args {
        files,
        count: *matches.get_one::<bool>("count").unwrap(),
        unique: *matches.get_one::<bool>("unique").unwrap(),
        repeated: *matches.get_one::<bool>("repeated").unwrap(),
        ignore_case: *matches.get_one::<bool>("ignore-case").unwrap(),
    }
}

pub fn execute(file: &str, args: &Args, output_file: Option<&String>) {
    let final_result = if file == "-" {
        read_from_stdin(args)
    } else {
        read_from_file(file, args)
    };

    match output_file {
        None => {
            for line in final_result {
                println!("{line}");
            }
        }
        Some(output_file) => {
            fs::write(output_file, final_result.join("\n")).expect("Unable to write to file");
        }
    }
}

fn read_from_stdin(args: &Args) -> Vec<String> {
    let stdin = BufReader::new(io::stdin());
    let result = process(stdin, args);

    display_result(result, args)
}

fn read_from_file(file: &str, args: &Args) -> Vec<String> {
    if let Err(error) = File::open(file) {
        eprintln!("runiq: {}: {}", file, error);
        return vec!["".to_string()];
    }

    let content = File::open(file).unwrap();
    let buffer = BufReader::new(content);
    let result = process(buffer, args);

    display_result(result, args)
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

fn display_result(result: Vec<(String, u32)>, args: &Args) -> Vec<String> {
    let mut final_text: Vec<String> = Vec::new();
    for (content, count) in result {
        if args.unique && count != 1 {
            continue;
        }
        if args.repeated && count == 1 {
            continue;
        }
        if args.count {
            final_text.push(format!("{:7} {content}", count));
        } else {
            final_text.push(content);
        }
    }

    final_text
}
