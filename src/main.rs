use axum::{
    body::Body,
    handler::HandlerWithoutStateExt,
    http::{Request, StatusCode},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower::ServiceExt;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Referencing https://github.com/tokio-rs/axum/blob/axum-v0.6.20/examples/static-file-server/src/main.rs
    tokio::join!(
        serve(using_serve_dir(), 8080),
        // serve(using_serve_dir_with_assets_fallback(), 3002),
        // serve(using_serve_dir_only_from_root_via_fallback(), 3003),
        // serve(using_serve_dir_with_handler_as_service(), 3004),
        // serve(two_serve_dirs(), 3005),
        // serve(calling_serve_dir_from_a_handler(), 3006),
    );
}

fn using_serve_dir() -> Router {
    // serve the file in the "assets" directory under `/assets`
    Router::new().nest_service("/", ServeDir::new("pomodoro-react/build"))
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();
}
