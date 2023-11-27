use axum::Router;

use std::{net::SocketAddr, path::PathBuf};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

pub fn using_serve_dir() -> Router {
    let serve_dir = ServeDir::new("web/dist").fallback(ServeFile::new("web/dist/index.html"));

    Router::new()
        .nest_service("/", serve_dir.clone())
        .fallback_service(serve_dir)
}

pub async fn serve(app: Router, _cert: PathBuf, _key: PathBuf) {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("HTTP server starting on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}
