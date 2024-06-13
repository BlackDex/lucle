use axum::Router;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

pub fn serve_dir() -> Router {
    let serve_dir = ServeDir::new("web/dist").fallback(ServeFile::new("web/dist/index.html"));

    Router::new()
        .nest_service("/", serve_dir.clone())
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http())
} 
