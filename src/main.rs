use axum::{http::StatusCode, response::IntoResponse, routing::get_service, Router};
use std::{io, net::SocketAddr};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod database;
mod database_errors;
mod query_helper;
mod rpc;

#[tokio::main]
async fn main() {
    //database::setup_database("sqlite");
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_writer(std::io::stdout)
                .with_target(false)
                .with_ansi(true)
                .with_line_number(false)
                .with_file(false)
        )
        .init();

        tokio::join!(serve(using_serve_dir(), 8080), rpc::start_rpc_server());
}

fn using_serve_dir() -> Router {
    let serve_dir = ServeDir::new("web/dist").fallback(ServeFile::new("web/dist/index.html"));
    let serve_dir = get_service(serve_dir).handle_error(handle_error);

    Router::new()
        .nest_service("/", serve_dir.clone())
        .fallback_service(serve_dir)
}

async fn handle_error(err: io::Error) -> impl IntoResponse {
    dbg!(err);
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("HTTP server starting on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();
}
