use rustls_pemfile::{certs, pkcs8_private_keys};
use std::path::{Path, PathBuf};
use std::{
    fs::write,
    fs::File,
    io::{BufReader, Read},
};
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::rustls::{Certificate, PrivateKey};
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

    //let dir = openssl_probe::probe().cert_dir.unwrap();
    let dir = PathBuf::from("/usr/lib/ssl/certs/ca-certificates.crt");
    tracing::info!(
        "ssl path : {}",
        dir.clone().into_os_string().into_string().unwrap()
    );

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
        match write(
            ".tls/server_private_key.pem",
            server_cert_key.private_key.as_bytes(),
        ) {
            Ok(_) => {}
            Err(err) => tracing::error!("{}", err),
        }
    }
    let mut cert_reader = BufReader::new(File::open(".tls/server_cert.pem").unwrap());

    let cert_file = File::open(".tls/server_cert.pem").unwrap();
    let mut cert_buf = BufReader::new(cert_file);
    let certs = certs(&mut cert_reader)
        .map(Certificate)
        .collect();

    let key_file = File::open(".tls/server_private_key.pem").unwrap();
    let mut key_buf = BufReader::new(key_file);
    let mut private_key = PrivateKey(pkcs8_private_keys(&mut key_buf).unwrap().remove(0));

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, private_key)
        .unwrap();

//    if let Err(e) = utils::save_cert_to_system_store(bytes) {
//        tracing::error!("error when saving cert into system store : {}", e);
//    } else {
//        tracing::info!("Adding certificate into system store successful !");
//    }

    tokio::join!(
        http::serve(http::using_serve_dir(), config),
        // rpc::start_rpc_server(&mut cert_buf, &mut key_buf)
    )
    .0;
    {};
}
