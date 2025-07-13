use axum::serve;
mod data;
mod router_local;
mod utils;
use std::env;
use std::time::Instant;

const CSV_PATH: &str = "src/data/data.csv";
#[tokio::main]
async fn main() {
    // Inicializar índice hash
    let start = Instant::now();
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
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

    // Iniciar la app
    let app = router_local::init();

    // Crear el listener con Tokio
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Server running on {}", addr);

    // En Axum 0.8.x, la función serve() funciona así:
    serve(listener, app).await.unwrap();
}
