use axum::Router;
use std::net::SocketAddr;
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

pub async fn serve_http(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("HTTP server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();
}

/* pub async fn serve_https(app: Router, rustls_config: ServerConfig) {
    let tls_acceptor = TlsAcceptor::from(Arc::new(rustls_config));
    let bind = "[::1]:8080";
    let tcp_listener = TcpListener::bind(bind).await.unwrap();
    tracing::info!("HTTPS server listening on {}", bind);

    pin_mut!(tcp_listener);
    loop {
        let tower_service = app.clone();
        let tls_acceptor = tls_acceptor.clone();

        let (cnx, addr) = tcp_listener.accept().await.unwrap();

        tokio::spawn(async move {
            let Ok(stream) = tls_acceptor.accept(cnx).await else {
                tracing::error!("error during tls handshake connection from {}", addr);
                return;
            };

            let stream = TokioIo::new(stream);

            let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
                tower_service.clone().call(request)
            });

            let ret = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new())
                .serve_connection_with_upgrades(stream, hyper_service)
                .await;

            if let Err(err) = ret {
                tracing::warn!("error serving connection from {}: {}", addr, err);
            }
        });
    }
} */
