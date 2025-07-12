use axum::{Router, routing::get, serve::Listener};

#[tokio::main]
async fn main() {
    //Build the app with just one route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
