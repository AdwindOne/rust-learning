use assert_cmd::Command; // 用于测试 CLI 程序
use predicates::prelude::*; // 用于断言输出
use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile; // 用于创建临时文件进行测试

// 辅助函数：创建一个包含指定内容的临时文件
fn create_temp_file(content: &str) -> NamedTempFile {
    let mut temp_file = NamedTempFile::new().expect("无法创建临时文件");
    temp_file.write_all(content.as_bytes()).expect("无法写入临时文件");
    temp_file
}

#[test]
fn test_no_args_reads_from_stdin_placeholder() {
    // 这个测试比较难自动化，因为它需要向 stdin 输入并发送 EOF
    // 通常可以通过管道输入内容来测试：`echo "hello" | target/debug/rwc`
    // assert_cmd 本身可能不直接支持模拟这种管道后立即EOF的场景，
    // 但可以测试带文件参数的情况。
    // 对于 stdin，我们可以简单地检查程序在没有参数时是否不会立即 panic。
    let mut cmd = Command::cargo_bin("rwc").unwrap();
    // 尝试运行，但不提供 stdin 输入，它应该等待。
    // 我们可以设置一个超时，或者只是检查它是否能启动。
    // 为了简单起见，我们这里只做一个基本检查。
    // 更好的测试方法是使用管道将内容传递给它。
    // cmd.timeout(std::time::Duration::from_secs(1)); // 设置超时
    // let assert = cmd.assert().success(); // 如果它等待输入，这可能不会立即成功
    // 这里我们暂时跳过复杂的 stdin 测试，或只做一个非常基本的检查。
    // 例如，检查不带参数运行时是否不立即出错。
    // 如果程序设计为无参数时读取stdin，它会阻塞，测试也会阻塞。
    // 我们可以通过 `cmd.write_stdin("some input\n").assert()...` 来模拟输入。

    let mut cmd = Command::cargo_bin("rwc").unwrap();
    cmd.write_stdin("hello world\nfrom stdin\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("2         3         26").and(predicate::str::contains("-")));
}

#[test]
fn test_file_does_not_exist() {
    let mut cmd = Command::cargo_bin("rwc").unwrap();
    cmd.arg("non_existent_file.txt")
        .assert()
        .failure() // 预期程序会失败 (通常是退出码非0)
        .stderr(predicate::str::contains("rwc: non_existent_file.txt:").and(predicate::str::contains("No such file or directory"))); // Unix
        // .stderr(predicate::str::contains("rwc: non_existent_file.txt:").and(predicate::str::contains("The system cannot find the file specified."))); // Windows
}

#[test]
fn test_single_file_default_options() {
    let temp_file = create_temp_file("hello world\nRust programming\n");
    let filepath_str = temp_file.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("rwc").unwrap();
    cmd.arg(filepath_str)
        .assert()
        .success()
        // 默认输出：lines words bytes filename
        // "hello world" (11+1=12 bytes, 2 words, 1 line)
        // "Rust programming" (16+1=17 bytes, 2 words, 1 line)
        // Total: 2 lines, 4 words, 12+17=29 bytes (if file ends with newline)
        // If temp_file adds a newline for the last line, then lines = 2.
        // String "hello world\nRust programming\n"
        // Lines: 2
        // Words: hello, world, Rust, programming -> 4
        // Bytes: 11 (hello world) + 1 (\n) + 16 (Rust programming) + 1 (\n) = 29
        .stdout(predicate::str::is_match(format!(r"^\s*2\s+4\s+29\s*{}\s*$", regex::escape(filepath_str))).unwrap());
}


#[test]
fn test_single_file_lines_option() {
    let temp_file = create_temp_file("line1\nline2\nline3\n");
    let filepath_str = temp_file.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("rwc").unwrap();
    cmd.arg("-l").arg(filepath_str)
        .assert()
        .success()
        .stdout(predicate::str::is_match(format!(r"^\s*3\s*{}\s*$", regex::escape(filepath_str))).unwrap());
}

#[test]
fn test_single_file_words_option() {
    let temp_file = create_temp_file("one two three four");
    let filepath_str = temp_file.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("rwc").unwrap();
    cmd.arg("-w").arg(filepath_str)
        .assert()
        .success()
        .stdout(predicate::str::is_match(format!(r"^\s*4\s*{}\s*$", regex::escape(filepath_str))).unwrap());
}

#[test]
fn test_single_file_bytes_option() {
    let content = "byte count test"; // 15 bytes
    let temp_file = create_temp_file(content);
    let filepath_str = temp_file.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("rwc").unwrap();
    cmd.arg("-c").arg(filepath_str)
        .assert()
        .success()
        .stdout(predicate::str::is_match(format!(r"^\s*15\s*{}\s*$", regex::escape(filepath_str))).unwrap());
}

#[test]
fn test_single_file_chars_option() {
    let content_utf8 = "你好 Rust"; // 你(3) 好(3)  (1) R(1) u(1) s(1) t(1) = 11 bytes, 6 chars
    let temp_file = create_temp_file(content_utf8);
    let filepath_str = temp_file.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("rwc").unwrap();
    cmd.arg("-m").arg(filepath_str)
        .assert()
        .success()
        .stdout(predicate::str::is_match(format!(r"^\s*6\s*{}\s*$", regex::escape(filepath_str))).unwrap());
}

#[test]
fn test_single_file_all_options() {
    let content = "First line here.\nSecond line with more words.\n";
    // Lines: 2
    // Words: First, line, here, Second, line, with, more, words -> 8
    // Bytes: "First line here." (16) + \n (1) + "Second line with more words." (28) + \n (1) = 17 + 29 = 46
    // Chars: 16 (line1) + 1 (\n) + 28 (line2) + 1 (\n) = 46 (since all ASCII)
    let temp_file = create_temp_file(content);
    let filepath_str = temp_file.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("rwc").unwrap();
    cmd.args(&["-l", "-w", "-c", "-m", filepath_str])
        .assert()
        .success()
        .stdout(predicate::str::is_match(format!(r"^\s*2\s+8\s+46\s+46\s*{}\s*$", regex::escape(filepath_str))).unwrap());
}

#[test]
fn test_multiple_files_default_options() {
    let temp_file1 = create_temp_file("file one content\n"); // 1 line, 3 words, 17 bytes
    let path1_str = temp_file1.path().to_str().unwrap();
    let temp_file2 = create_temp_file("file two has more words here\nand another line\n"); // 2 lines, 8 words, 43 bytes
    let path2_str = temp_file2.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("rwc").unwrap();
    cmd.args(&[path1_str, path2_str])
        .assert()
        .success()
        .stdout(
            predicate::str::contains(format!("  1   3  17 {}", path1_str))
            .and(predicate::str::contains(format!("  2   8  43 {}", path2_str)))
            .and(predicate::str::contains("  3  11  60 total")) // Total: 3 lines, 11 words, 60 bytes
        );
}

#[test]
fn test_multiple_files_specific_options() {
    let temp_file1 = create_temp_file("line one\n"); // 1 line, 2 words
    let path1_str = temp_file1.path().to_str().unwrap();
    let temp_file2 = create_temp_file("another line here\n"); // 1 line, 3 words
    let path2_str = temp_file2.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("rwc").unwrap();
    // Request only lines and words
    cmd.args(&["-l", "-w", path1_str, path2_str])
        .assert()
        .success()
        .stdout(
            predicate::str::contains(format!("  1   2 {}", path1_str))
            .and(predicate::str::contains(format!("  1   3 {}", path2_str)))
            .and(predicate::str::contains("  2   5 total")) // Total: 2 lines, 5 words
        );
}

#[test]
fn test_empty_file() {
    let temp_file = create_temp_file("");
    let filepath_str = temp_file.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("rwc").unwrap();
    cmd.arg(filepath_str) // Default options
        .assert()
        .success()
        .stdout(predicate::str::is_match(format!(r"^\s*0\s+0\s+0\s*{}\s*$", regex::escape(filepath_str))).unwrap());
}
