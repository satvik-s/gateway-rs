mod test_server;
mod types;
mod utils;

use axum::{
    extract::Extension,
    handler::Handler,
    http::{uri::Uri, Request, Response},
    response::IntoResponse,
    routing, AddExtensionLayer, Router,
};
use hyper::{client::Client, client::HttpConnector, Body, Method, StatusCode};
use std::sync::Arc;
use tokio::signal;
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{info, instrument, Level};
use types::{Config, State};

const ADDRESS: &str = "127.0.0.1:4000";

#[instrument]
#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::init();
    tokio::spawn(test_server::server());

    let conf_str = std::fs::read_to_string("src/config.toml").unwrap();
    let conf: Config = toml::from_str(&conf_str).unwrap();
    let processed_config = utils::get_processed_config(&conf);

    let mut app = Router::new();

    for i in processed_config.routes.values() {
        app = match i.method.to_lowercase().as_str() {
            "get" => app.route(&i.path, routing::get(handler)),
            "post" => app.route(&i.path, routing::post(handler)),
            _ => panic!("method no matched"),
        };
    }

    let state = State {
        client: Client::<HttpConnector, Body>::new(),
        processed_config,
    };
    app = app
        .layer(AddExtensionLayer::new(Arc::new(state)))
        .layer(CompressionLayer::new())
        .layer(CatchPanicLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new())
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Micros),
                ),
        )
        .fallback(handler_404.into_service());

    info!("reverse proxy listening on {}", ADDRESS);

    axum::Server::bind(&ADDRESS.parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn handler(
    Extension(state): Extension<Arc<State>>,
    mut req: Request<Body>,
) -> Response<Body> {
    let path = req.uri().path();
    let query = req.uri().query().unwrap_or_default();
    // let path_query = req.uri().path_and_query().map(|v| v.as_str()).unwrap();

    let matched_route_and_server =
        utils::get_matched_route_and_server(path, &state.processed_config).unwrap();

    let uri = match query {
        "" => format!(
            "{}{}",
            matched_route_and_server.server.endpoint, matched_route_and_server.route.server_path
        ),
        _ => format!(
            "{}{}?{}",
            matched_route_and_server.server.endpoint,
            matched_route_and_server.route.server_path,
            query
        ),
    };

    *req.uri_mut() = Uri::try_from(uri).unwrap();

    let response = state.client.request(req).await.unwrap();

    return response;
}

async fn handler_404(method: Method, uri: Uri) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        format!("no route configured for {} {}", method, uri),
    )
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}
