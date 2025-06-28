// tests/api_tests.rs

// 导入需要测试的 crate 中的项 (如果需要，但通常集成测试是黑盒的)
// use simple_api::*; // 通常不需要，因为我们通过 HTTP 调用

use reqwest::blocking::Client; // 使用 reqwest 的阻塞客户端进行简单测试
use reqwest::StatusCode;
use serde_json::{json, Value as JsonValue}; // 用于构建和比较 JSON
use std::time::Duration;
use std::net::SocketAddr;
use tokio::sync::oneshot; // 用于优雅关闭服务器

// 从 main.rs 复制或引用关键的结构体定义，以便测试时使用
// 更好的方式是如果这些结构体在 lib.rs 中定义，则可以直接 use crate_name::StructName
// 但由于这是个 bin crate，我们这里可以重新定义或假设它们与 main.rs 一致
#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
struct EchoPayload {
    message: String,
    count: i32,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
struct GreetingResponse {
    greeting: String,
}


// 辅助函数：在后台启动服务器并返回其地址和关闭句柄
// 注意：这个函数本身是 async 的，所以测试函数也需要是 async
async fn spawn_test_server() -> (SocketAddr, oneshot::Sender<()>) {
    let (tx, rx) = oneshot::channel(); // 用于发送关闭信号

    // 从 main.rs 复制路由逻辑，或者将其提取到 lib.rs 中共享
    // 为了简单，这里直接复制路由定义
    use axum::{
        routing::{get, post},
        Router,
        Json,
        extract::Path,
        response::Html,
        http::StatusCode as AxumStatusCode, // 避免与 reqwest::StatusCode 冲突
        response::IntoResponse,
    };

    async fn root_handler_test() -> Html<&'static str> {
        Html("<h1>欢迎来到 Axum 简单 API 服务!</h1><p>尝试访问 /hello, /greet/:name, 或 POST 到 /echo_json</p>")
    }
    async fn hello_handler_test() -> Json<GreetingResponse> {
        Json(GreetingResponse { greeting: "Hello, Web from Axum!".to_string() })
    }
    async fn greet_handler_test(Path(name): Path<String>) -> Json<GreetingResponse> {
        Json(GreetingResponse { greeting: format!("Hello, {}!", name) })
    }
    async fn echo_json_handler_test(Json(payload): Json<EchoPayload>) -> Json<EchoPayload> {
        Json(payload)
    }
    async fn handler_404_test() -> impl IntoResponse {
        (AxumStatusCode::NOT_FOUND, Html("<h2>404: 页面未找到</h2>"))
    }

    let app = Router::new()
        .route("/", get(root_handler_test))
        .route("/hello", get(hello_handler_test))
        .route("/greet/:name", get(greet_handler_test))
        .route("/echo_json", post(echo_json_handler_test))
        .fallback(handler_404_test);

    // 监听一个随机可用端口
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    println!("测试服务器正在监听: {}", addr);

    tokio::spawn(async move {
        axum::serve(listener, app.into_make_service())
            .with_graceful_shutdown(async {
                rx.await.ok(); // 等待关闭信号
                println!("测试服务器 (addr: {}) 正在关闭...", addr);
            })
            .await
            .unwrap();
    });

    // 给服务器一点时间启动
    tokio::time::sleep(Duration::from_millis(100)).await;

    (addr, tx)
}


#[tokio::test] // 使用 tokio 的测试宏
async fn test_root_endpoint() -> anyhow::Result<()> {
    let (addr, shutdown_tx) = spawn_test_server().await;
    let client = Client::new();
    let url = format!("http://{}", addr);

    let response = client.get(&url).send().await?;

    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.headers().get("content-type").unwrap().to_str()?.contains("text/html"));
    let body = response.text().await?;
    assert!(body.contains("欢迎来到 Axum 简单 API 服务!"));

    let _ = shutdown_tx.send(()); // 发送关闭信号
    Ok(())
}

#[tokio::test]
async fn test_hello_endpoint() -> anyhow::Result<()> {
    let (addr, shutdown_tx) = spawn_test_server().await;
    let client = Client::new();
    let url = format!("http://{}/hello", addr);

    let response = client.get(&url).send().await?;

    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.headers().get("content-type").unwrap().to_str()?.contains("application/json"));

    let expected_greeting = GreetingResponse { greeting: "Hello, Web from Axum!".to_string() };
    let actual_greeting: GreetingResponse = response.json().await?;

    assert_eq!(actual_greeting, expected_greeting);

    let _ = shutdown_tx.send(());
    Ok(())
}

#[tokio::test]
async fn test_greet_endpoint() -> anyhow::Result<()> {
    let (addr, shutdown_tx) = spawn_test_server().await;
    let client = Client::new();
    let name = "Rustacean";
    let url = format!("http://{}/greet/{}", addr, name);

    let response = client.get(&url).send().await?;

    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.headers().get("content-type").unwrap().to_str()?.contains("application/json"));

    let expected_greeting = GreetingResponse { greeting: format!("Hello, {}!", name) };
    let actual_greeting: GreetingResponse = response.json().await?;

    assert_eq!(actual_greeting, expected_greeting);

    let _ = shutdown_tx.send(());
    Ok(())
}

#[tokio::test]
async fn test_echo_json_endpoint() -> anyhow::Result<()> {
    let (addr, shutdown_tx) = spawn_test_server().await;
    let client = Client::new();
    let url = format!("http://{}/echo_json", addr);

    let payload_to_send = EchoPayload {
        message: "测试 JSON echo".to_string(),
        count: 123,
    };

    let response = client.post(&url)
        .json(&payload_to_send) // 发送 JSON 请求体
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.headers().get("content-type").unwrap().to_str()?.contains("application/json"));

    let echoed_payload: EchoPayload = response.json().await?;
    assert_eq!(echoed_payload, payload_to_send);

    let _ = shutdown_tx.send(());
    Ok(())
}

#[tokio::test]
async fn test_404_not_found() -> anyhow::Result<()> {
    let (addr, shutdown_tx) = spawn_test_server().await;
    let client = Client::new();
    let url = format!("http://{}/non_existent_path", addr);

    let response = client.get(&url).send().await?;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert!(response.headers().get("content-type").unwrap().to_str()?.contains("text/html"));
    let body = response.text().await?;
    assert!(body.contains("404: 页面未找到"));

    let _ = shutdown_tx.send(());
    Ok(())
}

// 更多测试可以添加，例如：
// - 测试无效的 JSON payload
// - 测试不同的 HTTP 方法 (如果路由支持)
// - 测试请求头
// - 测试错误处理 (如果 main.rs 中实现了 AppError 并用于 handler)
//   例如，发送一个空的 message 给 /echo_json (如果 _fallible_handler 被使用)
//   并期望一个 400 Bad Request 响应。
//
// fn create_app_for_test() -> Router { // 辅助函数创建测试用的 App Router
//     // ... (与 main.rs 或 spawn_test_server 中相同的路由定义)
//     // 这避免了在多个地方重复路由定义，如果路由逻辑复杂的话
// }
//
// 如果使用 tower::ServiceExt 进行内存测试 (更推荐的方式):
// use tower::ServiceExt; // for `oneshot` and `ready`
// use axum::body::Body;
// use axum::http::{Request, StatusCode as AxumStatusCode};
// use http_body_util::BodyExt; // for `to_bytes`
//
// #[tokio::test]
// async fn test_root_with_service_ext() {
//     // let app = create_app_for_test(); // 或者直接从 main.rs 导入 app() 函数
//     // ... (与 spawn_test_server 中相同的路由定义)
//     use axum::{routing::get, Router, response::Html, extract::Path, Json};
//     use serde::{Serialize, Deserialize};
//     #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
//     struct EchoPayloadTest { message: String, count: i32 }
//     #[derive(Serialize, Deserialize, Debug, PartialEq)]
//     struct GreetingResponseTest { greeting: String }
//     async fn root_handler_service() -> Html<&'static str> { Html("<h1>Welcome</h1>") }
//     let app = Router::new().route("/", get(root_handler_service));


//     let response = app
//         .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
//         .await
//         .unwrap();
//     assert_eq!(response.status(), AxumStatusCode::OK);
//     let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
//     assert!(String::from_utf8_lossy(&body_bytes).contains("<h1>Welcome</h1>"));
// }
// 这种方式更快，因为它不涉及实际的网络堆栈，并且更容易控制。
// 但它可能无法完全模拟真实网络环境中的某些行为。
// 对于本教程，使用 reqwest 的外部测试方式更直观地展示了 HTTP API 的行为。
