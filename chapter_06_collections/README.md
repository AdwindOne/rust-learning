# 第 6 章：常用集合类型

Rust 标准库提供了许多有用的数据结构，称为**集合 (collections)**。大多数其他数据结构都基于这些集合构建。与内置的数组和元组类型不同，这些集合指向的数据存储在**堆 (heap)** 上，这意味着数据的大小在编译时不需要确定，并且可以随着程序的运行而动态改变大小。

本章我们将重点介绍三种最常用的集合类型：
1.  **向量 (Vector, `Vec<T>`)**：允许你存储一系列相同类型的值。
2.  **字符串 (String)**：是字符的集合。我们之前已经接触过 `String`，但这里会更深入地探讨。
3.  **哈希映射 (Hash Map, `HashMap<K, V>`)**：允许你将值与特定的键 (key) 关联起来。

## 6.1 向量 (Vectors, `Vec<T>`)

向量 `Vec<T>` (通常读作 "vector") 允许你将多个**相同类型**的值存储在一个连续的内存区域中。向量是动态大小的，意味着你可以随时添加或删除元素。

### 6.1.1 创建向量

*   **`Vec::new()`**：创建一个空的向量。
    ```rust
    fn main() {
        let v_empty: Vec<i32> = Vec::new(); // 需要类型注解，因为是空的
        println!("Empty vector: {:?}", v_empty);
    }
    ```
*   **`vec!` 宏**：创建一个包含初始值的向量，更方便。编译器可以从初始值推断类型。
    ```rust
    fn main() {
        let v_macro = vec![1, 2, 3]; // 类型推断为 Vec<i32>
        println!("Vector from macro: {:?}", v_macro);
    }
    ```

### 6.1.2 更新向量

*   **`push` 方法**：向向量末尾添加元素。向量必须是可变的 (`mut`)。
    ```rust
    fn main() {
        let mut v = Vec::new(); // 创建一个可变的空向量
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
        println!("Vector after push: {:?}", v); // 输出 [5, 6, 7, 8]
    }
    ```
像任何其他结构体一样，当向量离开作用域时，它将被 `drop`，其所有元素也会被 `drop`，占用的内存会被释放。

### 6.1.3 读取向量元素

有两种主要方式读取向量中的元素：
1.  **通过索引 (`[]`)**：
    *   简单直接。
    *   如果索引越界，程序会 **panic**。
    ```rust
    fn main() {
        let v = vec![10, 20, 30, 40, 50];
        let third: &i32 = &v[2]; // 获取第三个元素 (索引从0开始) 的不可变引用
        println!("The third element is {}", third); // 输出 30

        // let does_not_exist = &v[100]; // 这行会 panic! index out of bounds
        // println!("Trying to access non-existent element: {}", does_not_exist);
    }
    ```
2.  **通过 `get` 方法**：
    *   `get` 方法接收一个索引作为参数，返回一个 `Option<&T>`。
    *   如果索引越界，它会返回 `None`，而不是 panic。这允许你优雅地处理越界情况。
    ```rust
    fn main() {
        let v = vec![10, 20, 30, 40, 50];
        match v.get(2) {
            Some(third) => println!("The third element (via get) is {}", third), // 输出 30
            None => println!("There is no third element."),
        }

        match v.get(100) {
            Some(non_existent) => println!("Element at index 100: {}", non_existent),
            None => println!("Element at index 100 does not exist (via get)."), // 会执行这行
        }
    }
    ```
选择哪种方式取决于你希望程序在尝试访问越界元素时的行为：立即崩溃还是优雅处理。

**借用规则与向量**

记住 Rust 的借用规则：你不能在同一作用域内同时拥有对同一数据的可变引用和不可变引用。
这在向量中尤其重要，因为向向量添加元素（一个可变操作）可能会导致其在内存中重新分配（如果当前容量不足）。如果此时存在对向量内部元素的旧引用，该引用可能会失效（悬垂引用）。

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // 获取第一个元素的不可变引用

    // v.push(6); // 编译错误！
                 // cannot borrow `v` as mutable because it is also borrowed as immutable
                 // 因为 first 是对 v 内部数据的引用，而 push 可能导致 v 重新分配内存，
                 // 使得 first 指向的内存失效。

    println!("The first element is: {}", first); // first 的生命周期到这里结束
    v.push(6); // 现在可以了，因为 first 不再被使用
    println!("Vector after push and first element usage: {:?}", v);
}
```

### 6.1.4 遍历向量中的值

*   **使用 `for` 循环和不可变引用**：
    ```rust
    fn main() {
        let v = vec![100, 32, 57];
        print!("Iterating (immutable): ");
        for i in &v { // i 是 &i32 类型
            print!("{} ", i);
        }
        println!();
    }
    ```
*   **使用 `for` 循环和可变引用** (修改元素)：
    ```rust
    fn main() {
        let mut v = vec![100, 32, 57];
        print!("Iterating (mutable): ");
        for i in &mut v { // i 是 &mut i32 类型
            *i += 50;    // 使用解引用运算符 `*` 来获取 i 指向的值
            print!("{} ", i);
        }
        println!("\nVector after mutable iteration: {:?}", v);
    }
    ```

### 6.1.5 使用枚举存储多种类型的值

向量只能存储相同类型的元素。但有时我们需要存储不同类型的数据列表。一种常见的解决方案是使用枚举。枚举的成员可以持有不同类型的数据，而枚举本身是一个单一的类型。

```rust
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Spreadsheet row: {:?}", row);

    // 可以使用 match 来处理不同类型的单元格
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(value) => println!("  Found Integer: {}", value),
            SpreadsheetCell::Float(value) => println!("  Found Float: {}", value),
            SpreadsheetCell::Text(value) => println!("  Found Text: '{}'", value),
        }
    }
}
```
如果编译时不知道所有可能的类型，可以使用 trait 对象（一种更高级的 Rust 特性）。

## 6.2 字符串 (Strings)

Rust 的核心语言中只有一种字符串类型：字符串切片 `str`（通常以借用形式 `&str` 出现）。字符串字面量就是 `&str` 类型，它们是不可变的。

`String` 类型由 Rust 标准库提供，而不是编码进核心语言，它是一个可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。当我们说 Rust 中的“字符串”时，通常指的是 `String` 类型和 `&str` 字符串切片类型。

### 6.2.1 创建字符串

*   **`String::new()`**：创建一个空的 `String`。
*   **`to_string()` 方法**：在实现了 `Display` trait 的类型上调用，包括字符串字面量。
*   **`String::from()` 函数**：从字符串字面量创建 `String`。

```rust
fn main() {
    let s_empty = String::new();
    println!("Empty string: '{}'", s_empty);

    let data = "initial contents";
    let s_from_data = data.to_string();
    println!("String from to_string(): '{}'", s_from_data);

    let s_from_literal = "another initial contents".to_string();
    println!("String from literal.to_string(): '{}'", s_from_literal);

    let s_from_fn = String::from("yet another way");
    println!("String from String::from(): '{}'", s_from_fn);

    // 字符串是 UTF-8 编码的
    let hello_utf8 = String::from("你好，世界！");
    println!("UTF-8 String: {}", hello_utf8);
}
```

### 6.2.2 更新字符串

*   **`push_str(&str)` 方法**：将一个字符串切片附加到 `String` 末尾。
*   **`push(char)` 方法**：将单个字符附加到 `String` 末尾。
*   **`+` 运算符 (使用 `add` trait)**：连接两个字符串。第一个必须是 `String`，第二个是 `&str`。`+` 运算符会获取第一个 `String` 的所有权。
*   **`format!` 宏**：连接多个字符串，类似于 `println!`，但不打印，而是返回一个包含结果的 `String`。它不获取任何参数的所有权。

```rust
fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // s1 现在是 "foobar"
    println!("s1 after push_str: {}", s1);
    println!("s2 is still: {}", s2); // s2 (一个 &str) 仍然有效

    let mut s_char = String::from("lo");
    s_char.push('l'); // s_char 现在是 "lol"
    println!("s_char after push: {}", s_char);

    let str1 = String::from("Hello, ");
    let str2 = String::from("world!");
    // let str3 = str1 + &str2; // str1 的所有权被移动，str1 不再有效
                              // add 方法签名: fn add(self, s: &str) -> String
                              // &str2 是通过 deref coercion 从 &String 转换来的
    // println!("str1 after +: {}", str1); // 编译错误！str1 已被移动
    // println!("str3 from +: {}", str3);

    // 为了继续使用 str1，可以这样做：
    let str1_clone = String::from("Hello, ");
    let str2_ref = String::from("world!");
    let str3_alt = str1_clone + &str2_ref; // str1_clone 被移动
    println!("str3_alt from +: {}", str3_alt);
    // println!("str1_clone after +: {}", str1_clone); // 编译错误

    // format! 宏
    let ticket1 = String::from("tic");
    let ticket2 = String::from("tac");
    let ticket3 = String::from("toe");
    let tickets = format!("{}-{}-{}", ticket1, ticket2, ticket3);
    println!("Formatted tickets: {}", tickets);
    println!("ticket1, ticket2, ticket3 are still valid: {}, {}, {}", ticket1, ticket2, ticket3);
}
```

### 6.2.3 索引字符串

Rust 的 `String` 不支持像其他一些语言那样通过整数索引直接访问字符 (`my_string[i]`)。这是因为 Rust 的字符串是 UTF-8 编码的，一个“字符”可能占用多个字节。

*   一个 `String` 是 `Vec<u8>` 的包装。
*   直接索引 `String` 可能会返回单个字节，而这个字节可能不是一个有效的 UTF-8 字符的开始。

```rust
fn main() {
    let s = String::from("hello");
    // let h = s[0]; // 编译错误! `String` cannot be indexed by `{integer}`

    let len_bytes = s.len(); // 返回字节长度，对于 "hello" 是 5
    println!("Byte length of 'hello': {}", len_bytes);

    let नमस्ते = String::from("नमस्ते"); // 印地语 "Namaste"
    let len_नमस्ते_bytes = नमस्ते.len(); // 返回字节长度，不是字符数量
                                     // "न" = 3 bytes, "म" = 3 bytes, "स" = 3 bytes, "्" = 3 bytes, "त" = 3 bytes, "े" = 3 bytes
                                     // 总共 18 字节 (不是 6 个字符)
    println!("Byte length of 'नमस्ते': {}", len_नमस्ते_bytes);
    // let first_char_नमस्ते = नमस्ते[0]; // 同样编译错误
}
```

### 6.2.4 遍历字符串

由于 UTF-8 的复杂性，Rust 提供了多种方式来解释字符串数据：
*   **`.bytes()` 方法**：返回每个原始字节 (类型 `u8`) 的迭代器。
*   **`.chars()` 方法**：返回每个 Unicode 标量值 (类型 `char`) 的迭代器。一个 `char` 可能由多个字节组成。
*   **Grapheme clusters (字形簇)**：这是最接近人们通常认为的“字母”或“字符”的概念，但标准库没有直接提供这个功能。需要使用外部 crate (如 `unicode-segmentation`)。

```rust
fn main() {
    let hello = "Здравствуйте"; // 俄语 "Hello"

    print!("Bytes in '{}': ", hello);
    for b in hello.bytes() {
        print!("{} ", b); // 打印每个字节的十进制值
    }
    println!();

    print!("Chars in '{}': ", hello);
    for c in hello.chars() {
        print!("'{}' ", c); // 打印每个 char
    }
    println!();

    // 对于 "नमस्ते"
    let namaste = "नमस्ते";
    print!("Bytes in '{}': ", namaste);
    for b in namaste.bytes() {
        print!("{} ", b);
    }
    println!(); // 18 字节

    print!("Chars in '{}': ", namaste);
    for c in namaste.chars() {
        print!("'{}' ", c); // 6 个 char: 'न', 'म', 'स', '्', 'त', 'े'
    }
    println!();
}
```

**字符串切片 (`&str`)**

可以获取字符串的切片，但必须小心 UTF-8 字符边界。如果切片索引不在有效的字符边界上，程序会 panic。

```rust
fn main() {
    let hello = "Здравствуйте";
    // let s_slice = &hello[0..1]; // Panic! 1 不是字符边界
    // println!("{}", s_slice);

    let s_slice_ok = &hello[0..4]; // "Зд" (第一个字符 "З" 占2字节，第二个 "д" 占2字节)
    println!("Valid slice [0..4] of '{}': '{}'", hello, s_slice_ok);
}
```
通常，如果你需要对字符串进行索引操作，最好是先转换为 `chars` 迭代器，或者使用更安全的字符串处理库。

## 6.3 哈希映射 (Hash Maps, `HashMap<K, V>`)

`HashMap<K, V>` 类型存储键 (key) 类型 `K` 和值 (value) 类型 `V` 之间的映射关系。它通过**哈希函数 (hashing function)** 来确定如何在内存中存储和查找键和值。

### 6.3.1 创建哈希映射

*   **`HashMap::new()`**：创建一个空的哈希映射。
*   **从元组的向量创建 (使用 `collect` 方法)**：
    ```rust
    use std::collections::HashMap; // 需要引入

    fn main() {
        let mut scores = HashMap::new(); // K 和 V 类型会被后续插入推断

        scores.insert(String::from("Blue"), 10); // K=String, V=i32
        scores.insert(String::from("Yellow"), 50);
        println!("Scores HashMap: {:?}", scores);

        // 从元组向量创建
        let teams_list = vec![
            (String::from("Blue Team"), 100),
            (String::from("Red Team"), 75),
        ];
        let teams_map: HashMap<_, _> = teams_list.into_iter().collect(); // collect 可以转换成多种集合类型
        println!("Teams HashMap from vector: {:?}", teams_map);
    }
    ```

### 6.3.2 哈希映射与所有权

对于实现了 `Copy` trait 的类型（如 `i32`），值会被复制到哈希映射中。
对于拥有所有权的值（如 `String`），值的所有权会被移动到哈希映射中。

```rust
use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name 和 field_value 在这里不再有效，它们的所有权被移动到了 map 中
    // println!("field_name: {}", field_name); // 编译错误!
    // println!("field_value: {}", field_value); // 编译错误!
    println!("Map after inserting owned values: {:?}", map);
}
```
如果将值的引用插入到哈希映射中，值本身不会被移动。但你需要确保被引用的值在哈希映射有效期间一直有效（这通常涉及到生命周期，后续章节会详细介绍）。

### 6.3.3 访问哈希映射中的值

*   **`get(&K)` 方法**：接收键的引用作为参数，返回一个 `Option<&V>`。
    ```rust
    use std::collections::HashMap;

    fn main() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name); // get 需要键的引用

        match score {
            Some(s) => println!("Score for Blue team: {}", s), // 输出 10
            None => println!("Blue team score not found."),
        }

        // 可以直接使用字符串字面量作为 get 的参数，因为 &String 可以被 deref 成 &str
        let yellow_score = scores.get("Yellow");
        println!("Score for Yellow team (Option): {:?}", yellow_score); // 输出 Some(50)
    }
    ```

### 6.3.4 遍历哈希映射

可以使用 `for` 循环遍历哈希映射中的键值对，顺序是任意的。

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Red"), 30);

    println!("Iterating through scores HashMap:");
    for (key, value) in &scores { // key 是 &String, value 是 &i32
        println!("  {}: {}", key, value);
    }
}
```

### 6.3.5 更新哈希映射

哈希映射的大小是可变的。当插入一个已存在的键时，有几种处理方式：

1.  **覆盖旧值**：默认行为。如果使用 `insert` 方法插入一个已存在的键，旧的值会被替换。
    ```rust
    use std::collections::HashMap;
    fn main() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25); // Blue 的值从 10 更新为 25
        println!("Scores after overwriting: {:?}", scores); // 输出 {"Blue": 25}
    }
    ```

2.  **仅当键不存在时插入值 (`entry` API 和 `or_insert`)**：
    `entry` 方法接收一个键作为参数，返回一个 `Entry` 枚举。`Entry` 的 `or_insert` 方法允许我们检查键对应的值是否存在：如果存在，则返回该值的可变引用；如果不存在，则将参数作为新值插入，并返回该值的可变引用。
    ```rust
    use std::collections::HashMap;
    fn main() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        // 尝试为 Blue 插入值，如果不存在则插入 50
        scores.entry(String::from("Blue")).or_insert(50); // Blue 已存在 (10)，不会修改
        // 尝试为 Yellow 插入值，如果不存在则插入 50
        scores.entry(String::from("Yellow")).or_insert(50); // Yellow 不存在，插入 50

        println!("Scores after or_insert: {:?}", scores); // 输出 {"Blue": 10, "Yellow": 50}
    }
    ```

3.  **基于旧值更新值**：
    通常与 `entry().or_insert()` 结合使用，获取一个可变引用，然后修改它。
    ```rust
    use std::collections::HashMap;
    fn main() {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() { // split_whitespace 返回 &str
            let count = map.entry(word.to_string()).or_insert(0); // or_insert(0) 返回 &mut i32
            *count += 1; // 解引用 count 来修改值
        }
        println!("Word counts: {:?}", map);
        // 输出类似 {"world": 2, "hello": 1, "wonderful": 1} (顺序不定)
    }
    ```

### 6.3.6 哈希函数

默认情况下，`HashMap` 使用一种名为 SipHash 的哈希函数，它能抵抗哈希表碰撞攻击 (DoS 攻击)。这不一定是最快的哈希算法，但它具有较好的安全性。
如果需要，你可以指定不同的哈希器 (hasher)。这通常用于性能关键的场景，并且你需要评估不同哈希函数的性能和安全特性。

## 6.4 总结

向量、字符串和哈希映射是 Rust 中非常强大且常用的集合类型，它们提供了灵活的方式来存储和管理动态数据。
*   **`Vec<T>`**：用于存储同类型的动态数组。
*   **`String`**：用于存储可变的 UTF-8 文本数据。
*   **`HashMap<K, V>`**：用于存储键值对映射。

理解它们的工作方式、所有权规则以及如何有效地使用它们的方法，对于编写高效的 Rust 程序非常重要。

## 6.5 常见陷阱

1.  **向量：在持有元素引用时修改向量**：
    *   **陷阱**：获取向量中某个元素的引用后，如果再调用可能导致向量重新分配内存的方法（如 `push`），之前的引用可能会失效，导致编译错误或未定义行为（如果编译器未能捕获）。
        ```rust
        // let mut v = vec![1, 2, 3];
        // let first = &v[0]; // 不可变引用
        // v.push(4);         // 编译错误：不能在存在不可变借用时可变借用 v
        // println!("{}", first);
        ```
    *   **避免**：确保在修改向量之前，所有指向其内部元素的引用都已结束生命周期。或者，克隆元素，或者使用索引而不是引用（但要注意索引有效性）。

2.  **字符串：错误的索引和切片**：
    *   **陷阱**：`String` 是 UTF-8 编码的，直接使用字节索引访问或切片 `String` 可能会导致 panic，如果索引不在有效的 UTF-8 字符边界上。
        ```rust
        // let s = String::from("你好"); // "你" 占3字节
        // let sub = &s[0..1]; // Panic! 1 不是字符边界
        ```
    *   **避免**：
        *   使用 `.chars()` 迭代字符。
        *   使用 `.bytes()` 迭代字节。
        *   如果需要按“字形簇”处理，使用如 `unicode-segmentation` crate。
        *   如果确实需要字节切片，确保了解 UTF-8 编码并小心操作。

3.  **字符串：`+` 运算符的所有权转移**：
    *   **陷阱**：使用 `+` 运算符连接 `String` 和 `&str` 时 (`let s3 = s1 + &s2;`)，`s1` (第一个 `String`) 的所有权会被转移给 `add` 方法，之后 `s1` 不再有效。
    *   **避免**：
        *   如果需要多次拼接或保留原始字符串，使用 `format!` 宏，它不获取参数所有权。
        *   如果仍想用 `+`，可以克隆第一个 `String`：`let s3 = s1.clone() + &s2;`。

4.  **哈希映射：所有权转移**：
    *   **陷阱**：将拥有所有权的类型（如 `String`）作为键或值插入 `HashMap` 时，其所有权会被转移到 `HashMap` 中。
        ```rust
        // let k = String::from("key");
        // let v = String::from("value");
        // let mut map = HashMap::new();
        // map.insert(k, v);
        // println!("{}", k); // 编译错误：k 已被移动
        ```
    *   **避免**：
        *   如果键或值需要在外部继续使用，插入它们的克隆：`map.insert(k.clone(), v.clone());`。
        *   或者，如果适用，插入引用（但这需要处理生命周期问题）。

5.  **哈希映射：键的类型与 `get` 方法参数类型**：
    *   **陷阱**：`HashMap::get` 方法的参数类型是 `&Q` 其中 `Q` 必须能够被哈希并且能够与存储的键类型 `K` 进行比较 (即 `K: Borrow<Q>`)。通常，这意味着如果你用 `String` 作为键，你可以用 `&String` 或 `&str` 来调用 `get`。但有时类型推断可能不按预期工作。
    *   **避免**：确保传递给 `get` 的是键的引用，并且类型兼容。例如，如果键是 `String`，`map.get("my_key_literal")` 通常可行。

6.  **迭代集合时修改集合 (通用陷阱)**：
    *   **陷阱**：在迭代一个集合（如 `Vec` 或 `HashMap`）的过程中，如果尝试修改该集合（例如，在 `for` 循环内部 `push` 元素到 `Vec`，或 `insert` 到 `HashMap`），通常会导致编译错误或逻辑问题，因为迭代器可能失效。
    *   **避免**：
        *   如果需要修改，先收集需要修改的内容，在迭代结束后再进行修改。
        *   对于 `Vec`，可以使用基于索引的循环（但要小心管理索引和长度），或者使用 `retain` 等方法。
        *   对于 `HashMap`，修改通常在 `entry` API 的上下文中进行，或者在迭代后进行。

## 6.6 常见面试题

1.  **Q: `Vec<T>` 和数组 `[T; N]` 有什么主要区别？什么时候应该选择使用 `Vec<T>`？**
    *   **A:**
        *   **主要区别**：
            1.  **大小**：
                *   数组 `[T; N]`：长度固定，在编译时确定 (N 是常量)。存储在栈上（如果元素类型和总大小允许）。
                *   向量 `Vec<T>`：长度动态可变，可以在运行时增长或缩小。数据存储在堆上。
            2.  **所有权和可变性**：
                *   数组：本身是值类型。如果是 `mut`，可以修改其元素。
                *   向量：是一个结构体，包含指向堆数据的指针、长度和容量。要修改向量（如添加元素），向量本身必须是 `mut`。
            3.  **性能**：
                *   数组：访问速度快，因为大小和位置固定。
                *   向量：访问也很快，但添加元素可能涉及内存重新分配和数据复制，这可能比数组操作慢。
        *   **何时选择 `Vec<T>`**：
            *   当你在编译时不知道需要存储多少元素时。
            *   当你需要在程序运行时动态添加或删除元素时。
            *   当你需要一个可以增长的列表时。
            *   当函数需要返回一个集合，但集合大小在编译时不确定时。

2.  **Q: Rust 中的 `String` 和 `&str` 有什么区别？它们分别在什么情况下使用？**
    *   **A:**
        *   **`&str` (字符串切片)**：
            *   是对一段 UTF-8 编码的字符串数据的**引用**。
            *   不拥有数据，只是指向数据。
            *   通常是不可变的（因为它们是引用）。
            *   字符串字面量 (如 `"hello"`) 的类型就是 `&'static str` (具有静态生命周期的字符串切片)。
            *   **使用场景**：
                *   当只需要读取字符串数据而不需要修改或拥有它时。
                *   作为函数参数，如果函数只需要读取字符串内容（这样函数可以接受 `String` 和字符串字面量）。
                *   字符串字面量。
        *   **`String`**：
            *   是一个**有所有权的**、可变的、在堆上分配的 UTF-8 编码的字符串类型。
            *   可以动态增长和修改。
            *   当你需要创建或修改字符串数据时使用。
            *   **使用场景**：
                *   当你需要在运行时构建或修改字符串时。
                *   当函数需要返回一个字符串，并且该字符串是在函数内部创建的时（需要转移所有权）。
                *   当结构体需要拥有其字符串数据时。
        *   **关系**：可以从 `String` 中获取 `&str` (例如，`&my_string[..]` 或通过 deref coercion `&my_string`)。可以从 `&str` 创建 `String` (例如，`str_slice.to_string()` 或 `String::from(str_slice)`)。

3.  **Q: 为什么 Rust 的 `String` 不支持直接通过整数索引访问字符？如何正确地迭代或访问 `String` 中的字符？**
    *   **A:**
        *   **不支持直接索引的原因**：
            Rust 的 `String` 是 UTF-8 编码的。在 UTF-8 中，一个“字符”（Unicode 标量值）可能由 1 到 4 个字节组成。如果允许直接通过整数索引访问 `String`，这个索引会对应字节位置。访问单个字节可能不会得到一个完整的、有效的字符，甚至可能导致程序 panic（如果索引位于多字节字符的中间并尝试将其解释为字符）。这违反了 Rust 的内存安全和正确性原则。
        *   **如何正确迭代或访问字符**：
            1.  **`.chars()` 方法**：迭代 `String` 或 `&str` 中的 `char` (Unicode 标量值)。这是最常用的按“字符”迭代的方式。
                ```rust
                // let s = String::from("你好");
                // for c in s.chars() { println!("{}", c); } // 输出 '你' 和 '好'
                ```
            2.  **`.bytes()` 方法**：迭代 `String` 或 `&str` 中的原始 `u8` 字节。
            3.  **字符串切片 (小心使用)**：可以获取 `&str` 的切片 `&my_string[start_byte..end_byte]`，但必须确保 `start_byte` 和 `end_byte` 位于有效的 UTF-8 字符边界，否则会 panic。
            4.  **使用外部 crate**：对于更复杂的文本处理，如按“字形簇”（用户感知到的字符）迭代，可以使用如 `unicode-segmentation` crate。

4.  **Q: `HashMap<K, V>` 中的键和值的所有权是如何处理的？**
    *   **A:**
        *   当向 `HashMap` 中插入键和值时：
            *   如果键 (`K`) 或值 (`V`) 的类型实现了 `Copy` trait (例如 `i32`, `bool`)，那么它们的副本会被存储在 `HashMap` 中，原始值仍然有效。
            *   如果键或值的类型拥有所有权且没有实现 `Copy` (例如 `String`, `Vec<T>`)，那么它们的所有权会被**移动**到 `HashMap` 中。插入后，原始变量将不再有效。
        *   **示例**：
            ```rust
            // use std::collections::HashMap;
            // let key_string = String::from("mykey");
            // let value_vec = vec![1, 2, 3];
            // let mut map = HashMap::new();
            // map.insert(key_string, value_vec);
            // // 此时 key_string 和 value_vec 的所有权已转移到 map 中
            // // println!("{}", key_string); // 编译错误
            ```
        *   如果不想转移所有权，可以插入键或值的克隆版本 (`map.insert(key_string.clone(), value_vec.clone());`)，或者在满足生命周期要求的情况下插入引用。

5.  **Q: 如何在 `HashMap` 中更新一个已存在的键的值，或者在键不存在时插入一个新值？请描述 `entry` API 的用法。**
    *   **A:**
        *   **覆盖旧值**：直接使用 `insert(key, new_value)` 会覆盖具有相同 `key` 的旧值。
        *   **`entry` API**：`HashMap` 的 `entry(key)` 方法提供了一种更灵活的方式来处理键可能存在或不存在的情况。它返回一个 `Entry` 枚举，该枚举有两个变体：
            *   `Entry::Occupied(entry)`：表示键已存在。`entry` 允许访问或修改现有的值。
            *   `Entry::Vacant(entry)`：表示键不存在。`entry` 允许插入新值。
        *   **`or_insert(default_value)` 方法**：是 `Entry` API 中最常用的方法之一。
            *   如果键已存在 (`Occupied`)，`or_insert` 返回一个指向现有值的**可变引用 (`&mut V`)**。
            *   如果键不存在 (`Vacant`)，`or_insert` 会将 `default_value` 插入到 `HashMap` 中，并返回一个指向新插入值的**可变引用 (`&mut V`)**。
        *   **用法示例 (统计单词频率)**：
            ```rust
            // use std::collections::HashMap;
            // let text = "hello world hello";
            // let mut word_counts = HashMap::new();
            // for word in text.split_whitespace() {
            //     let count = word_counts.entry(word.to_string()).or_insert(0);
            //     // count 是 &mut i32 类型
            //     *count += 1; // 解引用并修改值
            // }
            // // word_counts 会是 {"hello": 2, "world": 1}
            ```
        这种方式既简洁又高效，避免了先 `get` 再 `insert` 的两步操作和可能的竞争条件（在并发环境中）。

现在，我将为本章创建一个示例 Cargo 项目。
