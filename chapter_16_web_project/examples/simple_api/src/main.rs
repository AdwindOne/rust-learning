use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::Path,
    response::{Html, IntoResponse},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt}; // 可选日志

// --- 数据结构 (用于 JSON) ---
#[derive(Serialize, Deserialize, Debug, Clone)] // Clone 用于测试时的方便
struct EchoPayload {
    message: String,
    count: i32,
}

#[derive(Serialize, Debug)]
struct GreetingResponse {
    greeting: String,
}

// --- 主函数和服务器设置 ---
#[tokio::main]
async fn main() {
    // (可选) 初始化日志和追踪
    // tracing_subscriber::registry()
    //     .with(
    //         tracing_subscriber::EnvFilter::try_from_default_env()
    //             .unwrap_or_else(|_| "simple_api=debug,tower_http=debug".into()),
    //     )
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();
    // println!("日志已初始化 (如果启用了 tracing)。");


    // 构建我们的应用路由
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/hello", get(hello_handler))
        .route("/greet/:name", get(greet_handler))
        .route("/echo_json", post(echo_json_handler))
        .fallback(handler_404); // 添加一个 404 fallback处理器
        // .layer(tower_http::trace::TraceLayer::new_for_http()); // 可选的 HTTP 请求追踪中间件

    // 运行服务器
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Axum 服务器正在监听 http://{}", addr);
    // tracing::debug!("服务器正在监听 {}", addr); // 如果使用 tracing

    // axum::Server::bind(&addr) // 旧版 axum (如 0.6)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    // axum 0.7+ 使用 axum::serve
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

// --- 路由处理函数 (Handlers) ---

// GET /
async fn root_handler() -> Html<&'static str> {
    println!("处理 GET / 请求");
    Html("<h1>欢迎来到 Axum 简单 API 服务!</h1><p>尝试访问 /hello, /greet/:name, 或 POST 到 /echo_json</p>")
}

// GET /hello
async fn hello_handler() -> Json<GreetingResponse> {
    println!("处理 GET /hello 请求");
    Json(GreetingResponse {
        greeting: "Hello, Web from Axum!".to_string(),
    })
}

// GET /greet/:name
async fn greet_handler(Path(name): Path<String>) -> Json<GreetingResponse> {
    println!("处理 GET /greet/{} 请求", name);
    Json(GreetingResponse {
        greeting: format!("Hello, {}!", name),
    })
}

// POST /echo_json
async fn echo_json_handler(Json(payload): Json<EchoPayload>) -> Json<EchoPayload> {
    println!("处理 POST /echo_json 请求, payload: {:?}", payload);
    Json(payload) // 直接将解析后的 payload 返回为 JSON
}

// Fallback 处理器 (404 Not Found)
async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Html("<h2>404: 页面未找到</h2>"))
}


// --- (可选) 自定义错误处理 ---
// 如果 handler 返回 Result<T, AppError>，可以定义 AppError 并实现 IntoResponse
// enum AppError {
//     InternalServerError(String),
//     BadRequest(String),
//     NotFound,
// }

// impl IntoResponse for AppError {
//     fn into_response(self) -> axum::response::Response {
//         let (status, error_message) = match self {
//             AppError::InternalServerError(msg) => {
//                 eprintln!("服务器内部错误: {}", msg); // 记录到服务器日志
//                 (StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误".to_string())
//             }
//             AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, format!("错误的请求: {}", msg)),
//             AppError::NotFound => (StatusCode::NOT_FOUND, "资源未找到".to_string()),
//         };
//         (status, Json(serde_json::json!({"error": error_message}))).into_response()
//     }
// }

// // 允许 anyhow::Error 转换为 AppError (如果使用 anyhow)
// // impl From<anyhow::Error> for AppError {
// //     fn from(err: anyhow::Error) -> Self {
// //         AppError::InternalServerError(err.to_string())
// //     }
// // }

// // 示例：一个可能失败的 handler
// async fn _fallible_handler(Json(payload): Json<EchoPayload>) -> Result<Json<EchoPayload>, AppError> {
//     if payload.message.is_empty() {
//         return Err(AppError::BadRequest("消息不能为空".to_string()));
//     }
//     if payload.count < 0 {
//         // 模拟一个内部错误
//         // let _io_err = std::fs::read_to_string("nonexistent")
//         //     .map_err(|e| AppError::InternalServerError(format!("IO 错误: {}",e)))?;
//         return Err(AppError::InternalServerError("计数不能为负 (模拟)".to_string()));
//     }
//     Ok(Json(payload))
// }


// --- 用于测试的辅助函数 (如果测试代码需要启动服务器) ---
// 此函数可以在测试模块中使用，以编程方式启动和停止服务器。
// 为了本示例的简单性，集成测试将假设服务器已独立运行或使用测试库启动。
//
// use tokio::sync::oneshot;
// pub async fn run_test_server() -> (SocketAddr, oneshot::Sender<()>) {
//     let (tx, rx) = oneshot::channel(); // 用于优雅关闭
//     let app = Router::new() // ... 定义与 main 中相同的路由 ...
//         .route("/", get(root_handler))
//         .route("/hello", get(hello_handler))
//         .route("/greet/:name", get(greet_handler))
//         .route("/echo_json", post(echo_json_handler))
//         .fallback(handler_404);

//     let addr = SocketAddr::from(([127, 0, 0, 1], 0)); // 0 表示随机选择可用端口
//     let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
//     let actual_addr = listener.local_addr().unwrap();
//     println!("测试服务器正在监听 http://{}", actual_addr);

//     tokio::spawn(async move {
//         axum::serve(listener, app.into_make_service())
//             .with_graceful_shutdown(async {
//                 rx.await.ok(); // 等待关闭信号
//                 println!("测试服务器正在关闭...");
//             })
//             .await
//             .unwrap();
//     });

//     (actual_addr, tx)
// }
