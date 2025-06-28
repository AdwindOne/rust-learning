// main 函数是程序的入口
fn main() {
    println!("--- 4.1 所有权 ---");

    // 变量作用域
    {
        let s_inner = "内部作用域的字符串字面量"; // s_inner 在这个作用域内有效
        println!("s_inner: {}", s_inner);
    } // s_inner 的作用域结束
    // println!("{}", s_inner); // 编译错误：s_inner 在此无效

    // String 类型
    let mut s_heap = String::from("hello"); // 在堆上分配 "hello"
    s_heap.push_str(", world!"); // 可追加内容
    println!("s_heap: {}", s_heap);
    // 当 s_heap 离开 main 函数作用域时，其内存会被自动释放 (drop 被调用)

    // 所有权的移动 (Move)
    println!("\n--- 所有权的移动 (Move) ---");
    let s1 = String::from("移动我");
    let s2 = s1; // s1 的所有权移动到 s2
                 // s1 在此之后不再有效，以防止 double free

    // println!("s1 (尝试访问): {}", s1); // 编译错误: value borrowed here after move
    println!("s2 (拥有所有权): {}", s2);

    // 深拷贝 (Clone)
    println!("\n--- 深拷贝 (Clone) ---");
    let s3 = String::from("克隆我");
    let s4 = s3.clone(); // s3 的数据被深拷贝到 s4
    println!("s3: {}, s4: {}", s3, s4); // s3 和 s4 都有效，各自拥有数据

    // Copy Trait (栈上数据)
    println!("\n--- Copy Trait ---");
    let x = 5; // i32 实现了 Copy trait
    let y = x; // x 的值被复制到 y，x 仍然有效
    println!("x: {}, y: {}", x, y);

    // 所有权与函数
    println!("\n--- 所有权与函数 ---");
    let string_for_fn = String::from("传递给函数");
    takes_ownership(string_for_fn); // string_for_fn 的所有权被移动到函数中
    // println!("{}", string_for_fn); // 编译错误：值已被移动

    let int_for_fn = 10; // i32 是 Copy 类型
    makes_copy(int_for_fn);
    println!("int_for_fn 在调用 makes_copy 后仍然有效: {}", int_for_fn);

    // 返回值与作用域
    let returned_s1 = gives_ownership();
    println!("从 gives_ownership 返回: {}", returned_s1);
    let returned_s2 = String::from("原始字符串");
    let returned_s3 = takes_and_gives_back(returned_s2);
    // println!("returned_s2 (已被移动): {}", returned_s2); // 编译错误
    println!("从 takes_and_gives_back 返回: {}", returned_s3);
    println!();


    println!("--- 4.2 引用与借用 ---");
    let main_s1 = String::from("借用我");
    let len = calculate_length(&main_s1); // &main_s1 创建了对 main_s1 的引用
                                          // main_s1 的所有权没有移动
    println!("字符串 '{}' 的长度是 {}.", main_s1, len);

    // 可变引用
    let mut main_s_mut = String::from("可变");
    println!("修改前: {}", main_s_mut);
    change_string(&mut main_s_mut); // 传递可变引用
    println!("修改后: {}", main_s_mut);

    // 可变引用规则演示
    let mut s_ref_rules = String::from("规则");
    let r1_mut = &mut s_ref_rules;
    println!("第一个可变引用 r1_mut: {}", r1_mut);
    // let r2_mut = &mut s_ref_rules; // 编译错误: cannot borrow `s_ref_rules` as mutable more than once at a time
    // println!("{}, {}", r1_mut, r2_mut);

    // NLL (Non-Lexical Lifetimes) 使得 r1_mut 的生命周期在上次使用后结束
    let r2_mut_ok = &mut s_ref_rules; // 这是可以的，因为 r1_mut 不再被使用
    println!("第二个可变引用 r2_mut_ok: {}", r2_mut_ok);

    // 混合引用规则
    let mut s_mix_rules = String::from("混合规则");
    let r_immut1 = &s_mix_rules;
    let r_immut2 = &s_mix_rules;
    println!("不可变引用 r_immut1: {}, r_immut2: {}", r_immut1, r_immut2);
    // let r_mut_mix = &mut s_mix_rules; // 编译错误: cannot borrow `s_mix_rules` as mutable because it is also borrowed as immutable
    // println!("{}, {}, {}", r_immut1, r_immut2, r_mut_mix);

    // r_immut1 和 r_immut2 的生命周期在这里结束 (因为它们不再被使用)
    let r_mut_mix_ok = &mut s_mix_rules; // 这是可以的
    println!("在不可变引用之后的可变引用 r_mut_mix_ok: {}", r_mut_mix_ok);

    // 悬垂引用 (通过函数调用演示)
    // let reference_to_nothing = dangle(); // 这会导致编译错误，如 README 中所述
    let valid_string = no_dangle();
    println!("从 no_dangle 获取的有效字符串: {}", valid_string);
    println!();


    println!("--- 4.3 切片 (Slices) ---");
    let s_for_slice = String::from("你好 世界！"); // 注意UTF-8字符边界

    // 字符串切片
    // "你好" 占 6 字节 (每个汉字3字节), " " 占 1 字节, "世界" 占 6 字节, "！" 占 3 字节
    // "你好" -> 0..6
    // "世界" -> 7..13
    let hello_slice = &s_for_slice[0..6]; // 指向 "你好"
    let world_slice = &s_for_slice[7..13]; // 指向 "世界"
    println!("字符串切片: '{}' 和 '{}'", hello_slice, world_slice);

    // 省略写法
    let full_string = String::from("完整字符串");
    let slice_start = &full_string[..6]; // 从头开始的6个字节 (假设是 "完整字符")
    let slice_end = &full_string[6..];   // 从第6个字节到末尾 (假设是 "串")
    let slice_all = &full_string[..];    // 整个字符串的切片
    println!("省略写法切片: '{}', '{}', '{}'", slice_start, slice_end, slice_all);

    // 字符串字面量本身就是切片
    let literal_is_slice: &str = "这是一个字符串字面量切片";
    println!("{}", literal_is_slice);

    // 函数使用字符串切片
    let my_string_obj = String::from("第一个 单词");
    let first = first_word_slice(&my_string_obj[..]); // 传递整个 String 的切片
    println!("'{}' 的第一个词是: '{}'", my_string_obj, first);

    let my_literal = "字面量 的 第一个词";
    let first_lit = first_word_slice(my_literal); // 直接传递字符串字面量
    println!("'{}' 的第一个词是: '{}'", my_literal, first_lit);

    // 演示切片与可变性的交互
    let mut s_mutable_slice = String::from("foo bar baz");
    let word_ref = first_word_slice(&s_mutable_slice);
    // s_mutable_slice.clear(); // 编译错误! 因为 word_ref (不可变引用) 仍然存活
    println!("从可变字符串获取的第一个词: {}", word_ref); // word_ref 的生命周期在此结束
    s_mutable_slice.clear(); // 现在可以了
    println!("s_mutable_slice 清空后: '{}'", s_mutable_slice);


    // 其他类型的切片
    let array_a = [100, 200, 300, 400, 500];
    let array_slice: &[i32] = &array_a[1..4]; // 指向 [200, 300, 400]
    println!("数组: {:?}", array_a);
    println!("数组切片 (索引1到3): {:?}", array_slice);
    assert_eq!(array_slice, &[200, 300, 400]);
} // main 函数结束，所有在此作用域内拥有的变量（如 s_heap, s2, s3, s4, returned_s1, returned_s3, valid_string, s_for_slice, full_string, my_string_obj, s_mutable_slice, array_a）会被 drop

// 函数：获取 String 的所有权
fn takes_ownership(some_string: String) {
    println!("在 takes_ownership 中: {}", some_string);
} // some_string 在这里移出作用域并被 drop

// 函数：获取 i32 的副本
fn makes_copy(some_integer: i32) {
    println!("在 makes_copy 中: {}", some_integer);
} // some_integer 在这里移出作用域，无事发生

// 函数：返回一个 String (转移所有权)
fn gives_ownership() -> String {
    let s = String::from("来自gives_ownership");
    s // 返回 s，所有权被移出
}

// 函数：获取 String 所有权并返回它
fn takes_and_gives_back(a_string: String) -> String {
    a_string // 返回 a_string，所有权被移出
}

// 函数：计算 String 引用的长度
fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
    s.len()
} // s 在这里移出作用域，但它不拥有数据，所以什么也不会发生

// 函数：修改 String 的可变引用
fn change_string(some_string: &mut String) {
    some_string.push_str(" (已修改)");
}

// 函数：尝试返回悬垂引用 (会导致编译错误，仅用于说明)
// fn dangle() -> &String {
//     let s = String::from("悬垂");
//     &s // 返回 s 的引用
// } // s 在此被 drop，内存被释放，引用会悬垂

// 函数：正确返回 String (转移所有权)
fn no_dangle() -> String {
    let s = String::from("不悬垂");
    s
}

// 函数：接收字符串切片，返回第一个单词的切片
// s 可以是 String 的引用 (&String)，也可以是字符串字面量 (&str)
// 通过 Deref 强制转换，&String 可以变成 &str
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes(); // 将字符串转换为字节数组

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // 查找第一个空格的字节
            return &s[0..i]; // 返回第一个单词的切片
        }
    }
    &s[..] // 如果没有空格，整个字符串就是第一个单词
}
