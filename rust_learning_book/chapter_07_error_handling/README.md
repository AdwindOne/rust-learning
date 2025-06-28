# 第 7 章：错误处理

Rust 对可靠性的承诺延伸到了错误处理。错误是软件开发中不可避免的一部分，Rust 提供了多种处理错误的功能。Rust 将错误分为两大类：**不可恢复的 (unrecoverable)** 和 **可恢复的 (recoverable)** 错误。

*   **不可恢复错误**：通常是 bug，比如访问数组末尾之后的元素。Rust 使用 `panic!` 宏来处理这类错误。
*   **可恢复错误**：通常是那些应该被合理处理的情况，比如文件未找到。Rust 使用 `Result<T, E>` 枚举来处理这类错误。

## 7.1 不可恢复错误与 `panic!`

当 `panic!` 宏执行时，你的程序会打印一个失败信息，展开并清理调用栈，然后退出。默认情况下，当发生 panic 时，程序会开始**展开 (unwinding)**，这意味着 Rust 会回溯调用栈并清理它遇到的每个函数的数据。这个过程比较耗时。

另一种选择是立即**终止 (abort)**，这会不进行清理直接结束程序。程序占用的内存需要操作系统来清理。如果项目需要使最终的可执行文件尽可能小，可以把 panic 从展开切换为终止。在 `Cargo.toml` 文件的 `[profile]` 部分（例如 `[profile.release]`）添加 `panic = 'abort'`。

```rust
fn main() {
    // panic!("崩溃并退出！"); // 这行会立即导致 panic

    let v = vec![1, 2, 3];
    // v[99]; // 访问越界索引，这会 panic
             // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99'
}
```
通常，你应该在代码中那些表示程序处于无法处理的状态（例如，逻辑错误、违反了代码假设或契约）的地方使用 `panic!`。

**`panic!` 的回溯信息 (Backtrace)**

当 `panic!` 发生时，你可以通过设置 `RUST_BACKTRACE` 环境变量来获取一个回溯信息，它会显示导致错误的函数调用顺序。
例如，在 shell 中运行： `RUST_BACKTRACE=1 cargo run`。
回溯信息对于调试 panic 的原因非常有用。

## 7.2 可恢复错误与 `Result<T, E>`

大多数错误并非严重到需要程序完全停止。有时函数会因为一些可以被轻松理解和响应的原因而失败。例如，如果尝试打开一个文件而操作失败，可能是因为它不存在，或者你没有访问权限。这些情况是可预期的，程序应该能够处理它们。

Rust 使用 `Result<T, E>` 枚举来处理这类可恢复错误。
`Result` 枚举定义如下：
```rust
// enum Result<T, E> {
//     Ok(T),    // 包含成功时产生的值 T
//     Err(E),   // 包含失败时产生的错误 E
// }
```
`T` 代表成功时 `Ok` 成员里返回值的类型，`E` 代表失败时 `Err` 成员里返回的错误的类型。

### 7.2.1 使用 `match` 处理 `Result`

当函数返回 `Result` 时，调用者需要检查其结果是 `Ok` 还是 `Err`，并相应地处理。

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f_result = File::open("hello.txt"); // File::open 返回 Result<std::fs::File, std::io::Error>

    let f = match f_result {
        Ok(file) => {
            println!("文件打开成功！");
            file // f 将是 File 类型
        }
        Err(error) => match error.kind() { // error.kind() 返回一个 ErrorKind 枚举
            ErrorKind::NotFound => match File::create("hello.txt") { // 文件不存在，尝试创建
                Ok(fc) => {
                    println!("文件不存在，已成功创建！");
                    fc
                }
                Err(e) => panic!("尝试创建文件失败: {:?}", e),
            },
            other_error => { // 其他类型的错误
                panic!("打开文件时遇到问题: {:?}", other_error);
            }
        },
    };
    // 使用 f (类型是 std::fs::File)
    println!("文件句柄: {:?}", f);
}
```
这种嵌套的 `match` 表达式可能有点冗长。

### 7.2.2 `Result` 的快捷方式：`unwrap` 和 `expect`

`Result<T, E>` 类型有很多辅助方法来处理各种情况。
*   **`unwrap()`**：
    *   如果 `Result` 值是 `Ok` 成员，`unwrap` 会返回 `Ok` 内部的值。
    *   如果 `Result` 是 `Err` 成员，`unwrap` 会调用 `panic!` 宏。
    ```rust
    // let f = File::open("non_existent.txt").unwrap(); // 如果文件不存在，会 panic
    ```
*   **`expect(message: &str)`**：
    *   与 `unwrap` 类似，但允许你选择 `panic!` 的错误信息。
    ```rust
    // let f = File::open("another_non_existent.txt")
    //     .expect("打开 another_non_existent.txt 失败"); // panic 时显示此消息
    ```
`unwrap` 和 `expect` 在原型设计或示例代码中很方便，但在生产代码中应谨慎使用。如果你明确知道某个操作不应该失败，或者失败就意味着程序无法继续，那么使用它们是可以接受的。否则，最好使用 `match` 或其他更健壮的错误处理方式。

### 7.2.3 传播错误 (Propagating Errors)

当你编写一个其实现会调用一些可能会失败的操作的函数时，除了在函数内处理错误外，你还可以选择将错误返回给调用者，让调用者决定如何处理。这称为**传播错误 (propagating the error)**。

```rust
use std::io;
use std::io::Read;
use std::fs::File;

// 函数返回 Result<String, io::Error>
// 如果成功，返回包含文件内容的 String (Ok)
// 如果失败，返回 io::Error (Err)
fn read_username_from_file_long_way() -> Result<String, io::Error> {
    let f_result = File::open("username.txt");

    let mut f = match f_result {
        Ok(file) => file,
        Err(e) => return Err(e), // 如果打开文件失败，将错误返回给调用者
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) { // read_to_string 也返回 Result
        Ok(_) => Ok(s), // 如果读取成功，返回包含文件内容的 Ok(String)
        Err(e) => Err(e), // 如果读取失败，将错误返回给调用者
    }
}

fn main() {
    match read_username_from_file_long_way() {
        Ok(username) => println!("从文件读取的用户名: {}", username),
        Err(e) => println!("读取用户名失败: {:?}", e),
    }
}
```

### 7.2.4 `?` 运算符：传播错误的快捷方式

传播错误是一种常见的模式，Rust 提供了 `?` 运算符来使其更简洁。
如果 `Result` 的值是 `Ok`，`?` 运算符会从 `Ok` 中取出内部值，表达式将得到这个内部值。
如果值是 `Err`，`?` 运算符会使整个函数提前返回，并将该 `Err` 值作为函数的返回值。

`?` 运算符只能用于返回 `Result`（或 `Option`，或实现了 `Try` trait 的其他类型）的函数中。

```rust
use std::io;
use std::io::Read;
use std::fs::File;

// 使用 ? 运算符重写 read_username_from_file
fn read_username_from_file_short_way() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?; // 如果 File::open 失败，? 会返回 Err(e)
                                           // 否则，f 会得到 Ok 中的 File 句柄
    let mut s = String::new();
    f.read_to_string(&mut s)?; // 如果 read_to_string 失败，? 会返回 Err(e)
                               // 否则，s 会包含文件内容
    Ok(s) // 如果一切顺利，返回 Ok(s)
}

// 还可以链式调用
fn read_username_from_file_even_shorter() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// 最短的方式 (标准库提供了 fs::read_to_string)
// use std::fs;
// fn read_username_from_file_shortest() -> Result<String, io::Error> {
//     fs::read_to_string("username.txt")
// }

fn main() {
    // 创建一个 username.txt 文件用于测试
    // std::fs::write("username.txt", "Alice").expect("无法写入 username.txt");

    match read_username_from_file_short_way() {
        Ok(username) => println!("(short_way) 用户名: {}", username),
        Err(e) => println!("(short_way) 错误: {:?}", e),
    }

    match read_username_from_file_even_shorter() {
        Ok(username) => println!("(even_shorter) 用户名: {}", username),
        Err(e) => println!("(even_shorter) 错误: {:?}", e),
    }
}
```
`?` 运算符极大地简化了错误传播的代码。`?` 运算符会自动调用 `from` 函数，该函数定义在标准库的 `From` trait 中，用于在不同的错误类型之间进行转换。只要某个错误类型实现了 `From<OtherError>` trait，`?` 运算符就能在返回时自动将 `OtherError` 转换为该错误类型。

**`?` 运算符与 `Option<T>`**

`?` 运算符也可以用于 `Option<T>` 类型的值。如果 `Option` 是 `Some(value)`，它会返回 `value`；如果是 `None`，它会使函数提前返回 `None`。
使用 `?` 的函数必须返回 `Option` 类型。

```rust
fn first_char_of_first_word(text: &str) -> Option<char> {
    // text.split_whitespace().next() 返回 Option<&str>
    // .chars().next() 返回 Option<char>
    let first_word = text.split_whitespace().next()?; // 如果没有第一个词 (空字符串或只有空格)，返回 None
    first_word.chars().next() // 返回第一个词的第一个字符的 Option<char>
}

fn main_option() {
    let text1 = "hello world";
    let text2 = "  "; // 没有词
    let text3 = "";   // 没有词

    println!("First char of '{}': {:?}", text1, first_char_of_first_word(text1)); // Some('h')
    println!("First char of '{}': {:?}", text2, first_char_of_first_word(text2)); // None
    println!("First char of '{}': {:?}", text3, first_char_of_first_word(text3)); // None
}
```

## 7.3 自定义错误类型

虽然 `std::io::Error` 或其他标准库错误类型在很多情况下都适用，但有时你可能想定义自己的错误类型，以便更精确地表达错误情况或组合多种不同来源的错误。

一个好的自定义错误类型通常应该：
1.  实现 `std::fmt::Debug` trait，以便可以打印调试信息。
2.  实现 `std::fmt::Display` trait，以便为用户提供友好的错误信息。
3.  实现 `std::error::Error` trait，这是一个标记 trait，表明该类型用于错误处理。它还允许你可选地提供错误的“来源 (source)”（即导致此错误的另一个底层错误）。

```rust
use std::fmt;
use std::fs::File;
use std::io;

// 1. 定义我们的自定义错误类型
#[derive(Debug)]
enum MyError {
    Io(io::Error), // 可以包含其他错误类型
    Parse(String), // 自定义解析错误
    NotFound,
}

// 2. 为自定义错误类型实现 Display trait (用户友好的输出)
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::Io(err) => write!(f, "IO Error: {}", err),
            MyError::Parse(msg) => write!(f, "Parse Error: {}", msg),
            MyError::NotFound => write!(f, "Resource not found"),
        }
    }
}

// 3. 为自定义错误类型实现 Error trait
impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MyError::Io(err) => Some(err), // 提供底层 IO 错误作为 source
            MyError::Parse(_) => None,
            MyError::NotFound => None,
        }
    }
}

// (可选) 实现 From<io::Error> for MyError，以便 ? 运算符可以自动转换
impl From<io::Error> for MyError {
    fn from(err: io::Error) -> MyError {
        MyError::Io(err)
    }
}

// 示例函数，返回自定义错误类型
fn process_file(filename: &str) -> Result<String, MyError> {
    let mut file = File::open(filename)?; // ? 会调用 MyError::from(io::Error)
                                       // 如果 File::open 返回 Err(io_error)
                                       // 则 process_file 会返回 Err(MyError::Io(io_error))

    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // 同样适用 ?

    if contents.is_empty() {
        // 假设空文件是一个我们想特定处理的解析错误
        return Err(MyError::Parse("File is empty, cannot process".to_string()));
    }

    Ok(contents)
}

fn main_custom_error() {
    // std::fs::write("data.txt", "Some data").unwrap();
    // std::fs::write("empty.txt", "").unwrap();

    match process_file("data.txt") {
        Ok(data) => println!("Processed data.txt successfully: {}", data),
        Err(e) => println!("Error processing data.txt: {}", e), // 使用 Display
    }

    match process_file("empty.txt") {
        Ok(data) => println!("Processed empty.txt successfully: {}", data),
        Err(e) => {
            println!("Error processing empty.txt: {}", e); // 使用 Display
            if let Some(source) = e.source() {
                println!("  Caused by: {}", source);
            }
        }
    }

    match process_file("nonexistent.txt") {
        Ok(data) => println!("Processed nonexistent.txt successfully: {}", data),
        Err(e) => {
            println!("Error processing nonexistent.txt: {}", e); // 使用 Display
            if let Some(source) = e.source() { // 这里会打印底层的 io::Error
                println!("  Caused by: {}", source);
            }
        }
    }
}
```
创建自定义错误类型可以使你的库或应用程序的错误处理更加清晰和模块化。一些流行的错误处理库如 `thiserror` 和 `anyhow` 可以进一步简化自定义错误类型的创建和管理。

## 7.4 何时使用 `panic!` vs `Result`

决定何时使用 `panic!` 何时使用 `Result` 可能不总是那么清晰。指导原则是：

*   **使用 `panic!` 的情况**：
    *   当代码遇到某种**不应该发生**的状态，并且无法从中恢复时。这通常表明程序逻辑中存在 bug。
    *   在示例代码、原型或测试中，你希望在遇到错误时立即停止程序。`unwrap()` 和 `expect()` 在这些情况下很方便。
    *   当你调用的外部代码返回一个无效状态，而你无法修复它时。
    *   当违反了代码的基本假设或契约时。例如，如果一个函数期望输入一个非空列表，但收到了一个空列表，而这个条件在类型系统中无法静态保证，那么 panic 可能是一个合理的选择。

*   **使用 `Result` 的情况**：
    *   当错误是**预期可能发生**的，并且调用者应该能够决定如何响应时。例如，文件未找到、网络连接失败、用户输入无效等。
    *   当你希望给调用者一个机会来恢复或以特定方式处理错误时。
    *   作为库的公共 API，通常应该返回 `Result`，让库的使用者来决定如何处理错误，而不是直接 panic。

总的来说，如果一个函数可能会以一种预期的方式失败，它应该返回 `Result<T, E>`。如果失败意味着程序处于一种它无法处理或不应该处理的状态（例如，违反了不变量），那么 `panic!` 更合适。

## 7.5 常见陷阱

1.  **过度使用 `unwrap()` 或 `expect()`**：
    *   **陷阱**：在生产代码中不加区分地使用 `unwrap()` 或 `expect()`，当 `Result` 或 `Option` 为 `Err`/`None` 时会导致程序 panic。这可能隐藏了本应被优雅处理的错误。
    *   **避免**：只在以下情况使用 `unwrap`/`expect`：
        *   你绝对确定值不可能是 `Err`/`None`（例如，基于程序逻辑）。
        *   在示例、测试或快速原型中，panic 是可接受的行为。
        *   当 `Err`/`None` 确实代表一个不可恢复的程序 bug 时。
        *   在其他情况下，使用 `match`、`if let`、`?` 运算符或 `Result`/`Option` 的组合方法（如 `unwrap_or`, `map_err`）进行更健壮的错误处理。

2.  **忽略 `Result` 值**：
    *   **陷阱**：调用一个返回 `Result` 的函数，但不检查其返回值。Rust 编译器通常会对此发出警告 (`#[must_use]`)，但不处理 `Result` 可能会导致未被发现的错误。
        ```rust
        // use std::fs::remove_file;
        // remove_file("some_file.txt"); // 返回 Result<(), io::Error>，但被忽略了
        ```
    *   **避免**：始终处理 `Result` 返回值，即使只是简单地 `.expect("...")` 或记录错误。

3.  **`?` 运算符用在不返回 `Result` 或 `Option` 的函数中**：
    *   **陷阱**：`?` 运算符只能用在签名返回 `Result<T, E>`（其中 E 可以从 `?` 应用的错误类型转换而来）或 `Option<T>` 的函数中。在返回其他类型的函数（如 `()` 或 `String`）中使用 `?` 会导致编译错误。
        ```rust
        // fn my_func() -> String { // 假设返回 String
        //     let data = std::fs::read_to_string("file.txt")?; // 编译错误
        //     data
        // }
        ```
    *   **避免**：确保使用 `?` 的函数返回兼容的 `Result` 或 `Option` 类型。如果函数本身不应该传播错误，则在内部处理 `Result`（例如，使用 `match` 或 `unwrap_or_default`）。

4.  **错误类型不匹配与 `?` 运算符**：
    *   **陷阱**：当函数返回 `Result<T, MyError>`，而内部调用的函数返回 `Result<U, OtherError>` 时，如果 `MyError` 没有实现 `From<OtherError>`，则 `?` 运算符无法自动转换错误类型，导致编译错误。
    *   **避免**：
        *   为你的自定义错误类型实现 `From<OtherError>` trait。
        *   使用 `map_err` 方法手动转换错误类型：`another_fn().map_err(MyError::from_other_error)?`。
        *   使用像 `anyhow` 或 `thiserror` 这样的库来帮助管理不同的错误类型。

5.  **在 `main` 函数中直接使用 `?` (Rust 2018 之前)**：
    *   **陷阱**：在 Rust 2018 版本之前，`main` 函数不能直接返回 `Result`，因此不能在 `main` 中直接使用 `?` 运算符来传播错误。
    *   **避免 (Rust 2018 及之后)**：`main` 函数现在可以返回 `Result<(), E>` (其中 `E` 实现了 `std::error::Error`)，允许在 `main` 中使用 `?`。
        ```rust
        // // Rust 2018 及更高版本
        // use std::error::Error;
        // fn main() -> Result<(), Box<dyn Error>> {
        //     let content = std::fs::read_to_string("notes.txt")?;
        //     println!("File content: {}", content);
        //     Ok(())
        // }
        ```
    *   **避免 (Rust 2015)**：需要一个包装函数或在 `main` 内部使用 `match` 等方式处理 `Result`。

## 7.6 常见面试题

1.  **Q: Rust 中处理错误的两种主要方式是什么？它们分别适用于什么场景？**
    *   **A:**
        1.  **不可恢复错误 (Unrecoverable Errors) - `panic!` 宏**：
            *   当 `panic!` 执行时，程序默认会展开调用栈、清理数据然后退出（或者配置为立即终止）。
            *   **适用场景**：
                *   表示程序遇到了一个它无法恢复的 bug 或无效状态（例如，违反了代码的契约或不变量）。
                *   在示例、测试或原型代码中，快速失败是可接受的。
                *   当外部代码返回了无法修复的无效状态时。
                *   例如，访问数组越界 `vec[index_out_of_bounds]` 会 panic。
        2.  **可恢复错误 (Recoverable Errors) - `Result<T, E>` 枚举**：
            *   `Result<T, E>` 有两个成员：`Ok(T)` 表示操作成功并包含值 `T`，`Err(E)` 表示操作失败并包含错误信息 `E`。
            *   **适用场景**：
                *   用于处理那些预期可能发生并且调用者应该能够响应的错误（例如，文件未找到、网络请求失败、解析错误）。
                *   当函数可以将错误信息传递给调用者，让调用者决定如何处理时。
                *   作为库的公共 API，通常应该返回 `Result` 而不是 panic。

2.  **Q: 解释 `Result<T, E>` 枚举。如何处理一个返回 `Result` 的函数调用？**
    *   **A:**
        *   `Result<T, E>` 是一个标准库枚举，用于处理可能失败的操作。
            *   `Ok(T)`：表示操作成功，并包含成功的值，类型为 `T`。
            *   `Err(E)`：表示操作失败，并包含错误信息，类型为 `E`。
        *   **处理方式**：
            1.  **`match` 表达式**：最基本和最灵活的方式，可以分别处理 `Ok` 和 `Err` 的情况。
                ```rust
                // match my_function() {
                //     Ok(value) => println!("Success: {}", value),
                //     Err(error) => println!("Error: {:?}", error),
                // }
                ```
            2.  **`unwrap()` 方法**：如果 `Result` 是 `Ok`，返回 `Ok` 中的值；如果是 `Err`，则 `panic!`。应谨慎使用。
            3.  **`expect(message)` 方法**：类似 `unwrap()`，但在 `panic!` 时使用提供的 `message`。也应谨慎使用。
            4.  **`?` 运算符 (错误传播)**：如果用在返回 `Result` 的函数中，当遇到 `Err(e)` 时，它会使当前函数立即返回 `Err(e)`（可能需要类型转换）。如果遇到 `Ok(v)`，则表达式的值为 `v`。
            5.  **组合子方法**：`Result` 提供了很多有用的方法，如 `is_ok()`, `is_err()`, `ok()`, `err()`, `map()`, `map_err()`, `and_then()`, `unwrap_or()`, `unwrap_or_else()` 等，可以链式调用来处理或转换 `Result`。

3.  **Q: `?` 运算符的作用是什么？它用在什么上下文中？它对错误类型有什么要求？**
    *   **A:**
        *   **作用**：`?` 运算符用于简化错误传播。当应用于一个 `Result<T, E>` 值时：
            *   如果该值是 `Ok(v)`，则整个 `?` 表达式的结果是 `v`（即 `Ok` 中的值被提取出来）。
            *   如果该值是 `Err(e)`，则 `?` 会导致当前函数立即返回，并将 `Err(e)` 作为当前函数的返回值。
        *   **上下文**：`?` 运算符只能用在返回类型为 `Result<T', E'>` 或 `Option<T'>`（或其他实现了 `Try` trait 的类型）的函数中。
        *   **对错误类型要求**：
            *   当 `?` 应用于 `Result<_, E1>`，并且当前函数返回 `Result<_, E2>` 时，错误类型 `E1` 必须能够通过 `From` trait 转换为 `E2` (即 `E2: From<E1>`)。`?` 运算符会自动调用 `E1.into()` 来完成这个转换。
            *   这使得你可以从一个函数中传播多种不同类型的错误，只要它们都能被转换成函数签名中声明的那个统一的错误类型。

4.  **Q: 什么时候应该定义自定义错误类型？一个好的自定义错误类型通常需要实现哪些 trait？**
    *   **A:**
        *   **何时定义自定义错误类型**：
            *   当标准库错误类型（如 `std::io::Error`）不足以精确描述你的特定错误情况时。
            *   当你希望为你的库或应用程序的用户提供更具体、更有用的错误信息时。
            *   当你需要将来自不同来源的多种错误类型统一到一个错误类型中时（例如，一个操作可能因 IO 错误、解析错误或网络错误而失败）。
            *   当你希望错误类型能够携带额外的上下文信息时。
        *   **需要实现的 Trait**：
            1.  **`std::fmt::Debug`**：允许错误类型能够通过 `{:?}` 进行格式化打印，用于调试。通常可以通过 `#[derive(Debug)]` 自动实现。
            2.  **`std::fmt::Display`**：允许错误类型能够通过 `{}` 进行格式化打印，提供对用户更友好的错误信息。需要手动实现 `fmt` 方法。
            3.  **`std::error::Error`**：这是 Rust 中错误类型的标记 trait。它表明该类型是用于错误处理的。实现此 trait 还可以选择性地提供：
                *   `source()` 方法：返回导致此错误的底层错误（如果有的话），形成一个错误链。
            *   (可选，但推荐) **`From<OtherError>`**：为你的自定义错误类型实现 `From` trait，可以方便地将其他错误类型转换为你的自定义错误类型，这与 `?` 运算符的自动转换配合得很好。

5.  **Q: `main` 函数可以返回 `Result` 吗？如果可以，这样做有什么好处？**
    *   **A:**
        *   是的，从 Rust 2018 版本开始，`main` 函数可以返回 `Result<T, E>` 类型，通常是 `Result<(), E>`，其中 `E` 必须实现 `std::error::Error` trait (例如 `Box<dyn std::error::Error>`)。
        *   **好处**：
            1.  **简化错误处理**：允许在 `main` 函数中直接使用 `?` 运算符来传播错误。如果 `main` 函数中的某个操作返回 `Err`，并且使用了 `?`，那么 `main` 会提前退出，并将该错误打印到标准错误输出 (stderr)。
            2.  **更简洁的代码**：不再需要在 `main` 内部写大量的 `match` 或 `unwrap`/`expect` 来处理可能失败的操作。
            3.  **符合习惯**：使得 `main` 函数的错误处理方式与其他返回 `Result` 的函数更加一致。
        *   **示例**：
            ```rust
            // use std::error::Error;
            // use std::fs;
            //
            // fn main() -> Result<(), Box<dyn Error>> { // dyn Error 表示任何实现了 Error trait 的类型
            //     let contents = fs::read_to_string("my_file.txt")?; // 如果失败，main 返回 Err
            //     println!("File contents: {}", contents);
            //     Ok(()) // main 成功完成
            // }
            ```
            如果 `main` 返回 `Err`，程序的退出码通常会是非零值，表示发生了错误。

现在，我将为本章创建一个示例 Cargo 项目。
