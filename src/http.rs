use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use std::{net::SocketAddr, path::PathBuf};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

#[derive(Clone, Copy)]
struct Ports {
    http: u16,
    https: u16,
}

pub fn using_serve_dir() -> Router {
    let serve_dir = ServeDir::new("web/dist").fallback(ServeFile::new("web/dist/index.html"));

    Router::new()
        .nest_service("/", serve_dir.clone())
        .fallback_service(serve_dir)
}

pub async fn serve(app: Router, cert: PathBuf, key: PathBuf) {
    /*    let acceptor = state.axum_acceptor(state.default_rustls_config());

    tokio::spawn(async move {
        loop {
            match state.next().await.unwrap() {
                Ok(ok) => tracing::info!("{:?}", ok),
                Err(err) => tracing::error!("{:?}", err),
            }
        }
    });*/

    //let config = RustlsConfig::from_pem_file(cert, key).await.unwrap();

    let ports = Ports {
        http: 8080,
        https: 8443,
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], ports.http));
    tracing::info!("HTTP server starting on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();
    /*match axum_server::bind(addr).acceptor(acceptor).serve(app.into_make_service()).await {
     Ok(_) => {},
     Err(err) => tracing::error!("{}", error)
    }*/
}
