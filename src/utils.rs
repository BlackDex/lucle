use rcgen::Certificate;

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
    let ca_cert = Certificate::from_params(ca_params).unwrap();

    return ca_cert;
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
    let tls = TlsServer {
        cert: server_cert_string,
        private_key: server_key_string,
    };
    return tls;
}
