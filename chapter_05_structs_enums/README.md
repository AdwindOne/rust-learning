# 第 5 章：结构体 (Structs) 与枚举 (Enums)

结构体 (Structs) 和枚举 (Enums) 是 Rust 中创建自定义数据类型的两种主要方式。它们是面向数据编程 (data-oriented programming) 的核心，允许你将数据组织成有意义的、富有表现力的结构，并为其附加行为。掌握它们对于构建任何复杂的 Rust 应用程序都至关重要。

## 5.1 结构体 (Structs)

结构体（struct，structure 的缩写）是一种自定义数据类型，允许你将多个相关的值（称为**字段 (fields)**）组合在一起并给它们命名。它类似于其他语言中的对象（的属性部分）、记录 (records) 或 C 语言的 struct。

### 5.1.1 定义和实例化结构体

使用 `struct` 关键字来定义一个结构体，后跟结构体的名称（通常使用 `PascalCase` 命名法），以及一对花括号 `{}`，其中包含结构体的所有字段。每个字段都有一个名称（`snake_case`）和一个类型。

```rust
// 定义一个 User 结构体
struct User {
    active: bool,
    username: String, // 字段 username 的类型是 String
    email: String,
    sign_in_count: u64,
}

fn main_struct_instantiation() { // Renamed for clarity
    // 创建 User 结构体的一个实例 (instance)
    // 在实例化时，需要为每个字段提供具体的值。
    // 字段的顺序不必与定义时的顺序相同。
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 访问结构体实例的字段值，使用点号 `.`
    println!("User email: {}", user1.email);

    // 如果结构体实例是可变的 (用 `mut` 声明)，可以修改其字段的值
    user1.email = String::from("anotheremail@example.com");
    println!("User new email: {}", user1.email);
    user1.sign_in_count += 1;

    // 构建 User 实例的辅助函数
    fn build_user(email: String, username: String) -> User {
        User {
            email: email, // 字段名和参数名相同
            username: username, // 字段名和参数名相同
            active: true,
            sign_in_count: 1,
        }
    }
    let user2 = build_user(String::from("user2@example.com"), String::from("user2name"));
    println!("User2 username: {}", user2.username);
}
```

**字段初始化简写语法 (Field Init Shorthand)**
当结构体字段的名称与包含这些字段值的局部变量或函数参数的名称完全相同时，可以使用字段初始化简写语法，只写一次名称即可，而无需 `field_name: variable_name`。

```rust
// Helper struct for shorthand example
struct UserForShorthand { active: bool, username: String, email: String, sign_in_count: u64 }
fn build_user_shorthand(email: String, username: String) -> UserForShorthand {
    UserForShorthand { // Corrected to use UserForShorthand
        email,    // 简写：等同于 email: email
        username, // 简写：等同于 username: username
        active: true,
        sign_in_count: 1,
    }
}
```

**结构体更新语法 (Struct Update Syntax)**
当你想从一个现有结构体实例创建另一个新实例，并且新实例的大部分字段值与旧实例相同时，可以使用结构体更新语法 `..`。这可以减少代码重复。

```rust
// Helper struct for update example
struct UserForUpdate { active: bool, username: String, email: String, sign_in_count: u64 }
fn main_struct_update() { // Renamed
    let user1 = UserForUpdate {
        email: String::from("user1@example.com"),
        username: String::from("user1name"),
        active: true,
        sign_in_count: 10,
    };

    let user3 = UserForUpdate {
        email: String::from("user3@example.com"),
        username: String::from("user3name"),
        ..user1
    };
    println!("User3: active={}, sign_in_count={}, email={}", user3.active, user3.sign_in_count, user3.email);
}
```

### 5.1.2 元组结构体 (Tuple Structs)

元组结构体是一种看起来像元组的结构体。它有一个结构体名称，但其字段没有名称，只有类型。当你想要给整个元组一个名称，并使其在类型系统上不同于其他元组类型（即使它们的字段类型完全相同），且为每个字段单独命名又显得不必要时，元组结构体很有用。

```rust
// 定义元组结构体
struct Color(i32, i32, i32); // 代表 RGB 颜色，三个字段都是 i32
struct Point3D(f64, f64, f64); // 代表 3D 空间中的点，三个字段都是 f64

fn main_tuple_structs() { // Renamed
    let black = Color(0, 0, 0);
    let origin = Point3D(0.0, 0.0, 0.0);

    println!("Black color: R={}, G={}, B={}", black.0, black.1, black.2);
    println!("Origin point: x={}, y={}, z={}", origin.0, origin.1, origin.2);
}
```
每个元组结构体都是其自身独一无二的类型。

### 5.1.3 单元结构体 (Unit-Like Structs)

你还可以定义没有任何字段的结构体，称为**单元结构体 (unit-like structs)**，因为它们类似于 `()`，即单元类型。
单元结构体在你需要在某个类型上实现 trait 但不需要在该类型中存储任何数据时非常有用。我们将在第8章讨论 trait。

```rust
struct AlwaysEqual; // 单元结构体定义

fn main_unit_structs() { // Renamed
    let _subject = AlwaysEqual;
}
```

### 5.1.4 结构体数据的所有权 (Ownership in Struct Data)

在之前的 `User` 结构体定义中，我们为 `username` 和 `email` 字段使用了 `String` 类型，而不是 `&str` (字符串切片) 类型。这是一个经过深思熟虑的选择，关系到所有权。

*   **让结构体拥有其所有数据**: 当结构体的字段类型是像 `String`, `Vec<T>`, `Box<T>` 这样拥有其数据的类型时，这意味着只要结构体实例本身是有效的，其所有字段的数据也都是有效的。结构体实例负责其所有字段的生命周期管理。这是最常见和推荐的做法，因为它简化了生命周期的处理。

*   **在结构体中使用引用 (`&str`, `&T`)**: 如果你希望结构体存储对其他地方数据的引用，而不是拥有数据本身，那么字段类型可以是引用类型（如 `&str`）。但是，这种情况下，你**必须**为结构体定义引入**生命周期注解 (lifetime annotations)**，以确保结构体实例及其引用的数据在生命周期上是兼容的（即，结构体实例不能比它引用的数据活得更长）。我们将在第8章详细讨论生命周期。

    ```rust
    // 正确的带有生命周期的结构体 (将在第8章解释)
    struct UserWithLifetime<'a> {
        username: &'a str,
        email: &'a str,
    }
    // fn main_struct_lifetimes() { // Renamed
    //     let email_data = String::from("user@example.com");
    //     let user_ref = UserWithLifetime {
    //         email: &email_data,
    //         username: "static_username",
    //     };
    // }
    ```
为了简单起见，在本章和之前的章节中，我们主要让结构体拥有其所有数据。

## 5.2 结构体的方法 (Method Syntax)

方法 (Methods) 与函数 (functions) 类似：它们都使用 `fn` 关键字和名称声明，可以有参数和返回值，并且包含一些执行某些操作的代码。然而，方法与函数的关键区别在于：
1.  方法是在特定类型（如结构体、枚举或 trait 对象）的**上下文**中定义的。
2.  方法的**第一个参数总是 `self`** (或 `&self`, 或 `&mut self`)，它代表调用该方法的那个类型的实例。

### 5.2.1 定义方法

方法在 `impl` 块 (implementation block) 中为特定类型定义。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width_is_positive(&self) -> bool { // Renamed from width to avoid conflict if called directly
        self.width > 0
    }

    fn set_width(&mut self, new_width: u32) {
        self.width = new_width;
    }
    // fn consume(self) -> String {
    //     format!("Consumed a {}x{} rectangle.", self.width, self.height)
    // }
}

fn main_methods() { // Renamed
    let rect1 = Rectangle { width: 30, height: 50 };
    let mut rect2 = Rectangle { width: 10, height: 20 };

    println!("rect1 的面积是 {} 平方像素 (调用 rect1.area())。", rect1.area());

    if rect1.width > 0 {
        println!("rect1 的字段 width ({}) 大于 0。", rect1.width);
    }
    if rect1.width_is_positive() {
        println!("rect1.width_is_positive() 方法调用结果为 true (宽度为正)。");
    }

    rect2.set_width(15);
    println!("rect2 修改后的宽度: {}", rect2.width);
    println!("rect2 修改后的面积: {}", rect2.area());
}
```
**`self` 参数的含义总结**：
*   `&self` (不可变借用)：方法可以读取实例数据，但不能修改。
*   `&mut self` (可变借用)：方法可以读取和修改实例数据。
*   `self` (获取所有权)：方法会获取实例的所有权，通常会消耗或转换该实例。

**自动引用和解引用 (Automatic Referencing and Dereferencing)**
当使用点号 `.` 调用方法时 (例如 `object.method()`)，Rust 会自动进行引用或解引用，以便 `object` 的类型与方法签名中 `self` 参数的类型匹配。

### 5.2.2 关联函数 (Associated Functions)

`impl` 块中也可以定义**不把 `self` 作为第一个参数**的函数，这些称为**关联函数 (associated functions)**。它们仍然与结构体（或枚举）关联（定义在类型的命名空间下），但它们不是方法，因为它们没有可操作的结构体实例（没有 `self`）。

关联函数通常用作**构造器 (constructors)**，返回该结构体类型的新实例。

```rust
// (Rectangle struct definition from above)
// struct Rectangle { width: u32, height: u32, }
impl Rectangle { // Assuming Rectangle is defined
    fn new_constructor(width: u32, height: u32) -> Self { // Renamed from new
        Self { width, height }
    }
    fn square_constructor(size: u32) -> Self { // Renamed from square
        Self { width: size, height: size }
    }
}

fn main_associated_functions() { // Renamed
    let rect1 = Rectangle::new_constructor(30, 50);
    let sq = Rectangle::square_constructor(25);

    println!("rect1 ({}x{}) area: {}", rect1.width, rect1.height, rect1.area());
    println!("sq ({}x{}) area: {}", sq.width, sq.height, sq.area());
}
```
标准库中的 `String::from()` 和 `Vec::new()` 就是常见的关联函数（构造器）的例子。

### 5.2.3 多个 `impl` 块

每个结构体（或枚举）可以有多个 `impl` 块。这在逻辑上没有区别，所有定义在这些 `impl` 块中的方法和关联函数都会与该类型关联起来。

```rust
struct PointForMultiImpl { x: f64, y: f64 } // Renamed for clarity

impl PointForMultiImpl {
    fn new(x: f64, y: f64) -> Self { PointForMultiImpl { x, y } }
    fn x_coord(&self) -> f64 { self.x } // Renamed from x
}

impl PointForMultiImpl {
    fn y_coord(&self) -> f64 { self.y } // Renamed from y
    fn set_y_coord(&mut self, new_y: f64) { self.y = new_y; } // Renamed
}
```

## 5.3 枚举 (Enums)

枚举 (enumerations，也称 enums) 允许你通过列举所有可能的**成员 (variants)** 来定义一个类型。一个枚举类型的值只能是其定义的成员之一。

### 5.3.1 定义枚举

使用 `enum` 关键字定义枚举，后跟枚举名称 (`PascalCase`) 和花括号 `{}` 中的成员列表。

```rust
#[derive(Debug)]
enum IpAddrKind { V4, V6 }

fn main_enum_definition() { // Renamed
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route_ip(four); // Renamed route
    route_ip(six);
}

fn route_ip(ip_kind: IpAddrKind) { // Renamed route
    println!("Routing IP of kind: {:?}", ip_kind);
}
```

**枚举成员可以附加数据**

Rust 枚举的一个非常强大的特性是，每个成员都可以存储不同类型和数量的关联数据。

```rust
#[derive(Debug)]
enum IpAddrData {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main_enum_with_data() { // Renamed
    let home_ip = IpAddrData::V4(127, 0, 0, 1);
    let loopback_ip = IpAddrData::V6(String::from("::1"));
    println!("Home IP: {:?}", home_ip);
    println!("Loopback IP: {:?}", loopback_ip);
}
```
你甚至可以为枚举的每个成员定义不同类型的结构体（匿名或具名）作为其关联数据：
```rust
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
    ComplexData(Box<CustomPayloadForMessage>), // Changed to Box for heap allocation if CustomPayload is large
}
#[derive(Debug)] struct CustomPayloadForMessage { id: u32, data: Vec<u8> } // Renamed

impl Message {
    fn process_message(&self) { // Renamed from call
        println!("Processing message: {:?}", self);
        match self {
            Message::Quit => println!("  Action: Terminate."),
            Message::Move { x, y } => println!("  Action: Move to ({}, {}).", x, y),
            Message::Write(text) => println!("  Action: Write text '{}'.", text),
            Message::ChangeColor(r, g, b) => println!("  Action: Change color to RGB({}, {}, {}).", r, g, b),
            Message::ComplexData(payload) => println!("  Action: Process complex data with id {}.", payload.id),
        }
    }
}

fn main_message_enum() { // Renamed
    let m1 = Message::Write(String::from("hello from enum method"));
    m1.process_message();
}
```

### 5.3.2 `Option<T>` 枚举：处理可选值 (空值)

`Option<T>` 是 Rust 标准库中定义的一个非常重要且普遍使用的泛型枚举。它用于编码一个值**可能是某个东西 (Some value of type T)，也可能什么都不是 (None)** 的概念。

`Option<T>` 的定义（概念上）如下：
```rust
// enum Option<T> {
//     None,
//     Some(T),
// }
```

```rust
fn main_option_enum() { // Renamed
    let some_number = Some(5);
    let absent_number: Option<i32> = None;
    println!("some_number: {:?}, absent_number: {:?}", some_number, absent_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    match y {
        Some(val) => println!("Sum with Some: {}", x + val),
        None => println!("Cannot sum with None"),
    }
}
```

### 5.3.3 `match` 控制流运算符 (回顾与深化)

`match` 运算符在处理枚举时尤其强大和常用。

```rust
// Simplified Coin for this example
enum CoinForMatch { Penny, Nickel, Dime, Quarter }

fn value_in_cents_for_match(coin: CoinForMatch) -> u8 { // Renamed
    match coin {
        CoinForMatch::Penny => 1,
        CoinForMatch::Nickel => 5,
        CoinForMatch::Dime => 10,
        CoinForMatch::Quarter => 25,
    }
}

fn plus_one_option(x: Option<i32>) -> Option<i32> { // Renamed
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main_match_option_usage() { // Renamed
    let five = Some(5);
    let six = plus_one_option(five);
    println!("plus_one_option(Some(5)) is: {:?}", six);
}
```

**`_` (下划线) 通配符和 `other` 变量**
在 `match` 中，`_` 匹配任何值不绑定，`other` 匹配任何值并绑定。

```rust
fn main_match_wildcard() { // Renamed
    let dice_roll = 9;
    match dice_roll {
        1 => println!("Rolled a 1!"),
        other => println!("Rolled something else: {}.", other),
    }
}
```

### 5.3.4 `if let` 简洁控制流 (回顾与深化)

`if let` 是 `match` 的一种语法糖，用于处理只关心一个模式的情况。

```rust
fn main_if_let_deep_dive() { // Renamed
    let config_max_value: Option<u8> = Some(3u8);
    if let Some(max) = config_max_value {
        println!("(if let) The maximum is configured to be {}", max);
    } else {
        println!("(if let) Maximum is not configured.");
    }
}
```

## 5.4 总结

结构体和枚举是 Rust 中创建自定义数据类型的基石。
*   **结构体 (Structs)**：用于将相关数据字段组合成有意义的单元。
    *   **普通结构体 (Named-field structs)**
    *   **元组结构体 (Tuple structs)**
    *   **单元结构体 (Unit-like structs)**
    *   可以使用 `impl` 块为结构体定义**方法**和**关联函数**。
*   **枚举 (Enums)**：用于定义一个可以是一系列不同可能**成员 (variants)** 之一的类型。
    *   枚举的每个成员可以拥有不同类型和数量的**关联数据**。
    *   标准库中的 `Option<T>` 和 `Result<T, E>` 是重要的泛型枚举。
    *   `match` 表达式是处理枚举成员的主要方式，`if let` 和 `while let` 提供了简洁的单模式匹配。

## 5.5 常见陷阱 (本章相关)

1.  **结构体字段所有权问题 (使用 `&str` 而不是 `String` 时忘记生命周期)**：
    *   **陷阱**：在结构体定义中直接为字段使用引用类型（如 `&str`）而没有指定生命周期参数，通常会导致编译错误 ("missing lifetime specifier")。
    *   **避免**：首选让结构体拥有其数据 (使用 `String`, `Vec<T>`)。如果必须使用引用，则需引入生命周期注解 (详见第8章)。

2.  **方法定义中 `self`, `&self`, 或 `&mut self` 的选择和误用**：
    *   **陷阱**：忘记 `self` 参数导致定义的是关联函数；或错误选择 `self` 形式，如在只读方法中使用 `&mut self`。
    *   **避免**：明确方法是否需要读取 (`&self`)、修改 (`&mut self`)或获取所有权 (`self`)。

3.  **`match` 表达式非穷尽 (Non-Exhaustive Match)**：
    *   **陷阱**：对枚举使用 `match` 时，如果遗漏了某些成员且没有使用 `_` 通配符，编译器会报错。
    *   **避免**：确保 `match` 覆盖所有可能性，对枚举列出所有成员或使用 `_`。

4.  **在 `match` 分支或 `if let` 中处理 `Option<T>` 或 `Result<T, E>` 时忘记解构内部值**：
    *   **陷阱**：模式只写 `Some` 或 `Ok` 而没有用变量绑定内部值 (如 `Some(x)`), 或错误地尝试直接使用 `Some` 本身。
    *   **避免**：使用 `Some(variable_name)` 或 `Ok(variable_name)` 来绑定内部值。如果不需要值，用 `Some(_)`。

5.  **混淆结构体更新语法 (`..`) 中的移动和复制语义**：
    *   **陷阱**：使用 `..s1` 时，`s1` 中非 `Copy` 类型的字段所有权会被移动到新结构体，导致 `s1` 的这些字段失效。
    *   **避免**：理解字段类型。如果 `s1` 需保持完全有效，为非 `Copy` 字段提供新值或显式克隆。

6.  **枚举成员和结构体字段的可见性 (`pub`)**：
    *   **陷阱**：默认私有。模块外访问私有字段或创建私有字段结构体实例会失败。
    *   **避免**：使用 `pub` 控制结构体、枚举及其字段/成员的可见性。通常推荐通过公共方法封装字段访问。

## 5.6 常见面试题 (本章相关，已补充和深化)

1.  **Q: Rust 中的结构体有哪几种类型？请分别举例说明它们的定义、实例化和典型用途。**
    *   **A: (详细解释)**
        Rust 中的结构体主要有三种类型，它们提供了不同的方式来组织和命名相关数据：
        1.  **普通结构体 (Named-field structs / Classic C-like structs)**：
            *   **定义**: 使用 `struct` 关键字，后跟结构体名称，以及一对花括号 `{}`，其中包含一系列命名的字段，每个字段都有其类型。
                ```rust
                struct User { id: u32, username: String, email: String, is_active: bool }
                ```
            *   **实例化**: 通过指定结构体名称和为每个字段提供值来创建实例。
                ```rust
                let user1 = User { id: 1, username: String::from("alice"), email: String::from("alice@example.com"), is_active: true };
                ```
            *   **访问字段**: 使用点号 `.`。 `user1.username`
            *   **典型用途**: 当你需要将多个相关的、具有明确名称和不同类型的数据片段组合成一个有意义的单元时。这是最常用的结构体类型。

        2.  **元组结构体 (Tuple structs)**：
            *   **定义**: 使用 `struct` 关键字，后跟结构体名称，以及一对圆括号 `()`，其中包含一系列类型（字段没有名称）。
                ```rust
                struct Color(u8, u8, u8);
                struct Point3D(f64, f64, f64);
                ```
            *   **实例化**: 像创建普通元组一样创建实例，但使用结构体名称。
                ```rust
                let red = Color(255, 0, 0);
                ```
            *   **访问字段**: 使用点号 `.` 后跟元素的索引 (从 0 开始)。 `red.0`
            *   **典型用途**: 当元组中的字段含义清晰，不需要单独命名，但你仍希望它是一个独立的类型以增强类型安全时。Newtype pattern 的一种形式。

        3.  **单元结构体 (Unit-like structs)**：
            *   **定义**: 使用 `struct` 关键字，后跟结构体名称，然后是一个分号 `;` (或一对空花括号 `{}`)。它们没有任何字段。
                ```rust
                struct AlwaysEqualMarker;
                ```
            *   **实例化**: 像普通结构体一样，但不提供任何字段值。 `let marker = AlwaysEqualMarker;`
            *   **典型用途**: 当需要在某个类型上实现 trait，但该类型本身不需要存储任何数据时。作为泛型参数的占位符。

2.  **Q: 结构体的方法 (methods) 和关联函数 (associated functions) 有什么区别？请分别解释它们的定义方式 (第一个参数) 和调用方式。**
    *   **A: (详细解释)**
        *   **方法 (Methods)**：
            *   **定义方式**: 第一个参数总是 `self`、`&self` 或 `&mut self`，代表调用该方法的实例。
            *   **调用方式**: 通过实例和点号 `.` 调用，例如 `instance.method_name()`。
            *   **目的**: 通常用于操作或查询特定实例的状态和数据。
        *   **关联函数 (Associated Functions)**：
            *   **定义方式**: 参数列表中没有 `self`、`&self` 或 `&mut self` 作为第一个参数。
            *   **调用方式**: 通过类型名称和双冒号 `::` 调用，例如 `TypeName::function_name()`。
            *   **目的**: 通常用作构造器 (如 `new()`) 或执行与类型相关但不依赖于特定实例状态的通用操作。

3.  **Q: 什么是枚举 (Enum)？Rust 的枚举与 C/C++ 或 Java 中的枚举有何本质不同和强大之处？请举例说明其关联数据特性。**
    *   **A: (详细解释)**
        *   **枚举 (Enum)**：允许定义一个类型，该类型可以是一组预定义的、命名的**成员 (variants)** 之一。
        *   **与 C/C++ 或 Java 枚举的不同和强大之处**:
            1.  **关联数据 (Associated Data)**: Rust 枚举的每个成员可以拥有不同类型和数量的关联数据。这使 Rust 枚举更像“和类型 (sum types)”或“标签联合 (tagged union)”，而不仅仅是符号常量（如C/C++）或有固定字段结构的特殊类（如Java）。
            2.  **类型安全和模式匹配**: Rust 枚举是类型安全的，`match` 表达式强制穷尽性检查。
            3.  **方法定义**: 可以为枚举类型定义方法。
            4.  **标准库应用**: 核心类型如 `Option<T>` 和 `Result<T, E>` 都是枚举。
        *   **关联数据举例**:
            ```rust
            enum WebEvent {
                PageLoad,
                KeyPress(char),
                Click { x: i32, y: i32 },
                Message(String),
            }
            // let event = WebEvent::Click { x:10, y:20 };
            // match event {
            //     WebEvent::KeyPress(c) => println!("Key: {}", c),
            //     WebEvent::Click{x,y} => println!("Click at {},{}", x,y),
            //     _ => {}
            // }
            ```

4.  **Q: 解释 Rust 中的 `Option<T>` 枚举。它如何帮助避免空指针错误？请列举几个常用的 `Option<T>` 方法及其作用。**
    *   **A: (详细解释)**
        *   **`Option<T>` 枚举**: 表示一个值**可能存在 (`Some(T)`) 或不存在 (`None`)** 的情况。Rust 用它处理“空值”，取代了其他语言中的 `null`。
        *   **如何避免空指针错误**:
            1.  **显式可选性**: `Option<T>` 在类型系统中明确了值的可选性。`T` 和 `Option<T>` 是不同类型。
            2.  **编译时强制处理**: 编译器要求在使用 `Option<T>` 内部值前，必须处理 `None` 的情况 (通过 `match`, `if let`, 或 `Option` 方法)。
        *   **常用方法**:
            *   `is_some()`, `is_none()`: 检查是否有值。
            *   `unwrap()`: 获取 `Some` 中的值，若是 `None` 则 panic。**慎用**。
            *   `expect(msg)`: 类似 `unwrap()`，但在 `None` 时 panic 并显示 `msg`。**慎用**。
            *   `unwrap_or(default)`: 获取值，若是 `None` 则返回 `default`。
            *   `unwrap_or_else(fn)`: 获取值，若是 `None` 则调用函数 `fn` 并返回其结果。
            *   `map(fn)`: 若是 `Some(v)`，对 `v` 应用函数 `fn` 并返回 `Some(fn(v))`；若是 `None`，返回 `None`。
            *   `and_then(fn)`: 若是 `Some(v)`，对 `v` 应用返回 `Option` 的函数 `fn`；若是 `None`，返回 `None`。
            *   `filter(predicate)`: 若是 `Some(v)` 且 `predicate(v)` 为 `true`，返回 `Some(v)`；否则返回 `None`。
            *   `ok_or(err)`, `ok_or_else(fn_err)`: 转换为 `Result<T, E>`。

5.  **Q: 在结构体定义中，字段类型选择 `String` 和 `&str` 有什么主要的权衡？在什么情况下，为结构体字段使用 `&str` 时必须引入生命周期注解？**
    *   **A: (详细解释)**
        *   **`String` 作为字段**:
            *   **优点**: 结构体拥有数据，生命周期管理简单，字段可变。
            *   **缺点**: 涉及堆分配和可能的复制开销。
        *   **`&str` 作为字段**:
            *   **优点**: 可能更高效（无额外分配/复制），如果引用现有数据。
            *   **缺点**: 结构体不拥有数据，**几乎总是需要生命周期注解**来确保引用有效性，管理更复杂，字段不可直接修改（因为 `&str` 不可变）。
        *   **何时为 `&str` 引入生命周期**: **只要结构体包含任何非 `'static` 的引用类型字段（包括 `&str`），结构体定义就必须使用泛型生命周期参数**。这是为了让编译器能验证引用字段不会比结构体实例或其引用的外部数据活得更长，从而防止悬垂引用。例外是当 `&str` 字段只引用字符串字面量 (它们是 `&'static str`) 时，可能不需要为结构体本身添加生命周期参数（如果所有引用字段都是 `'static`）。
        *   **权衡总结**: 优先 `String` 以简化生命周期。如果性能是关键瓶颈且能正确管理生命周期，可考虑 `&str`。

6.  **Q: 一个类型可以有多个 `impl` 块吗？这样做有什么实际的好处或常见的使用场景？**
    *   **A: (详细解释)**
        是的，一个 Rust 类型（如结构体或枚举）**可以有多个 `impl` 块**。
        *   **好处和场景**:
            1.  **代码组织**: 将逻辑相关的方法分组到不同的 `impl` 块，提高可读性。
            2.  **实现不同 Trait**: 为每个实现的 trait 使用单独的 `impl TraitName for TypeName { ... }` 块，结构清晰。
            3.  **泛型条件实现**: 为泛型类型定义通用 `impl<T> MyType<T>` 块，再为满足特定 trait bound 的 `T` 定义额外 `impl<T: SomeTrait> MyType<T>` 块。
            4.  **模块化**: 不同方面的实现可物理分离到不同文件/模块。
            5.  **宏生成**: `derive` 宏通常会生成新的 `impl` 块。
        *   最终效果与所有方法放在一个 `impl` 块中相同，主要是代码组织上的便利。

7.  **Q: 在 `impl` 块中，`Self` (大写S) 和 `self` (小写s) 分别代表什么？它们之间有什么关系？**
    *   **A: (详细解释)**
        *   **`Self` (大写 S)**:
            *   **代表**: 一个**类型别名**，指代当前 `impl` 块所针对的那个**类型本身**。
            *   **用途**: 作为构造器或方法的返回类型 (`-> Self`)；在类型签名中引用当前类型。
        *   **`self` (小写 s)**:
            *   **代表**: 一个**特殊的参数名称**，用作方法的**第一个参数**，代表调用该方法的那个类型的**实例**。
            *   **形式**: `self` (获取所有权), `&self` (不可变借用), `&mut self` (可变借用)。
            *   **用途**: 使方法能够操作或访问其所属实例的数据。
        *   **关系**: `self` 参数的类型（在省略类型注解时）是 `Self`、`&Self` 或 `&mut Self`。`Self` 定义了 `self` 参数所代表实例的类型。

8.  **Q: 除了 `Option<T>`，Rust 标准库中还有哪些常用的、基于枚举实现的、用于特定目的的核心类型？请至少举一个例子并解释其用途。**
    *   **A: (详细解释)**
        *   **`Result<T, E>` 枚举**:
            *   **定义**: `enum Result<T, E> { Ok(T), Err(E) }` (`T`: 成功值类型, `E`: 错误值类型)。
            *   **用途**: Rust 中用于**可恢复错误处理**的主要机制。当操作可能失败且失败是预期情况时，函数返回 `Result`。调用者必须处理 `Ok` 和 `Err` 两种情况。
            *   **重要性**: 强制显式错误处理，避免忽略潜在错误，提高代码健壮性。`?` 运算符简化了 `Result` 的传播。
        *   **其他例子**:
            *   **`std::cmp::Ordering`**: `enum Ordering { Less, Equal, Greater }`。由比较操作返回，表示相对顺序。
            *   **`std::io::ErrorKind`**: `enum ErrorKind { NotFound, PermissionDenied, ... }`。`std::io::Error` 的 `.kind()` 方法返回此枚举，用于区分不同类型的 I/O 错误。

// Helper main function for example snippets if this file were to be compiled.
fn main() {
    struct User { active: bool, username: String, email: String, sign_in_count: u64 }
    fn main_struct_instantiation_call() {
        let mut user1 = User { email: String::from("s@e.com"), username: String::from("u"), active: true, sign_in_count: 1};
        user1.email = String::from("n@e.com");
        fn build_user(email: String, username: String) -> User { User { email, username, active: true, sign_in_count: 1 } }
        let _user2 = build_user(String::from("u2@e.com"), String::from("u2"));
    }
    main_struct_instantiation_call();

    struct UserForShorthand { active: bool, username: String, email: String, sign_in_count: u64 }
    fn build_user_shorthand_call(email: String, username: String) -> UserForShorthand { UserForShorthand{ email, username, active:true, sign_in_count:1} }
    build_user_shorthand_call(String::from("e"), String::from("u"));

    struct UserForUpdate { active: bool, username: String, email: String, sign_in_count: u64 }
    fn main_struct_update_call() { let u1 = UserForUpdate{email:String::from("e"),username:String::from("u"),active:true,sign_in_count:1}; let _u3=UserForUpdate{email:String::from("e3"),username:String::from("u3"),..u1}; }
    main_struct_update_call();

    struct Color(i32,i32,i32); struct Point3D(f64,f64,f64);
    fn main_tuple_structs_call(){ let _b = Color(0,0,0); let _o = Point3D(0.0,0.0,0.0); }
    main_tuple_structs_call();

    struct AlwaysEqual; fn main_unit_structs_call(){ let _s=AlwaysEqual; }
    main_unit_structs_call();

    #[derive(Debug)] struct Rectangle { width:u32, height:u32 }
    impl Rectangle { fn area(&self)->u32{self.width*self.height} fn width_is_positive(&self)->bool{self.width>0} fn set_width(&mut self, w:u32){self.width=w;} fn new_constructor(w:u32,h:u32)->Self{Self{width:w,height:h}} fn square_constructor(s:u32)->Self{Self{width:s,height:s}}}
    fn main_methods_call(){ let r1=Rectangle{width:1,height:1}; let _a=r1.area(); }
    main_methods_call();
    fn main_associated_functions_call(){ let _r = Rectangle::new_constructor(1,1); }
    main_associated_functions_call();

    #[derive(Debug)] enum IpAddrKind {V4,V6} fn route_ip(_:IpAddrKind){} fn main_enum_definition_call(){ route_ip(IpAddrKind::V4); }
    main_enum_definition_call();

    #[derive(Debug)] enum IpAddrData { V4(u8,u8,u8,u8), V6(String) }
    fn main_enum_with_data_call(){ let _h=IpAddrData::V4(1,1,1,1); }
    main_enum_with_data_call();

    #[derive(Debug)] enum Message { Write(String), Move{x:i32,y:i32}, ComplexData(Box<CustomPayloadForMessage>) } #[derive(Debug)] struct CustomPayloadForMessage{id:u32,data:Vec<u8>}
    impl Message { fn process_message(&self){match self { Message::Write(t) => println!("{}",t), Message::Move{x,y} => println!("{},{}",x,y), Message::ComplexData(d) => println!("{:?}",d)  }}}
    fn main_message_enum_call(){ let m=Message::Write(String::from("h")); m.process_message(); }
    main_message_enum_call();

    fn main_option_enum_call(){ let _s = Some(5); let _n:Option<i32>=None; }
    main_option_enum_call();

    enum CoinForMatch { Penny, Nickel, Dime, Quarter } fn value_in_cents_for_match(_c: CoinForMatch) -> u8 {1} fn plus_one_option(x:Option<i32>)->Option<i32>{match x{Some(i)=>Some(i+1),None=>None}}
    fn main_match_option_usage_call(){ let _s=plus_one_option(Some(1));}
    main_match_option_usage_call();

    fn main_match_wildcard_call(){ match 9 { 1=>(), other=>println!("{}",other),};}
    main_match_wildcard_call();

    fn main_if_let_deep_dive_call(){ let opt:Option<u8>=Some(1); if let Some(m)=opt {println!("{}",m)} else {println!("n");}}
    main_if_let_deep_dive_call();
}

// Dummy struct and enum for main_match_example in Markdown, which is self-contained there.
// enum Coin { Penny, Nickel, Dime, Quarter(UsState) }
// #[derive(Debug)] enum UsState { Alabama, Alaska }
// fn value_in_cents(coin: Coin) -> u8 { match coin { Coin::Penny=>1, Coin::Nickel=>5, Coin::Dime=>10, Coin::Quarter(state)=>{println!("{:?}",state); 25}}}
// fn main_match_example() { value_in_cents(Coin::Quarter(UsState::Alaska)); }
```
第五章 `README.md` 已更新并包含以上面试题及其详细解释。