// 这是 Rust 程序的主文件。
// Rust 程序通常以 `.rs` 结尾。

// `fn` 关键字用于声明一个新函数。
// `main` 函数是特殊的：它始终是每个可执行 Rust 程序中运行的第一个代码。
fn main() {
    // `println!` 是一个 Rust 宏。如果它是一个函数，则会写为 `println` (没有 `!`)。
    // 宏并不总是遵循与函数相同的规则。
    // 这个宏将文本打印到控制台。
    println!("Hello, world!"); // 打印 "Hello, world!"
    println!("你好，世界！"); // 同样可以打印中文

    println!("Hello blockchain!");
    println!("Hello Web3!");
    println!("Hello, Rust!");

    // 你可以使用 `cargo run` 命令来编译并运行这个程序。
    // 或者，先使用 `cargo build` 编译，
    // 然后在 `target/debug/` 目录下找到可执行文件并运行它。
    // 例如，在 Linux/macOS 上是 `./target/debug/hello_world_example`
    // 在 Windows 上是 `.\target\debug\hello_world_example.exe`
}

// 练习：
// 1. 尝试修改 `println!` 中的文本，然后重新运行程序。
// 2. 添加更多的 `println!` 语句来打印不同的消息。
// 3. 如果你直接使用 `rustc main.rs` 来编译（而不是 `cargo run`），
//    可执行文件会生成在当前目录（`src` 目录）。
//    尝试一下，然后运行它。注意，通常我们使用 `cargo` 来管理项目。
//    (需要先 cd 到 src 目录，或者执行 rustc src/main.rs)
//    (更推荐的方式是在项目根目录执行 cargo run 或 cargo build)
