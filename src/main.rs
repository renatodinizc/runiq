use runiq::{execute, get_args};

fn main() {
    let args = get_args();

    if args.files.len() >= 3 {
        eprintln!(
            "runiq: extra operand ‘{}’\nTry 'runiq --help' for more information.",
            args.files[2]
        );
    } else if args.files.len() == 2 {
        execute(args.files.first().unwrap(), &args, args.files.last());
    } else {
        execute(args.files.last().unwrap(), &args, None);
    }
}
