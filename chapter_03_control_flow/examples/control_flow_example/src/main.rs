// 用于演示 match 的枚举
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Quarter 变体现在包含一个 UsState 值
}

// 用于 Quarter 变体的枚举
#[derive(Debug)] // 派生 Debug trait 以便能够打印 UsState
enum UsState {
    Alabama,
    Alaska,
    California,
    // ... 其他州
}

// 函数：根据硬币类型返回其价值 (美分)
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("一枚幸运的便士!");
            1 // 返回值
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // state 变量绑定到 UsState 的值
            println!("来自 {:?} 州的25美分硬币!", state);
            25
        }
    }
}

fn main() {
    println!("--- 3.1 if 表达式 ---");
    let number = 7;

    if number < 5 {
        println!("条件为 true: number ({}) 小于 5", number);
    } else {
        println!("条件为 false: number ({}) 不小于 5", number);
    }

    let another_number = 6;
    if another_number % 4 == 0 {
        println!("{} 可以被 4 整除", another_number);
    } else if another_number % 3 == 0 {
        println!("{} 可以被 3 整除", another_number);
    } else if another_number % 2 == 0 {
        println!("{} 可以被 2 整除", another_number);
    } else {
        println!("{} 不能被 4, 3, 或 2 整除", another_number);
    }

    // 在 let 语句中使用 if
    let condition = true;
    let assigned_number = if condition { 5 } else { 6 };
    println!("通过 if 表达式赋的值是: {}", assigned_number);
    println!();

    println!("--- 3.2 循环 ---");
    // 3.2.1 loop
    println!("--- 3.2.1 loop ---");
    let mut counter = 0;
    let result_from_loop = loop {
        counter += 1;
        println!("loop counter: {}", counter);
        if counter == 3 { // 为了示例简洁，我们让它循环3次
            break counter * 2; // 从循环返回值
        }
    };
    println!("loop 执行完毕，返回结果: {}", result_from_loop);

    // loop 标签
    let mut count_outer = 0;
    'outer_loop: loop {
        println!("外层循环 count_outer: {}", count_outer);
        let mut count_inner = 0;
        loop {
            println!("  内层循环 count_inner: {}", count_inner);
            if count_inner == 1 {
                break; // 中断内层循环
            }
            if count_outer == 1 && count_inner == 0 { // 演示中断外层循环
                 println!("  即将中断外层循环 'outer_loop'");
                 break 'outer_loop;
            }
            count_inner +=1;
        }
        if count_outer == 1 { // 如果外层循环被中断，这里不会执行
            println!("外层循环在内层循环 break 后继续...");
        }
        count_outer += 1;
        if count_outer == 2 { // 确保外层循环有自己的退出条件
            break;
        }
    }
    println!("带标签的循环结束，count_outer: {}", count_outer); // 如果从内部 break 'outer_loop'，这里会是1
    println!();


    // 3.2.2 while
    println!("--- 3.2.2 while ---");
    let mut while_number = 3;
    while while_number != 0 {
        println!("while_number: {}!", while_number);
        while_number -= 1;
    }
    println!("发射 (while)!");
    println!();

    // 3.2.3 for
    println!("--- 3.2.3 for ---");
    let array_a = [10, 20, 30, 40, 50];

    print!("遍历数组元素 (iter()): ");
    for element in array_a.iter() {
        print!("{} ", element);
    }
    println!();

    print!("遍历数组元素 (直接 for in array): ");
    for element in array_a { // 从 Rust 1.53 开始支持
        print!("{} ", element);
    }
    println!();


    print!("遍历范围 (1..4).rev(): ");
    for number_in_range in (1..4).rev() { // 范围是 3, 2, 1
        print!("{}! ", number_in_range);
    }
    println!("发射 (for range)!");

    print!("遍历范围并获取索引 (enumerate()): ");
    for (index, value) in array_a.iter().enumerate() {
        print!("索引 {} 值 {} | ", index, value);
    }
    println!();
    println!();


    println!("--- 3.3 match 表达式 ---");
    let my_coin = Coin::Penny;
    println!("硬币价值: {} 美分", value_in_cents(my_coin));

    let quarter_coin = Coin::Quarter(UsState::California);
    println!("硬币价值: {} 美分", value_in_cents(quarter_coin));

    let five_cents = Coin::Nickel;
    let value = match five_cents {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25, // 使用 _ 忽略 Quarter 内部的 UsState
    };
    println!("五美分硬币通过 match 得到的值: {}", value);

    // match 与 Option<T>
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;

    fn print_option(opt: Option<i32>) {
        match opt {
            Some(number) => println!("Option 包含值: {}", number),
            None => println!("Option 不包含值 (None)"),
        }
    }
    print_option(some_number);
    print_option(no_number);

    // _ 通配符
    let dice_roll = 9;
    print!("掷骰子结果 {}: ", dice_roll);
    match dice_roll {
        3 => println!("获得一个新帽子!"),
        7 => println!("失去一个帽子!"),
        other => println!("移动 {} 步.", other), // other 会绑定到 dice_roll 的值
        // 如果上面的 other 分支不存在，且没有其他分支覆盖 9，则必须有通配符
        // _ => println!("重新掷骰子!"), // 如果不想用值，可以用 _
        // _ => (), // 如果什么都不想做
    }
    println!();

    println!("--- if let 语法糖 ---");
    let favorite_color: Option<&str> = None;
    let is_tuesday = true;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("使用你最喜欢的颜色 {} 作为背景", color);
    } else if is_tuesday {
        println!("周二，使用绿色背景!");
    } else if let Ok(parsed_age) = age {
        if parsed_age > 30 {
            println!("年龄大于30 ({}岁)，使用紫色背景", parsed_age);
        } else {
            println!("年龄不大于30 ({}岁)，使用橙色背景", parsed_age);
        }
    } else { // favorite_color 是 None, is_tuesday 是 false, age 解析失败
        println!("使用蓝色背景");
    }

    // while let
    println!("--- while let ---");
    let mut stack = Vec::new();
    stack.push(10);
    stack.push(20);
    stack.push(30);

    print!("从栈中弹出元素: ");
    while let Some(top) = stack.pop() { // pop() 返回 Option<T>
        print!("{} ", top); // 依次打印 30, 20, 10
    }
    println!("\n栈现在为空: {:?}", stack);
}
