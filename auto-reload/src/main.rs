use axum::{response::Html, routing::get, Router};
use listenfd::ListenFd;
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    // 创建一个路由器，并添加一个路由
    let app = Router::new()
        .route("/", get(handler));
    // 从环境变量中获取监听器
    let mut listenfd = ListenFd::from_env();

    // 获取监听器，如果获取不到，则使用本地监听
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        Some(listener) => {
            // 设置监听器为非阻塞模式
            listener.set_nonblocking(true).unwrap();
            // 将监听器转换为TcpListener
            TcpListener::from_std(listener).unwrap()
        }
        // otherwise fall back to local listening
        None => TcpListener::bind("127.0.0.1:3000").await.unwrap(),
    };

    // 运行应用
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// 处理请求，返回一个HTML响应
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, Changyou!</h1>")
}