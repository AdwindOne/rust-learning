# 第 10 章：并发 (Concurrency)

并发 (Concurrency) 是指程序的不同部分独立执行。并行 (Parallelism) 是指程序的不同部分同时执行。Rust 提供了强大的工具来编写并发和并行的代码，同时其所有权和类型系统有助于在编译时捕获许多常见的并发错误。

本章主要内容：
*   如何创建线程以同时运行多段代码。
*   **消息传递 (Message Passing)** 并发模型，线程通过发送消息进行通信。
*   **共享状态 (Shared State)** 并发模型，多个线程可以访问同一份数据。
*   `Sync` 和 `Send` trait，以及它们如何在编译时增强 Rust 的并发安全。
*   对异步编程 (Async/Await) 的简要介绍。

## 10.1 使用线程同时运行代码

在大多数现代操作系统中，执行的程序代码在一个**进程 (process)** 中运行，操作系统会为该进程管理资源。在你的程序内部，可以拥有独立运行的部分，这些部分通过**线程 (threads)** 来实现。将程序拆分为多个线程可以提高性能，因为程序可以同时做多件事情，但也增加了复杂性，因为线程可能会访问共享资源，导致竞争条件 (race conditions) 或死锁 (deadlocks) 等问题。

### 10.1.1 创建新线程 (`thread::spawn`)

使用 `std::thread::spawn` 函数可以创建一个新线程。它接收一个闭包 (closure) 作为参数，这个闭包包含了新线程要运行的代码。

```rust
use std::thread;
use std::time::Duration;

fn main_spawn() {
    // 启动一个新线程
    let handle = thread::spawn(|| { // 闭包
        for i in 1..=5 {
            println!("(新线程) hi number {}!", i);
            thread::sleep(Duration::from_millis(1)); // 让线程暂停一小段时间
        }
    });

    // 主线程继续执行
    for i in 1..=3 {
        println!("(主线程) hi number {}!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 等待新线程结束 (可选)
    // 如果主线程结束，所有其他线程也会被关闭，无论它们是否已完成。
    // handle.join().unwrap(); // 调用 join 会阻塞当前线程，直到 handle 代表的线程结束
    // println!("所有线程执行完毕。");
}
```
如果运行上面的代码（不带 `handle.join()`），你会发现新线程的输出可能没有完全打印，或者与主线程的输出交错出现。这是因为主线程结束时，它会终止所有其他派生线程。

### 10.1.2 使用 `join` 等待所有线程完成

`thread::spawn` 返回一个 `JoinHandle<T>`。调用其 `join` 方法会阻塞当前线程，直到被 `join` 的线程执行完毕。

```rust
use std::thread;
use std::time::Duration;

fn main_join() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("(新线程) hi number {}!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 主线程做一些工作
    for i in 1..=3 {
        println!("(主线程) hi number {}!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // 等待新线程完成
                            // unwrap 是因为 join 返回一个 Result
    println!("新线程执行完毕，主线程继续...");
    // 主线程的其他工作...
    println!("主线程执行完毕。");
}
```
现在，主线程会等待新线程完成后再退出。

### 10.1.3 `move` 闭包与线程

当创建线程时，传递给 `thread::spawn` 的闭包通常需要捕获其环境中的值。为了确保这些值在线程的整个生命周期内都有效（因为线程可能比创建它的函数活得更长），我们经常在闭包前使用 `move` 关键字。`move` 会强制闭包获取它所使用的所有外部变量的所有权。

```rust
use std::thread;

fn main_move_closure() {
    let v = vec![1, 2, 3];

    // 如果不使用 move，编译器会报错，因为它无法确定 v 的引用在线程中是否一直有效
    // let handle = thread::spawn(|| {
    //     println!("这是一个来自线程的向量: {:?}", v);
    // });

    // 使用 move 关键字，v 的所有权被移动到闭包中，然后闭包被传递给新线程
    let handle = thread::spawn(move || {
        println!("这是一个来自线程的向量 (v 的所有权已移动): {:?}", v);
        // v 在这里有效，但在外部 main_move_closure 函数中不再有效
    });

    // drop(v); // 编译错误！v 的所有权已经移动到线程的闭包中了
    handle.join().unwrap();
}
```

## 10.2 使用消息传递在线程间传输数据

一种确保并发安全的方式是不让线程共享数据，而是让它们通过**消息传递 (Message Passing)** 进行通信。每个线程拥有自己的数据，并通过发送消息将数据副本或所有权传递给其他线程。这类似于 Go 语言中的 "Do not communicate by sharing memory; instead, share memory by communicating." 哲学。

Rust 标准库提供了**通道 (channels)** 来实现消息传递。一个通道包含一个发送端 (transmitter) 和一个接收端 (receiver)。
*   `mpsc` 是 "multiple producer, single consumer" 的缩写。这意味着一个通道可以有多个发送端，但只能有一个接收端。

### 10.2.1 创建通道并发送接收数据

使用 `std::sync::mpsc::channel()` 函数创建一个通道。它返回一个包含发送端 (`Sender<T>`) 和接收端 (`Receiver<T>`) 的元组。

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main_channel() {
    // 创建一个通道 (tx 是发送端, rx 是接收端)
    // T 会被推断为 String，因为我们发送 String 类型的值
    let (tx, rx) = mpsc::channel();

    // 派生一个线程来发送消息
    let tx1 = tx.clone(); // 克隆发送端，以便在另一个线程中使用
                         // 原始的 tx 也可以继续使用
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            println!("(发送线程) 正在发送: {}", val);
            tx1.send(val).unwrap(); // send 返回 Result，这里用 unwrap 简化
                                   // val 的所有权被移动到通道中
            thread::sleep(Duration::from_secs(1));
        }
        // val 在这里不再有效
    });

    // 主线程接收消息
    // rx.recv() 会阻塞主线程，直到有消息可用。它返回 Result<T, E>。
    // rx.try_recv() 不会阻塞，立即返回 Result，如果没消息则是 Err。

    // 将接收端 rx 视为迭代器
    println!("(主线程) 等待接收消息...");
    for received_msg in rx { // rx 可以被当作迭代器使用，当通道关闭且无消息时循环结束
        println!("(主线程) 收到: {}", received_msg);
    }
    println!("(主线程) 通道已关闭，不再接收消息。");
    // 当所有发送端 (tx, tx1) 都被 drop 后，通道会关闭，rx 的迭代会结束。
}
```
`Sender<T>` 和 `Receiver<T>` 都是泛型类型，`T` 是要通过通道发送的数据的类型。
调用 `send` 方法会获取被发送值的所有权，并将其移动到通道中。

### 10.2.2 克隆发送端以实现多生产者

`mpsc` 通道允许有多个生产者。可以通过克隆 `Sender` 来创建多个发送端。

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main_multiple_producers() {
    let (tx, rx) = mpsc::channel();

    let tx_clone1 = tx.clone();
    thread::spawn(move || {
        tx_clone1.send(String::from("消息来自线程1")).unwrap();
    });

    let tx_clone2 = tx.clone(); // 也可以从原始 tx 克隆
    thread::spawn(move || {
        tx_clone2.send(String::from("消息来自线程2")).unwrap();
        thread::sleep(Duration::from_millis(100));
        tx_clone2.send(String::from("线程2的第二条消息")).unwrap();
    });

    // 主线程使用原始的 tx 发送
    thread::spawn(move || {
        tx.send(String::from("消息来自主线程派生的tx")).unwrap();
    });

    for received in rx {
        println!("(主线程) 收到: {}", received);
    }
}
```

## 10.3 共享状态并发

消息传递是处理并发的一种好方法，但并非唯一。另一种方法是让多个线程访问相同的共享数据。然而，共享可变数据是并发 bug 的主要来源（如数据竞争）。

Rust 使用**互斥锁 (Mutex, Mutual Exclusion)** 来允许在任何时候只有一个线程可以访问某些数据。要访问数据，线程必须首先获取互斥锁的**锁 (lock)**。当线程使用完受互斥锁保护的数据后，必须**释放锁 (unlock)**，以便其他线程可以获取它。

### 10.3.1 使用 `Mutex<T>` 允许多线程访问数据

`Mutex<T>` 是一个智能指针。更准确地说，它的 `lock` 方法返回一个智能指针 `MutexGuard<T>`，它实现了 `Deref` 来指向内部数据 `T`，并且在 `MutexGuard<T>` 离开作用域时会自动释放锁（通过 `Drop` trait）。

```rust
use std::sync::Mutex; // Mutex 来自 std::sync 而不是 std::thread
use std::thread;

fn main_mutex() {
    // Mutex<i32> 包装一个 i32 值
    let counter = Mutex::new(0); // counter 是 Mutex<i32>，不是 i32
    let mut handles = vec![];

    for _ in 0..10 { // 创建 10 个线程
        // 为了在线程间共享 counter (Mutex)，我们需要使用某种形式的引用计数。
        // 但是 Mutex<T> 本身并不实现 Copy，也不能直接移动到多个线程的闭包中。
        // 因此，我们需要 Arc<Mutex<T>> (原子引用计数)。
        // 这里为了简化，我们暂时不使用 Arc，而是演示 Mutex 的基本用法，
        // 但要注意这在实际多线程共享 Mutex 时是不完整的。
        // 稍后会结合 Arc。

        // 假设 counter 可以被某种方式安全地共享 (例如，如果它是一个 'static Mutex)
        // 或使用 Arc 来包装它。
        // let counter_clone = /* ... 如何共享 counter ... */;

        // 这里我们先只看 Mutex 的 lock 和 unlock 行为，暂时不考虑 Arc。
        // 这种写法实际上不能直接编译通过，因为 counter 不能被移动多次。
        // let handle = thread::spawn(move || {
        //     let mut num = counter.lock().unwrap(); // 获取锁，返回 MutexGuard<i32>
        //                                          // unwrap 是因为 lock 可能失败 (例如，如果持有锁的线程 panic 了)
        //     *num += 1; // 通过 MutexGuard 修改内部数据
        // }); // MutexGuard 在这里离开作用域，锁被自动释放
        // handles.push(handle);
    }

    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!("Mutex 示例结果 (不完整): counter = {:?}", counter.lock().unwrap());


    // --- 正确使用 Mutex 和 Arc 进行多线程共享 ---
    use std::sync::Arc; // Atomic Reference Counted

    let shared_counter = Arc::new(Mutex::new(0)); // Arc<Mutex<i32>>
    let mut arc_handles = vec![];

    for i in 0..10 {
        let counter_clone_for_thread = Arc::clone(&shared_counter); // 克隆 Arc，增加引用计数
        let handle = thread::spawn(move || {
            let mut num = counter_clone_for_thread.lock().unwrap(); // 获取锁
            *num += 1;
            println!("线程 {} 将计数器增加到 {}", i, *num);
        }); // 锁在这里释放
        arc_handles.push(handle);
    }

    for handle in arc_handles {
        handle.join().unwrap();
    }

    println!("最终计数器值 (Arc<Mutex<i32>>): {}", *shared_counter.lock().unwrap());
    // 结果应该是 10
}
```
`Mutex<T>` 可能会导致**死锁 (deadlock)**。当一个操作需要获取多个锁，并且两个或多个线程以不同的顺序获取这些锁时，就可能发生死锁。例如，线程 A 持有锁 L1 并等待 L2，而线程 B 持有锁 L2 并等待 L1。

### 10.3.2 `Arc<T>`：原子引用计数

`Arc<T>` (Atomic Reference Counted) 是 `Rc<T>` 的线程安全版本。它使用原子操作来管理引用计数，因此可以在多个线程之间安全地共享所有权。
当你需要让多个线程拥有指向同一数据的指针，并且这些线程可能会并发地增加或减少引用计数时，应该使用 `Arc<T>`。

上面 `Mutex` 的例子已经展示了 `Arc<Mutex<T>>` 的用法，这是在线程间共享可变数据的常见模式。

*   `Arc<T>` 用于共享所有权。
*   `Mutex<T>` 用于确保在任何时候只有一个线程可以修改数据。

## 10.4 `Send` 和 `Sync` Trait：可扩展的并发

Rust 的类型系统通过两个 trait 来保证并发安全：`Send` 和 `Sync`。

*   **`Send` Trait**：
    *   一个类型 `T` 如果是 `Send` 的，意味着该类型的值的所有权可以安全地在线程间**转移 (sent)**。
    *   几乎所有的 Rust 类型都是 `Send` 的。例外包括：
        *   原始指针 (`*const T`, `*mut T`)，因为它们不进行安全检查。
        *   `Rc<T>`，因为它不是线程安全的引用计数。
        *   `RefCell<T>`，因为它不是线程安全的内部可变性。
        *   包含非 `Send` 类型的结构体或枚举。
    *   `Arc<T>` 是 `Send` 的，前提是 `T` 也是 `Send` 和 `Sync` 的。

*   **`Sync` Trait**：
    *   一个类型 `T` 如果是 `Sync` 的，意味着该类型的不可变引用 `&T` 可以安全地在线程间**共享 (synchronized access)**。
    *   换句话说，如果 `&T` 是 `Send` 的，那么 `T` 就是 `Sync` 的。
    *   几乎所有的 Rust 类型也是 `Sync` 的。例外包括：
        *   `Cell<T>` 和 `RefCell<T>`，因为它们提供了非同步的内部可变性。
        *   `Rc<T>`，因为它不是线程安全的。
        *   包含非 `Sync` 类型的结构体或枚举。
    *   `Mutex<T>` 是 `Sync` 的，前提是 `T` 是 `Send` 的。（即使 `T` 不是 `Sync`，`Mutex<T>` 也可以是 `Sync`，因为它通过锁来同步访问）。

`Send` 和 `Sync` trait 是**自动 trait (auto traits)**。这意味着如果一个类型的所有组成部分都是 `Send` (或 `Sync`) 的，那么该类型本身也会自动成为 `Send` (或 `Sync`)。
你通常不需要手动实现这些 trait。它们主要由 Rust 编译器用来在编译时进行并发安全检查。

例如，`thread::spawn` 要求传递给它的闭包是 `Send` 的（因为闭包可能被发送到新线程执行），并且闭包捕获的任何变量的所有权也必须是 `Send` 的。

## 10.5 异步编程 (Async/Await) 简介

除了使用操作系统线程进行并发外，Rust 还支持**异步编程 (Asynchronous Programming)**，通常通过 `async` 和 `await` 关键字实现。
异步编程允许在单个线程上处理大量并发任务，而不会因为等待 I/O 操作（如网络请求、文件读写）而阻塞整个线程。

*   **`async fn`**：定义一个异步函数。调用异步函数会返回一个**Future**。Future 是一个代表尚未完成的计算的值。
*   **`.await`**：用在 Future 上，它会暂停当前异步函数的执行，直到 Future 完成。在等待期间，异步运行时 (runtime) 可以执行其他任务。

Rust 的 `async/await` 功能本身不提供执行 Future 的运行时。你需要选择一个异步运行时库，如 `tokio` 或 `async-std`。

```rust
// 这个例子需要一个异步运行时，如 tokio
// 在 Cargo.toml 中添加：
// [dependencies]
// tokio = { version = "1", features = ["full"] }

// async fn say_hello_async() {
//     println!("(Async) Hello from async function!");
//     // 模拟一个异步操作，比如网络请求
//     tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//     println!("(Async) Async operation complete.");
// }

// #[tokio::main] // tokio 提供的宏，用于设置异步 main 函数
// async fn main_async() {
//     println!("(Async Main) Starting...");
//
//     let future1 = say_hello_async();
//     let future2 = say_hello_async();
//
//     // 同时运行多个 future (并发)
//     futures::join!(future1, future2); // 需要 futures crate，或者 tokio::join!
//
//     println!("(Async Main) All async tasks finished.");
// }
```
异步编程是一个庞大的主题，通常用于构建高性能的网络服务和 I/O 密集型应用。它与基于线程的并发是互补的。

## 10.6 总结

Rust 提供了强大的并发原语，并通过其类型系统和所有权规则在编译时保证并发安全，防止了许多常见的并发 bug，如数据竞争。
*   使用 `thread::spawn` 创建线程。
*   使用消息传递 (`mpsc::channel`) 在线程间安全地通信。
*   使用共享状态 (`Mutex`, `Arc`) 在线程间安全地共享和修改数据。
*   `Send` 和 `Sync` trait 是 Rust 并发安全模型的基石。
*   异步编程 (`async/await`) 提供了另一种高效处理并发 I/O 的方式。

虽然 Rust 的并发模型有助于避免错误，但并发编程本身仍然是复杂的，需要仔细设计以避免死锁等逻辑问题。

## 10.7 常见陷阱

1.  **线程生命周期与数据所有权**：
    *   **陷阱**：创建的线程可能比其捕获的外部变量的引用活得更长，导致悬垂引用。
        ```rust
        // let data = vec![1,2,3];
        // thread::spawn(|| { // 编译错误，data 的引用可能无效
        //     println!("{:?}", data);
        // });
        ```
    *   **避免**：使用 `move` 关键字将数据的所有权转移到线程的闭包中。如果数据需要在主线程和新线程之间共享，使用 `Arc`。

2.  **`Mutex` 死锁**：
    *   **陷阱**：当多个线程需要获取多个 `Mutex` 锁，并且它们以不同的顺序获取这些锁时，可能导致死锁。例如，线程 A 锁住 M1 等待 M2，线程 B 锁住 M2 等待 M1。
    *   **避免**：
        *   始终以相同的固定顺序获取多个锁。
        *   尽量减少锁的持有时间。
        *   避免在持有锁时调用可能阻塞或再次尝试获取其他锁的外部代码。
        *   考虑使用更高级的并发原语或无锁数据结构（如果适用且有把握）。

3.  **`MutexGuard` 的生命周期**：
    *   **陷阱**：`Mutex::lock()` 返回的 `MutexGuard` 必须在作用域内，锁才会保持。如果 `MutexGuard` 被太早 `drop`（例如，赋值给 `_` 或者在一个过早结束的块中），锁会提前释放。
        ```rust
        // let my_mutex = Mutex::new(0);
        // {
        //     let mut guard = my_mutex.lock().unwrap();
        //     *guard += 1;
        // } // 锁在这里释放
        // // let _ = my_mutex.lock().unwrap(); // 如果这样写，锁会立即释放
        ```
    *   **避免**：确保 `MutexGuard` 的生命周期覆盖你需要保护的代码区域。通常，`MutexGuard` 的 RAII 特性会自动处理好。

4.  **在 `Arc<Mutex<T>>` 中忘记克隆 `Arc`**：
    *   **陷阱**：当需要将 `Arc<Mutex<T>>` 传递给多个线程时，必须克隆 `Arc` (使用 `Arc::clone()`)，而不是尝试移动原始的 `Arc`。原始的 `Arc` 只能被移动一次。
    *   **避免**：为每个需要共享访问的线程都创建一个 `Arc` 的克隆。`Arc::clone()` 只增加引用计数，不复制底层数据。

5.  **消息传递中的发送端 `drop` 行为**：
    *   **陷阱**：在 `mpsc` 通道中，如果所有 `Sender` 端都被 `drop` 了，`Receiver` 在尝试 `recv()` 时会收到一个错误（表明通道已关闭）。如果 `Receiver` 被 `drop` 了，`Sender` 在尝试 `send()` 时也会收到错误。
    *   **避免**：理解通道的生命周期。确保在需要通信时，发送端和接收端都处于活动状态。如果预期通道可能关闭，需要正确处理 `send()` 或 `recv()` 返回的 `Result`。

6.  **混淆 `Send` 和 `Sync`**：
    *   **陷阱**：不清楚 `Send` 和 `Sync` 的确切含义，可能导致在设计并发数据结构或 API 时出错。
    *   **避免**：
        *   `Send`：类型的值可以安全地从一个线程**发送**到另一个线程（所有权转移）。
        *   `Sync`：类型的不可变引用 `&T` 可以安全地在多个线程之间**共享**。
        *   大多数基本类型都是 `Send` 和 `Sync` 的。`Rc` 和 `RefCell` 不是 `Send` 或 `Sync`。`Mutex<T>` 是 `Sync` 如果 `T` 是 `Send`。

7.  **异步代码阻塞运行时**：
    *   **陷阱**：在 `async` 函数或 Future 中执行长时间运行的、阻塞的 CPU 密集型操作，或者调用阻塞的同步 I/O 函数，会阻塞执行该 Future 的异步运行时线程，影响其他并发任务的执行。
    *   **避免**：
        *   对于 CPU 密集型工作，使用 `tokio::task::spawn_blocking` (或类似 API) 将其移到专门的阻塞线程池中执行。
        *   始终使用异步版本的 I/O 操作（例如 `tokio::fs`, `tokio::net`）。

## 10.8 常见面试题

1.  **Q: Rust 如何保证并发安全？请提及所有权、`Send` 和 `Sync` trait。**
    *   **A:** Rust 通过其所有权系统和类型系统在编译时保证并发安全，主要防止两类并发 bug：数据竞争和悬垂指针。
        1.  **所有权系统**：
            *   确保任何数据在任何时候都只有一个所有者。这天然地防止了多个线程同时可变地访问同一数据（因为可变访问需要所有权或可变借用，而可变借用是独占的）。
            *   当数据的所有者离开作用域时，数据被自动清理，防止了悬垂指针。
        2.  **`Send` Trait**：
            *   标记一个类型的值的所有权可以被安全地从一个线程**转移 (sent)** 到另一个线程。
            *   如果一个类型 `T` 是 `Send`，意味着将 `T` 的值移动到另一个线程不会导致内存安全问题。
            *   大多数类型都是 `Send` 的。`Rc<T>` 和 `RefCell<T>` (及其包含它们的类型) 不是 `Send`，因为它们不是线程安全的。
        3.  **`Sync` Trait**：
            *   标记一个类型的不可变引用 `&T` 可以被安全地在多个线程之间**共享 (synchronized access)**。
            *   如果一个类型 `T` 是 `Sync`，意味着多个线程可以同时持有对 `T` 值的不可变引用而不会导致数据竞争。
            *   如果 `&T` 是 `Send`，那么 `T` 就是 `Sync`。
            *   大多数类型也是 `Sync` 的。`Cell<T>` 和 `RefCell<T>` 不是 `Sync`。`Mutex<T>` 是 `Sync` 如果 `T` 是 `Send`。
        *   编译器使用 `Send` 和 `Sync` trait 来静态地检查线程间的操作是否安全。例如，`thread::spawn` 要求闭包是 `Send` 的，闭包捕获的变量也必须是 `Send` 的。`Arc<T>` 要求 `T` 是 `Send + Sync` 才能在线程间安全共享。

2.  **Q: 解释消息传递并发模型和共享状态并发模型。Rust 中分别用什么原语实现它们？**
    *   **A:**
        *   **消息传递并发模型 (Message Passing)**：
            *   线程或 Actor 之间通过发送和接收消息进行通信，而不是直接共享内存。每个线程拥有自己的数据。
            *   这种模型有助于避免共享状态带来的复杂性和风险。 "Do not communicate by sharing memory; instead, share memory by communicating."
            *   **Rust 实现**：主要通过 `std::sync::mpsc` 模块提供的**通道 (channels)** 实现。`mpsc::channel()` 创建一个发送端 (`Sender<T>`) 和一个接收端 (`Receiver<T>`)。线程可以通过 `Sender` 发送数据（所有权转移），另一个线程可以通过 `Receiver` 接收数据。
        *   **共享状态并发模型 (Shared State)**：
            *   多个线程可以访问和修改同一块内存区域（共享数据）。
            *   这种模型需要同步机制来防止数据竞争和保证操作的原子性。
            *   **Rust 实现**：主要通过 `std::sync::Mutex<T>` (互斥锁) 和 `std::sync::RwLock<T>` (读写锁) 实现。
                *   `Mutex<T>`：确保在任何时候只有一个线程可以访问被包裹的数据 `T`。线程必须先获取锁，使用完数据后释放锁。
                *   通常与 `Arc<T>` (原子引用计数) 结合使用 (`Arc<Mutex<T>>`)，以允许多个线程安全地共享对 `Mutex` 的所有权。

3.  **Q: `Arc<T>` 和 `Rc<T>` 有什么区别？为什么在多线程环境下需要 `Arc<T>`？**
    *   **A:**
        *   **`Rc<T>` (Reference Counted)**：
            *   提供共享所有权，允许多个所有者指向同一份数据。通过引用计数跟踪所有者数量，当计数为零时数据被清理。
            *   `Rc::clone()` 只增加引用计数，不进行深拷贝。
            *   **非线程安全**：`Rc<T>` 的引用计数更新不是原子操作。如果在多个线程中同时修改 `Rc<T>` 的引用计数，可能导致数据竞争和不正确的计数，从而引发内存安全问题（如过早释放或内存泄漏）。因此，`Rc<T>` 没有实现 `Send` 和 `Sync` (如果 `T` 也没有的话)，不能安全地在线程间共享。
        *   **`Arc<T>` (Atomic Reference Counted)**：
            *   与 `Rc<T>` 功能类似，也提供共享所有权和引用计数。
            *   **线程安全**：`Arc<T>` 使用**原子操作**来更新引用计数。原子操作是不可分割的，可以保证在并发环境中的正确性。
            *   因此，`Arc<T>` 实现了 `Send` 和 `Sync` (如果 `T` 也实现了 `Send + Sync`)，可以安全地在线程间共享所有权。
        *   **为什么需要 `Arc<T>`**：在多线程环境中，如果需要多个线程共同拥有某个数据（例如，通过 `Arc<Mutex<T>>` 共享一个可变状态），就必须使用 `Arc<T>` 来管理对该数据的引用计数，以确保线程安全。直接使用 `Rc<T>` 会导致编译错误（因为它不是 `Send`/`Sync`）或潜在的运行时数据竞争。

4.  **Q: 什么是互斥锁 (`Mutex`)？它如何防止数据竞争？`MutexGuard` 的作用是什么？**
    *   **A:**
        *   **互斥锁 (`Mutex<T>`)**：是一种同步原语，用于保护共享数据，确保在任何给定时刻只有一个线程可以访问该数据。Mutex 代表 "Mutual Exclusion" (互斥)。
        *   **如何防止数据竞争**：
            1.  线程在访问受 `Mutex` 保护的数据之前，必须先尝试**获取锁 (lock)**。
            2.  如果锁已被其他线程持有，当前线程会阻塞，直到锁被释放。
            3.  一旦线程成功获取锁，它就可以独占地访问数据。
            4.  当线程完成对数据的操作后，它必须**释放锁 (unlock)**，以便其他等待的线程可以获取它。
            通过这种机制，`Mutex` 确保了对共享数据的访问是串行的，从而防止了多个线程同时修改数据导致的数据竞争。
        *   **`MutexGuard<T>` 的作用**：
            *   当调用 `Mutex<T>::lock()` 方法成功获取锁时，它返回一个 `Result<MutexGuard<T>, PoisonError>`。如果成功，`Ok` 变体中包含一个 `MutexGuard<T>`。
            *   `MutexGuard<T>` 是一个智能指针，它实现了 `Deref` 和 `DerefMut` trait，允许你像访问 `&T` 或 `&mut T` 一样访问被 `Mutex` 包裹的数据 `T`。
            *   **RAII (Resource Acquisition Is Initialization)**：`MutexGuard<T>` 最重要的作用是利用 RAII 原则自动管理锁的释放。当 `MutexGuard<T>` 离开作用域时（例如，函数返回或代码块结束），它的 `Drop` trait 实现会自动被调用，从而**自动释放互斥锁**。这大大减少了忘记释放锁导致死锁的风险。

5.  **Q: 什么是异步编程？Rust 中的 `async/await` 是如何工作的？它与基于线程的并发有何不同？**
    *   **A:**
        *   **异步编程 (Asynchronous Programming)**：是一种并发编程模型，允许程序在等待长时间操作（如 I/O 操作：网络请求、文件读写）完成时，不会阻塞当前线程，而是可以切换去执行其他任务。当等待的操作完成后，程序会回来继续处理结果。这使得单个线程可以高效地处理大量并发的、I/O 密集型的任务。
        *   **Rust 中的 `async/await`**：
            *   `async fn`：用于定义一个异步函数。调用 `async fn` 不会立即执行函数体，而是返回一个**Future**。Future 是一个代表未来某个时刻才会完成的计算的值。
            *   `.await`：是一个操作符，用于 Future 上。当在一个 `async` 函数中对一个 Future 使用 `.await` 时，如果该 Future 尚未完成，当前 `async` 函数的执行会暂停（而不是阻塞线程），并将控制权交还给异步运行时。异步运行时可以去调度执行其他准备好的任务。当被 `.await` 的 Future 完成后，运行时会恢复暂停的 `async` 函数的执行。
            *   **异步运行时 (Async Runtime)**：Rust 的 `async/await` 语法本身不包含执行 Future 的机制。你需要一个异步运行时库（如 `tokio`, `async-std`）来驱动 Future 的执行，管理任务调度和处理事件。
        *   **与基于线程的并发的不同**：
            1.  **资源消耗**：
                *   **线程并发**：每个线程都需要操作系统分配独立的栈空间和上下文切换开销。创建大量线程可能会消耗大量系统资源。
                *   **异步编程**：可以在单个或少量线程上管理成千上万个并发任务（Futures）。每个 Future 的状态机通常比整个线程的栈小得多，上下文切换（任务切换）也通常比线程切换轻量。
            2.  **阻塞行为**：
                *   **线程并发**：如果一个线程执行阻塞 I/O 操作，该线程会被操作系统挂起，无法执行其他工作，直到 I/O 完成。
                *   **异步编程**：当一个异步任务等待 I/O 时，它通过 `.await` 释放执行线程，允许该线程去执行其他异步任务。
            3.  **编程模型**：
                *   **线程并发**：代码通常是同步的、顺序的（在一个线程内）。线程间的协调需要显式的同步原语（如 `Mutex`, channels）。
                *   **异步编程**：代码涉及 `async/await`，看起来像同步代码，但执行流程是非阻塞的。需要异步生态系统（异步库、运行时）。
            4.  **适用场景**：
                *   **线程并发**：更适合 CPU 密集型任务（可以利用多核并行计算）或需要与操作系统进行深度交互的场景。
                *   **异步编程**：特别适合 I/O 密集型任务（如网络服务器、数据库客户端），因为可以高效处理大量并发连接而无需为每个连接都创建一个线程。
