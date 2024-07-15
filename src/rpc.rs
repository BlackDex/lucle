use super::diesel;
use super::surrealdb;
use super::user;
use crate::errors::Error;
use email_address_parser::EmailAddress;
use luclerpc::{
    lucle_server::{Lucle, LucleServer},
    Credentials, Database, DatabaseType, Empty, Message, ResetPassword, UpdateServer, User,
    UserCreation,
};
use std::pin::Pin;
use std::{fs::File, io::BufReader};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::{
    service::RoutesBuilder,
    transport::{server::Router, Server},
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
        match DatabaseType::try_from(db_type) {
            Ok(DatabaseType::Sqlite) => {
                if let Err(err) = diesel::create_database("lucle.db").await {
                    tracing::error!("Unable to create database : {}", err);
                    return Err(Status::internal(err.to_string()));
                }
            }
            Ok(DatabaseType::Mysql) => {
                if let Err(err) = diesel::create_database("mysql://").await {
                    tracing::error!("Unable to create database : {}", err);
                    return Err(Status::internal(err.to_string()));
                }
            }
            Ok(DatabaseType::Postgresql) => {
                if let Err(err) = diesel::create_database("postgres://").await {
                    tracing::error!("Unable to create database : {}", err);
                    return Err(Status::internal(err.to_string()));
                }
            }
            Ok(DatabaseType::Surrealdb) => {
                if let Err(err) = surrealdb::create_database().await {
                    tracing::error!("Unable to create database : {}", err);
                    return Err(Status::internal(err.to_string()));
                }
            }
            _ => {}
        }

        let reply = Empty {};
        Ok(Response::new(reply))
    }

    async fn create_user(&self, request: Request<UserCreation>) -> Result<Response<Empty>, Status> {
        let inner = request.into_inner();
        let username = inner.username;
        let password = inner.password;
        let email = inner.email;
        let reply = Empty {};
        if EmailAddress::is_valid(&email.clone(), None) {
            match user::create_user(username.clone(), password, email).await {
                Ok(()) => {
                    tracing::info!("user {} created", username);
                    return Ok(Response::new(reply));
                }
                Err(err) => {
                    tracing::error!("{}", err);
                    return Err(Status::internal(err.to_string()));
                }
            }
        } else {
            return Err(Status::internal("Email not valid".to_string()));
        }
    }

    async fn register_update_server(
        &self,
        request: Request<UpdateServer>,
    ) -> Result<Response<Empty>, Status> {
        let inner = request.into_inner();
        let username = inner.username;
        let path = inner.path;
        let reply = Empty {};
        match user::register_update_server(username.clone(), path.clone()).await {
            Ok(()) => {
                tracing::info!("User {} created {} repository", username, path);
                return Ok(Response::new(reply));
            }
            Err(err) => {
                tracing::error!("{}", err);
                return Err(Status::internal(err.to_string()));
            }
        };
    }

    async fn login(&self, request: Request<Credentials>) -> Result<Response<User>, Status> {
        let inner = request.into_inner();
        let username_or_email = inner.username_or_email;
        let password = inner.password;
        match user::login(username_or_email, password).await {
            Ok(user) => {
                let user = User {
                    username: user.username,
                    token: user.token,
                    repositories: user.repositories,
                };
                Ok(Response::new(user))
            }
            Err(err) => {
                tracing::error!("{}", err);
                return Err(Status::internal(err.to_string()));
            }
        }
    }

    async fn is_database_created(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Empty>, Status> {
        let reply = Empty {};
        match user::is_table_and_user_created().await {
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
        let email = inner.email;
        let reply = Empty {};
        if EmailAddress::is_valid(email.as_str(), None) {
            if let Err(err) = user::reset_password(email).await {
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
