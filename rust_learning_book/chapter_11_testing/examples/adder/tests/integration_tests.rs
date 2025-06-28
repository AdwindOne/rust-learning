// tests/integration_tests.rs

// 导入我们要测试的库 crate (adder)
// Cargo 会将 adder 编译为一个库，然后这个测试文件会像外部 crate 一样链接它。
use adder; // crate 名与 Cargo.toml 中 [package] name 字段一致

// 可以在集成测试文件中定义模块，用于组织测试或共享代码
// 但这些模块本身不会被当作单独的测试 crate
mod common_integration_setup {
    pub fn setup() {
        println!("(集成测试通用设置) 正在执行集成测试的通用设置...");
        // 这里可以放一些所有集成测试都可能需要的设置代码
        // 例如，创建临时文件，启动模拟服务器等。
    }
}

#[test]
fn integration_test_add_two_and_two() {
    common_integration_setup::setup(); // 调用共享的设置函数
    // 调用 adder crate 中的公共函数 add
    assert_eq!(adder::add(2, 2), 4, "集成测试：2+2 应该等于 4");
}

#[test]
fn integration_test_greeting() {
    common_integration_setup::setup();
    let name = "集成测试员";
    let greeting_message = adder::greeting(name);
    assert!(
        greeting_message.contains(name),
        "集成测试：问候语 '{}' 应包含 '{}'",
        greeting_message,
        name
    );
}

#[test]
fn integration_test_guess_creation() {
    common_integration_setup::setup();
    let guess_val = 50;
    let g = adder::Guess::new(guess_val); // 调用公共结构体 Guess 的 new 方法
    assert_eq!(g.value(), guess_val, "集成测试：Guess 值不匹配");
}

// 集成测试也可以使用 #[should_panic]
// 但要注意，如果 Guess::new 的 panic 发生在 adder 库内部，
// 并且没有被捕获或转换，它会传播到这个测试 crate 并导致测试按预期通过。
#[test]
#[should_panic(expected = "less than or equal to 100")]
fn integration_test_guess_new_panics_too_large() {
    common_integration_setup::setup();
    adder::Guess::new(101);
}

// 集成测试也可以返回 Result
#[test]
fn integration_test_returns_result() -> Result<(), String> {
    common_integration_setup::setup();
    if adder::add(10, 10) == 20 {
        Ok(())
    } else {
        Err(String::from("集成测试：10 + 10 不等于 20"))
    }
}

// 注意：集成测试不能直接访问 adder crate 中的私有项，
// 例如在 src/lib.rs 中定义的 internal_adder 函数。
// fn integration_test_internal_adder() {
//     assert_eq!(adder::internal_adder(1,1), 2); // 编译错误: function `internal_adder` is private
// }
