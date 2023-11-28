
use std::path::{Path, PathBuf};
use std::{fs::write, fs::File, io::{BufReader, Read}};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use rustls_pemfile::{certs, rsa_private_keys};
use tokio_rustls::rustls::{PrivateKey, Certificate};
use rustls_pemfile::pkcs8_private_keys;
use tokio_rustls::rustls::ServerConfig;

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
    tracing::info!("ssl path : {}", dir.clone().into_os_string().into_string().unwrap());


    /*let mut child = std::process::Command::new("ls").uid(0).spawn().expect("failed to execute child");
    let stdout = child.stdout.take().unwrap();
    tracing::info!("{:?}", stdout);*/

    let ca_cert;
    let server_cert_key;
    let mut cert_buf: BufReader<File>;
    let mut key_buf: BufReader<File>;

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
    let cert_file = File::open(".tls/server_cert.pem").unwrap();
    cert_buf = BufReader::new(cert_file);
    let cert_chain = certs(&mut cert_buf).unwrap().into_iter().map(Certificate).collect();
    let mut bytes = Vec::new();
    cert_buf.read_to_end(&mut bytes).expect("Unable to read data");
    
    let key_file = File::open(".tls/server_private_key.pem").unwrap();
    key_buf = BufReader::new(key_file.try_clone().unwrap());
    let mut private_keys = pkcs8_private_keys(&mut BufReader::new(key_file)).unwrap();
    let private_key = private_keys.remove(0);
    let tokio_private_key = PrivateKey(private_key);

    let config = ServerConfig::builder()
    .with_safe_defaults()
    .with_no_client_auth()
    .with_single_cert(cert_chain, tokio_private_key).unwrap();

    if let Err(e) = utils::save_cert_to_system_store(bytes) {
        tracing::error!("error when saving cert into system store : {}", e);
    } else {
        tracing::info!("Adding certificate into system store successful !");
    }

    tokio::join!(
        http::serve(
            http::using_serve_dir(),
            config
        ),
       // rpc::start_rpc_server(&mut cert_buf, &mut key_buf)
    )
    .0;
    {};
}
