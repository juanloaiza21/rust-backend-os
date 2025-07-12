use crate::utils;

use axum::{
    Router,
    response::IntoResponse,
    routing::{get, post},
};

/*
* Las dos primeras peticiones son para validar que este funcionando correctamente la API.
* Las llamadas posteriores tienen como finalidad cumplir el proyecto.
* */

async fn hello_world() -> impl IntoResponse {
    "hello_world!"
}

async fn api_endpoint() -> impl IntoResponse {
    utils::intoresponse::ApiResponse::OK
}

pub fn init() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/api", get(api_endpoint))
}
