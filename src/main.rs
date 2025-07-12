use axum::{Router, response::IntoResponse, response::IntoResponseParts, routing::get};

async fn hello_world() -> impl IntoResponse {
    "hello_world!"
}

fn init_router() -> Router {
    Router::new().route("/", get(hello_world))
}

#[tokio::main]
async fn main() {
    let app = init_router();
    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listner, app).await.unwrap();
}
