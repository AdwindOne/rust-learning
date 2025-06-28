// build.rs
// 这个构建脚本用于编译 C 代码 (my_c_lib.c)，以便 Rust FFI 示例可以链接它。

fn main() {
    // 告诉 Cargo 如果 my_c_lib.c 或 build.rs 改变了，就重新运行这个构建脚本
    println!("cargo:rerun-if-changed=src/my_c_lib.c");
    println!("cargo:rerun-if-changed=build.rs");

    // 使用 cc crate 来编译 C 文件
    cc::Build::new()
        .file("src/my_c_lib.c") // 指定要编译的 C 源文件
        .compile("my_c_lib");   // 指定编译后的库名 (例如，会生成 libmy_c_lib.a)
                                // cc crate 会自动处理链接指令 (如 cargo:rustc-link-lib 和 cargo:rustc-link-search)

    println!("build.rs: C library 'my_c_lib' compilation configured.");
}
