use std::fmt::{Debug, Display}; // 用于 trait bound

// --- 8.1 泛型 (Generics) ---

// 泛型函数 (约束 T 必须实现 PartialOrd 以便比较，Copy 以便赋值)
// 如果返回 T 的值，通常需要 Copy。如果返回 &T，则不需要 Copy。
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest_val = list[0];
    for &item in list {
        if item > largest_val {
            largest_val = item;
        }
    }
    largest_val
}

// 泛型函数返回引用，只需要 PartialOrd
fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest_item = &list[0];
    for item in list {
        if item > largest_item { // item 是 &T, largest_item 是 &T
            largest_item = item;
        }
    }
    largest_item
}


// 泛型结构体 Point<T>
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 为泛型结构体 Point<T> 实现方法
impl<T> Point<T> {
    fn x_ref(&self) -> &T {
        &self.x
    }
    fn new_point(x_val: T, y_val: T) -> Self {
        Point { x: x_val, y: y_val }
    }
}

// 为特定具体类型的泛型结构体 Point<f32> 实现方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 结构体方法本身也可以有泛型参数
struct PointMix<X1, Y1> {
    x: X1,
    y: Y1,
}
impl<X1, Y1> PointMix<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointMix<X2, Y2>) -> PointMix<X1, Y2> {
        PointMix {
            x: self.x,
            y: other.y,
        }
    }
}


// --- 8.2 Trait：定义共享行为 ---
pub trait Summary {
    fn summarize_author(&self) -> String; // 必须实现

    fn summarize(&self) -> String { // 默认实现
        format!("(阅读更多来自 {} 的内容...)", self.summarize_author())
    }
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub author: String,
}
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("作者: {}", self.author)
    }
    // 使用默认的 summarize
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
}
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String { // 覆盖默认实现
        format!("{}: {}", self.username, self.content)
    }
}

// Trait 作为参数 (impl Trait 语法)
pub fn notify(item: &(impl Summary + Debug)) { // item 必须同时实现 Summary 和 Debug
    println!("[通知] 新摘要: {} (来自: {:?})", item.summarize(), item);
}
// Trait 作为参数 (Trait Bound 泛型语法)
pub fn notify_generic<T: Summary + Display>(item: &T) { // T 必须实现 Summary 和 Display
    println!("[泛型通知] {} (来自: {})", item.summarize(), item); // item 可以用 {} 打印
}

// 返回实现了 Trait 的类型
fn create_summarizable_tweet(username: String, content: String) -> impl Summary {
    Tweet { username, content }
    // 注意：如果这里可能返回 NewsArticle，则会编译错误，因为返回类型必须是单一具体类型。
    // 若要返回不同类型，需用 Box<dyn Summary>
}

// Trait Bound 有条件地实现方法
struct Container<T> {
    value: T,
}
impl<T: Display> Container<T> { // 仅当 T 实现 Display 时，才有 to_string_val 方法
    fn to_string_val(&self) -> String {
        format!("[{}]", self.value)
    }
}

// Trait 对象
pub trait Drawable {
    fn draw(&self);
}
pub struct Button { pub label: String }
impl Drawable for Button { fn draw(&self) { println!("绘制按钮: {}", self.label); } }
pub struct TextField { pub text: String }
impl Drawable for TextField { fn draw(&self) { println!("绘制文本域: {}", self.text); } }

pub struct Screen {
    pub components: Vec<Box<dyn Drawable>>, // Trait 对象列表
}
impl Screen {
    pub fn run(&self) {
        println!("\n--- 运行屏幕绘制 ---");
        for component in self.components.iter() {
            component.draw(); // 动态分发
        }
        println!("--- 屏幕绘制结束 ---");
    }
}


// --- 8.3 生命周期 (Lifetimes) ---

// 函数中的泛型生命周期
// 返回的引用生命周期与 x 和 y 中较短的那个相同
fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 结构体定义中的生命周期注解
#[derive(Debug)]
struct Excerpt<'a> { // Excerpt 实例不能比它引用的 part 活得更长
    part: &'a str,
}
impl<'a> Excerpt<'a> {
    fn get_level(&self) -> i32 { 1 } // 生命周期省略规则应用
    fn announce(&self, announcement: &str) -> &'a str { // 返回的 &str 与 self.part 生命周期相同
        println!("通知: {}", announcement);
        self.part
    }
}

// 静态生命周期
fn static_string() -> &'static str {
    "这是一个静态生命周期的字符串字面量。"
}

// 结合泛型、Trait 和生命周期
fn longest_and_announcement<'a, T: Display>(
    str1: &'a str,
    str2: &'a str,
    announcement: T,
) -> &'a str {
    println!("重要通知: {}", announcement);
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}


fn main() {
    println!("--- 8.1 泛型示例 ---");
    let numbers = vec![34, 50, 25, 100, 65];
    println!("最大数字 (largest): {}", largest(&numbers));
    println!("最大数字引用 (largest_ref): {}", largest_ref(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("最大字符 (largest): {}", largest(&chars));
    println!("最大字符引用 (largest_ref): {}", largest_ref(&chars));

    let int_point = Point::new_point(5, 10);
    let float_point = Point { x: 1.0_f32, y: 4.0_f32 }; // 需要指定 f32
    println!("整数点: {:?}, x={}", int_point, int_point.x_ref());
    println!("浮点点: {:?}, x={}, 距离原点={}",
        float_point, float_point.x_ref(), float_point.distance_from_origin());

    let p_mix1 = PointMix { x: 5, y: "hello" };
    let p_mix2 = PointMix { x: 10.5_f64, y: 'C' };
    let p_mix_result = p_mix1.mixup(p_mix2); // x from p_mix1 (i32), y from p_mix2 (char)
    println!("混合点结果: x={}, y={}", p_mix_result.x, p_mix_result.y);
    println!();


    println!("--- 8.2 Trait 示例 ---");
    let article1 = NewsArticle {
        headline: String::from("Rust 新版本发布!"),
        author: String::from("Rust 团队"),
    };
    let tweet1 = Tweet {
        username: String::from("rustacean_joy"),
        content: String::from("学习 Trait 很有趣!"),
    };
    println!("新闻摘要: {}", article1.summarize());
    println!("推文摘要: {}", tweet1.summarize());

    notify(&article1); // article1 实现了 Summary 和 Debug (通过 derive)
    // notify_generic(&tweet1); // 编译错误，Tweet 没有实现 Display

    // 为 Tweet 实现 Display 以便用于 notify_generic
    impl Display for Tweet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "推文来自 @{} 内容: '{}'", self.username, self.content)
        }
    }
    notify_generic(&tweet1); // 现在可以了

    let returned_summary = create_summarizable_tweet("test_user".to_string(), "返回 trait".to_string());
    println!("返回的摘要: {}", returned_summary.summarize());

    let int_container = Container { value: 100 };
    println!("整数容器 to_string_val: {}", int_container.to_string_val());
    struct NoDisplay {} // 这个结构体没有实现 Display
    let _no_display_container = Container { value: NoDisplay {} };
    // println!("{}", no_display_container.to_string_val()); // 编译错误

    let screen = Screen {
        components: vec![
            Box::new(Button { label: "提交".to_string() }),
            Box::new(TextField { text: "输入你的名字".to_string() }),
        ],
    };
    screen.run();
    println!();


    println!("--- 8.3 生命周期示例 ---");
    let s1 = String::from("这是一个长字符串");
    let s2 = "短";
    let longest = longest_str(s1.as_str(), s2);
    println!("两个字符串中较长的是: '{}'", longest);

    // 悬垂引用演示 (通过作用域)
    let string_outer = String::from("外部字符串，生命周期长");
    let result_ref;
    {
        let string_inner = String::from("内部字符串");
        // result_ref = longest_str(string_outer.as_str(), string_inner.as_str()); // 如果这样，result_ref 的生命周期会是 string_inner 的
                                                                              // 导致下面 println! 编译错误
        result_ref = longest_str(string_outer.as_str(), "字面量也行"); // "字面量也行" 是 'static
                                                                   // result_ref 的生命周期是 string_outer 的
    }
    // println!("内部作用域后的最长字符串: {}", result_ref); // 如果 result_ref 引用了 string_inner，这里会报错

    let novel_content = String::from("很久以前，在一个遥远的星系...");
    let first_sentence = novel_content.split('.').next().unwrap_or("默认句子");
    let excerpt_obj = Excerpt { part: first_sentence };
    println!("Excerpt: {:?}, level: {}", excerpt_obj, excerpt_obj.get_level());
    let announced_part = excerpt_obj.announce("新发现的片段!");
    println!("宣布的片段: {}", announced_part);

    println!("{}", static_string());
    println!();

    println!("--- 结合泛型、Trait 和生命周期 ---");
    let str_a = "Rust语言";
    let str_b = "非常强大且安全";
    let ann = 12345; // i32 实现了 Display
    let combined_longest = longest_and_announcement(str_a, str_b, ann);
    println!("最长且带通知的字符串: '{}'", combined_longest);
}
