use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

// --- 9.1 Box<T> ---
#[derive(Debug)]
enum RecursiveList {
    Cons(i32, Box<RecursiveList>), // Box 用于递归类型
    Nil,
}

fn box_example() {
    println!("--- Box<T> 示例 ---");
    let b = Box::new(100); // 在堆上分配 i32 值 100
    println!("b (Box<i32>) = {} (值), *b = {}", b, *b); // Box 实现了 Deref

    use RecursiveList::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("递归列表: {:?}", list);
}


// --- 9.2 Deref Trait ---
struct MyBox<T>(T); // 自定义 Box 类型

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; // 关联类型，指定 * 运算符返回的引用类型
    fn deref(&self) -> &Self::Target {
        &self.0 // 返回内部数据的引用
    }
}

// 如果希望 MyBox 也可以被可变解引用，需要实现 DerefMut
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn deref_example() {
    println!("\n--- Deref Trait 示例 ---");
    let x_val = 50;
    let y_mybox = MyBox::new(x_val);

    assert_eq!(50, x_val);
    assert_eq!(50, *y_mybox); // *y_mybox 调用了 deref()

    // Deref 强制转换 (Deref Coercion)
    fn print_greeting(name: &str) {
        println!("你好, {}!", name);
    }
    let m_string = MyBox::new(String::from("Rust 开发者"));
    print_greeting(&m_string); // &MyBox<String> -> &String -> &str

    let mut z_mybox = MyBox::new(String::from("可变"));
    // *z_mybox = String::from("已修改"); // 如果 MyBox 没有实现 DerefMut，这会报错
                                     // 因为 *z_mybox (通过 Deref) 得到 &String，不能直接赋值
    z_mybox.push_str("内容"); // 如果 MyBox 实现了 DerefMut，则 *z_mybox (隐式) 得到 &mut String
                                // 然后可以调用 String 的方法
    println!("z_mybox (可变解引用后): {}", *z_mybox);

}


// --- 9.3 Drop Trait ---
struct DroppablePointer {
    name: String,
    data: String,
}

impl Drop for DroppablePointer {
    fn drop(&mut self) { // drop 方法获取 &mut self
        println!("正在 drop DroppablePointer '{}' (数据: '{}')", self.name, self.data);
        // 在这里执行自定义清理逻辑
    }
}

fn drop_example() {
    println!("\n--- Drop Trait 示例 ---");
    let dp1 = DroppablePointer { name: "指针A".to_string(), data: "一些数据".to_string() };
    let dp2 = DroppablePointer { name: "指针B".to_string(), data: "其他数据".to_string() };
    println!("DroppablePointer dp1 和 dp2 已创建。");

    // dp1.drop(); // 编译错误：不能显式调用 drop 方法

    println!("准备提前 drop dp1...");
    drop(dp1); // 使用 std::mem::drop 来提前销毁 dp1
    println!("dp1 已被提前 drop。");

    // dp2 会在 drop_example 函数结束时自动被 drop
    println!("drop_example 函数即将结束。");
}


// --- 9.4 Rc<T> 引用计数 ---
#[derive(Debug)]
enum RcConsList {
    Cons(i32, Rc<RcConsList>),
    Nil,
}

fn rc_example() {
    println!("\n--- Rc<T> 引用计数示例 ---");
    use RcConsList::{Cons, Nil};

    let common_tail = Rc::new(Cons(10, Rc::new(Cons(15, Rc::new(Nil)))));
    println!("创建 common_tail, strong_count = {}", Rc::strong_count(&common_tail));

    let list_a = Rc::new(Cons(5, Rc::clone(&common_tail))); // list_a 共享 common_tail
    println!("创建 list_a, common_tail strong_count = {}, list_a strong_count = {}",
             Rc::strong_count(&common_tail), Rc::strong_count(&list_a));

    let list_b = Cons(3, Rc::clone(&common_tail)); // list_b 也共享 common_tail (注意 list_b 不是 Rc<T>)
    let list_c = Cons(4, Rc::clone(&common_tail)); // list_c 也共享 common_tail
    println!("创建 list_b 和 list_c 后, common_tail strong_count = {}", Rc::strong_count(&common_tail));

    println!("list_a: {:?}", list_a);
    println!("list_b: {:?}", list_b);
    println!("list_c: {:?}", list_c);

    drop(list_a); // 显式 drop list_a
    println!("drop list_a 后, common_tail strong_count = {}", Rc::strong_count(&common_tail));
    // list_b 和 list_c 仍然持有对 common_tail 的引用
    // 当 list_b 和 list_c (或包含它们的结构) 也被 drop 后，common_tail 的计数才会减少
}


// --- 9.5 RefCell<T> 与内部可变性 ---
pub trait Logger {
    fn log(&self, message: &str); // &self 方法
}

pub struct MessageQuotaTracker<'a, L: Logger> {
    logger: &'a L,
    messages_sent: usize,
    quota: usize,
}

impl<'a, L: Logger> MessageQuotaTracker<'a, L> {
    pub fn new(logger: &'a L, quota: usize) -> Self {
        MessageQuotaTracker { logger, messages_sent: 0, quota }
    }

    pub fn send_message(&mut self, message_content: &str) { // &mut self 方法
        if self.messages_sent < self.quota {
            self.logger.log(message_content);
            self.messages_sent += 1;
            if self.messages_sent == self.quota {
                self.logger.log("警告: 已达到消息限额!");
            }
        } else {
            self.logger.log("错误: 无法发送消息，已超出限额。");
        }
    }
}

// 测试用的 MockLogger
struct MockLogger {
    // 使用 RefCell 实现内部可变性，允许在 &self 方法中修改 logged_messages
    logged_messages: RefCell<Vec<String>>,
}

impl MockLogger {
    fn new() -> MockLogger {
        MockLogger { logged_messages: RefCell::new(vec![]) }
    }
}

impl Logger for MockLogger {
    fn log(&self, message: &str) { // log 方法接收 &self
        // self.logged_messages.push(String::from(message)); // 编译错误，不能直接修改
        self.logged_messages.borrow_mut().push(String::from(message));
        // borrow_mut() 返回 RefMut<Vec<String>>，它实现了 DerefMut
    }
}

fn refcell_example() {
    println!("\n--- RefCell<T> 与内部可变性示例 ---");
    let mock_logger = MockLogger::new();
    let mut tracker = MessageQuotaTracker::new(&mock_logger, 2);

    tracker.send_message("第一条消息");
    tracker.send_message("第二条消息 (达到限额)");
    tracker.send_message("第三条消息 (超出限额)");

    println!("MockLogger 记录的消息:");
    for msg in mock_logger.logged_messages.borrow().iter() { // borrow() 获取 Ref<Vec<String>>
        println!("  - {}", msg);
    }

    // 演示 RefCell 运行时 panic
    let data_cell = RefCell::new(10);
    let _borrow1 = data_cell.borrow(); // 不可变借用
    // let _borrow_mut1 = data_cell.borrow_mut(); // Panic! 已有不可变借用
    // println!("尝试在不可变借用时可变借用 (会 panic)");

    drop(_borrow1); // 释放不可变借用
    let _borrow_mut2 = data_cell.borrow_mut(); // 现在可以了
    // let _borrow2 = data_cell.borrow(); // Panic! 已有可变借用
    // println!("尝试在可变借用时不可变借用 (会 panic)");
}


// --- 9.6 Rc<RefCell<T>> 组合与引用循环 ---
// Rc<RefCell<T>> 组合
#[derive(Debug)]
enum ListRcRefCell {
    Cons(Rc<RefCell<i32>>, Rc<ListRcRefCell>),
    Nil,
}

// 引用循环与 Weak<T>
#[derive(Debug)]
struct TreeNode {
    value: i32,
    parent: RefCell<Weak<TreeNode>>, // 父节点使用 Weak 引用以避免循环
    children: RefCell<Vec<Rc<TreeNode>>>, // 子节点使用 Rc 强引用
}

fn rc_refcell_and_cycle_example() {
    println!("\n--- Rc<RefCell<T>> 组合示例 ---");
    use ListRcRefCell::{Cons as RCons, Nil as RNil};
    let shared_val = Rc::new(RefCell::new(100));

    let list1 = Rc::new(RCons(Rc::clone(&shared_val), Rc::new(RNil)));
    let list2 = RCons(Rc::new(RefCell::new(200)), Rc::clone(&list1)); // list2 共享 list1
    let list3 = RCons(Rc::new(RefCell::new(300)), Rc::clone(&list1)); // list3 共享 list1

    println!("初始 shared_val: {:?}", shared_val);
    println!("list1: {:?}", list1);
    println!("list2: {:?}", list2);
    println!("list3: {:?}", list3);

    *shared_val.borrow_mut() += 50; // 修改共享的值

    println!("\n修改后 shared_val: {:?}", shared_val);
    println!("list1 (修改后): {:?}", list1); // list1 中的值也变了
    println!("list2 (修改后): {:?}", list2);
    println!("list3 (修改后): {:?}", list3);


    println!("\n--- 引用循环与 Weak<T> 示例 ---");
    let leaf_node = Rc::new(TreeNode {
        value: 3,
        parent: RefCell::new(Weak::new()), // 初始无父节点
        children: RefCell::new(vec![]),
    });
    println!("leaf_node: value={}, strong_count={}, weak_count={}",
             leaf_node.value, Rc::strong_count(&leaf_node), Rc::weak_count(&leaf_node));
    println!("  leaf_node parent (before branch): {:?}", leaf_node.parent.borrow().upgrade());


    let branch_node = Rc::new(TreeNode {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf_node)]), // branch 持有 leaf 的强引用
    });
    println!("\nbranch_node: value={}, strong_count={}, weak_count={}",
             branch_node.value, Rc::strong_count(&branch_node), Rc::weak_count(&branch_node));

    // 设置 leaf_node 的父节点为 branch_node (使用弱引用)
    *leaf_node.parent.borrow_mut() = Rc::downgrade(&branch_node);

    println!("\nAfter linking leaf to branch:");
    println!("  leaf_node parent (should be Some(branch_node)): value={:?}",
             leaf_node.parent.borrow().upgrade().map(|p| p.value));
    println!("  leaf_node: strong_count={}, weak_count={}",
             Rc::strong_count(&leaf_node), Rc::weak_count(&leaf_node)); // leaf 的强引用计数不变 (来自 branch)
    println!("  branch_node: strong_count={}, weak_count={}",
             Rc::strong_count(&branch_node), Rc::weak_count(&branch_node)); // branch 的弱引用计数增加 (来自 leaf)

    // 模拟 branch_node 离开作用域
    let branch_value_before_drop = branch_node.value;
    drop(branch_node); // branch_node 被 drop，其强引用计数变为 0
    println!("\nAfter branch_node is dropped (value was {}):", branch_value_before_drop);
    println!("  leaf_node parent (should be None now): {:?}", leaf_node.parent.borrow().upgrade());
    println!("  leaf_node: strong_count={}, weak_count={}", // leaf 的强引用计数不变 (外部main持有)
             Rc::strong_count(&leaf_node), Rc::weak_count(&leaf_node)); // leaf 的弱引用计数可能仍为1，但指向的数据已drop

}


fn main() {
    box_example();
    deref_example();
    drop_example();
    rc_example();
    refcell_example();
    rc_refcell_and_cycle_example();
}
