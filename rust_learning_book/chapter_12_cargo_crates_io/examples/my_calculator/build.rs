// build.rs
// 这是一个构建脚本的示例。
// 对于 my_calculator 这个简单的库，我们可能不需要复杂的构建脚本。
// 但为了演示，我们可以放一些简单的逻辑。

fn main() {
    println!("cargo:rerun-if-changed=build.rs"); // 如果 build.rs 本身改变，重新运行
    // println!("cargo:rerun-if-changed=src/some_static_data.txt"); // 如果某个文件改变，重新运行

    // 可以在这里执行一些构建时操作，例如：
    // 1. 编译 C/C++ 代码 (使用 cc crate)
    // 2. 代码生成
    // 3. 设置环境变量供后续编译使用

    // 示例：设置一个环境变量，可以在 lib.rs 或 main.rs 中通过 env!宏访问
    // std::env::set_var("MY_BUILD_TIME_INFO", "Built at ... (timestamp)"); // 这通常不是推荐的做法，
                                                                        // 因为它可能导致不必要的重新编译。
                                                                        // 更好的方式是让 build.rs 生成一个 .rs 文件然后 include! 它。

    println!("cargo:warning=my_calculator 构建脚本正在运行 (示例警告)。");

    // 检查某个特性是否启用，并据此执行操作
    if std::env::var("CARGO_FEATURE_ADVANCED_MATH").is_ok() {
        println!("cargo:warning='advanced_math' 特性在构建时被检测到已启用。");
        // 例如，可以根据特性链接不同的本地库或执行不同的代码生成
    }

    // 假设我们有一个 C 库需要编译和链接 (使用 cc crate)
    // 这需要在 Cargo.toml 的 [build-dependencies] 中添加 cc = "1.0"
    /*
    if cfg!(feature = "native_lib_feature") { // 假设有一个特性控制是否编译C库
        cc::Build::new()
            .file("src/native/my_c_code.c")
            .compile("my_c_lib_name"); // 编译成 libmy_c_lib_name.a

        // 告诉 Cargo 链接这个库 (如果 cc crate 没有自动处理)
        // println!("cargo:rustc-link-lib=static=my_c_lib_name");
        // println!("cargo:rustc-link-search=native={}", std::env::var("OUT_DIR").unwrap());
    }
    */

    println!("my_calculator 构建脚本执行完毕。");
}
