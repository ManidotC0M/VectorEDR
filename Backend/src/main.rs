mod models;
mod handlers;
mod routes;

use std::net::SocketAddr;
use axum::Router;

#[tokio::main]
async fn main() {
    let app: Router = routes::create_routes();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Backend listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
