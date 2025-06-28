# 第 11 章：测试

Rust 拥有一流的测试支持，测试是 Rust 语言的一等公民。编写测试是保证代码正确性、健壮性和易于维护的重要手段。Rust 的测试框架是内置的，无需额外安装。

本章主要内容：
*   如何编写单元测试。
*   如何编写集成测试。
*   如何编写文档测试。
*   测试的组织和运行。

## 11.1 如何编写测试

测试是 Rust 函数，它们用于验证非测试代码是否按照预期的方式运行。测试函数体通常执行三个操作：
1.  设置任何所需的数据或状态 (Arrange)。
2.  运行你想要测试的代码 (Act)。
3.  断言 (Assert) 结果是你所期望的。

### 11.1.1 测试函数剖析

测试函数需要使用 `#[test]` 属性（attribute）进行注解。属性是关于 Rust 代码片段的元数据。当使用 `cargo test` 命令运行测试时，Rust 会构建一个测试运行器二进制文件，该文件会运行标记有 `#[test]` 属性的函数，并报告其是成功还是失败。

```rust
// 在一个名为 `adder` 的库项目中，src/lib.rs 文件：
// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)] // 这个属性告诉 Rust 只在 cargo test 时编译和运行测试代码
// mod tests {
//     use super::*; // 导入外部模块 (即 adder 库本身) 的所有内容

//     #[test] // 标记这是一个测试函数
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4); // 断言 result 等于 4
//     }

//     #[test]
//     fn exploration() {
//         assert_eq!(add(2, 2), 4); // 测试我们自己的 add 函数
//     }

//     #[test]
//     fn another() {
//         // panic!("Make this test fail"); // 使用 panic! 来使测试失败
//     }
// }
```
*   `#[cfg(test)]`：这个配置属性指示 Rust 只在执行 `cargo test` 命令时才编译和运行 `mod tests` 模块中的代码。这可以节省编译时间（当只运行 `cargo build` 时）并减小最终二进制文件的大小。
*   `mod tests { ... }`：通常将测试代码放在一个名为 `tests` 的子模块中，这是一种社区约定。
*   `use super::*;`：将父模块（即库的主模块）中的所有项导入到 `tests` 模块的作用域中，这样就可以在测试中调用库函数了。
*   `#[test]`：这个属性将一个函数标记为测试函数。
*   **断言宏 (Assertion Macros)**：
    *   `assert!(expression)`：如果表达式求值为 `true` 则通过，否则 `panic!` (导致测试失败)。
    *   `assert_eq!(left, right)`：检查 `left` 和 `right` 是否相等。如果不想等，则 `panic!` 并打印它们的值。
    *   `assert_ne!(left, right)`：检查 `left` 和 `right` 是否不相等。如果相等，则 `panic!`。
    这些宏在测试失败时会打印有用的信息。

### 11.1.2 使用 `should_panic` 检查 Panic

除了检查代码是否返回正确的值，检查代码是否如预期那样 `panic` 也同样重要。我们可以为测试函数添加另一个属性 `#[should_panic]`，来断言代码会发生 panic。

```rust
// pub struct Guess {
//     value: i32,
// }

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("Guess value must be between 1 and 100, got {}.", value);
//         }
//         Guess { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     #[should_panic] // 这个测试期望发生 panic
//     fn greater_than_100() {
//         Guess::new(200);
//     }

//     #[test]
//     #[should_panic(expected = "Guess value must be between 1 and 100")] // 期望 panic 消息包含特定文本
//     fn greater_than_100_with_message() {
//         Guess::new(200);
//     }

//      #[test]
//      #[should_panic(expected = "less than or equal to 100")] // 也可以只匹配部分消息
//      fn greater_than_100_partial_message() {
//          Guess::new(200);
//      }
// }
```
`#[should_panic]` 属性可以带一个可选的 `expected` 参数，它会检查 panic 消息是否包含指定的文本。这使得 `should_panic` 测试更加精确。

### 11.1.3 使用 `Result<T, E>` 编写测试

测试函数也可以返回 `Result<T, E>` 类型。如果测试函数返回 `Ok(())`，则测试通过；如果返回 `Err(...)`，则测试失败。这允许你在测试函数中使用 `?` 运算符，使某些操作失败时能方便地使测试失败。

```rust
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works_with_result() -> Result<(), String> { // 返回 Result
//         if 2 + 2 == 4 {
//             Ok(()) // 测试通过
//         } else {
//             Err(String::from("two plus two does not equal four")) // 测试失败
//         }
//     }
// }
```
注意：不能对返回 `Result<T, E>` 的测试函数使用 `#[should_panic]` 属性。如果需要断言一个返回 `Result` 的操作会返回 `Err`，应该在测试函数中显式匹配 `Err` 值。

## 11.2 控制测试如何运行

`cargo test` 命令有一些命令行选项可以用来控制测试的运行方式。

*   **并行或串行运行测试**：
    默认情况下，测试是并行运行的，使用多个线程。这可以加快测试速度。
    如果你不希望测试并行运行（例如，测试依赖共享状态或顺序执行），可以使用 `--test-threads` 选项：
    `cargo test -- --test-threads=1` (注意两个 `--` 分隔符)
    这会将测试线程数设置为 1，使测试串行执行。

*   **显示函数输出**：
    默认情况下，如果测试通过，Rust 的测试库会捕获所有打印到标准输出的内容，不显示它们。如果测试失败，则会打印标准输出和标准错误输出。
    如果你希望即使在测试通过时也显示打印的输出，可以使用 `--show-output` 选项：
    `cargo test -- --show-output`

*   **通过名称运行部分测试**：
    可以向 `cargo test` 命令传递你想要运行的测试的名称（或名称的一部分）作为参数。
    `cargo test my_test_function_name`：只运行名为 `my_test_function_name` 的测试。
    `cargo test module_name::`：运行模块 `module_name` 中的所有测试。
    `cargo test part_of_name`：运行名称中包含 `part_of_name` 的所有测试。

*   **忽略某些测试 (`#[ignore]`)**：
    有时某些测试执行非常耗时，你可能希望在正常的 `cargo test` 运行中排除它们。可以使用 `#[ignore]` 属性标记这些测试：
    ```rust
    // #[test]
    // fn expensive_test() { /* ... */ }

    // #[test]
    // #[ignore] // 这个测试默认会被忽略
    // fn very_expensive_test() { /* ... */ }
    ```
    被标记为 `#[ignore]` 的测试默认不会运行。要专门运行这些被忽略的测试，使用：
    `cargo test -- --ignored`
    要运行所有测试（包括被忽略的），使用：
    `cargo test -- --include-ignored`

## 11.3 测试的组织

Rust 社区主要将测试分为两类：**单元测试 (unit tests)** 和 **集成测试 (integration tests)**。

### 11.3.1 单元测试 (Unit Tests)

*   **目的**：测试程序中较小的、隔离的部分，通常是单个模块或私有接口。
*   **位置**：通常与被测试的代码放在同一个文件（`src/main.rs` 或 `src/lib.rs`）的 `tests` 模块中。
*   **特点**：
    *   可以测试私有函数和接口（因为 `tests` 模块与被测试代码在同一作用域，可以访问私有项）。
    *   通常快速、小巧。

    ```rust
    // src/lib.rs
    // fn internal_adder(a: i32, b: i32) -> i32 { // 假设这是个私有函数
    //     a + b
    // }

    // #[cfg(test)]
    // mod tests {
    //     use super::*; // 访问父模块 (即库本身) 的项

    //     #[test]
    //     fn internal() {
    //         assert_eq!(4, internal_adder(2, 2)); // 可以测试私有函数
    //     }
    // }
    ```

### 11.3.2 集成测试 (Integration Tests)

*   **目的**：测试库的公共 API 是否按预期工作。它们像库的外部用户一样使用代码。
*   **位置**：放在项目根目录下的 `tests` 目录中（与 `src` 目录同级）。Cargo 会自动查找这个目录中的 `.rs` 文件并将其作为单独的 crate 来编译和运行测试。
    ```
    my_project/
    ├── Cargo.toml
    ├── src/
    │   └── lib.rs  (或 main.rs)
    └── tests/      <-- 集成测试目录
        └── integration_test.rs
        └── another_integration_test.rs
    ```
*   **特点**：
    *   每个 `tests` 目录下的 `.rs` 文件都会被编译成一个独立的测试 crate。
    *   只能调用被测试库的公共 API (因为它们是外部 crate)。
    *   通常比单元测试更全面，但可能更慢。
    *   如果项目是二进制 crate (只有 `src/main.rs`，没有 `src/lib.rs`)，则不能创建集成测试，因为它没有外部可链接的库。通常的做法是将二进制 crate 的主要逻辑提取到一个库 crate (`src/lib.rs`) 中，然后让 `src/main.rs` 调用该库，这样就可以为库编写集成测试了。

    ```rust
    // tests/integration_test.rs (假设 adder 是我们的库名)
    // use adder; // 导入我们的库 crate

    // #[test]
    // fn it_adds_two_integration() {
    //     assert_eq!(4, adder::add(2, 2)); // 调用公共 API
    // }

    // 如果需要共享代码，可以在 tests 目录下创建模块
    // tests/common.rs
    // pub fn setup() { /* ... */ }

    // tests/integration_test.rs
    // mod common; // 导入 common 模块
    // #[test]
    // fn test_with_setup() {
    //     common::setup();
    //     assert!(true);
    // }
    // 注意：`tests/common.rs` 本身不会被当作测试文件运行，因为它是一个模块。
    // 如果想让 `tests` 子目录中的文件也被当作测试文件，可以在 `tests` 目录下创建更深层次的目录结构，
    // 并在这些目录中放置 `.rs` 文件，例如 `tests/feature_a/test_suite.rs`。
    ```
    `tests` 目录下的子目录中的文件不会被自动识别为单独的测试 crate，除非它们是模块的根文件。通常，`tests` 目录下的每个 `.rs` 文件都是一个独立的测试入口点。

## 11.4 文档测试 (Doc Tests)

Rust 还支持在文档注释中编写代码示例，并且 `cargo test` 也会运行这些示例作为测试。这是一种极好的方式，既能提供清晰的文档，又能保证文档中的示例代码是正确且最新的。

文档测试主要用于库 crate，因为它们展示了如何使用库的公共 API。

*   **位置**：写在文档注释（以 `///` 或 `//!` 开头）中的代码块里。
*   **语法**：代码块以三个反引号 ``` 开始，可以指定语言（如 `rust`）。如果代码块中的代码应该 panic，可以在 ``` 后面加上 `should_panic`。如果代码块不应该被编译（例如，仅作说明），可以使用 `no_run`。如果希望隐藏某些设置代码但仍运行它，可以使用 `#` 开头的行（这些行不会显示在最终文档中，但会被编译和执行）。

```rust
// src/lib.rs
//! # Adder Crate
//!
//! `adder` 是一个提供加法功能的简单库。

/// 将两个 usize 数字相加。
///
/// # Examples
///
/// ```
/// use adder::add; // 假设 adder 是 crate 名
/// let result = add(2, 2);
/// assert_eq!(result, 4);
/// ```
///
/// ```rust // 明确指定语言
/// # use adder::add_five; // # 开头的行是隐藏的设置代码
/// assert_eq!(10, add_five(5));
/// ```
///
/// # Panics
/// 这个函数不会 panic。
///
/// ```should_panic
/// // use adder::bad_add;
/// // bad_add(1, 2); // 假设这个函数会 panic
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// pub fn add_five(num: usize) -> usize {
//     num + 5
// }
```
当运行 `cargo test` 时，它会查找所有文档注释中的代码块，将它们编译并作为测试运行。
文档测试有助于确保你的文档示例始终与代码保持同步。

## 11.5 总结

Rust 的内置测试框架提供了强大而灵活的方式来验证代码的正确性：
*   **单元测试**：用于测试模块内部的细节和私有接口，通常与代码放在一起。
*   **集成测试**：用于测试库的公共 API，像外部用户一样使用库，放在 `tests` 目录中。
*   **文档测试**：直接在文档注释中编写可运行的示例代码，确保文档的准确性。

通过 `cargo test` 命令和各种选项，可以方便地运行和管理这些测试。编写良好的测试是构建高质量 Rust 软件的关键部分。

## 11.6 常见陷阱

1.  **单元测试模块未正确配置 `#[cfg(test)]`**：
    *   **陷阱**：如果忘记在包含单元测试的 `mod tests { ... }` 上添加 `#[cfg(test)]` 属性，测试代码会在非测试构建（如 `cargo build`）中也被编译，可能增加编译时间和最终二进制文件大小。
    *   **避免**：始终为单元测试模块添加 `#[cfg(test)]`。

2.  **集成测试无法访问私有项**：
    *   **陷阱**：集成测试在单独的 crate 中编译，因此它们只能访问被测试库的公共 API。试图在集成测试中访问私有函数或模块会导致编译错误。
    *   **避免**：集成测试应该只关注公共接口。如果需要测试内部逻辑，应使用单元测试。

3.  **二进制 crate 的集成测试**：
    *   **陷阱**：如果项目是一个二进制 crate (只有 `src/main.rs` 而没有 `src/lib.rs`)，它不能有集成测试，因为没有可供外部链接的库。
    *   **避免**：将二进制 crate 的核心逻辑提取到一个库 crate (`src/lib.rs`) 中，然后让 `src/main.rs` 依赖并调用这个库。这样就可以为库 crate 编写集成测试了。

4.  **文档测试中的路径问题**：
    *   **陷阱**：在文档测试的代码示例中，`use` 语句的路径可能需要相对于 crate 的根。有时开发者可能会忘记 `use crate_name::...` 或者在模块内部的文档中路径处理不当。
    *   **避免**：确保文档测试中的 `use` 语句正确地引用了你的 crate 或模块。通常，`use my_crate_name::module::item;` 是一个好的开始。对于模块内的文档，可能需要 `use super::item;` 或 `use crate::module::item;`。运行 `cargo test` 会帮助发现这些路径问题。

5.  **忽略测试 (`#[ignore]`) 的管理**：
    *   **陷阱**：过度使用 `#[ignore]` 可能导致重要的测试被长期忽略，从而掩盖了潜在的 bug。
    *   **避免**：定期审查被忽略的测试，看是否可以修复它们或使其更快。在 CI (持续集成) 环境中，考虑定期运行所有测试，包括被忽略的测试 (使用 `cargo test -- --include-ignored`)。

6.  **测试依赖外部状态或顺序**：
    *   **陷阱**：测试如果依赖于外部文件、网络服务、环境变量或特定的执行顺序，可能会变得不稳定和难以维护（称为 "flaky tests"）。
    *   **避免**：
        *   尽量使测试自包含，不依赖外部环境。使用 mock 对象、桩代码 (stubs) 或测试替身 (test doubles) 来模拟外部依赖。
        *   如果测试必须操作文件系统，确保它们在临时目录中创建和清理文件，并且不会相互干扰。
        *   避免测试之间共享可变状态。如果必须，并且测试需要串行执行，使用 `cargo test -- --test-threads=1`。

7.  **`#[should_panic]` 不够精确**：
    *   **陷阱**：只使用 `#[should_panic]` 而不带 `expected` 参数，可能会导致测试通过，即使 panic 的原因并非你所预期的那个。
    *   **避免**：尽可能为 `#[should_panic]` 提供 `expected = "部分 panic 消息"` 参数，以确保 panic 是因为正确的原因发生的。

## 11.7 常见面试题

1.  **Q: Rust 中有哪几种类型的测试？它们各自的用途和存放位置是什么？**
    *   **A:** Rust 主要有三种类型的测试：
        1.  **单元测试 (Unit Tests)**：
            *   **用途**：测试程序中最小的可测试单元，如单个函数、模块或私有接口，以确保它们独立工作正常。
            *   **位置**：通常与被测试的代码放在同一个文件（例如 `src/lib.rs` 或 `src/module.rs`）中，在一个名为 `tests` 的子模块内，并使用 `#[cfg(test)]` 属性进行注解。
        2.  **集成测试 (Integration Tests)**：
            *   **用途**：测试库的各个部分是否能正确地协同工作，或者测试库的公共 API 是否符合预期。它们像外部用户一样使用库。
            *   **位置**：放在项目根目录下的 `tests` 目录中。每个 `.rs` 文件在 `tests` 目录下都会被编译成一个独立的测试 crate。
        3.  **文档测试 (Doc Tests)**：
            *   **用途**：测试文档注释中的代码示例是否正确并且保持最新。这有助于确保文档的质量和实用性。
            *   **位置**：直接写在 Rust 源代码文件（通常是库文件 `src/lib.rs` 或公共模块）的文档注释（以 `///` 或 `//!` 开头）中的代码块里。

2.  **Q: 如何在 Rust 中编写一个基本的测试函数？请提及 `#[test]` 属性和断言宏。**
    *   **A:**
        一个基本的 Rust 测试函数：
        1.  通常放在一个使用 `#[cfg(test)]` 属性注解的 `mod tests { ... }` 模块中。
        2.  函数本身使用 `#[test]` 属性进行标记。
        3.  函数体内通常包含设置、执行和断言三个步骤。
        4.  使用断言宏来检查结果是否符合预期。
        *   **`#[test]` 属性**：告诉 Rust 编译器这个函数是一个测试函数，应该由测试运行器执行。
        *   **断言宏 (Assertion Macros)**：
            *   `assert!(expression)`：断言 `expression` 的结果为 `true`。如果为 `false`，则测试失败并 panic。
            *   `assert_eq!(left, right)`：断言 `left` 和 `right` 两个表达式的值相等。如果不等，则测试失败并 panic，同时会打印出 `left` 和 `right` 的值。
            *   `assert_ne!(left, right)`：断言 `left` 和 `right` 两个表达式的值不相等。如果相等，则测试失败并 panic。
        *   **示例**：
            ```rust
            // pub fn add_two(a: i32) -> i32 { a + 2 }
            // #[cfg(test)]
            // mod tests {
            //     use super::*; // 导入外部模块的 add_two
            //     #[test]
            //     fn test_add_two() {
            //         let result = add_two(2); // 执行
            //         assert_eq!(result, 4, "2 + 2 应该等于 4"); // 断言，可选的自定义失败消息
            //     }
            // }
            ```

3.  **Q: `#[should_panic]` 属性的作用是什么？如何使其更精确？**
    *   **A:**
        *   **作用**：`#[should_panic]` 属性用于标记一个测试函数，表明这个测试预期会发生 `panic`。如果函数在执行过程中确实 `panic` 了，则测试通过；如果函数没有 `panic` 而正常结束，则测试失败。
        *   **如何使其更精确**：
            `#[should_panic]` 属性可以接受一个可选的 `expected` 参数，该参数是一个字符串。如果提供了 `expected` 参数，那么测试不仅要求函数 `panic`，还要求 `panic` 发生时打印的错误消息中**包含** `expected` 指定的子字符串。
            ```rust
            // pub fn divide(a: i32, b: i32) -> i32 {
            //     if b == 0 { panic!("attempt to divide by zero"); }
            //     a / b
            // }
            // #[cfg(test)]
            // mod tests {
            //     use super::*;
            //     #[test]
            //     #[should_panic(expected = "divide by zero")] // 检查 panic 消息
            //     fn test_divide_by_zero() {
            //         divide(10, 0);
            //     }
            // }
            ```
            使用 `expected` 参数可以确保 `panic` 是由于预期的原因发生的，而不是因为代码中其他地方的意外 `panic`。

4.  **Q: `cargo test` 命令有哪些常用的选项来控制测试的执行？**
    *   **A:** `cargo test` 有一些常用的命令行选项，通常跟在两个 `--` 分隔符之后传递给测试运行器：
        1.  **运行特定测试**：
            `cargo test <test_name_filter>`：只运行名称匹配 `<test_name_filter>` 的测试（可以是函数名、模块名或部分名称）。
        2.  **控制并行性**：
            `cargo test -- --test-threads=<N>`：设置用于运行测试的线程数量为 `<N>`。例如，`--test-threads=1` 会使测试串行执行。
        3.  **显示输出**：
            `cargo test -- --show-output`：即使测试通过，也显示测试函数中打印到标准输出的内容。默认情况下，只有失败测试的输出会被显示。
        4.  **运行被忽略的测试**：
            `cargo test -- --ignored`：只运行那些被 `#[ignore]` 属性标记的测试。
            `cargo test -- --include-ignored`：运行所有测试，包括被 `#[ignore]` 属性标记的测试。
        5.  **列出测试**：
            `cargo test -- --list`：列出所有可用的测试，而不实际运行它们。
        6.  **精确匹配测试名称**：
            `cargo test -- --exact <test_name>`：只运行名称完全匹配 `<test_name>` 的测试（通常与名称过滤器一起使用以避免部分匹配）。

5.  **Q: 什么是文档测试？如何在 Rust 中编写它们？它们有什么好处？**
    *   **A:**
        *   **文档测试 (Doc Tests)**：是直接写在 Rust 项目文档注释中的代码示例，`cargo test` 命令会自动提取、编译并运行这些示例作为测试。
        *   **如何编写**：
            *   在文档注释（以 `///` 开头用于项，或 `//!` 开头用于模块/crate）中，使用 Markdown 的代码块语法 (三个反引号 ```) 来包含 Rust 代码示例。
            *   代码块通常以 ` ```rust ` 或仅 ` ``` ` 开始。
            *   示例代码应该能够独立编译和运行，通常包含必要的 `use` 语句和断言。
            *   可以使用 `#` 开头的行来隐藏某些设置代码（这些行仍会被编译和执行，但不显示在最终渲染的文档中）。
            *   可以在 ``` 后面添加 `should_panic` 来标记预期会 panic 的示例，或 `no_run` 来标记不应运行的示例（仅编译）。
            ```rust
            /// Adds one to the given number.
            ///
            /// # Examples
            ///
            /// ```
            /// use my_crate::add_one; // 假设 my_crate 是 crate 名
            /// assert_eq!(add_one(5), 6);
            /// ```
            // pub fn add_one(x: i32) -> i32 { x + 1 }
            ```
        *   **好处**：
            1.  **文档与代码同步**：确保文档中的代码示例始终是正确和最新的。如果代码实现发生变化导致示例失败，`cargo test` 会报告错误。
            2.  **实用的文档**：提供可直接复制粘贴并运行的代码示例，帮助用户快速理解如何使用库的 API。
            3.  **测试覆盖**：文档测试也贡献了测试覆盖率，特别是对于公共 API 的基本用法。
            4.  **鼓励编写文档**：将测试集成到文档编写过程中，可以激励开发者编写更全面和准确的文档。

现在，我将为本章创建一个示例 Cargo 项目 (`adder`)。
