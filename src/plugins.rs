use dlopen2::wrapper::{Container, WrapperApi};
use minisign_verify::{PublicKey, Signature};
use std::fs::{self, File};
use std::future::Future;
use std::io::Read;
use std::pin::Pin;

#[derive(WrapperApi)]
struct Api {
    run: fn() -> Pin<Box<dyn Future<Output = ()>>>,
}

fn download_plugin() {}

fn load_publickey() -> PublicKey {
    let pk_content = fs::read_to_string("plugins/mail/minisign.key").unwrap();
    PublicKey::from_base64(&pk_content).unwrap()
}

fn load_signature() -> Signature {
    let signature_content = fs::read_to_string("plugins/mail/libsmtp_server.so.minisig").unwrap();
    Signature::decode(&signature_content).expect("error when reading signature")
}

pub async fn verify_plugins() {
    /*let mut plugin = File::open("plugins/mail/libsmtp_server.so").unwrap();
    let mut data = vec![];
    plugin.read_to_end(&mut data).unwrap();
    let pkey = load_publickey();
    let signature = load_signature();*/
    //    if pkey.verify(&data, &signature, false).is_ok() {
    let container: Container<Api> = unsafe { Container::load("plugins/mail/libsmtp_server.so") }
        .expect("Could not open library");

    let future = Box::pin(container.run());
    future.await;
    //}
}
