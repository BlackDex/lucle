use jsonwebtoken::{encode, EncodingKey, Header};
use lettre::{
    message::{header, MultiPart, SinglePart},
    FileTransport, Message, Transport,
};
use rcgen::{Certificate, DnType, KeyUsagePurpose};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, process::Command};
use tera::{Context, Tera};
use time::{Duration, OffsetDateTime};
use users::{get_effective_uid, get_user_by_uid};
use which::which;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    sub: String,
    company: String,
    exp: usize,
}

pub fn send_mail(from: &str, dest: &str, subject: &str, _body: &str) {
    let context = Context::new();
    let tera = match Tera::new("templates/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let rendered_content = tera.render("email.html", &context).unwrap();

    let email = Message::builder()
        .from(from.parse().unwrap())
        .to(dest.parse().unwrap())
        .subject(subject)
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(String::from("Hello from Lettre! A mailer library for Rust")), // Every message should have a plain text fallback.
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(rendered_content),
                ),
        )
        .unwrap();

    let mailer = FileTransport::new("./");

    // Store the message when you're ready.
    mailer.send(&email).expect("failed to deliver message");
}

pub struct TlsServer {
    pub cert: String,
    pub private_key: String,
}

pub fn generate_ca_cert() -> Certificate {
    let mut ca_params = rcgen::CertificateParams::new(Vec::new());
    let (yesterday, tomorrow) = validity_period();
    ca_params
        .distinguished_name
        .push(DnType::OrganizationName, "Rustls Server Acceptor");
    ca_params
        .distinguished_name
        .push(DnType::CommonName, "Example CA");
    ca_params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
    ca_params.key_usages = vec![
        KeyUsagePurpose::KeyCertSign,
        KeyUsagePurpose::DigitalSignature,
        KeyUsagePurpose::CrlSign,
    ];
    ca_params.not_before = yesterday;
    ca_params.not_after = tomorrow;
    Certificate::from_params(ca_params).unwrap()
}

pub fn generate_server_cert_key(ca_cert: Certificate) -> TlsServer {
    let mut server_ee_params = rcgen::CertificateParams::new(vec!["localhost".to_string()]);
    server_ee_params.is_ca = rcgen::IsCa::NoCa;
    let (yesterday, tomorrow) = validity_period();
    server_ee_params
        .distinguished_name
        .push(DnType::CommonName, "localhost");
    server_ee_params.use_authority_key_identifier_extension = true;
    server_ee_params
        .key_usages
        .push(KeyUsagePurpose::DigitalSignature);
    server_ee_params.not_before = yesterday;
    server_ee_params.not_after = tomorrow;
    let server_cert = Certificate::from_params(server_ee_params).unwrap();
    let server_cert_string = server_cert.serialize_pem_with_signer(&ca_cert).unwrap();
    let server_key_string = server_cert.serialize_private_key_pem();
    TlsServer {
        cert: server_cert_string,
        private_key: server_key_string,
    }
}

fn validity_period() -> (OffsetDateTime, OffsetDateTime) {
    let day = Duration::new(86400, 0);
    let yesterday = OffsetDateTime::now_utc().checked_sub(day).unwrap();
    let tomorrow = OffsetDateTime::now_utc().checked_add(day).unwrap();
    (yesterday, tomorrow)
}

pub fn generate_jwt(username: String, email: String) -> String {
    let key = b"secret";
    let my_claims = Claims {
        aud: username.to_owned(),
        sub: email.to_owned(),
        company: "lucle".to_owned(),
        exp: 10000000000,
    };
    match encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(key),
    ) {
        Ok(t) => t,
        Err(_) => panic!(),
    }
}

pub fn save_cert_to_system_store() {
    let mut sudo = "";
    let euid = get_effective_uid();

    if let Some(user) = get_user_by_uid(euid) {
        if user.uid() != 0 {
            if which("sudo").is_ok() {
                sudo = "sudo"
            } else {
                tracing::error!("sudo is not installed")
            }
        }
    }

    if let Ok(result) = Command::new(sudo)
        .arg("cp")
        .arg(".tls/ca_cert.pem")
        .arg("/usr/local/share/ca-certificates/ca_cert.crt")
        .output()
    {
        if result.status.success() {
            tracing::info!("CA Cert copied successfully to certificates path");
        } else {
            tracing::error!("{}", String::from_utf8_lossy(&result.stderr));
        }
    }
    start_command(sudo, "update-ca-certificates");
}

fn start_command(command: &'static str, arg: &'static str) {
    let output = Command::new(command).arg(arg).output();

    if let Ok(output) = output {
        if output.status.success() {
            tracing::info!("Certificate added successully to system store");
        } else {
            tracing::error!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }
}
