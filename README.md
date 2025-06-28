# Rust 学习代码之书 (The Rust Programming Language - Code Companion)

欢迎来到《Rust 学习代码之书》！本项目旨在通过一系列精心组织的章节、代码示例、常见陷阱分析以及详尽的面试题，为 Rust 学习者提供一个全面且实践性强的学习资源。

无论你是编程新手，还是有其他语言经验并希望学习 Rust 的开发者，这本书都希望能帮助你系统地掌握 Rust 的核心概念和实用技巧。

## 项目目标

*   **系统性学习**: 覆盖从 Rust 基础语法到高级特性的完整学习路径。
*   **代码驱动**: 每一章都伴随有可运行的示例代码项目，帮助读者在实践中理解概念。
*   **深度解析**: 不仅解释“是什么”，更注重“为什么”以及相关的设计哲学。
*   **实战导向**: 通过多个实战项目章节，将理论知识应用于解决实际问题。
*   **面试准备**: 每章均包含详尽的常见面试题及其解释，助力读者应对技术面试。
*   **避坑指南**: 每章均总结了常见的编程陷阱，帮助读者少走弯路。

## 本书综合特性与能力

本书涵盖了 Rust 语言的广泛主题，包括但不限于：

*   **基础入门**: Rust 简介、环境搭建、Cargo基础。
*   **核心语法**: 变量、数据类型、函数、控制流。
*   **所有权系统**: Rust 最核心的特性——所有权、借用、生命周期（初步）。
*   **复合类型**: 结构体、枚举、模式匹配。
*   **常用集合**: Vector、String、HashMap。
*   **错误处理**: `panic!` 与 `Result<T, E>`，自定义错误类型。
*   **泛型、Trait 与生命周期 (高级)**: 深入理解 Rust 的抽象能力和内存安全基石。
*   **智能指针**: `Box<T>`, `Rc<T>`, `Arc<T>`, `RefCell<T>`, `Mutex<T>`, `Weak<T>` 等。
*   **并发编程**: 线程、消息传递、共享状态同步、`Send`/`Sync`、`async/await` 简介。
*   **测试**: 单元测试、集成测试、文档测试。
*   **Cargo 与 Crates.io**: 项目管理、依赖、特性、构建配置、发布。
*   **高级主题概览**: 宏、Unsafe Rust、FFI、闭包与迭代器深入。
*   **Rust 惯用法与最佳实践**: 编写地道、高效、可维护的 Rust 代码。
*   **实战项目**:
    *   命令行工具 (如单词计数器 `rwc`)。
    *   Web 开发简介 (如使用 Axum/Tokio 构建简单 API)。
    *   高级并发/并行项目 (如多线程端口扫描器)。
*   **附录**: Rust 术语表、更多学习资源。

每一章节的 `README.md` 文件是该章节的主要学习材料，包含了详细的解释、代码片段、常见陷阱分析和面试题详解。`examples` 子目录则包含了与该章节内容配套的可运行示例项目。

## 目录结构说明

本项目的根目录 `rust_learning_book/` 下主要包含以下结构的子目录：

*   `README.md` (本文件): 项目的顶层介绍。
*   `AGENTS.md`: (如果存在) 给 AI 代理的特定指示。

*   **`chapter_XX_topic_name/`**: 代表本书的正式技术章节或实战项目章节。
    *   `README.md`: 该章节的主要学习内容，包含详细解释、代码片段、陷阱分析和面试题。
    *   `examples/`: 包含与本章内容相关的、可独立运行的 Cargo 示例项目。
        *   `example_project_name/`:
            *   `Cargo.toml`: 示例项目的清单文件。
            *   `src/`: 示例项目的源代码目录。
                *   `main.rs` (对于二进制项目) 或 `lib.rs` (对于库项目)。
                *   (可能包含) `build.rs` (构建脚本)。
            *   `tests/`: (如果适用) 示例项目的集成测试。
            *   (其他) 示例项目可能包含的其他文件，如测试用的样本数据。

*   **`appendix_X_appendix_name/`**: 代表本书的附录章节。
    *   `README.md`: 附录的主要内容。
    *   (通常不包含 `examples/` 目录，除非附录内容也涉及代码演示)。

**当前已完成的章节和附录包括：**

*   `chapter_01_introduction_setup/`
*   `chapter_02_basic_syntax_data_types/`
*   `chapter_03_control_flow/`
*   `chapter_04_ownership_borrowing/`
*   `chapter_05_structs_enums/`
*   `chapter_06_collections/`
*   `chapter_07_error_handling/`
*   `chapter_08_generics_traits_lifetimes/`
*   `chapter_09_smart_pointers/`
*   `chapter_10_concurrency/`
*   `chapter_11_testing/`
*   `chapter_12_cargo_crates_io/`
*   `chapter_13_advanced_topics/`
*   `chapter_14_idioms_best_practices/`
*   `chapter_15_cli_project/` (实战：命令行工具 rwc)
*   `chapter_16_web_project/` (实战：简单 Web API - Axum)
*   `chapter_17_concurrent_project/` (实战：多线程端口扫描器)
*   `appendix_a_glossary/`
*   `appendix_b_resources/`

## 如何使用本书

1.  **顺序阅读**: 建议按照章节顺序阅读 `README.md` 文件，因为后续章节通常基于前面章节的知识。
2.  **运行示例代码**: 对于每个章节的 `examples/` 目录下的示例项目，建议在本地环境中：
    *   进入示例项目目录 (例如 `cd chapter_XX_.../examples/project_name/`)。
    *   运行 `cargo build` 来编译代码。
    *   运行 `cargo run` (如果项目是二进制程序) 来查看运行结果。
    *   运行 `cargo test` 来执行该示例项目中的单元测试、集成测试和文档测试。
    *   仔细阅读示例代码，并尝试修改它、扩展它，以加深理解。
3.  **关注陷阱和面试题**: “常见陷阱”部分可以帮助你避免编程中易犯的错误。“常见面试题”及其详解则有助于你检验对核心概念的掌握程度，并为技术面试做准备。
4.  **动手实践**: Rust 是一门实践性很强的语言。学习 Rust 最好的方式就是多写代码。尝试完成书中实战项目，或者自己构思一些小项目来练习。
5.  **查阅官方文档**: 当遇到疑问或想深入了解某个特定API时，及时查阅 Rust 官方文档（本书附录B中列有链接）。

## 贡献与反馈
为了方便大家学习与交流，本项目基于 Apache License 2.0 开源发布。协议内容相对宽泛，大家可以自由补充、修改、使用与分发，也欢迎将改进内容回馈社区。
如果您在使用过程中发现任何错误、不清晰之处，或者有更好的实现方式，欢迎提出：

提交 Issue 或 Pull Request

邮件联系维护者（Email: adwindone@gmail.com）

在社区平台进行交流与反馈（qq群：1050966031）

非常感谢您的参与和支持，您的反馈将有助于本项目不断完善！
如果您在学习过程中发现任何错误、有不清晰之处，或者有任何改进建议，欢迎提出。
---
祝您在 Rust 的学习旅程中收获满满！
