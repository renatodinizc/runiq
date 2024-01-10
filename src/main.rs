use runiq::{execute, get_args};

fn main() {
    let args = get_args();

    if args.files.len() > 2 {
        return eprintln!(
            "runiq: extra operand ‘{}’\nTry 'runiq --help' for more information.",
            args.files[2]
        );
    }

    for file in &args.files {
        execute(file, &args)
    }
}
