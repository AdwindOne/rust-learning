// --- 13.1 宏 (Macros) ---

// 13.1.1 声明宏 (Declarative Macros with macro_rules!)
// 定义一个名为 `my_vec` 的简单声明宏，行为类似 `vec!`
// 这个宏定义通常会放在模块的顶层，或者如果要在其他模块/crate中使用，会用 #[macro_export]
#[macro_export]
macro_rules! my_vec {
    // 规则1: 匹配空调用 my_vec![] -> 创建一个空的 Vec
    () => {
        std::vec::Vec::new()
    };
    // 规则2: 匹配 my_vec![elem1, elem2, ..., elemN] (至少一个元素)
    // $($element:expr),+ 表示匹配一个或多个由逗号分隔的表达式 ($element)
    // $(,)? 允许末尾有可选的逗号
    ( $($element:expr),+ $(,)? ) => {
        { // 使用块来创建一个新的作用域
            let mut temp_vec = std::vec::Vec::new();
            // $( ... )* 或 $( ... )+ 用于重复匹配块内的内容
            $( // 对每个匹配到的 $element 执行以下代码
                temp_vec.push($element);
            )*
            temp_vec
        }
    };
    // 规则3: 匹配 my_vec![initial_value; count] -> 创建包含 count 个 initial_value 的 Vec
    // $value:expr 表示匹配一个表达式作为初始值
    // $count:expr 表示匹配一个表达式作为数量
    ($value:expr; $count:expr) => {
        {
            let count = $count;
            let mut temp_vec = std::vec::Vec::with_capacity(count);
            // 如果 $value 不是 Copy 类型，直接赋值会导致所有权问题，需要 clone
            // for _ in 0..count {
            //     temp_vec.push($value); // 如果 $value 是 String，这里会移动
            // }
            // 使用 clone 来处理非 Copy 类型的值
            let val_to_repeat = $value; // 先求值一次
            for _ in 0..count {
                temp_vec.push(val_to_repeat.clone()); // 假设 $value 实现了 Clone
            }
            // 或者使用 std::iter::repeat 和 take (更惯用)
            // temp_vec.extend(std::iter::repeat($value.clone()).take(count));
            temp_vec
        }
    };
}

fn declarative_macros_example() {
    println!("--- 13.1.1 声明宏 (macro_rules!) 示例 ---");
    let v1: Vec<i32> = my_vec![];
    let v2 = my_vec![10, 20, 30];
    let v3 = my_vec!["a", "b", "c",]; // 带末尾逗号
    let v4 = my_vec![0i32; 5]; // 需要 i32 实现 Clone (基本类型默认实现)
    let v5 = my_vec![String::from("hi"); 2]; // String 实现了 Clone

    println!("my_vec![]: {:?}", v1);
    println!("my_vec![10, 20, 30]: {:?}", v2);
    println!("my_vec![\"a\", \"b\", \"c\",]: {:?}", v3);
    println!("my_vec![0i32; 5]: {:?}", v4);
    println!("my_vec![String::from(\"hi\"); 2]: {:?}", v5);
}

// 13.1.2 过程宏 (Procedural Macros)
// 过程宏的定义通常在单独的 `proc-macro = true` 类型的 crate 中。
// 这里我们只演示如何 *使用* 一个假设存在的自定义 derive 宏。
// 假设我们有一个名为 `MyCustomDerive` 的 trait 和一个 `#[derive(MyCustomDerive)]` 宏。

// pub trait MyCustomDerive {
//     fn derived_fn();
// }
// // (假设 my_custom_derive_proc_macro crate 提供了 #[derive(MyCustomDerive)])
// // use my_custom_derive_proc_macro::MyCustomDerive;
//
// // #[derive(MyCustomDerive)] // <-- 使用自定义 derive 宏
// struct ExampleStruct;

fn procedural_macros_example() {
    println!("\n--- 13.1.2 过程宏示例 (概念性) ---");
    println!("过程宏（如自定义 derive、属性宏、函数式宏）的定义较为复杂，");
    println!("通常在独立的 'proc-macro'类型的 crate 中使用 'syn' 和 'quote' 库编写。");
    println!("这里仅作概念说明。标准库的 `#[derive(Debug)]` 就是一个过程宏的例子。");
    // ExampleStruct::derived_fn(); // 如果宏和 trait 实际存在并被实现
}


// --- 13.2 Unsafe Rust ---
static mut UNSAFE_STATIC_COUNTER: u32 = 0;

unsafe fn unsafe_function() {
    println!("这是一个不安全的函数内部！");
    // 执行一些不安全的操作...
}

fn unsafe_rust_example() {
    println!("\n--- 13.2 Unsafe Rust 示例 ---");
    let mut num = 10;

    // 创建裸指针 (本身是安全的)
    let r1_const_ptr: *const i32 = &num as *const i32;
    let r2_mut_ptr: *mut i32 = &mut num as *mut i32; // num 必须是 mut

    // 解引用裸指针 (不安全操作)
    unsafe {
        println!("通过 r1_const_ptr 读取值: {}", *r1_const_ptr);
        *r2_mut_ptr = 20; // 通过 r2_mut_ptr 修改值
        println!("通过 r2_mut_ptr 修改后，num = {}", num);
        // 注意：必须确保裸指针是有效的 (非空、未悬垂、正确对齐)
    }

    // 调用不安全的函数
    unsafe {
        unsafe_function();
    }

    // 访问和修改可变的静态变量
    fn increment_unsafe_static_counter() {
        unsafe { // 修改静态可变变量是不安全的，可能导致数据竞争
            UNSAFE_STATIC_COUNTER += 1;
        }
    }
    increment_unsafe_static_counter();
    increment_unsafe_static_counter();
    unsafe {
        println!("UNSAFE_STATIC_COUNTER 的值: {}", UNSAFE_STATIC_COUNTER); // 读取也是不安全的
    }
}


// --- 13.3 外部函数接口 (FFI) ---
// 声明在 my_c_lib.c 中定义的外部 C 函数
// build.rs 会负责编译 my_c_lib.c 并链接它
#[link(name = "my_c_lib", kind="static")] // 告诉 rustc 链接 libmy_c_lib.a
extern "C" {
    fn multiply_by_two_from_c(x: std::os::raw::c_int) -> std::os::raw::c_int;
    fn print_string_from_c(s: *const std::os::raw::c_char);
    // fn call_rust_callback_from_c(value: c_int, callback: extern "C" fn(c_int) -> c_int) -> c_int;
}

// 一个 Rust 函数，将被 C 调用 (如果设置)
// #[no_mangle]
// pub extern "C" fn rust_callback_for_c(val: std::os::raw::c_int) -> std::os::raw::c_int {
//     println!("[Rust callback] 接收到来自 C 的值: {}", val);
//     val * 10
// }

fn ffi_example() {
    println!("\n--- 13.3 FFI (与 C 交互) 示例 ---");

    // 调用 C 函数 multiply_by_two_from_c
    let input_val: std::os::raw::c_int = 7;
    let result_from_c: std::os::raw::c_int;
    unsafe { // 调用外部 C 函数是不安全的
        result_from_c = multiply_by_two_from_c(input_val);
    }
    println!("C 函数 multiply_by_two_from_c({}) 返回: {}", input_val, result_from_c);

    // 调用 C 函数 print_string_from_c
    // 需要将 Rust String 转换为 C 兼容的 null 结尾字符串 (*const c_char)
    let rust_string = String::from("你好，来自 Rust 的 FFI！");
    // CString 会在堆上分配并确保 null 结尾，需要处理其生命周期
    match std::ffi::CString::new(rust_string.as_str()) {
        Ok(c_string) => {
            unsafe {
                print_string_from_c(c_string.as_ptr()); // as_ptr() 返回 *const c_char
            }
        }
        Err(e) => {
            eprintln!("创建 CString 失败: {:?}", e); // 如果 Rust String 包含内部 null 字节
        }
    }

    // 调用 C 函数并传递 Rust 回调 (如果 C 函数和 Rust 回调已定义)
    // let c_callback_input: c_int = 5;
    // let c_callback_result: c_int;
    // unsafe {
    //     c_callback_result = call_rust_callback_from_c(c_callback_input, rust_callback_for_c);
    // }
    // println!("C 函数调用 Rust 回调结果: {}", c_callback_result);
}


// --- 13.4 闭包 (Closures) 和迭代器 (Iterators) 深入 ---
fn closures_and_iterators_deep_dive() {
    println!("\n--- 13.4 闭包和迭代器深入示例 ---");

    // 闭包捕获方式示例
    println!("闭包捕获方式:");
    let data_shared = vec![1, 2, 3];
    let closure_fn = || println!("  Fn: {:?}", data_shared); // 不可变借用
    closure_fn();
    closure_fn();
    println!("  data_shared 仍然有效: {:?}\n", data_shared);

    let mut count_mut = 0;
    let mut closure_fn_mut = || { // 可变借用
        count_mut += 1;
        println!("  FnMut: count_mut = {}", count_mut);
    };
    closure_fn_mut();
    closure_fn_mut();
    // drop(closure_fn_mut); // 如果不 drop，下一行访问 count_mut 会因借用冲突而编译失败
    println!("  count_mut 的最终值: {}\n", count_mut);


    let data_owned = String::from("将被移动的字符串");
    let closure_fn_once = move || { // `move` 强制获取所有权
        println!("  FnOnce (move): 消耗了 '{}'", data_owned);
        // data_owned 在此作用域结束时被 drop (如果它是 String 等类型)
    };
    closure_fn_once();
    // closure_fn_once(); // 编译错误：FnOnce 闭包（如果消耗了捕获值）不能调用多次
    // println!("{}", data_owned); // 编译错误：data_owned 已被移动

    // 迭代器适配器示例
    println!("\n迭代器适配器:");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map, filter, collect
    let processed_numbers: Vec<_> = numbers
        .iter()                     // 创建迭代器，元素是 &i32
        .map(|&x| x * x)            // 对每个元素平方，元素是 i32
        .filter(|&sq| sq % 2 == 0)  // 只保留偶数平方值
        .collect();                 // 收集到新的 Vec<i32>
    println!("  平方后的偶数: {:?}", processed_numbers); // [4, 16, 36, 64, 100]

    // fold
    let sum_of_squares: i32 = processed_numbers.iter().fold(0, |acc, &val| acc + val);
    println!("  平方后的偶数之和 (fold): {}", sum_of_squares); // 4+16+36+64+100 = 220

    // enumerate, skip, take
    let some_elements: Vec<_> = numbers
        .iter()
        .enumerate() // 产生 (index, &value)
        .skip(3)     // 跳过前3个元素: (3,&4), (4,&5), ...
        .take(4)     // 取接下来4个元素: (3,&4), (4,&5), (5,&6), (6,&7)
        .map(|(idx, &val)| format!("idx {}: val {}", idx, val)) // 格式化
        .collect();
    println!("  枚举、跳过、获取并格式化的元素: {:?}", some_elements);

    // flat_map
    let sentences = vec![
        "hello world",
        "rust is fun",
    ];
    let words: Vec<&str> = sentences
        .iter()
        .flat_map(|&sentence| sentence.split_whitespace()) // 每个句子变成单词的迭代器，然后扁平化
        .collect();
    println!("  扁平化后的单词 (flat_map): {:?}", words); // ["hello", "world", "rust", "is", "fun"]
}


fn main() {
    declarative_macros_example();
    procedural_macros_example(); // 概念性
    unsafe_rust_example();
    ffi_example();
    closures_and_iterators_deep_dive();
}
