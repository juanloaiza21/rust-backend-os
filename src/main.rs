use axum::serve;
mod data;
mod router_local;
mod utils;
use std::time::Instant;

const CSV_PATH: &str = "src/data/data.csv";
const PORT: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    let app = router_local::init();
    let start = Instant::now();
    match data::filters::initialize_hash_index(CSV_PATH) {
        Ok(count) => println!(
            "Índice hash inicializado con {} registros en {:?}",
            count,
            start.elapsed()
        ),
        Err(e) => {
            eprintln!("Error al inicializar índice hash: {}", e);
            return;
        }
    }
    let listner = tokio::net::TcpListener::bind(PORT).await.unwrap();
    println!("Server running on {}", PORT);
    serve(listner, app).await.unwrap();
}
