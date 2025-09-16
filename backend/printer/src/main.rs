use axum::{routing::get, Router};
use backend::print::*;
use http::{header, Method};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/invitation", get(print_invitation))
        .route("/print", get(print_code));

    // let cors = CorsLayer::new()
    //     .allow_origin(Any)
    //     .allow_methods([Method::GET, Method::POST])
    //     .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);
    //
    // let app = app.layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
