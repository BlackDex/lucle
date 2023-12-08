use dlopen2::wrapper::{Container, WrapperApi};
use minisign_verify::{PublicKey, Signature};
use std::fs::{self};
use std::future::Future;

use std::path::Path;
use std::pin::Pin;

#[derive(WrapperApi)]
struct Api {
    run: fn() -> Pin<Box<dyn Future<Output = ()>>>,
}

fn load_publickey() -> PublicKey {
    let pk_content = fs::read_to_string("plugins/mail/minisign.key").unwrap();
    PublicKey::from_base64(&pk_content).unwrap()
}

fn load_signature() -> Signature {
    let signature_content = fs::read_to_string("plugins/mail/libsmtp_server.so.minisig").unwrap();
    Signature::decode(&signature_content).expect("error when reading signature")
}

pub fn find_plugins(path: &Path, extension: &'static str) -> Vec<String> {
    let mut plugins: Vec<String> = vec![];
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.path().is_dir() {
                        find_plugins(&entry.path(), extension);
                    }
                    if entry.path().is_file()
                        && entry.path().extension().map_or(false, |e| e == extension)
                    {
                        plugins.push(entry.path().display().to_string())
                    }
                }
            }
        }
        Err(e) => {
            tracing::error!("Path error : {:?}", e);
        }
    }
    plugins
}

pub fn load_backend_plugin() {
    for plugin in find_plugins(Path::new("/plugins"), "so") {
        let container: Result<Container<Api>, _> = unsafe { Container::load(plugin) };
        match container {
            Ok(container) => {
                tracing::info!("Load library successfully !");
                container.run();
            }
            Err(err) => {
                tracing::error!("Could not open library : {}", err);
            }
        }
    }
}

pub async fn verify_plugins() {
    /*let mut plugin = File::open("plugins/mail/libsmtp_server.so").unwrap();
    let mut data = vec![];
    plugin.read_to_end(&mut data).unwrap();
    let pkey = load_publickey();
    let signature = load_signature();*/
    //if pkey.verify(&data, &signature, false).is_ok() {
}
