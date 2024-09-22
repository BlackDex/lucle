use axum::Router;
use std::net::SocketAddr;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

pub async fn serve_dir() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let local_addr = listener.local_addr().unwrap();
    tracing::info!("HTTP listening on {local_addr}");

    let serve_dir = ServeDir::new("web/dist").fallback(ServeFile::new("web/dist/index.html"));

    let app = Router::new()
        .nest_service("/", serve_dir.clone())
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http());

    axum::serve(listener, app).await.unwrap();
}
