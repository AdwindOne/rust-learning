use clap::Parser;
use anyhow::{Context, Result}; // anyhow::Result 用于简化错误处理
use std::fs::File;
use std::io::{self, Read, BufReader, BufRead}; // 引入所需的 io traits 和类型
use std::path::PathBuf;

// 使用 clap derive 宏来定义命令行参数
#[derive(Parser, Debug)]
#[clap(
    name = "rwc", // 程序名
    version = "0.1.0", // 版本
    author = "Jules The AI Assistant", // 作者
    about = "Rust 版本的 wc (word count) 工具", // 程序简介
    long_about = "一个简单的命令行工具，用于统计文件或标准输入中的行数、单词数、字节数和字符数。"
)]
struct Cli {
    #[clap(
        short = 'l',
        long,
        help = "打印行数 (lines)"
    )]
    lines: bool,

    #[clap(
        short = 'w',
        long,
        help = "打印单词数 (words)"
    )]
    words: bool,

    #[clap(
        short = 'c', // wc 的 -c 是字节数
        long,
        help = "打印字节数 (bytes)"
    )]
    bytes: bool,

    #[clap(
        short = 'm', // wc 的 -m 是字符数 (在某些实现中)
        long,
        help = "打印字符数 (chars)"
    )]
    chars: bool,

    // value_parser 用于 PathBuf，可以更好地处理路径
    #[clap(value_parser = clap::value_parser!(PathBuf), name = "FILE", help = "输入文件列表 (如果为空，则从 stdin 读取)")]
    files: Vec<PathBuf>, // 接收一个或多个文件名/路径
}

// 用于存储统计结果的结构体
#[derive(Debug, Default, Clone, Copy)] // Default 用于方便初始化，Copy 用于多文件总计
struct Stats {
    lines: usize,
    words: usize,
    bytes: usize,
    chars: usize,
}

fn main() -> Result<()> { // main 函数返回 anyhow::Result 以便使用 `?`
    let mut cli_args = Cli::parse();

    // 如果没有指定任何统计选项 (-l, -w, -c, -m)，则默认显示行、词、字节
    let no_options_specified = !(cli_args.lines || cli_args.words || cli_args.bytes || cli_args.chars);
    if no_options_specified {
        cli_args.lines = true;
        cli_args.words = true;
        cli_args.bytes = true;
    }

    if cli_args.files.is_empty() {
        // 从标准输入读取
        // println!("从 stdin 读取..."); // 调试信息
        let stdin_stats = process_input(io::stdin(), &cli_args, "-")?;
        print_stats(&stdin_stats, &cli_args, "-");
    } else {
        let mut total_stats = Stats::default();
        let num_files = cli_args.files.len();

        for filepath in &cli_args.files {
            let filename_str = filepath.to_string_lossy(); // 处理非 UTF-8 路径名
            // println!("处理文件: {}", filename_str); // 调试信息
            match File::open(filepath) {
                Ok(file) => {
                    let file_stats = process_input(file, &cli_args, &filename_str)?;
                    print_stats(&file_stats, &cli_args, &filename_str);
                    if num_files > 1 {
                        total_stats.lines += file_stats.lines;
                        total_stats.words += file_stats.words;
                        total_stats.bytes += file_stats.bytes;
                        total_stats.chars += file_stats.chars;
                    }
                }
                Err(e) => {
                    // 使用 eprintln! 将错误信息打印到 stderr
                    eprintln!("rwc: {}: {}", filename_str, e);
                    // 可以在这里选择是否继续处理其他文件，或者直接 panic/exit
                    // 对于 wc 行为，通常会打印错误并继续
                }
            }
        }

        if num_files > 1 {
            print_stats(&total_stats, &cli_args, "total");
        }
    }

    Ok(())
}

// 处理输入源 (可以是文件或标准输入) 并返回统计结果
// R: Read 泛型参数允许我们传入 File 或 Stdin
fn process_input<R: Read>(input_source: R, cli: &Cli, source_name: &str) -> Result<Stats> {
    let mut reader = BufReader::new(input_source);
    let mut stats = Stats::default();
    let mut buffer = String::new(); // 用于存储所有内容以计算字符和单词

    // 为了分别计算字节和字符/单词/行，我们可能需要读取两次或更智能地处理
    // 简单起见，先读取所有内容到 String，这对于非常大的文件不理想
    // 但对于 wc 的常见用例（统计字节、行、词、字符）通常需要完整内容

    // 如果只需要字节数，可以流式读取并计数
    if cli.bytes && !(cli.lines || cli.words || cli.chars) { // 仅字节数优化
        let mut byte_buffer = [0; 8192]; // 8KB buffer
        loop {
            let n = reader.read(&mut byte_buffer)
                .with_context(|| format!("读取 '{}' 时发生错误", source_name))?;
            if n == 0 { break; }
            stats.bytes += n;
        }
        return Ok(stats); // 如果只计算字节，提前返回
    }

    // 对于行、词、字符，需要读取到 String
    reader.read_to_string(&mut buffer)
        .with_context(|| format!("读取 '{}' 内容到字符串时发生错误", source_name))?;

    // 字节数可以直接从 String 的字节长度获取
    if cli.bytes || cli.lines || cli.words || cli.chars { // 如果需要任何非纯字节统计
        stats.bytes = buffer.len(); // String.len() 返回字节数
    }


    if cli.lines {
        stats.lines = buffer.lines().count();
    }

    if cli.words {
        stats.words = buffer.split_whitespace().count();
    }

    if cli.chars {
        stats.chars = buffer.chars().count();
    }

    Ok(stats)
}

// 打印统计结果
fn print_stats(stats: &Stats, cli: &Cli, filename: &str) {
    let mut output_parts = Vec::new();

    if cli.lines {
        output_parts.push(format!("{:>7}", stats.lines)); // 右对齐，宽度7
    }
    if cli.words {
        output_parts.push(format!("{:>7}", stats.words));
    }
    if cli.bytes { // 默认行为包含字节，或显式指定 -c
        output_parts.push(format!("{:>7}", stats.bytes));
    }
    if cli.chars { // 只有显式指定 -m 时才打印字符数
        output_parts.push(format!("{:>7}", stats.chars));
    }

    // 如果什么选项都没指定，但我们走到了这里（意味着 files 非空，或者 stdin）
    // 并且 print_stats 被调用，我们应该打印默认的 行、词、字节
    // 这个逻辑已在 main 中处理，决定哪些字段被计算并传递给 cli
    // 这里只根据 cli 中为 true 的字段打印

    if !output_parts.is_empty() {
        print!("{}", output_parts.join(" "));
        if filename != "-" || !cli.files.is_empty() { // 如果不是来自 stdin 且没有文件名，或者有文件名
            println!(" {}", filename);
        } else {
            println!(); // 对于 stdin 且无文件名的情况，只打印数字
        }
    } else if filename == "-" && cli.files.is_empty() && output_parts.is_empty() {
        // 如果是 stdin，没有指定任何选项，并且没有文件，
        // 默认行为（行、词、字节）应该已经被 cli 结构体设置了。
        // 如果 output_parts 仍然为空，说明没有任何统计被请求，这不应该发生除非逻辑错误。
        // 但为了安全，如果没有任何输出部分，且是 stdin，也打印一个换行。
        // 实际上，如果 options_specified 为 false，cli.lines/words/bytes 会被设为 true。
        // 所以 output_parts 不会为空。
        // println!(); // 这个分支可能永远不会到达
    }
}

// 可以在这里添加单元测试模块
#[cfg(test)]
mod tests {
    use super::*; // 导入外部模块的项

    // 辅助函数，用于从字符串创建模拟的 Read 输入
    fn mock_reader_from_string(s: &str) -> impl Read {
        io::Cursor::new(s.as_bytes().to_vec())
    }

    #[test]
    fn test_stats_calculation_lines() {
        let content = "line one\nline two\nline three";
        let cli = Cli { lines: true, words: false, bytes: false, chars: false, files: vec![] };
        let stats = process_input(mock_reader_from_string(content), &cli, "test").unwrap();
        assert_eq!(stats.lines, 3);
    }

    #[test]
    fn test_stats_calculation_words() {
        let content = "word1 word2  word3\nword4"; // 注意双空格
        let cli = Cli { lines: false, words: true, bytes: false, chars: false, files: vec![] };
        let stats = process_input(mock_reader_from_string(content), &cli, "test").unwrap();
        assert_eq!(stats.words, 4);
    }

    #[test]
    fn test_stats_calculation_bytes() {
        let content = "byte test"; // 9 字节 (ASCII)
        let cli = Cli { lines: false, words: false, bytes: true, chars: false, files: vec![] };
        let stats = process_input(mock_reader_from_string(content), &cli, "test").unwrap();
        assert_eq!(stats.bytes, 9);

        let content_utf8 = "你好"; // 6 字节 (UTF-8)
        let stats_utf8 = process_input(mock_reader_from_string(content_utf8), &cli, "test_utf8").unwrap();
        assert_eq!(stats_utf8.bytes, 6);
    }

    #[test]
    fn test_stats_calculation_bytes_only_optimization() {
        let content = "just bytes please";
        let cli = Cli { lines: false, words: false, bytes: true, chars: false, files: vec![] };
        // process_input 内部有优化，如果只请求字节，会流式读取
        let stats = process_input(mock_reader_from_string(content), &cli, "test_bytes_only").unwrap();
        assert_eq!(stats.bytes, content.len());
    }


    #[test]
    fn test_stats_calculation_chars() {
        let content_ascii = "char test"; // 9 字符
        let cli = Cli { lines: false, words: false, bytes: false, chars: true, files: vec![] };
        let stats_ascii = process_input(mock_reader_from_string(content_ascii), &cli, "test_ascii").unwrap();
        assert_eq!(stats_ascii.chars, 9);

        let content_utf8 = "你好世界"; // 4 字符
        let stats_utf8 = process_input(mock_reader_from_string(content_utf8), &cli, "test_utf8").unwrap();
        assert_eq!(stats_utf8.chars, 4);
    }

    #[test]
    fn test_stats_calculation_all_options() {
        let content = "hello world\n你好 Rustaceans\n";
        // Expected:
        // Bytes: "hello world" (11) + \n (1) + "你好 Rustaceans" (18 for 你好, 1 for space, 9 for Rustaceans) + \n (1) = 12 + 18 + 1 + 1 = 32
        // Chars: "hello world" (11) + \n (1) + "你好 Rustaceans" (2 for 你好, 1 for space, 9 for Rustaceans) + \n (1) = 12 + 2 + 1 + 9 + 1 = 25
        // Lines: 2 (因为末尾有换行符，所以是2行，如果末尾没有换行，lines().count() 会少1，但 wc 行为是算最后一个换行符)
        //        实际上，`buffer.lines().count()` 对于 "a\nb\n" 是 2，对于 "a\nb" 是 2。
        //        对于 `wc` 来说，如果文件末尾没有换行符，最后一行也算一行。
        //        `buffer.lines().count()` 会将末尾无换行符的最后一行也计入。
        // Words: hello, world, 你好, Rustaceans -> 4
        let cli = Cli { lines: true, words: true, bytes: true, chars: true, files: vec![] };
        let stats = process_input(mock_reader_from_string(content), &cli, "test_all").unwrap();

        assert_eq!(stats.lines, 2, "Lines count mismatch");
        assert_eq!(stats.words, 4, "Words count mismatch");
        assert_eq!(stats.bytes, content.len(), "Bytes count mismatch"); // content.len() 是字节数
        assert_eq!(stats.chars, 25, "Chars count mismatch");
    }

    #[test]
    fn test_empty_input() {
        let content = "";
        let cli = Cli { lines: true, words: true, bytes: true, chars: true, files: vec![] };
        let stats = process_input(mock_reader_from_string(content), &cli, "test_empty").unwrap();
        assert_eq!(stats.lines, 0); // "" -> 0 lines, " \n" -> 1 line
        assert_eq!(stats.words, 0);
        assert_eq!(stats.bytes, 0);
        assert_eq!(stats.chars, 0);
    }

     #[test]
    fn test_only_newlines() {
        let content = "\n\n\n"; // 3 newlines -> 3 lines
        let cli = Cli { lines: true, words: true, bytes: true, chars: true, files: vec![] };
        let stats = process_input(mock_reader_from_string(content), &cli, "test_newlines").unwrap();
        assert_eq!(stats.lines, 3);
        assert_eq!(stats.words, 0); // No words
        assert_eq!(stats.bytes, 3);
        assert_eq!(stats.chars, 3); // Newline is a char
    }
}
