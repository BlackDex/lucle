use rustls_pemfile::certs;
use std::path::Path;
use std::{fs::write, fs::File, io::BufReader};
use tokio_rustls::rustls::ServerConfig;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod database;
mod database_errors;
mod http;
mod infer_schema_internals;
pub mod models;
mod plugins;
mod print_schema;
mod query_helper;
//mod rpc;
pub mod schema;
mod user;
mod utils;

#[tokio::main]
async fn main() {
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
                .with_file(false),
        )
        .init();

    //load plugin
    plugins::verify_plugins().await;

    let ca_cert;
    let server_cert_key;

    if !Path::new(".tls/cert.pem").exists()
        || !Path::new(".tls/server_cert.pem").exists()
        || !Path::new(".tls/server_private_key.pem").exists()
    {
        ca_cert = utils::generate_ca_cert();

        match std::fs::create_dir_all(".tls") {
            Ok(_) => {}
            Err(err) => tracing::error!("{}", err),
        }
        match write(
            ".tls/ca_cert.pem",
            ca_cert.serialize_pem().unwrap().as_bytes(),
        ) {
            Ok(_) => {}
            Err(err) => tracing::error!("{}", err),
        }

        server_cert_key = utils::generate_server_cert_key(ca_cert);
        match write(".tls/server_cert.pem", server_cert_key.cert.as_bytes()) {
            Ok(_) => {}
            Err(err) => tracing::error!("{}", err),
        }
        match write(".tls/server_cert.crt", server_cert_key.cert.as_bytes()) {
            Ok(_) => {}
            Err(err) => tracing::error!("{}", err),
        }
        match write(
            ".tls/server_private_key.pem",
            server_cert_key.private_key.as_bytes(),
        ) {
            Ok(_) => {}
            Err(err) => tracing::error!("{}", err),
        }
    }
    let _cert_reader = BufReader::new(File::open(".tls/server_cert.pem").unwrap());

    let cert_file = File::open(".tls/server_cert.pem").unwrap();
    let mut cert_buf = BufReader::new(cert_file);
    let certs = certs(&mut cert_buf).map(|result| result.unwrap()).collect();

    let key_file = File::open(".tls/server_private_key.pem").unwrap();
    let mut key_buf = BufReader::new(key_file);
    let private_key = rustls_pemfile::private_key(&mut key_buf).unwrap().unwrap();

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, private_key)
        .unwrap();

    utils::save_cert_to_system_store();

    tokio::join!(
        http::serve(http::using_serve_dir(), config),
        // rpc::start_rpc_server(&mut cert_buf, &mut key_buf)
    )
    .0;
    {};
}
