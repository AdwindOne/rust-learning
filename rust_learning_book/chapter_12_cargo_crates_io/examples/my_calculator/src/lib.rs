//! # My Calculator Lib
//!
//! 这是一个简单的计算器库，用于演示 Cargo 的各种功能，
//! 包括依赖管理、特性、构建配置和发布到 Crates.io 的准备工作。
//!
//! ## 功能
//!
//! - 基本的加法、减法、乘法、除法。
//! - (可选特性) 使用 `num-bigint` 进行大数运算。

// 引入可选依赖，需要通过特性来启用
// #[cfg(feature = "advanced_math")]
// extern crate num_bigint;
// #[cfg(feature = "advanced_math")]
// use num_bigint::BigInt;

/// 将两个 i32 数字相加。
///
/// # Examples
///
/// ```
/// use my_calculator::add;
/// assert_eq!(add(2, 3), 5);
/// ```
///
/// # Panics
///
/// 这个函数在正常情况下不应该 panic。
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 将两个 i32 数字相减。
///
/// # Examples
///
/// ```
/// use my_calculator::subtract;
/// assert_eq!(subtract(5, 3), 2);
/// ```
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// 将两个 i32 数字相乘。
///
/// # Examples
///
/// ```
/// use my_calculator::multiply;
/// assert_eq!(multiply(4, 3), 12);
/// ```
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// 将两个 i32 数字相除。
///
/// 返回 `Option<i32>`，因为除以零是没有意义的。
/// 如果 `b` 是 0，则返回 `None`。
///
/// # Examples
///
/// ```
/// use my_calculator::divide;
/// assert_eq!(divide(10, 2), Some(5));
/// assert_eq!(divide(10, 0), None);
/// ```
pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

/// 使用 "advanced_math" 特性进行的大数加法。
/// 只有当 "advanced_math" 特性被启用时，这个函数才会被编译。
#[cfg(feature = "advanced_math")]
pub fn add_big_numbers(a_str: &str, b_str: &str) -> Result<String, String> {
    use num_bigint::BigInt; // 在条件编译块内部导入
    use std::str::FromStr;

    let big_a = BigInt::from_str(a_str).map_err(|e| e.to_string())?;
    let big_b = BigInt::from_str(b_str).map_err(|e| e.to_string())?;
    Ok((big_a + big_b).to_string())
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*; // 导入父模块 (my_calculator 库) 的所有公共项

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 2), 3);
        assert_eq!(subtract(2, 5), -3);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(-3, 4), -12);
        assert_eq!(multiply(3, 0), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Some(5));
        assert_eq!(divide(10, 3), Some(3)); // 整数除法
        assert_eq!(divide(10, 0), None);
        assert_eq!(divide(0, 5), Some(0));
    }

    // 测试 "advanced_math" 特性 (仅当特性启用时才编译和运行)
    #[cfg(feature = "advanced_math")]
    #[test]
    fn test_add_big_numbers_with_feature() {
        let num1 = "12345678901234567890";
        let num2 = "98765432109876543210";
        let expected_sum = "111111111011111111100"; // 123... + 987...

        match add_big_numbers(num1, num2) {
            Ok(sum_str) => assert_eq!(sum_str, expected_sum),
            Err(e) => panic!("add_big_numbers 失败: {}", e),
        }

        assert!(add_big_numbers("abc", "123").is_err(), "无效数字输入应该返回错误");
    }

    // 这个测试用于演示：如果特性未启用，依赖该特性的代码不会被编译
    #[cfg(not(feature = "advanced_math"))]
    #[test]
    fn test_big_numbers_feature_not_enabled_placeholder() {
        // 如果 "advanced_math" 未启用，add_big_numbers 函数不存在
        // 我们可以放一个占位测试或什么都不做
        // 或者测试在特性未启用时的行为（如果函数仍然存在但行为不同）
        println!("'advanced_math' 特性未启用，跳过大数加法测试。");
        assert!(true); // 简单通过
    }
}
