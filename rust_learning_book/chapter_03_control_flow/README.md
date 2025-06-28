# 第 3 章：控制流

控制流允许我们根据条件来决定是否执行某些代码，或者重复执行某些代码。Rust 中的主要控制流结构是 `if` 表达式和多种循环结构 (`loop`, `while`, `for`)，以及强大的 `match` 表达式。

## 3.1 `if` 表达式

`if` 表达式允许你根据条件来分支代码。

```rust
fn main() {
    let number = 7;

    if number < 5 {
        println!("条件为 true");
    } else {
        println!("条件为 false");
    }

    // 条件必须是 bool 类型
    // let another_number = 3;
    // if another_number { // 错误！期望 bool 类型，得到整数
    //     println!("number was three");
    // }
}
```

**`else if` 处理多个条件**

你可以通过组合 `if` 和 `else` 来使用 `else if` 处理多个互斥的条件。

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("数字能被 4 整除");
    } else if number % 3 == 0 {
        println!("数字能被 3 整除");
    } else if number % 2 == 0 {
        println!("数字能被 2 整除");
    } else {
        println!("数字不能被 4, 3, 或 2 整除");
    }
}
```
如果使用了太多的 `else if`，代码可能会显得杂乱。对于这种情况，Rust 提供了强大的 `match` 表达式，我们稍后会介绍。

**在 `let` 语句中使用 `if`**

因为 `if` 是一个表达式，所以我们可以在 `let` 语句的右侧使用它来赋值。
`if` 的每个分支的返回值类型必须相同。

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" }; // 错误！`if` 和 `else` 分支的值类型必须相同

    println!("The value of number is: {}", number); // 输出 5
}
```

## 3.2 循环 (`loop`, `while`, `for`)

Rust 提供了几种循环结构，用于重复执行代码块。

### 3.2.1 `loop`：无限循环与 `break`

`loop` 关键字告诉 Rust 一遍又一遍地执行一个代码块，直到你显式地告诉它停止。
可以使用 `break` 关键字退出循环。`break` 还可以从循环中返回值。

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("再次执行! counter = {}", counter);

        if counter == 10 {
            break counter * 2; // 退出循环，并返回 counter * 2 的值
        }
    };

    println!("循环结束后的结果是: {}", result); // 输出 20
}
```

**循环标签 (Loop Labels)**

如果存在嵌套循环，`break` 和 `continue` 应用于此时最内层的循环。你可以选择为一个循环指定一个**循环标签（loop label）**，然后将标签与 `break` 或 `continue` 结合使用，使这些关键字应用于已标记的循环而不是最内层的循环。循环标签必须以单引号开头。

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop { // 给外部循环加上标签 'counting_up
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break; // 这个 break 应用于内层循环
            }
            if count == 2 {
                break 'counting_up; // 这个 break 应用于标签为 'counting_up' 的外层循环
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count); // 输出 End count = 2
}
```

### 3.2.2 `while`：条件循环

当条件为 `true` 时，`while` 循环会一直执行其代码块。

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("发射！");
}
```
这种结构消除了 `loop`、`if`、`else` 和 `break` 可能需要的大量嵌套，因此更清晰。

### 3.2.3 `for`：遍历集合

`for` 循环用于遍历一个集合中的每个元素，例如数组或范围 (Range)。

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    // 使用 while 循环遍历数组 (易出错)
    let mut index = 0;
    while index < 5 { // 如果数组长度改变，这里可能出错
        println!("while - a[{}] 的值是: {}", index, a[index]);
        index += 1;
    }
    println!();

    // 使用 for 循环遍历数组 (更安全、更简洁)
    for element in a.iter() { // iter() 方法返回数组的迭代器
        println!("for - 元素的值是: {}", element);
    }
    println!();

    // 直接遍历数组 (Rust 1.53.0 及更高版本支持直接在数组上迭代)
    for element in a {
         println!("for (direct) - 元素的值是: {}", element);
    }
    println!();


    // 使用 for 循环和范围 (Range)
    // `(1..4)` 创建一个从 1 开始到 4 (不包括 4) 的范围
    // `.rev()` 方法反转这个范围
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("发射！ (使用 for 和 range)");
}
```
`for` 循环通常是 Rust中最常用的循环结构，因为它更简洁且不易出错（例如，避免了索引越界的风险）。

## 3.3 `match` 表达式

`match` 允许你将一个值与一系列的模式（patterns）相比较，并根据匹配的模式执行相应的代码。模式可以由字面值、变量名、通配符以及许多其他东西构成。

`match` 表达式类似于其他语言中的 `switch` 语句，但功能更强大，并且 Rust 强制 `match` 是**穷尽的（exhaustive）**，意味着你必须覆盖所有可能的情况。

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("幸运的便士!");
            1 // 返回值
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    } // match 是一个表达式，其结果是匹配分支中最后表达式的值
}

fn main() {
    println!("一美分是 {} 美分", value_in_cents(Coin::Penny));
    println!("五美分是 {} 美分", value_in_cents(Coin::Nickel));

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("最大值被配置为 {}", max),
        None => (), // 如果 config_max 是 None，什么也不做
    }

    // 使用 if let 进行简洁的单分支匹配
    if let Some(max) = config_max {
        println!("(if let) 最大值被配置为 {}", max);
    } else {
        println!("(if let) 没有配置最大值");
    }
}
```

**`match` 的模式绑定**

`match` 的分支可以绑定到被匹配值的部分值。

```rust
#[derive(Debug)] // 派生 Debug trait 以便打印
enum UsState {
    Alabama,
    Alaska,
    // -- snip --
}

enum CoinWithState {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Quarter 变体包含一个 UsState 值
}

fn value_in_cents_with_state(coin: CoinWithState) -> u8 {
    match coin {
        CoinWithState::Penny => 1,
        CoinWithState::Nickel => 5,
        CoinWithState::Dime => 10,
        CoinWithState::Quarter(state) => { // 绑定 state 变量到 UsState 值
            println!("来自 {:?} 州的25美分硬币!", state);
            25
        }
    }
}

fn main() {
    value_in_cents_with_state(CoinWithState::Quarter(UsState::Alaska));
}
```

**`_` (下划线) 通配符**

`_` 是一个特殊的模式，它会匹配任何值并且不绑定到该值。它通常用作 `match` 表达式的最后一个分支，用于处理所有未明确列出的情况。

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // `other` 会绑定到 dice_roll 的值
        // _ => reroll(), // 如果不想使用这个值，可以用 _
        // _ => (), // 如果什么都不想做，可以用 _ 和 unit tuple
    }
    println!("掷骰子结果: {}", dice_roll);
}

fn add_fancy_hat() { println!("戴上漂亮的帽子!"); }
fn remove_fancy_hat() { println!("摘下漂亮的帽子!"); }
fn move_player(num_spaces: i32) { println!("玩家移动 {} 格", num_spaces); }
// fn reroll() { println!("重新掷骰子!"); }
```
如果省略了 `_` 并且没有覆盖所有可能性，Rust 会在编译时报错，因为 `match` 必须是穷尽的。

**`if let` 语法糖**

有时，`match` 表达式可能有些冗长，特别是当你只关心一个或少数几个模式，而忽略其他所有模式时。对于这种情况，可以使用 `if let`。

`if let` 语法让你能够以一种不那么冗长的方式结合 `if` 和 `let`，来处理只匹配一个模式的值而忽略其他模式的情况。

```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("使用你最喜欢的颜色, {}, 作为背景", color);
    } else if is_tuesday {
        println!("周二涂绿色!");
    } else if let Ok(age_val) = age { // age 是 Result<u8, ParseIntError>
        if age_val > 30 {
            println!("使用紫色作为背景色");
        } else {
            println!("使用橙色作为背景色");
        }
    } else { // 对应 favorite_color 是 None，不是周二，并且 age 解析失败
        println!("使用蓝色作为背景色");
    }

    // `while let` 循环
    // 当模式匹配时，`while let` 允许循环一直运行。
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() { // pop() 返回 Option<T>
        println!("{}", top); // 输出 3, 2, 1
    }
    // 循环结束后，stack 为空
}
```
`if let` 失去了 `match` 强制要求的穷尽性检查。选择 `match` 还是 `if let` 取决于具体情况以及你是否想要穷尽性检查。通常，如果只处理一两种情况，`if let` 更简洁；如果需要处理多种情况并确保所有情况都被考虑到，`match` 更合适。

## 3.4 常见陷阱

1.  **`if` 条件不是布尔值**：
    *   **陷阱**：与其他一些语言不同，Rust 不会自动尝试将非布尔类型转换为布尔类型。`if` 表达式的条件必须是 `bool` 类型。
        ```rust
        // let number = 3;
        // if number { ... } // 编译错误
        ```
    *   **避免**：确保 `if` 后的条件表达式求值结果为 `bool`。例如，使用 `if number != 0 { ... }`。

2.  **`if let` 和 `match` 分支的类型不一致 (在 `let` 语句中使用时)**：
    *   **陷阱**：当 `if` 作为表达式用于 `let` 赋值时，所有分支（`if` 块和 `else` 块）必须返回相同类型的值。
        ```rust
        // let condition = true;
        // let number = if condition { 5 } else { "six" }; // 编译错误
        ```
    *   **避免**：确保所有分支返回兼容的类型。如果需要返回不同类型，可能需要使用枚举或其他更复杂的结构。

3.  **无限循环没有 `break`**：
    *   **陷阱**：使用 `loop` 时，如果没有明确的 `break` 条件或 `break` 条件永远不满足，会导致无限循环，程序卡死。
    *   **避免**：确保 `loop` 内部有逻辑可以触发 `break` 语句，或者该无限循环是程序设计中有意为之的（例如服务器监听循环）。

4.  **`for` 循环中索引越界 (如果手动管理索引)**：
    *   **陷阱**：虽然 `for element in collection` 的形式很安全，但如果仍旧使用类似 C 风格的索引方式（例如通过 `while` 和手动递增的索引变量），很容易出现索引越界导致 panic。
        ```rust
        // let a = [1, 2, 3];
        // let mut i = 0;
        // while i <= a.len() { // 错误：应该是 i < a.len()
        //     println!("{}", a[i]);
        //     i += 1;
        // }
        ```
    *   **避免**：优先使用 `for ... in ...` 迭代器语法。如果确实需要索引，可以使用 `for (index, value) in collection.iter().enumerate()`。

5.  **`match` 表达式非穷尽**：
    *   **陷阱**：`match` 表达式必须覆盖所有可能的值。如果遗漏了某些情况，并且没有使用 `_` 通配符来处理其余情况，编译器会报错。
        ```rust
        // enum MyEnum { A, B, C }
        // fn check(val: MyEnum) {
        //     match val {
        //         MyEnum::A => println!("A"),
        //         MyEnum::B => println!("B"),
        //         // 编译错误：pattern `C` not covered
        //     }
        // }
        ```
    *   **避免**：确保 `match` 的所有分支覆盖了输入类型的所有可能性。使用 `_` 通配符作为最后一个分支来捕获所有其他情况，或者如果确定不需要处理，则使用 `_ => ()`。对于 `Option` 和 `Result` 等枚举，确保处理了 `None`/`Some` 或 `Ok`/`Err` 的所有变体。

6.  **在 `match` 分支中忘记返回值或返回了错误的类型**：
    *   **陷阱**：如果 `match` 表达式被用于赋值或作为函数返回值，那么所有分支必须返回相同类型的值。分支中的代码块如果以分号结尾，则其值为 `()`。
        ```rust
        // fn get_val(input: Option<i32>) -> i32 {
        //     match input {
        //         Some(x) => x,
        //         None => {
        //             println!("No value!");
        //             // 编译错误：期望 i32，得到 ()。需要一个 i32 类型的默认值。
        //             // 0; // 应该这样
        //         }
        //     }
        // }
        ```
    *   **避免**：仔细检查每个 `match` 分支的最后一个表达式，确保它返回期望的类型并且没有多余的分号（除非你想返回 `()`）。

## 3.5 常见面试题

1.  **Q: Rust 中的 `if` 和其他一些语言（如 Python 或 JavaScript）中的 `if` 有什么主要区别？**
    *   **A:**
        1.  **表达式 vs 语句**：在 Rust 中，`if` 是一个表达式，意味着它可以返回值。这允许你在 `let` 语句中使用 `if`，例如 `let x = if condition { val1 } else { val2 };`。在许多其他语言中，`if` 主要是语句，不直接返回值（或者说返回的是 `undefined` 或类似的东西）。
        2.  **条件类型**：Rust 中的 `if` 条件必须是一个 `bool` 类型的值。它不会像 Python 或 JavaScript 那样自动将 "truthy" 或 "falsy" 值（如非零数字、非空字符串）转换成布尔值。你必须显式地进行比较，例如 `if number != 0 { ... }`。
        3.  **分支类型一致性**：当 `if` 作为表达式使用时，其所有分支（`if` 块和 `else` 块）必须返回相同类型的值。

2.  **Q: 什么时候应该使用 `loop`，什么时候应该使用 `while` 或 `for`？**
    *   **A:**
        *   **`loop`**：
            *   当你需要一个**无限循环**，并且退出条件在循环体内部通过 `break` 来控制时，使用 `loop`。
            *   当你需要从循环中**返回值**时，`loop` 结合 `break value;` 非常有用。
            *   例如，重试操作直到成功，或者某些事件循环。
        *   **`while condition`**：
            *   当你需要在每次迭代**之前检查一个条件**，并且只要条件为真就继续循环时，使用 `while`。
            *   如果循环开始时条件就为假，则循环体一次也不会执行。
            *   适用于循环次数不确定，但依赖于某个状态的场景。
        *   **`for item in collection`**：
            *   当你需要**遍历一个集合**（如数组、Vector、切片、Range 或任何实现了 `IntoIterator` trait 的类型）中的每个元素时，使用 `for`。
            *   这是最常用且推荐的循环方式，因为它更简洁、更安全（避免了手动索引可能导致的越界错误）。
            *   适用于已知迭代次数或需要对集合中每个项执行操作的场景。

3.  **Q: 解释 Rust 中 `match` 表达式的穷尽性 (exhaustiveness) 是什么意思，为什么它很重要？**
    *   **A:**
        *   **穷尽性**：`match` 表达式的穷尽性意味着 `match` 的所有分支必须覆盖被匹配值的所有可能性。换句话说，你必须为输入类型的每一个可能的值都提供一个处理分支。如果存在未被覆盖的情况，Rust 编译器会报错。
        *   **重要性**：
            1.  **代码健壮性**：穷尽性检查确保你不会意外地遗漏某些情况。这在处理枚举（如 `Option` 或 `Result`）或自定义枚举时尤其重要，因为它可以防止因未处理某个变体而导致的运行时错误或逻辑缺陷。
            2.  **编译时保证**：Rust 将这种检查放在编译时，而不是运行时。这意味着在程序运行之前，你就能发现并修复这些潜在的问题，从而减少了生产环境中出现 bug 的可能性。
            3.  **代码清晰和可维护性**：它迫使开发者思考所有可能的状态和情况。当你添加或修改枚举的变体时，编译器会提醒你更新所有相关的 `match` 表达式，这有助于保持代码的同步和正确性。
        *   可以使用 `_` (下划线) 通配符作为最后一个分支来匹配所有未被其他分支捕获的情况，从而满足穷尽性要求。

4.  **Q: `if let` 和 `match` 有什么区别？什么时候应该优先使用 `if let`？**
    *   **A:**
        *   **`match`**：
            *   允许你将一个值与多个模式进行比较，并为每个匹配的模式执行不同的代码块。
            *   强制进行穷尽性检查，必须覆盖所有可能的情况。
        *   **`if let pattern = expression { ... } else if let ... { ... } else { ... }`**：
            *   是一种语法糖，用于处理只关心一个或少数几个模式，而忽略其他所有模式的情况。
            *   它不强制穷尽性检查。如果 `if let` 的模式不匹配，则执行 `else` 块（如果存在），或者什么也不做。
        *   **区别总结**：
            *   **关注点**：`match` 用于处理多种可能的情况并确保覆盖所有情况；`if let` 用于简洁地处理单一或少数几种情况，忽略其他。
            *   **穷尽性**：`match` 强制穷尽性；`if let` 不强制。
            *   **语法复杂度**：对于单模式匹配，`if let` 通常比 `match` 更简洁。
        *   **何时优先使用 `if let`**：
            *   当你只对某个枚举变体的一个特定值感兴趣，而其他所有变体都可以被忽略或用一个简单的 `else` 块处理时。
            *   例如，当处理 `Option<T>` 时，你可能只关心 `Some(value)` 的情况，而对 `None` 的情况执行一个简单的默认操作或什么都不做。
                ```rust
                let opt_val: Option<i32> = Some(5);
                if let Some(val) = opt_val {
                    println!("Value is {}", val);
                } // 如果是 None，则什么也不做

                // 使用 match 会稍微冗长一些：
                // match opt_val {
                //     Some(val) => println!("Value is {}", val),
                //     None => (), // 必须处理 None
                // }
                ```
            *   当你想在一系列 `if/else if` 条件中混合模式匹配时。

5.  **Q: 如何从 `loop` 循环中返回值？**
    *   **A:**
        你可以通过在 `break` 关键字后面跟上一个表达式来从 `loop` 循环中返回值。这个表达式的值将成为整个 `loop` 表达式的值。
        ```rust
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2; // 当 counter 等于 10 时，退出循环并返回 counter * 2 (即 20)
            }
        };
        // result 的值将是 20
        println!("Result: {}", result);
        ```
        这对于需要反复尝试某个操作直到成功并获取结果的场景非常有用。只有 `loop` 支持这种通过 `break` 返回值的行为；`while` 和 `for` 循环不直接支持这种方式（它们本身的值是 `()`）。

现在，我将为本章创建一个示例 Cargo 项目。
