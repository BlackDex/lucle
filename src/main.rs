use dlopen2::wrapper::{Container, WrapperApi};
use minisign_verify::{PublicKey, Signature};
use std::path::{Path, PathBuf};
use std::{fs::write, fs::File, io::BufReader};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod database;
mod database_errors;
mod http;
mod infer_schema_internals;
pub mod models;
mod print_schema;
mod query_helper;
mod rpc;
pub mod schema;
mod user;
mod utils;

#[derive(WrapperApi)]
struct Api {
    smtp_server: fn(),
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
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

    //TODO: create a module for load plugin
    let public_key =
        PublicKey::from_base64("RWQf6LRCGA9i53mlYecO4IzT51TGPpvWucNSCh1CBM0QTaLn73Y7GFO3")
            .expect("Unable to decode the public key");
    let signature = Signature::decode(
     "untrusted comment: signature from minisign secret key
        RWQf6LRCGA9i59SLOFxz6NxvASXDJeRtuZykwQepbDEGt87ig1BNpWaVWuNrm73YiIiJbq71Wi+dP9eKL8OC351vwIasSSbXxwA=
        trusted comment: timestamp:1555779966\tfile:test
        QtKMXWyYcwdpZAlPF7tE2ENJkRd1ujvKjlj1m9RtHTBnZPa5WKU5uWRs5GoP5M/VqE81QFuMKI5k/SfNQUaOAA==",
            ).expect("Unable to decode the signature");
    let bin = b"test";
    public_key
        .verify(bin, &signature, false)
        .expect("Signature didn't verify");

    let container: Container<Api> = unsafe { Container::load("libexample.so") }
        .expect("Could not open library or load symbols");
    container.smtp_server();

    //    let dir = openssl_probe::probe().cert_dir.unwrap();
    //    tracing::info!("ssl path : {}", dir.into_os_string().into_string().unwrap());

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
    let key_file = File::open(".tls/server_private_key.pem").unwrap();
    key_buf = BufReader::new(key_file);

    match tokio::join!(
        http::serve(
            http::using_serve_dir(),
            PathBuf::from(".tls/server_cert.pem"),
            PathBuf::from(".tls/server_private_key.pem")
        ),
        rpc::start_rpc_server(&mut cert_buf, &mut key_buf)
    )
    .0
    {
        _ => {}
    };
}
