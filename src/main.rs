use runiq::get_args;

fn main() {
    let input = get_args();

    if input.files.len() > 2 {
        eprintln!("uniq: extra operand ‘{}’\nTry 'runiq --help' for more information.", input.files[2])
    }
}
