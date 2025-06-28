// Rust 程序入口点
fn main() {
    // 2.1 变量和可变性
    println!("--- 2.1 变量和可变性 ---");
    let x = 5; // 不可变变量
    println!("x 的值是: {}", x);
    // x = 6; // 取消此行注释会导致编译错误: cannot assign twice to immutable variable

    let mut y = 10; // 可变变量
    println!("y 的初始值是: {}", y);
    y = 20;
    println!("y 的新值是: {}", y);

    // 常量
    const MAX_POINTS: u32 = 100_000;
    println!("最大点数: {}", MAX_POINTS);

    // 遮蔽 (Shadowing)
    let z = 5;
    println!("z 的值是: {}", z);

    let z = z + 1; // z 被遮蔽，新的 z 是 6
    println!("z 被遮蔽后的值是: {}", z);

    {
        let z = z * 2; // 内部作用域的 z 遮蔽了外部的 z，新的 z 是 12
        println!("内部作用域中 z 的值是: {}", z);
    }
    println!("外部作用域中 z 的值恢复为: {}", z); // 输出 6

    let spaces = "   "; // spaces 是字符串类型
    let spaces = spaces.len(); // spaces 被遮蔽，现在是数字类型 (usize)
    println!("spaces 变量被遮蔽后，表示空格的数量: {}", spaces);
    println!(); // 打印一个空行，用于分隔

    // 2.2 数据类型
    println!("--- 2.2 数据类型 ---");
    // 2.2.1 标量类型
    println!("--- 2.2.1 标量类型 ---");
    // 整数
    let an_integer: i32 = 98_222; // i32 类型
    let default_integer = 123;    // 默认为 i32
    let hex_val = 0xff;           // 十六进制: 255
    let octal_val = 0o77;         // 八进制: 63
    let binary_val = 0b1111_0000; // 二进制: 240
    let byte_val = b'A';          // 字节 (u8 类型): 65
    println!(
        "整数: {}, {}, 十六进制: {}, 八进制: {}, 二进制: {}, 字节: {}",
        an_integer, default_integer, hex_val, octal_val, binary_val, byte_val
    );

    // 浮点数
    let float_f64 = 2.0; // f64 (默认)
    let float_f32: f32 = 3.0; // f32
    println!("浮点数: f64={}, f32={}", float_f64, float_f32);

    // 布尔类型
    let is_true = true;
    let is_false: bool = false; // 显式类型注解
    println!("布尔值: {} 和 {}", is_true, is_false);

    // 字符类型
    let char_val = 'z';
    let unicode_char: char = 'ℤ';
    let emoji_char = '😻';
    println!(
        "字符: '{}', Unicode字符: '{}', Emoji字符: '{}'",
        char_val, unicode_char, emoji_char
    );
    println!();

    // 2.2.2 复合类型
    println!("--- 2.2.2 复合类型 ---");
    // 元组 (Tuples)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (val_x, val_y, val_z) = tup; // 解构元组
    println!(
        "元组: ({}, {}, {}), 解构后 y = {}",
        tup.0, tup.1, tup.2, val_y
    );

    let unit_tuple = (); // 单元组
    println!("单元组 (用于表示空): {:?}", unit_tuple); // 调试打印

    // 数组 (Arrays)
    let arr_a = [1, 2, 3, 4, 5]; // 类型推断为 [i32; 5]
    let arr_b: [i32; 3] = [10, 20, 30];
    let arr_c = [3; 5]; // 等同于 [3, 3, 3, 3, 3]
    println!(
        "数组 a 的第一个元素: {}, 数组 b: {:?}, 数组 c: {:?}",
        arr_a[0], arr_b, arr_c
    );

    // 访问数组元素 (注意不要越界)
    // let out_of_bounds = arr_a[10]; // 这行会 panic
    // println!("试图访问越界元素: {}", out_of_bounds);
    // 使用 .get() 方法安全访问
    let safe_access = arr_a.get(2); // 返回 Option<&i32>
    let safe_access_out = arr_a.get(10);
    println!("安全访问 arr_a[2]: {:?}, 安全访问 arr_a[10]: {:?}", safe_access, safe_access_out);

    println!();

    // 2.3 注释 (在代码中已展示)
    // 单行注释
    /*
       块注释
    */
    /// 文档注释 (通常用于库函数)
    println!("--- 2.3 注释 ---");
    println!("请查看源代码中的注释示例。");
    println!();

    // 2.4 函数
    println!("--- 2.4 函数 ---");
    another_function(); // 调用在 main 外定义的函数
    function_with_parameters(100, 'X');
    let sum_result = add_numbers(15, 7);
    println!("15 + 7 = {}", sum_result);
    let five_val = five();
    println!("函数 five 返回: {}", five_val);
    println!();

    // 2.5 表达式与语句
    println!("--- 2.5 表达式与语句 ---");
    // `let` 绑定是语句
    // let s = (let t = 6); // 错误：`let t = 6` 不返回值

    // 代码块是表达式
    let expr_val = {
        let inner_x = 3;
        inner_x + 1 // 这个块的值是 inner_x + 1 的结果，即 4。注意没有分号。
    };
    println!("代码块表达式的值: {}", expr_val);

    let expr_val_unit = {
        let inner_x = 3;
        inner_x + 1; // 如果这里有分号，这个块就变成了语句，其值为 ()
    };
    println!("带分号的代码块表达式的值 (unit type): {:?}", expr_val_unit);
} // main 函数结束

// 另一个函数定义
fn another_function() {
    println!("你好，来自 another_function!");
}

// 带参数的函数
// 必须声明每个参数的类型
fn function_with_parameters(value: i32, unit_label: char) {
    println!("参数化函数接收到的值: {}{}", value, unit_label);
}

// 带返回值的函数
// 使用 `->` 后跟返回类型
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b // 这是一个表达式，它的值将作为函数的返回值
          // 如果写成 `a + b;` (带分号)，则会返回 unit type `()`，导致编译错误
}

// 隐式返回最后一个表达式的值
fn five() -> i32 {
    5 // 返回 5
}

// 这是一个公共函数，用于演示文档注释 (虽然在这个 main.rs 中不直接生成外部文档)
/// 给定一个 i32 数字，返回其加一后的结果。
///
/// # 示例
///
/// ```
/// let arg = 5;
/// // 注意：在 main.rs 内部这样直接调用可能需要模块路径，
/// // 但这里仅为演示文档注释的写法。
/// // let answer = basic_syntax_example::increment(arg);
/// // assert_eq!(6, answer);
/// ```
pub fn increment(x: i32) -> i32 {
    x + 1
}
