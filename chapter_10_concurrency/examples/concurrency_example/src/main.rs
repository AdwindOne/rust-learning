use std::thread;
use std::time::Duration;
use std::sync::mpsc; // multiple producer, single consumer
use std::sync::{Mutex, Arc};

// --- 10.1 使用线程 ---
fn basic_threading_example() {
    println!("--- 10.1 基础线程示例 ---");

    // 启动一个新线程
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("(新线程 id:{:?}) 计数: {}", thread::current().id(), i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    // 主线程继续执行
    for i in 1..=3 {
        println!("(主线程 id:{:?}) 计数: {}", thread::current().id(), i);
        thread::sleep(Duration::from_millis(30));
    }

    // 等待新线程结束
    // 如果不调用 join，主线程结束时，新线程可能未执行完毕就被终止
    match handle.join() {
        Ok(_) => println!("新线程执行完毕。"),
        Err(e) => println!("新线程 panic: {:?}", e),
    }
    println!("主线程执行完毕。");
}

fn move_closure_example() {
    println!("\n--- move 闭包与线程示例 ---");
    let data = vec![10, 20, 30];

    // 使用 move 将 data 的所有权转移到新线程
    let handle = thread::spawn(move || {
        println!("(新线程 id:{:?}) 接收到的数据: {:?}", thread::current().id(), data);
        // data 在这里有效
    });

    // println!("{:?}", data); // 编译错误：data 的所有权已被移动到线程中
    handle.join().expect("新线程 panic 了");
    println!("move 闭包示例结束。");
}


// --- 10.2 消息传递 (通道) ---
fn channel_example() {
    println!("\n--- 10.2 消息传递 (通道) 示例 ---");
    let (tx, rx) = mpsc::channel(); // tx: Sender, rx: Receiver

    // 派生线程发送消息
    let tx_clone = tx.clone(); // 克隆发送端给新线程
    thread::spawn(move || {
        let messages = vec![
            String::from("你好"),
            String::from("来自"),
            String::from("发送线程"),
        ];
        for msg in messages {
            println!("(发送线程 id:{:?}) 正在发送: '{}'", thread::current().id(), msg);
            tx_clone.send(msg).unwrap(); // msg 的所有权被移动
            thread::sleep(Duration::from_millis(100));
        }
        println!("(发送线程 id:{:?}) 发送完毕。", thread::current().id());
        // tx_clone 在此 drop，如果 tx 也 drop，通道会关闭
    });

    // 主线程可以继续发送 (如果需要)
    // tx.send(String::from("主线程的问候")).unwrap();

    // 主线程接收消息
    // rx.recv() 会阻塞直到有消息或通道关闭
    // rx 是一个迭代器，当通道关闭且无消息时，迭代结束
    println!("(主线程 id:{:?}) 等待接收消息...", thread::current().id());
    for received_msg in rx {
        println!("(主线程 id:{:?}) 收到: '{}'", thread::current().id(), received_msg);
    }
    println!("(主线程 id:{:?}) 通道已关闭，接收结束。", thread::current().id());
}


// --- 10.3 共享状态 (Mutex 和 Arc) ---
fn shared_state_mutex_arc_example() {
    println!("\n--- 10.3 共享状态 (Mutex 和 Arc) 示例 ---");

    // Arc<Mutex<T>> 是在线程间共享可变数据的标准方式
    // Arc: Atomic Reference Counting，线程安全的引用计数
    // Mutex: Mutual Exclusion，互斥锁，保证一次只有一个线程访问数据
    let counter = Arc::new(Mutex::new(0)); // 共享的可变计数器
    let mut handles = vec![];

    for i in 0..5 { // 创建5个线程
        let counter_for_thread = Arc::clone(&counter); // 克隆 Arc 指针 (增加引用计数)
        let handle = thread::spawn(move || {
            // 获取锁，返回 MutexGuard，它实现了 Deref 和 DerefMut
            let mut num_guard = counter_for_thread.lock().unwrap();
            // unwrap 是因为 lock() 返回 Result (如果持有锁的线程 panic，会导致 PoisonError)

            *num_guard += 1; // 修改 Mutex 内部的数据
            println!("(线程 id:{:?}, 序号 {}) 将计数器增加到: {}", thread::current().id(), i, *num_guard);
            // MutexGuard 在这里离开作用域，锁被自动释放
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 读取最终的计数器值
    println!("所有线程执行完毕，最终计数器值为: {}", *counter.lock().unwrap());
    // 预期结果是 5
}


// --- 10.5 异步编程 (Async/Await) 简介 ---
// 注意：这部分代码需要异步运行时 (如 tokio) 才能实际运行。
// 这里只做概念性演示，不包含完整的运行时设置。
//
// Cargo.toml 可能需要:
// tokio = { version = "1", features = ["full"] }
// futures = "0.3"

// async fn async_task(id: u32) {
//     println!("(异步任务 {}) 开始", id);
//     // 模拟异步I/O操作，例如网络请求或文件读写
//     // tokio::time::sleep(Duration::from_millis(100 * id as u64)).await;
//     println!("(异步任务 {}) 完成", id);
// }

// // 使用 tokio 的 main 宏来启动异步运行时
// // #[tokio::main]
// async fn async_example() {
//     println!("\n--- 10.5 异步编程 (Async/Await) 示例 (概念性) ---");
//
//     let task1 = async_task(1); // 调用 async fn 返回一个 Future
//     let task2 = async_task(2);
//     let task3 = async_task(3);
//
//     // 同时运行多个 Future (并发执行，但不一定是并行)
//     // futures::join!(task1, task2, task3); // 需要 futures crate
//     // 或者使用 tokio::join!
//     // tokio::join!(task1, task2, task3);
//
//     println!("所有异步任务已安排执行 (实际完成顺序取决于运行时和 .await)。");
// }


fn main() {
    basic_threading_example();
    move_closure_example();
    channel_example();
    shared_state_mutex_arc_example();

    // 要运行 async_example，需要配置异步运行时
    // println!("\n要运行异步示例，请取消注释相关代码并配置 Tokio 运行时。");
    // async_example(); // 如果运行时已配置，可以取消注释来调用
}
