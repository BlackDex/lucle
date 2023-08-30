use super::database;
use luclerpc::{
    lucle_server::{Lucle, LucleServer},
    Database, DatabaseType, Empty, Message, ResponseResult,
};
use std::fmt::Display;
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};

use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream};

pub mod luclerpc {
    tonic::include_proto!("luclerpc");
}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;
type StreamResult<T> = Result<Response<T>, Status>;

#[derive(Default)]
pub struct LucleApi {}

#[tonic::async_trait]
impl Lucle for LucleApi {
    async fn create_db(
        &self,
        request: Request<Database>,
    ) -> Result<Response<ResponseResult>, Status> {
        let inner = request.into_inner();
        let db_type = inner.db_type;
        let migration_path = inner.migration_path;
        let mut db_error: String = "".to_string();
        match DatabaseType::from_i32(db_type) {
            Some(DatabaseType::Sqlite) => {
                let migrations_dir = database::create_migrations_dir(migration_path)
                    .unwrap_or_else(database::handle_error);
                database::setup_database("lucle.db", &migrations_dir).unwrap_or_else(|err| {
                    tracing::error!("{}", err);
                    db_error = err.to_string();
                });

                //Some(DatabaseType::Mysql) => database::setup_database("mysql://").unwrap_or_else(handle_error),
            }
            _ => {}
        }
        let reply = ResponseResult { error: db_error };
        Ok(Response::new(reply))
    }

    async fn create_table(&self, request: Request<Database>) -> Result<Response<Empty>, Status> {
        let reply = Empty {};
        Ok(Response::new(reply))
    }

    async fn create_user(
        &self,
        request: Request<Database>,
    ) -> Result<Response<ResponseResult>, Status> {
        let reply = ResponseResult {
            error: "".to_string(),
        };
        Ok(Response::new(reply))
    }

    async fn is_db_created(&self, request: Request<Database>) -> Result<Response<Empty>, Status> {
        let reply = Empty {};
        Ok(Response::new(reply))
    }

    type ServerStreamingEchoStream = ResponseStream;

    async fn server_streaming_echo(
        &self,
        req: Request<Empty>,
    ) -> StreamResult<Self::ServerStreamingEchoStream> {
        tracing::info!("client connected from {:?}", req.remote_addr().unwrap());

        let message = Message {
            plugin: "allo".to_string(),
        };

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            match tx.send(Result::<_, Status>::Ok(message)).await {
                Ok(_) => {}
                Err(item) => {}
            }
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::ServerStreamingEchoStream
        ))
    }
}

fn handle_error<E: Display, T>(error: E) -> T {
    tracing::error!("{}", error);
    ::std::process::exit(1);
}

pub async fn start_rpc_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = "127.0.0.1:50051".parse().unwrap();
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
