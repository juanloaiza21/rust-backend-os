use crate::utils;

use axum::{
    Router,
    response::IntoResponse,
    routing::{get, post},
};

async fn hello_world() -> impl IntoResponse {
    "hello_world!"
}

// Example using your custom ApiResponse
async fn api_endpoint() -> impl IntoResponse {
    utils::intoresponse::ApiResponse::OK
}

pub fn init() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/api", get(api_endpoint))
}
