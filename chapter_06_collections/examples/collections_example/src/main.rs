// 需要显式导入 HashMap
use std::collections::HashMap;

fn main() {
    println!("--- 6.1 向量 (Vectors, Vec<T>) ---");

    // 创建向量
    let v_empty: Vec<i32> = Vec::new(); // 需要类型注解
    println!("空的向量 v_empty: {:?}", v_empty);

    let v_macro = vec![10, 20, 30]; // 使用 vec! 宏，类型可推断
    println!("使用 vec! 宏创建的向量 v_macro: {:?}", v_macro);

    // 更新向量 (push)
    let mut v_mutable = Vec::new();
    v_mutable.push(5);
    v_mutable.push(6);
    v_mutable.push(7);
    println!("可变向量 v_mutable push 后: {:?}", v_mutable);

    // 读取向量元素
    let v_read = vec![100, 200, 300, 400, 500];
    let third_val: &i32 = &v_read[2]; // 通过索引访问 (panic on out of bounds)
    println!("v_read 的第三个元素 (索引2): {}", third_val);

    // 使用 get 方法安全访问
    match v_read.get(2) {
        Some(val) => println!("v_read.get(2) 得到: Some({})", val),
        None => println!("v_read.get(2) 得到: None"),
    }
    match v_read.get(10) { // 索引越界
        Some(val) => println!("v_read.get(10) 得到: Some({})", val),
        None => println!("v_read.get(10) 得到: None (索引越界)"),
    }

    // 借用规则与向量 (尝试在持有元素引用时修改向量)
    let mut v_borrow_rule = vec![1, 2, 3, 4];
    let first_el = &v_borrow_rule[0]; // 不可变引用
    // v_borrow_rule.push(5); // 这会导致编译错误，因为 first_el 仍然存活
    println!("v_borrow_rule 的第一个元素: {}", first_el); // first_el 的生命周期到此结束
    v_borrow_rule.push(5); // 现在可以了
    println!("v_borrow_rule 修改后: {:?}", v_borrow_rule);


    // 遍历向量
    let v_iter = vec![11, 22, 33];
    print!("遍历 v_iter (不可变): ");
    for i in &v_iter {
        print!("{} ", i);
    }
    println!();

    let mut v_iter_mut = vec![10, 20, 30];
    print!("遍历 v_iter_mut (可变并修改): ");
    for i in &mut v_iter_mut {
        *i *= 2; // 解引用并修改
        print!("{} ", i);
    }
    println!("\nv_iter_mut 修改后: {:?}", v_iter_mut);

    // 使用枚举存储多种类型
    #[derive(Debug)]
    enum MixedData {
        Num(i32),
        Txt(String),
    }
    let mixed_vec = vec![
        MixedData::Num(42),
        MixedData::Txt(String::from("混合数据")),
        MixedData::Num(-5),
    ];
    println!("混合数据向量 mixed_vec: {:?}", mixed_vec);
    for item in &mixed_vec {
        match item {
            MixedData::Num(n) => println!("  枚举项 Num: {}", n),
            MixedData::Txt(s) => println!("  枚举项 Txt: '{}'", s),
        }
    }
    println!();


    println!("--- 6.2 字符串 (Strings) ---");
    // 创建字符串
    let s_new = String::new();
    println!("String::new() 创建的空字符串 s_new: '{}'", s_new);
    let s_from_literal = "字面量".to_string();
    println!("从字面量.to_string() 创建的 s_from_literal: '{}'", s_from_literal);
    let s_from_fn = String::from("String::from");
    println!("从 String::from() 创建的 s_from_fn: '{}'", s_from_fn);

    // 更新字符串
    let mut s_update = String::from("初始");
    s_update.push_str(" 追加文本"); // 追加 &str
    println!("s_update.push_str 后: '{}'", s_update);
    s_update.push('!'); // 追加 char
    println!("s_update.push 后: '{}'", s_update);

    // 使用 + 运算符连接字符串
    let s_concat1 = String::from("你好，");
    let s_concat2 = String::from("世界");
    let s_concat_result = s_concat1 + &s_concat2; // s_concat1 的所有权被移动
    // println!("s_concat1: {}", s_concat1); // 编译错误
    println!("使用 + 连接后的 s_concat_result: '{}'", s_concat_result);

    // 使用 format! 宏连接字符串 (不获取所有权)
    let fmt_s1 = String::from("tic");
    let fmt_s2 = String::from("tac");
    let fmt_s3 = String::from("toe");
    let fmt_result = format!("{}-{}-{}", fmt_s1, fmt_s2, fmt_s3);
    println!("使用 format! 连接后的 fmt_result: '{}'", fmt_result);
    println!("fmt_s1, fmt_s2, fmt_s3 仍然有效: '{}', '{}', '{}'", fmt_s1, fmt_s2, fmt_s3);

    // 字符串索引 (Rust 不允许直接字节索引访问字符)
    // let s_index = String::from("test");
    // let c = s_index[0]; // 编译错误

    // 遍历字符串
    let utf8_string = "नमस्ते"; // 印地语 "Namaste"
    println!("UTF-8 字符串: {}", utf8_string);
    print!("  字节 (bytes): ");
    for b in utf8_string.bytes() {
        print!("{} ", b);
    }
    println!();
    print!("  字符 (chars): ");
    for c in utf8_string.chars() {
        print!("'{}' ", c);
    }
    println!();

    // 字符串切片 (必须在 UTF-8 字符边界)
    let slice_source = String::from("你好 Rust");
    // "你好" 占 6 字节, " " 占 1 字节, "Rust" 占 4 字节
    let slice_ok1 = &slice_source[0..6]; // "你好"
    let slice_ok2 = &slice_source[7..11]; // "Rust"
    println!("字符串切片 slice_ok1: '{}', slice_ok2: '{}'", slice_ok1, slice_ok2);
    // let slice_bad = &slice_source[0..1]; // 这会 panic
    // println!("错误的切片: {}", slice_bad);
    println!();


    println!("--- 6.3 哈希映射 (Hash Maps, HashMap<K, V>) ---");
    // 创建哈希映射
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("哈希映射 scores: {:?}", scores);

    // 从元组向量创建
    let team_data = vec![
        (String::from("Red Team"), 75),
        (String::from("Green Team"), 90),
    ];
    let teams_map: HashMap<String, i32> = team_data.into_iter().collect();
    println!("从元组向量创建的 teams_map: {:?}", teams_map);

    // 所有权 (String 键和值被移动)
    let map_key = String::from("颜色");
    let map_val = String::from("蓝色");
    let mut owned_map = HashMap::new();
    owned_map.insert(map_key, map_val);
    // println!("map_key: {}", map_key); // 编译错误
    println!("owned_map 包含移动后的键值: {:?}", owned_map);

    // 访问哈希映射中的值 (get)
    let blue_score = scores.get("Blue"); // &str 可以用于查询 String 键
    match blue_score {
        Some(score) => println!("Blue 队的得分: {}", score),
        None => println!("Blue 队未找到"),
    }
    if let Some(score_val) = scores.get(&String::from("Yellow")) { // 也可以用 &String
        println!("Yellow 队的得分 (if let): {}", score_val);
    }

    // 遍历哈希映射
    println!("遍历 scores 哈希映射:");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }

    // 更新哈希映射
    // 1. 覆盖旧值
    scores.insert(String::from("Blue"), 25); // Blue 的值从 10 更新为 25
    println!("scores 更新 Blue 的值后: {:?}", scores);

    // 2. entry().or_insert() - 仅当键不存在时插入
    scores.entry(String::from("Blue")).or_insert(100); // Blue 已存在，值不变 (25)
    scores.entry(String::from("Green")).or_insert(60); // Green 不存在，插入 60
    println!("scores 使用 or_insert 后: {:?}", scores);

    // 3. 基于旧值更新 (统计单词频率)
    let text_for_counts = "hello world beautiful world";
    let mut word_counts = HashMap::new();
    for word in text_for_counts.split_whitespace() {
        let count = word_counts.entry(word.to_string()).or_insert(0);
        *count += 1; // count 是 &mut i32
    }
    println!("文本 '{}' 中的单词频率: {:?}", text_for_counts, word_counts);
}
