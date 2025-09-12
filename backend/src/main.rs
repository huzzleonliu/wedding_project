use axum::{routing::get, Router};
use backend::print::*;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/invitation", get(print_invitation))
        .route("/print", get(print_code));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8085));
    println!("Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
