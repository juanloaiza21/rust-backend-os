use crate::data::{get_trips_by_destination, get_trips_by_index, get_trips_by_price_range};
use crate::utils;

mod data_intput_struct;
mod trip_rorutes;

use axum::{Router, http::Method, response::IntoResponse, routing::get};
use serde_json::json;
use std::time::Instant;
use std::{net::SocketAddr, sync::Arc};

use tower_http::cors::{Any, CorsLayer};

/*
* Las dos primeras peticiones son para validar que este funcionando correctamente la API.
* Las llamadas posteriores tienen como finalidad cumplir el proyecto.
* */

pub struct AppState {
    start_time: Instant,
}

async fn hello_world() -> impl IntoResponse {
    "hello_world!"
}

async fn api_endpoint() -> impl IntoResponse {
    utils::intoresponse::ApiResponse::OK
}

pub fn init() -> Router {
    let _ = env_logger::try_init(); // Usamos try_init en lugar de init para evitar pánico si ya está inicializado

    let state = Arc::new(AppState {
        start_time: Instant::now(),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    Router::new()
        .route("/", get(hello_world))
        .route("/api", get(api_endpoint))
        .nest("/trip", trip_rorutes::routes())
        .with_state(state)
        .layer(cors)
}
