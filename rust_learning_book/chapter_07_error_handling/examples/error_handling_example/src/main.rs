use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::error::Error; // 用于 main 返回 Result 和自定义错误
use std::fmt;

// --- 7.1 panic! ---
fn cause_panic() {
    // panic!("这是一个故意的 panic!");
    let v = vec![1, 2];
    // v[5]; // 索引越界会导致 panic
    println!("这行代码在 panic 后不会执行。");
}

// --- 7.2 Result<T, E> ---

// 7.2.1 使用 match 处理 Result
fn open_or_create_file(filename: &str) {
    let f_result = File::open(filename);

    let _f = match f_result {
        Ok(file) => {
            println!("文件 '{}' 打开成功。", filename);
            file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(fc) => {
                    println!("文件 '{}' 未找到，已成功创建。", filename);
                    fc
                }
                Err(e) => {
                    panic!("尝试创建文件 '{}' 失败: {:?}", filename, e);
                }
            },
            other_error => {
                panic!("打开文件 '{}' 时遇到问题: {:?}", filename, other_error);
            }
        },
    };
    // 可以对 _f 进行操作
}

// 7.2.2 unwrap 和 expect
fn unwrap_example() {
    // 假设 temp_ok.txt 存在
    // fs::write("temp_ok.txt", "content").expect("无法写入 temp_ok.txt");
    // let _f_ok = File::open("temp_ok.txt").unwrap();
    // println!("temp_ok.txt 打开成功 (unwrap)。");
    // fs::remove_file("temp_ok.txt").ok(); // 清理，忽略结果

    // 假设 temp_err.txt 不存在
    // let _f_err = File::open("temp_err.txt").unwrap(); // 这会 panic
    // println!("这行不会执行 (unwrap)。");
}

fn expect_example() {
    // 假设 temp_expect.txt 不存在
    // let _f_expect = File::open("temp_expect.txt")
    //     .expect("打开 temp_expect.txt 失败，请确保文件存在！"); // panic 并显示此消息
    // println!("这行不会执行 (expect)。");
}

// 7.2.3 & 7.2.4 传播错误 与 ? 运算符
fn read_username_from_file() -> Result<String, io::Error> {
    // 最简洁的方式，使用链式 ?
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)

    // 或者分步：
    // let mut f = File::open("username.txt")?;
    // let mut username = String::new();
    // f.read_to_string(&mut username)?;
    // Ok(username)
}

// ? 与 Option<T>
fn get_first_char(text: &str) -> Option<char> {
    text.lines().next()?.chars().next() // lines().next() -> Option<&str>, chars().next() -> Option<char>
}


// --- 7.3 自定义错误类型 ---
#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(String),
    Timeout,
    ContentMissing(String),
}

// 实现 Display for AppError
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "应用程序 IO 错误: {}", e),
            AppError::Parse(s) => write!(f, "应用程序解析错误: {}", s),
            AppError::Timeout => write!(f, "应用程序操作超时"),
            AppError::ContentMissing(s) => write!(f, "内容缺失: {}", s),
        }
    }
}

// 实现 Error for AppError
impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e), // 底层错误是 io::Error
            _ => None, // 其他变体没有底层错误源
        }
    }
}

// 实现 From<io::Error> for AppError 以便与 ? 配合使用
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> AppError {
        AppError::Io(err)
    }
}

// 示例函数返回自定义错误
fn load_config(path: &str) -> Result<String, AppError> {
    let content = fs::read_to_string(path)?; // ? 会自动将 io::Error 转换为 AppError::Io

    if content.is_empty() {
        return Err(AppError::ContentMissing(path.to_string()));
    }
    if !content.contains("version=") {
        return Err(AppError::Parse("配置文件缺少 'version' 字段".to_string()));
    }
    // 假设还有其他检查...
    Ok(content)
}


// main 函数可以返回 Result，以便在 main 中使用 ?
fn main() -> Result<(), Box<dyn Error>> { // Box<dyn Error> 是一种通用的错误类型
    println!("--- 7.1 panic! ---");
    // cause_panic(); // 取消注释会导致 panic
    println!("如果 cause_panic() 被调用且 panic，这行不会打印。\n");

    println!("--- 7.2 Result<T, E> ---");
    // 7.2.1 使用 match
    println!("尝试打开或创建 'test_file.txt':");
    open_or_create_file("test_file.txt");
    fs::remove_file("test_file.txt").ok(); // 清理，忽略结果
    println!();

    // 7.2.2 unwrap 和 expect (通常在测试或确定不会失败时使用)
    // println!("unwrap 示例:");
    // unwrap_example(); // 取消注释并在特定条件下运行会导致 panic
    // println!("\nexpect 示例:");
    // expect_example(); // 取消注释会导致 panic
    // println!();

    // 7.2.3 & 7.2.4 传播错误与 ?
    // 先创建一个测试文件
    fs::write("username.txt", "Rustacean").expect("无法写入 username.txt");
    match read_username_from_file() {
        Ok(name) => println!("从 'username.txt' 读取的用户名: {}", name),
        Err(e) => println!("读取 'username.txt' 失败: {}", e),
    }
    fs::remove_file("username.txt").ok(); // 清理
    println!();

    // ? 与 Option<T>
    println!("? 与 Option<T> 示例:");
    let text_with_content = "第一行\n第二行";
    let text_empty = "";
    println!("'{}' 的第一个字符: {:?}", text_with_content, get_first_char(text_with_content)); // Some('第')
    println!("'{}' 的第一个字符: {:?}", text_empty, get_first_char(text_empty)); // None
    println!();

    println!("--- 7.3 自定义错误类型 ---");
    // 创建测试配置文件
    fs::write("config_ok.toml", "version=1.0\nname=App").expect("无法写入 config_ok.toml");
    fs::write("config_empty.toml", "").expect("无法写入 config_empty.toml");
    fs::write("config_no_version.toml", "name=AppOnly").expect("无法写入 config_no_version.toml");

    println!("加载 'config_ok.toml':");
    match load_config("config_ok.toml") {
        Ok(config) => println!("  成功加载配置:\n{}", config),
        Err(e) => println!("  加载配置错误: {}", e), // 使用 Display
    }

    println!("\n加载 'config_empty.toml':");
    match load_config("config_empty.toml") {
        Ok(config) => println!("  成功加载配置:\n{}", config),
        Err(e) => {
            println!("  加载配置错误: {}", e);
            if let Some(source) = e.source() {
                println!("    Caused by: {}", source);
            }
        }
    }

    println!("\n加载 'config_no_version.toml':");
    match load_config("config_no_version.toml") {
        Ok(config) => println!("  成功加载配置:\n{}", config),
        Err(e) => {
            println!("  加载配置错误: {}", e);
             if let Some(source) = e.source() {
                println!("    Caused by: {}", source);
            }
        }
    }

    println!("\n加载 'non_existent_config.toml':");
    match load_config("non_existent_config.toml") {
        Ok(config) => println!("  成功加载配置:\n{}", config),
        Err(e) => {
            println!("  加载配置错误: {}", e); // 会是 AppError::Io
            if let Some(source) = e.source() { // source 会是底层的 io::Error
                println!("    Caused by: {}", source);
            }
        }
    }

    // 清理测试文件
    fs::remove_file("config_ok.toml").ok();
    fs::remove_file("config_empty.toml").ok();
    fs::remove_file("config_no_version.toml").ok();

    Ok(()) // main 函数成功返回
}
