use super::database;
use luclerpc::{
    lucle_server::{Lucle, LucleServer},
    Database, DatabaseType, Empty,
};
use std::fmt::Display;
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};

pub mod luclerpc {
    tonic::include_proto!("luclerpc");
}

#[derive(Default)]
pub struct LucleApi {}

#[tonic::async_trait]
impl Lucle for LucleApi {
    async fn install(&self, request: Request<Database>) -> Result<Response<Empty>, Status> {
        let inner = request.into_inner();
        let db_type = inner.db_type;
        let migration_path = inner.migration_path;
        match DatabaseType::from_i32(db_type) {
            Some(DatabaseType::Sqlite) => {
                let migrations_dir = database::create_migrations_dir(migration_path)
                    .unwrap_or_else(database::handle_error);
                database::setup_database("lucle.db", &migrations_dir).unwrap_or_else(handle_error)
            }
            //Some(DatabaseType::Mysql) => database::setup_database("mysql://").unwrap_or_else(handle_error),
            _ => {}
        }
        let reply = Empty {};
        Ok(Response::new(reply))
    }
}

fn handle_error<E: Display, T>(error: E) -> T {
    eprintln!("{}", error);
    ::std::process::exit(1);
}

pub async fn start_rpc_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let api = LucleApi::default();

    let api = LucleServer::new(api);

    tracing::info!("RPCServer listening on {}", addr);

    let cors_layer = CorsLayer::new().allow_origin(Any).allow_headers(Any);

    Server::builder()
        .accept_http1(true)
        .layer(cors_layer)
        .layer(GrpcWebLayer::new())
        .add_service(api)
        .serve(addr)
        .await?;

    Ok(())
}
