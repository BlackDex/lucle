use super::user;

use email_address_parser::EmailAddress;
use luclerpc::{
    lucle_server::{Lucle, LucleServer},
    Credentials, Empty, Message, ResetPassword, UpdateServer, User, UserCreation,
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
    async fn create_user(&self, request: Request<UserCreation>) -> Result<Response<Empty>, Status> {
        let inner = request.into_inner();
        let username = inner.username;
        let password = inner.password;
        let email = inner.email;
	let role = inner.role;
        let database_url = "lucle.db".to_string();
        let reply = Empty {};
        if EmailAddress::is_valid(&email.clone(), None) {
            match user::create_user(&database_url, username.clone(), password, email, role).await {
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
        let database_url = "lucle.db".to_string();
        if let Err(err) = user::update_user(database_url, username, path).await {
            tracing::error!("{}", err);
            return Err(Status::internal(err.to_string()));
        }
        let reply = Empty {};
        Ok(Response::new(reply))
    }

    async fn login(&self, request: Request<Credentials>) -> Result<Response<User>, Status> {
        let inner = request.into_inner();
        let username_or_email = inner.username_or_email;
        let password = inner.password;
        let database_url = "lucle.db".to_string();
        match user::login(database_url, username_or_email, password).await {
            Ok(user) => {
                let user = User {
                    username: user.username,
                    token: user.token,
                    repository: user.role,
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
        let database_url = "lucle.db".to_string();
        let reply = Empty {};
        match user::is_table_and_user_created(database_url).await {
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
        let database_url = "lucle.db".to_string();
        let email = inner.email;
        let reply = Empty {};
        if EmailAddress::is_valid(email.as_str(), None) {
            if let Err(err) = user::reset_password(database_url, email).await {
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
