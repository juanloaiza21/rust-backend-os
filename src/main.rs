use axum::serve;
mod router_local;
mod utils;

#[tokio::main]
async fn main() {
    let port = "0.0.0.0:3000";
    let app = router_local::init();
    let listner = tokio::net::TcpListener::bind(port).await.unwrap();
    println!("Server running on {}", port);
    serve(listner, app).await.unwrap();
}
