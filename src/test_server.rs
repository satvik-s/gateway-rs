use std::net::SocketAddr;

use axum::{routing, Json, Router};
use serde_json::json;
use tracing::info;

pub async fn server() {
    let app = Router::new().route(
        "/server-route-1",
        routing::get(|| async { Json(json! ({"a": 1})) }),
    ).route(
        "/server-route-2",
        routing::get(|| async { Json(json! ({"b": 2})) }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
