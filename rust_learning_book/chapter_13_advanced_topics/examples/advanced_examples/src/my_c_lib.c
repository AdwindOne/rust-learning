// src/my_c_lib.c
// 一个简单的 C 函数，用于 FFI 演示

#include <stdio.h> // For printf, if needed for debugging C side

// 一个简单的 C 函数，接收一个整数，返回其两倍
int multiply_by_two_from_c(int x) {
    // printf("[C] Received: %d, Returning: %d\n", x, x * 2);
    return x * 2;
}

// 一个 C 函数，接收一个字符串并打印它
// 注意：这个函数期望一个以 null 结尾的 C 字符串
void print_string_from_c(const char* s) {
    printf("[C] String from Rust: %s\n", s);
}

// 一个 C 函数，它会调用一个 Rust 回调函数
// typedef int (*rust_callback_type)(int); // 定义回调函数类型
// int call_rust_callback_from_c(int value, rust_callback_type callback) {
//     printf("[C] Calling Rust callback with value: %d\n", value);
//     return callback(value);
// }

// 一个 C 结构体示例（如果需要在 Rust 中操作）
// struct MyCStruct {
//     int id;
//     double value;
// };

// void process_c_struct(struct MyCStruct* data) {
//     if (data) {
//         printf("[C] Processing MyCStruct: id=%d, value=%f\n", data->id, data->value);
//         data->value += 1.0;
//     }
// }
