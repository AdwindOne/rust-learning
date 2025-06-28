// 5.1 结构体 (Structs)

// 定义一个 User 结构体
#[derive(Debug)] // 派生 Debug trait 以便能用 {:?} 打印 User
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 元组结构体
struct Color(i32, i32, i32); // RGB
struct Point(i32, i32, i32); // 3D 点

// 单元结构体
struct AlwaysEqual;

// Rectangle 结构体用于演示方法
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl 块，用于为 Rectangle 定义方法和关联函数
impl Rectangle {
    // 方法：计算面积 (借用 &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 方法：检查是否能容纳另一个 Rectangle (借用 &self)
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 方法：修改宽度 (可变借用 &mut self)
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // 关联函数：创建 Rectangle 实例 (构造器)
    fn new(width: u32, height: u32) -> Self { // Self 是 Rectangle 的别名
        Self { width, height }
    }

    // 关联函数：创建正方形 Rectangle 实例
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}


// 5.3 枚举 (Enums)

// IP 地址类型枚举
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// IP 地址枚举，成员可以附加数据
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 消息枚举，演示不同类型的数据附加
#[derive(Debug)]
enum Message {
    Quit,                       // 无关联数据
    Move { x: i32, y: i32 },    // 关联匿名结构体
    Write(String),              // 关联 String
    ChangeColor(i32, i32, i32), // 关联三个 i32
}

// 为 Message 枚举实现方法
impl Message {
    fn call(&self) {
        println!("Message method called. Message is: {:?}", self);
        // 可以在这里使用 match 处理不同的成员
        match self {
            Message::Write(text) => println!("  -> It's a Write message with text: '{}'", text),
            Message::Move{x, y} => println!("  -> It's a Move message to x:{}, y:{}", x, y),
            _ => println!("  -> It's another kind of message."), // 处理其他情况
        }
    }
}

// 硬币枚举，用于演示 match
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Quarter 现在包含一个 UsState
}

// 美国州枚举，用于 Coin::Quarter
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... etc.
}

// 函数：根据硬币类型返回其价值 (美分)
fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("  - Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // state 绑定到 UsState
            println!("  - State quarter from {:?}!", state);
            25
        }
    }
}

// 函数：处理 Option<i32>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


fn main() {
    println!("--- 5.1 结构体 (Structs) ---");
    // 创建 User 实例
    let mut user1 = User {
        email: String::from("user1@example.com"),
        username: String::from("user1name"),
        active: true,
        sign_in_count: 1,
    };
    println!("User1: {:?}", user1);
    user1.email = String::from("new.user1@example.com");
    println!("User1 (email updated): {:?}", user1);

    // 使用构建函数和字段初始化简写
    fn build_user(email: String, username: String) -> User {
        User {
            email,    // 简写
            username, // 简写
            active: true,
            sign_in_count: 1,
        }
    }
    let user2 = build_user(String::from("user2@example.com"), String::from("user2name"));
    println!("User2: {:?}", user2);

    // 结构体更新语法
    let user3 = User {
        email: String::from("user3@example.com"),
        username: String::from("user3name"),
        ..user2 // active 和 sign_in_count 从 user2 获取
                // 注意：如果 user2.username 和 user2.email 不是 Copy 类型，
                // 且这里没有显式赋值，它们的所有权会移动到 user3。
                // 但因为我们为 user3 的 email 和 username 提供了新值，
                // user2 的 email 和 username 字段仍然有效（如果它们是 String）。
    };
    println!("User3 (from user2 with update syntax): {:?}", user3);
    // println!("User2 after user3 init: {:?}", user2); // user2 的 active 和 sign_in_count 被复制

    // 元组结构体实例
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Color black: R={}, G={}, B={}", black.0, black.1, black.2);
    println!("Point origin: x={}, y={}, z={}", origin.0, origin.1, origin.2);

    // 单元结构体实例 (通常不这样用，但可以创建)
    let _always_equal_instance = AlwaysEqual;
    println!();


    println!("--- 5.2 结构体的方法 ---");
    let rect1 = Rectangle { width: 30, height: 50 };
    let mut rect2 = Rectangle { width: 10, height: 20 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("rect1 is {:?}", rect1);
    println!("The area of rect1 is {} square pixels.", rect1.area());

    rect2.set_width(12);
    println!("rect2 after set_width(12): {:?}, area: {}", rect2, rect2.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 使用关联函数 (构造器)
    let rect_new = Rectangle::new(25, 55);
    println!("rect_new (from Rectangle::new): {:?}, area: {}", rect_new, rect_new.area());
    let sq1 = Rectangle::square(20);
    println!("sq1 (from Rectangle::square): {:?}, area: {}", sq1, sq1.area());
    println!();


    println!("--- 5.3 枚举 (Enums) ---");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("IpAddrKind four: {:?}, six: {:?}", four, six);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("IpAddr home: {:?}, loopback: {:?}", home, loopback);

    let msg_quit = Message::Quit;
    let msg_move = Message::Move { x: 10, y: -5 };
    let msg_write = Message::Write(String::from("Hello Enum Method"));
    let msg_color = Message::ChangeColor(255, 0, 128);

    println!("Message Quit: {:?}", msg_quit);
    msg_write.call();
    msg_move.call();
    msg_color.call(); // 仅打印，未在 call 方法中特殊处理
    println!();


    println!("--- Option<T> 枚举 ---");
    let some_num = Some(50);
    let some_str = Some("a string value");
    let absent_num: Option<i32> = None; // 需要类型注解，因为 None 可以是任何 Option<T>

    println!("some_num: {:?}, some_str: {:?}, absent_num: {:?}", some_num, some_str, absent_num);

    let x_opt: Option<i32> = Some(10);
    let y_opt: i32 = 5;
    // let sum_opt = x_opt + y_opt; // 编译错误! Option<i32> 和 i32 类型不兼容
    // 需要先从 Option 中取出值
    if let Some(val_x) = x_opt {
        println!("Sum of Some({}) and {} is {}", val_x, y_opt, val_x + y_opt);
    }
    println!();


    println!("--- match 控制流与枚举 ---");
    let my_penny = Coin::Penny;
    let my_quarter = Coin::Quarter(UsState::Alaska);
    println!("Value of my_penny: {} cents", value_in_cents(&my_penny));
    println!("Value of my_quarter: {} cents", value_in_cents(&my_quarter));

    let five_opt = Some(5);
    let six_opt = plus_one(five_opt);    // Some(6)
    let none_opt = plus_one(None); // None
    println!("plus_one(Some(5)): {:?}, plus_one(None): {:?}", six_opt, none_opt);

    // _ 通配符
    let dice_roll = 9;
    print!("Dice roll is {}. ", dice_roll);
    match dice_roll {
        3 => println!("You win a prize!"),
        7 => println!("Try again!"),
        other => println!("Move {} spaces.", other), // other 绑定到 9
    }

    let another_roll = 1;
    print!("Another roll is {}. ", another_roll);
    match another_roll {
        3 => println!("You win a prize!"),
        7 => println!("Try again!"),
        _ => println!("Nothing special happened."), // _ 匹配任何未被处理的值，但不绑定
    }
    println!();


    println!("--- if let 简洁控制流 ---");
    let config_max_val: Option<u8> = Some(3u8);

    // 使用 match
    print!("config_max_val (match): ");
    match config_max_val {
        Some(max) => println!("Max is {}", max),
        _ => println!("Not configured or None"),
    }

    // 使用 if let
    print!("config_max_val (if let): ");
    if let Some(max) = config_max_val {
        println!("Max is {}", max);
    } else {
        println!("Not configured or None");
    }

    let a_coin = Coin::Penny;
    let mut non_quarter_count = 0;
    if let Coin::Quarter(state) = a_coin { // a_coin 不是 Quarter
        println!("This is a state quarter from {:?} (using if let)", state);
    } else {
        non_quarter_count += 1;
    }
    println!("Non-quarter coins counted (after checking a_coin): {}", non_quarter_count);

    let b_coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = b_coin { // b_coin 是 Quarter
        println!("This is a state quarter from {:?} (using if let)", state);
    } else {
        non_quarter_count += 1; // 这不会执行
    }
    println!("Non-quarter coins counted (after checking b_coin): {}", non_quarter_count);
}
