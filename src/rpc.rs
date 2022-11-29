use tonic::{transport::Server, Request, Response};

#[derive(Default)]
pub struct MyGreeter {}



pub async fn start_rpc_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:3000".parse().unwrap();
    let repository = RemoteRepository::default();

    let repository = RepoServer::new(repository);
    //let repository = tonic_web::config()
    //    .enable(repository);
    println!("SpeedupdateRPCServer listening on {}", addr);

    Server::builder()
        .add_service(repository)
        .serve(addr)
        .await?;

    Ok(())
}
