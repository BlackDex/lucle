use tonic::{transport::Server, Request, Response, Status};
use luclerpc::{
  lucle_server::{
    LucleServer,
    Lucle,
  },
  Database, Empty
,};
use super:: database;
use std::fmt::Display;
use tonic_web::GrpcWebLayer;
use tower_http::cors::{
  CorsLayer,
  Any,
};

pub mod luclerpc {
    tonic::include_proto!("luclerpc");
}

#[derive(Default)]
pub struct LucleApi {}

#[tonic::async_trait]
impl Lucle for LucleApi {
  async fn install(&self, request: Request<Database>) -> Result<Response<Empty>, Status> {
    println!("13");
    database::setup_database(&request.into_inner().path).unwrap_or_else(handle_error); 
    let reply = Empty {
//	result: "12".to_string(),
    };
    Ok(Response::new(reply))
  }
}

fn handle_error<E: Display, T>(error: E) -> T {
    eprintln!("{}", error);
	::std::process::exit(1);
}

pub async fn start_rpc_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:3000".parse().unwrap();
    let api = LucleApi::default();

    let api = LucleServer::new(api);

    println!("RPCServer listening on {}", addr);

    let cors_layer = CorsLayer::new()
	.allow_origin(Any)
	.allow_headers(Any);

    Server::builder()
	.accept_http1(true)
	.layer(cors_layer)
	.layer(GrpcWebLayer::new())
        .add_service(api)
        .serve(addr)
        .await?;

    Ok(())
}
