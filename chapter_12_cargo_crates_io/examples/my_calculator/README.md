# My Calculator Crate

这是一个简单的计算器库，用于演示 Rust 的 Cargo 包管理器和 Crates.io 发布流程。

## 功能

*   基本的整数加、减、乘、除运算。
*   可选的 "advanced_math" 特性，启用后可以使用 `num-bigint` crate 进行大数加法。

## 如何使用

将此 crate 添加到你的 `Cargo.toml` 文件的 `[dependencies]` 部分：

```toml
[dependencies]
my_calculator = "0.1.0" # 或者你发布的版本
```

如果需要大数运算功能，启用 `advanced_math` 特性：

```toml
[dependencies]
my_calculator = { version = "0.1.0", features = ["advanced_math"] }
```

### 示例代码

```rust
use my_calculator::{add, divide};

fn main() {
    println!("2 + 3 = {}", add(2, 3));

    match divide(10, 2) {
        Some(result) => println!("10 / 2 = {}", result),
        None => println!("10 / 0 = 不能除以零!"),
    }

    // 如果启用了 "advanced_math" 特性
    #[cfg(feature = "advanced_math_in_example")] // 假设在用户项目中也用特性控制
    {
        use my_calculator::add_big_numbers;
        match add_big_numbers("100000000000000000000", "200000000000000000000") {
            Ok(sum) => println!("大数相加结果: {}", sum),
            Err(e) => println!("大数相加错误: {}", e),
        }
    }
}
```

## 构建和测试

*   构建: `cargo build`
*   运行测试: `cargo test`
*   构建并运行 (如果项目有 main.rs): `cargo run`
*   构建发布版本: `cargo build --release`
*   测试并启用特性: `cargo test --features "advanced_math"`
*   构建并启用特性: `cargo build --features "advanced_math"`

## 发布到 Crates.io (步骤概览)

1.  确保 `Cargo.toml` 中的元数据（`description`, `license` 等）完整且正确。
2.  登录到 Crates.io: `cargo login <your_api_token>`
3.  (可选) 打包检查: `cargo package`
4.  发布: `cargo publish`

## 许可证

本 crate 使用 `MIT OR Apache-2.0` 双重许可证。
详见 `LICENSE-MIT` 和 `LICENSE-APACHE` 文件 (如果这些文件实际存在于项目中)。
