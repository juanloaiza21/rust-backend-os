use axum::{
    Router,
    response::IntoResponse,
    routing::{get, post},
};
async fn hello_world() -> impl IntoResponse {
    "hello_world!"
}

pub fn init() -> Router {
    Router::new().route("/", get(hello_world))
}
