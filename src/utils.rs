use jsonwebtoken::{encode, EncodingKey, Header};
use lettre::{
    message::{header, MultiPart, SinglePart},
    FileTransport, Message, Transport,
};
use rcgen::Certificate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    sub: String,
    company: String,
    exp: usize,
}
use tera::{Context, Tera};

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
    let alg = &rcgen::PKCS_ECDSA_P256_SHA256;
    let mut ca_params = rcgen::CertificateParams::new(Vec::new());
    ca_params
        .distinguished_name
        .push(rcgen::DnType::OrganizationName, "Rustls Server Acceptor");
    ca_params
        .distinguished_name
        .push(rcgen::DnType::CommonName, "Example CA");
    ca_params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
    ca_params.key_usages = vec![
        rcgen::KeyUsagePurpose::KeyCertSign,
        rcgen::KeyUsagePurpose::DigitalSignature,
        rcgen::KeyUsagePurpose::CrlSign,
    ];
    ca_params.alg = alg;
    Certificate::from_params(ca_params).unwrap()
}

pub fn generate_server_cert_key(ca_cert: Certificate) -> TlsServer {
    let alg = &rcgen::PKCS_ECDSA_P256_SHA256;
    // Create a server end entity cert issued by the CA.
    let mut server_ee_params = rcgen::CertificateParams::new(vec!["localhost".to_string()]);
    server_ee_params.is_ca = rcgen::IsCa::NoCa;
    server_ee_params.extended_key_usages = vec![rcgen::ExtendedKeyUsagePurpose::ServerAuth];
    server_ee_params.alg = alg;
    let server_cert = Certificate::from_params(server_ee_params).unwrap();
    let server_cert_string = server_cert.serialize_pem_with_signer(&ca_cert).unwrap();
    let server_key_string = server_cert.serialize_private_key_pem();
    TlsServer {
        cert: server_cert_string,
        private_key: server_key_string,
    }
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
