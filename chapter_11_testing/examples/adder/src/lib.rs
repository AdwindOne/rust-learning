//! # Adder Crate
//!
//! `adder` 是一个非常简单的库，提供了几个用于演示 Rust 测试功能的函数。

/// 这个结构体用于演示 `should_panic` 测试。
#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    /// 创建一个新的 `Guess` 实例。
    ///
    /// # Panics
    ///
    /// 如果 `value` 小于 1 或大于 100，则会 panic。
    ///
    /// # Examples
    ///
    /// ```
    /// use adder::Guess; // 注意：在文档测试中，需要用 crate 名 `adder`
    /// let g = Guess::new(50);
    /// assert_eq!(g.value(), 50);
    /// ```
    ///
    /// ```should_panic
    /// use adder::Guess;
    /// let g = Guess::new(0); // 这会 panic
    /// ```
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }
        Guess { value }
    }

    /// 返回 `Guess` 的值。
    pub fn value(&self) -> i32 {
        self.value
    }
}

/// 将两个 `usize` 数字相加。
///
/// # Examples
///
/// ```
/// use adder::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// 一个简单的问候函数。
///
/// # Examples
///
/// ```
/// use adder::greeting;
/// let name = "世界";
/// assert!(greeting(name).contains(name));
/// ```
pub fn greeting(name: &str) -> String {
    format!("你好, {}!", name)
    // format!("你好!") // 故意引入一个 bug，让某些测试失败
}

// 私有函数，用于演示单元测试可以测试私有部分
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// 单元测试模块
#[cfg(test)] // 只在 `cargo test` 时编译和运行
mod tests {
    use super::*; // 导入外部模块 (即 adder 库本身) 的所有公共项
                  // 对于私有项如 internal_adder，因为 tests 是 lib.rs 的子模块，所以可以直接访问

    #[test] // 标记这是一个测试函数
    fn it_works_basic_assertion() {
        let result = 2 + 2;
        assert_eq!(result, 4, "2+2 应该等于 4"); // assert_eq! 检查相等性
    }

    #[test]
    fn test_add_function() {
        assert_eq!(add(5, 7), 12); // 测试公共函数 add
    }

    #[test]
    fn test_internal_adder() {
        assert_eq!(internal_adder(-2, 2), 0); // 测试私有函数 internal_adder
    }

    #[test]
    fn test_greeting_contains_name() {
        let name_to_test = "Rustacean";
        let result = greeting(name_to_test);
        assert!(
            result.contains(name_to_test),
            "问候语 '{}' 没有包含名字 '{}'", // 自定义失败消息
            result,
            name_to_test
        );
    }

    #[test]
    #[should_panic] // 这个测试期望发生 panic
    fn guess_new_panics_if_too_large() {
        Guess::new(200); // 这会调用 Guess::new 并触发 panic
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")] // 期望 panic 消息包含特定文本
    fn guess_new_panics_if_too_large_specific_message() {
        Guess::new(101);
    }

    #[test]
    #[should_panic(expected = "greater than or equal to 1")]
    fn guess_new_panics_if_too_small() {
        Guess::new(0);
    }

    #[test]
    fn it_works_with_result() -> Result<(), String> {
        if add(2, 2) == 4 {
            Ok(()) // 测试通过
        } else {
            Err(String::from("2 + 2 不等于 4 (来自 Result 测试)")) // 测试失败
        }
    }

    // #[test]
    // fn this_test_will_fail_intentionally() {
    //     panic!("故意让这个测试失败!");
    // }

    #[test]
    #[ignore] // 这个测试默认会被忽略，因为它可能很耗时
    fn expensive_test_example() {
        // 模拟一个耗时的操作
        // std::thread::sleep(std::time::Duration::from_secs(2));
        assert!(true); // 假设它最终会通过
    }

    // 辅助函数可以在测试模块中使用，但它们本身不是测试
    fn setup_common_test_state() -> i32 {
        println!("(测试模块内部) 正在执行通用设置..."); // 如果测试通过，这个不会显示，除非用 --show-output
        42
    }

    #[test]
    fn test_using_helper() {
        let state = setup_common_test_state();
        assert_eq!(state, 42);
    }
}
