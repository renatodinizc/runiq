# Rust UNIQ Utility

`runiq` is a simple command-line utility written in Rust that mimics the functionality of the classic GNU uniq command, that filters adjacent matching lines from input. It can be used to count, filter unique lines, or print duplicate lines from a given set of files or standard input.

## Usage

```bash
runiq [OPTIONS] [FILES...]
```

### Options

- `-c`, `--count`: Prefix lines by the number of occurrences.
- `-u`, `--unique`: Only print unique lines.
- `-d`, `--repeated`: Only print duplicate lines, one for each group.
- `-i`, `--ignore-case`: Ignore differences in case when comparing.

### Arguments

- `FILES`: Input files. Use "-" to read from standard input.

## Examples

```bash
# Count occurrences of each line in a file
runiq -c file.txt

# Print only unique lines from standard input
echo -e "apple\norange\napple\nbanana" | runiq -u

# Print duplicate lines along with their counts
runiq -d file.txt

# Ignore case when comparing lines
runiq -i file.txt
```

## How to Build and Install

Ensure you have Rust installed on your system. Then, clone the repository and build the project using the following commands:

```bash
git clone https://github.com/yourusername/runiq.git
cd runiq
cargo build --release
```

The executable will be available in the `target/release` directory. You can either run it directly from there or copy it to a directory in your system's `PATH`.

## Additional Test Cases

Here are three test cases to demonstrate the functionality of of various flags in `runiq`:

### Test Case 1: Counting Unique Lines

#### Input
```bash
echo -e "apple\napple\napple\ngrape\nbanana" | runiq -cu
```

#### Expected Output
```
      1 grape
      1 banana
```

### Test Case 2: Filtering Duplicate Lines, Ignoring Case

#### Input
```bash
echo -e "Apple\napple\norange\napple\nBanana\nBaNaNa" | runiq -ui
```

#### Expected Output
```
orange
apple
```

### Test Case 3: Counting Sequentially Repeated Lines, Ignoring Case

#### Input
```bash
echo -e "apple\nApple\nbanana\nOrange\nApple" | runiq -cdi
```

#### Expected Output
```
      2 apple
```

These test cases showcase more complex scenarios by combining multiple flags. Feel free to experiment with different combinations of flags to suit your specific use cases.

## Dependencies

- [clap](https://docs.rs/clap): A powerful command-line argument parser for Rust.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or create a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.
