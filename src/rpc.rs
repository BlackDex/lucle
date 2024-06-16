use super::database;
use super::user;

use email_address_parser::EmailAddress;
use luclerpc::{
    lucle_server::{Lucle, LucleServer},
    Database, DatabaseType, Empty, Jwt, Message, ResetPassword, User,
};
use std::pin::Pin;
use std::{fs::File, io::BufReader};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::{
    transport::{
        server::{Router, RoutesBuilder},
        Server,
    },
    Request, Response, Status,
};
use tonic_web::GrpcWebLayer;
use tower::layer::util::Stack;
use tower_http::cors::{Any, CorsLayer};

pub mod luclerpc {
    tonic::include_proto!("luclerpc");
}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;
type StreamResult<T> = Result<Response<T>, Status>;

#[derive(Default)]
pub struct LucleApi {}

#[tonic::async_trait]
impl Lucle for LucleApi {
    async fn create_db(&self, request: Request<Database>) -> Result<Response<Empty>, Status> {
        let inner = request.into_inner();
        let db_type = inner.db_type;
        let _db_name = inner.db_name;
        let migration_path = inner.migration_path;
        let _username = inner.username;
        let _password = inner.password;
        let _hostname = inner.hostname;
        let _port = inner.port;
        // let name
        let reply = Empty {};
        let migrations_dir =
            database::create_migrations_dir(migration_path).unwrap_or_else(database::handle_error);
        let mut database_url: &str = "";
        match DatabaseType::try_from(db_type) {
            Ok(DatabaseType::Sqlite) => database_url = "lucle.db",
            Ok(DatabaseType::Mysql) => database_url = "mysql://",
            Ok(DatabaseType::Postgresql) => database_url = "postgres://",
            _ => {}
        }

        if let Err(err) = database::setup_database(database_url, &migrations_dir) {
            tracing::error!("{}", err);
            return Err(Status::internal(err.to_string()));
        }
        Ok(Response::new(reply))
    }

    async fn create_table(&self, _request: Request<Database>) -> Result<Response<Empty>, Status> {
        let reply = Empty {};
        Ok(Response::new(reply))
    }

    async fn delete_db(&self, request: Request<Database>) -> Result<Response<Empty>, Status> {
        let _inner = request.into_inner();
        let database_url = "lucle.db";
        let reply = Empty {};
        if let Err(err) = database::drop_database(database_url) {
            tracing::error!("{}", err);
            return Err(Status::internal(err.to_string()));
        }
        Ok(Response::new(reply))
    }

    async fn create_user(&self, request: Request<User>) -> Result<Response<Empty>, Status> {
        let inner = request.into_inner();
        let username = inner.username;
        let password = inner.password;
        let email = inner.email;
        let reply = Empty {};
        if EmailAddress::is_valid(&email.clone(), None) {
            if let Err(err) = user::create_user("lucle.db", username, password, email) {
                tracing::error!("{}", err);
                return Err(Status::internal(err.to_string()));
            }
        } else {
            return Err(Status::internal("Email not valid".to_string()));
        }
        Ok(Response::new(reply))
    }

    async fn login(&self, request: Request<User>) -> Result<Response<Jwt>, Status> {
        let inner = request.into_inner();
        let username = inner.username;
        let password = inner.password;
        match user::login("lucle.db", username, password) {
            Ok(token) => Ok(Response::new(Jwt { token: token })),
            Err(err) => {
                tracing::error!("{}", err);
                return Err(Status::internal(err.to_string()));
            }
        }
    }

    async fn is_created_user(
        &self,
        _request: Request<Database>,
    ) -> Result<Response<Empty>, Status> {
        let reply = Empty {};
        match user::is_table_and_user_created("lucle.db") {
            Ok(()) => Ok(Response::new(reply)),
            Err(err) => {
                tracing::error!("{}", err);
                Err(Status::internal(err.to_string()))
            }
        }
    }

    async fn forgot_password(
        &self,
        request: Request<ResetPassword>,
    ) -> Result<Response<Empty>, Status> {
        let inner = request.into_inner();
        let database_url = "lucle.db";
        let email = inner.email;
        let reply = Empty {};
        if EmailAddress::is_valid(email.as_str(), None) {
            if let Err(err) = user::reset_password(database_url, email) {
                tracing::error!("{}", err);
                return Err(Status::internal(err.to_string()));
            }
        }
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
                Ok(_) => (),
                Err(_item) => (),
            }
            tracing::info!("client disconnected from {:?}", req.remote_addr().unwrap());
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::ServerStreamingEchoStream
        ))
    }
}

pub fn rpc_api(
    _cert: &mut BufReader<File>,
    _key: &mut BufReader<File>,
) -> Router<Stack<GrpcWebLayer, Stack<CorsLayer, tower::layer::util::Identity>>> {
    let api = LucleApi::default();
    let api = LucleServer::new(api);

    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .expose_headers(Any);

    let mut routes_builder = RoutesBuilder::default();
    routes_builder.add_service(api);

    Server::builder()
        .accept_http1(true)
        .layer(cors_layer)
        .layer(GrpcWebLayer::new())
        .add_routes(routes_builder.routes())
}
