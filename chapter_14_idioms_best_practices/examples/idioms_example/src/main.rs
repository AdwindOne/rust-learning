use std::fmt::{self, Display};
// use anyhow::{Context, Result as AnyhowResult, bail}; // 用于演示 anyhow

// --- 14.2 高效的错误处理模式 ---

// 自定义错误类型 (可以使用 thiserror crate 简化)
#[derive(Debug)]
enum DataProcessingError {
    Io(std::io::Error),
    ParseError(String),
    Validation(String),
}

// 实现 Display for DataProcessingError
impl Display for DataProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataProcessingError::Io(err) => write!(f, "IO 错误: {}", err),
            DataProcessingError::ParseError(msg) => write!(f, "解析错误: {}", msg),
            DataProcessingError::Validation(msg) => write!(f, "验证错误: {}", msg),
        }
    }
}

// 实现 Error for DataProcessingError
impl std::error::Error for DataProcessingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DataProcessingError::Io(err) => Some(err),
            _ => None,
        }
    }
}

// 实现 From<std::io::Error> for DataProcessingError 以便 `?` 自动转换
impl From<std::io::Error> for DataProcessingError {
    fn from(err: std::io::Error) -> Self {
        DataProcessingError::Io(err)
    }
}

// 使用自定义错误的函数示例
fn process_data_file(path: &str) -> Result<String, DataProcessingError> {
    let content = std::fs::read_to_string(path)?; // `?` 将 std::io::Error 转换为 DataProcessingError::Io

    if content.is_empty() {
        return Err(DataProcessingError::Validation("文件内容为空".to_string()));
    }
    if !content.contains("KEY:") {
        return Err(DataProcessingError::ParseError("缺少 'KEY:' 字段".to_string()));
    }
    // 假设更多处理...
    Ok(format!("处理后的内容: {}", content.to_uppercase()))
}

// 使用 anyhow (通常用于应用程序的 main.rs 或顶层错误处理)
// fn process_with_anyhow(path: &str) -> AnyhowResult<String> {
//     let content = std::fs::read_to_string(path)
//         .with_context(|| format!("使用 anyhow: 无法读取文件 '{}'", path))?;

//     if content.len() < 5 {
//         bail!("使用 anyhow: 文件内容 '{}' 太短 (<5 字符)", path);
//     }
//     Ok(content.trim().to_string())
// }


// --- 14.3 有效利用 Rust 的类型系统 ---

// Newtype Pattern
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)] // 为 Newtype 实现常用 trait
struct CustomerId(u64);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct OrderId(u64);

fn handle_customer_order(customer_id: CustomerId, order_id: OrderId) {
    println!("处理客户 {:?} 的订单 {:?}", customer_id, order_id);
}

// 类型别名
type Age = u8;
type Name = String;
type UserProfile = (Name, Age, CustomerId); // 元组类型别名

fn print_user_profile(profile: &UserProfile) {
    println!("用户档案: 姓名={}, 年龄={}, ID={:?}", profile.0, profile.1, profile.2);
}

// --- 14.4 编写高效的 Rust 代码 ---

// 避免不必要的克隆
fn greet_user(name: &str) { // 接收 &str，而不是 String，避免调用者克隆或移动
    println!("你好, {}!", name);
}

// 迭代器和闭包
fn sum_of_even_squares(numbers: &[i32]) -> i32 {
    numbers
        .iter()            // 迭代 &i32
        .filter(|&&n| n % 2 == 0) // 过滤偶数 (&&n 是因为 filter 接收 &&i32)
        .map(|&n| n * n)   // 平方 ( &n 是 &i32, *n 是 i32)
        .sum()             // 求和 (sum 会消耗迭代器)
}


// --- 14.5 其他最佳实践 ---

// RAII 示例：一个简单的文件包装器，在 drop 时打印信息
struct FileLogger {
    path: String,
}
impl FileLogger {
    fn new(path: &str) -> Self {
        println!("[FileLogger] 正在 '打开' (模拟) 文件: {}", path);
        FileLogger { path: path.to_string() }
    }
}
impl Drop for FileLogger {
    fn drop(&mut self) {
        println!("[FileLogger] 正在 '关闭' (模拟) 文件: {}", self.path);
        // 实际的文件操作会在这里关闭文件句柄
    }
}


fn main() {
    println!("--- 14.2 错误处理模式示例 ---");
    // 假设存在 test_data.txt, test_empty.txt, test_no_key.txt
    // std::fs::write("test_data.txt", "KEY:value").ok();
    // std::fs::write("test_empty.txt", "").ok();
    // std::fs::write("test_no_key.txt", "some content").ok();

    match process_data_file("test_data.txt") {
        Ok(processed) => println!("  成功: {}", processed),
        Err(e) => eprintln!("  错误: {}", e), // eprintln! 打印到 stderr
    }
    match process_data_file("test_empty.txt") {
        Ok(processed) => println!("  成功: {}", processed),
        Err(e) => eprintln!("  错误: {}", e),
    }
    match process_data_file("test_no_key.txt") {
        Ok(processed) => println!("  成功: {}", processed),
        Err(e) => eprintln!("  错误: {}", e),
    }
    match process_data_file("non_existent_file.txt") {
        Ok(processed) => println!("  成功: {}", processed),
        Err(e) => {
            eprintln!("  错误: {}", e);
            if let Some(source) = e.source() {
                eprintln!("    Caused by: {}", source);
            }
        }
    }
    // 清理文件
    // std::fs::remove_file("test_data.txt").ok();
    // std::fs::remove_file("test_empty.txt").ok();
    // std::fs::remove_file("test_no_key.txt").ok();
    println!();

    // Anyhow 示例 (如果启用)
    // match process_with_anyhow("src/main.rs") { // 假设这个文件存在且不短
    //     Ok(s) => println!("Anyhow: Read {} bytes from main.rs", s.len()),
    //     Err(e) => eprintln!("Anyhow error: {:?}", e), // {:?} 打印错误链
    // }
    // match process_with_anyhow("Cargo.toml") { // 假设这个文件存在但可能短
    //     Ok(s) => println!("Anyhow: Read {} bytes from Cargo.toml", s.len()),
    //     Err(e) => eprintln!("Anyhow error: {:?}", e),
    // }


    println!("--- 14.3 类型系统 Idioms ---");
    let cust_id = CustomerId(1001);
    let ord_id = OrderId(5005);
    handle_customer_order(cust_id, ord_id);
    // handle_customer_order(ord_id, cust_id); // 编译错误，类型不匹配

    let user_profile: UserProfile = (String::from("爱丽丝"), 30, CustomerId(1002));
    print_user_profile(&user_profile);

    // Option<T> 的惯用法
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;

    println!("  Option map: Some -> {:?}, None -> {:?}",
             some_value.map(|x| x * 2), // Some(84)
             none_value.map(|x| x * 2)); // None

    println!("  Option and_then: Some -> {:?}, None -> {:?}",
             some_value.and_then(|x| if x > 0 { Some(x.to_string()) } else { None }), // Some("42")
             none_value.and_then(|x: i32| if x > 0 { Some(x.to_string()) } else { None })); // None

    println!("  Option unwrap_or: Some -> {}, None -> {}",
             some_value.unwrap_or(0), // 42
             none_value.unwrap_or(0)); // 0
    println!();


    println!("--- 14.4 高效代码 Idioms ---");
    let user_name_string = String::from("鲍勃");
    greet_user(&user_name_string); // 传递引用，避免移动或克隆
    greet_user("查理");      // 字符串字面量也是 &str

    let numbers_for_sum = vec![1, 2, 3, 4, 5, 6];
    println!("  Sum of even squares for {:?}: {}", numbers_for_sum, sum_of_even_squares(&numbers_for_sum));

    // 预分配容量
    let mut vec_with_cap = Vec::with_capacity(10); // 预分配10个元素的空间
    println!("  Vec with capacity: len={}, capacity={}", vec_with_cap.len(), vec_with_cap.capacity());
    for i in 0..5 { vec_with_cap.push(i); } // 通常不会触发重新分配
    println!("  After pushes: len={}, capacity={}", vec_with_cap.len(), vec_with_cap.capacity());
    println!();


    println!("--- 14.5 其他最佳实践 (RAII) ---");
    {
        let _logger1 = FileLogger::new("important_log.txt");
        let _logger2 = FileLogger::new("audit_trail.log");
        println!("  FileLoggers _logger1 和 _logger2 在作用域内。");
        // 当 _logger1 和 _logger2 离开这个作用域时，它们的 drop 方法会被调用
    }
    println!("  FileLoggers 已离开作用域并被 drop。");

    // 运行 `cargo fmt` 和 `cargo clippy` 的演示不直接在代码中，
    // 而是作为开发流程的一部分。
    println!("\n提醒：请在您的项目上运行 'cargo fmt' 和 'cargo clippy'！");
}
