use assert_cmd::Command;

#[test]
fn process_empty_file() {
    let mut cmd = Command::cargo_bin("runiq").unwrap();

    cmd.arg("tests/inputs/empty.txt");

    cmd.assert().success().stdout("");
}

#[test]
fn process_more_than_two_files() {
    let mut cmd = Command::cargo_bin("runiq").unwrap();

    cmd.arg("tests/inputs/empty.txt");
    cmd.arg("tests/inputs/file1.txt");
    cmd.arg("tests/inputs/file2.txt");
    cmd.arg("tests/inputs/file3.txt");

    cmd.assert().success().stderr(
        "runiq: extra operand ‘tests/inputs/file2.txt’
Try 'runiq --help' for more information.\n",
    );
}

#[test]
fn process_file1() {
    let mut cmd = Command::cargo_bin("runiq").unwrap();

    cmd.arg("tests/inputs/file1.txt");

    cmd.assert().success().stdout("Civil government, so far as it is instituted for the security of property, is in reality instituted for the defence of the rich against the poor, or of those who have some property against those who have none at all - Adam Smith\n");
}

#[test]
fn process_file1_with_c_flag() {
    let mut cmd = Command::cargo_bin("runiq").unwrap();

    cmd.arg("-c");
    cmd.arg("tests/inputs/file1.txt");

    cmd.assert().success().stdout("      1 Civil government, so far as it is instituted for the security of property, is in reality instituted for the defence of the rich against the poor, or of those who have some property against those who have none at all - Adam Smith\n");
}

#[test]
fn process_file2() {
    let mut cmd = Command::cargo_bin("runiq").unwrap();

    cmd.arg("tests/inputs/file2.txt");

    cmd.assert().success().stdout("It was not by gold or by silver, but by labour, that all wealth of the world was originally purchased - Adam Smith\n");
}

#[test]
fn process_file2_with_c_flag() {
    let mut cmd = Command::cargo_bin("runiq").unwrap();

    cmd.arg("-c");
    cmd.arg("tests/inputs/file2.txt");

    cmd.assert().success().stdout("      2 It was not by gold or by silver, but by labour, that all wealth of the world was originally purchased - Adam Smith\n");
}

#[test]
fn process_file3() {
    let mut cmd = Command::cargo_bin("runiq").unwrap();

    cmd.arg("tests/inputs/file3.txt");

    cmd.assert().success().stdout(
        "Our truest life is when we are in dreams awake - Thoreau
Not until we are lost do we begin to understand ourselves - Thoreau
Our truest life is when we are in dreams awake - Thoreau
Not until we are lost do we begin to understand ourselves - Thoreau\n",
    );
}

#[test]
fn process_file3_with_c_flag() {
    let mut cmd = Command::cargo_bin("runiq").unwrap();

    cmd.arg("-c");
    cmd.arg("tests/inputs/file3.txt");

    cmd.assert().success().stdout(
        "      3 Our truest life is when we are in dreams awake - Thoreau
      2 Not until we are lost do we begin to understand ourselves - Thoreau
      1 Our truest life is when we are in dreams awake - Thoreau
      1 Not until we are lost do we begin to understand ourselves - Thoreau\n",
    );
}

#[test]
fn process_file3_with_cu_flags() {
    let mut cmd = Command::cargo_bin("runiq").unwrap();

    cmd.arg("-cu");
    cmd.arg("tests/inputs/file3.txt");

    cmd.assert().success().stdout(
        "      1 Our truest life is when we are in dreams awake - Thoreau
      1 Not until we are lost do we begin to understand ourselves - Thoreau\n",
    );
}

#[test]
fn process_file3_with_cd_flags() {
    let mut cmd = Command::cargo_bin("runiq").unwrap();

    cmd.arg("-cd");
    cmd.arg("tests/inputs/file3.txt");

    cmd.assert().success().stdout(
        "      3 Our truest life is when we are in dreams awake - Thoreau
      2 Not until we are lost do we begin to understand ourselves - Thoreau\n",
    );
}

#[test]
fn process_file4_with_ci_flags() {
    let mut cmd = Command::cargo_bin("runiq").unwrap();

    cmd.arg("-ci");
    cmd.arg("tests/inputs/file4.txt");

    cmd.assert().success().stdout(
        "      3 banana
      1 melancia
      2 Cenoura\n",
    );
}

#[test]
fn process_stdin() {
    let mut cmd = Command::cargo_bin("runiq").unwrap();

    cmd.arg("-cu");
    cmd.write_stdin("apple\napple\napple\ngrape\nbanana");

    cmd.assert().success().stdout(
        "      1 grape
      1 banana\n",
    );
}

#[test]
fn process_stdin2() {
    let mut cmd = Command::cargo_bin("runiq").unwrap();

    cmd.arg("-ui");
    cmd.write_stdin("Apple\napple\norange\napple\nBanana\nBaNaNa");

    cmd.assert().success().stdout(
        "orange
apple\n",
    );
}

#[test]
fn process_stdin3() {
    let mut cmd = Command::cargo_bin("runiq").unwrap();

    cmd.arg("-cdi");
    cmd.write_stdin("apple\nApple\nbanana\nOrange\nApple");

    cmd.assert().success().stdout("      2 apple\n");
}
