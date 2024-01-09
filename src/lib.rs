use clap::{command, Arg, ArgAction};

pub struct Input {
    pub files: Vec<String>,
    pub count: bool,
}

pub fn get_args() -> Input {
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
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    Input {
        files: files,
        count: *matches.get_one::<bool>("count").unwrap(),
    }
}
