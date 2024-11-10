use axum::Router;
use anyhow;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;

#[tokio::main]
async fn main() {
    // 创建一个路由器，并添加一个路由
    let app = Router::new()
        .route("/", get(handler));

    // 绑定监听地址
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878")
        .await
        .unwrap();

    // 打印监听地址
    println!("listening on {}", listener.local_addr().unwrap());

    // 启动服务器
    axum::serve(listener, app).await.unwrap();
}

// 定义一个错误类型
struct AppError(anyhow::Error);


// 定义一个处理函数
async fn handler() -> Result<(), AppError> {
    // 调用try_thing函数
    try_thing()?;
    Ok(())
}

// 定义一个try_thing函数，返回Result类型
fn try_thing() -> Result<(), anyhow::Error> {
    // 抛出一个错误
    anyhow::bail!("it failed!")
}

// 实现IntoResponse trait，将AppError转换为Response类型
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 返回一个包含错误信息的Response
        // self.0 就是 appError
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        ).into_response()
    }
}

// 实现From trait，将E类型转换为AppError类型
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        // 将E类型转换为anyhow::Error类型，并封装为AppError类型
        Self(error.into())
    }
}