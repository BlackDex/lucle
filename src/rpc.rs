use super::database;
use super::user;
use crate::database::{handle_error, Backend};

use crate::models::Users;
use crate::schema::users;
use crate::utils;
use diesel::prelude::*;
use email_address_parser::EmailAddress;

use luclerpc::{
    lucle_server::{Lucle, LucleServer},
    Database, DatabaseType, Empty, Message, ResetPassword, ResponseResult, User,
};
use std::pin::Pin;
use std::{fs::File, io::BufReader, net::SocketAddr};

use tokio::sync::mpsc;
use tokio_rustls::rustls::Certificate;
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
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
    async fn create_db(
        &self,
        request: Request<Database>,
    ) -> Result<Response<ResponseResult>, Status> {
        let inner = request.into_inner();
        let db_type = inner.db_type;
        let _db_name = inner.db_name;
        let migration_path = inner.migration_path;
        let _username = inner.username;
        let _password = inner.password;
        let _hostname = inner.hostname;
        let _port = inner.port;
        // let name;
        let mut db_error: String = "".to_string();
        let migrations_dir =
            database::create_migrations_dir(migration_path).unwrap_or_else(database::handle_error);
        let mut database_url: &str = "";
        match DatabaseType::try_from(db_type) {
            Ok(DatabaseType::Sqlite) => database_url = "lucle.db",
            Ok(DatabaseType::Mysql) => database_url = "mysql://",
            Ok(DatabaseType::Postgresql) => database_url = "postgres://",
            _ => {}
        }

        database::setup_database(database_url, &migrations_dir).unwrap_or_else(|err| {
            tracing::error!("test : {}", err);
            db_error = err.to_string();
        });
        let reply = ResponseResult { error: db_error };
        Ok(Response::new(reply))
    }

    async fn create_table(&self, _request: Request<Database>) -> Result<Response<Empty>, Status> {
        let reply = Empty {};
        Ok(Response::new(reply))
    }

    async fn delete_db(
        &self,
        request: Request<Database>,
    ) -> Result<Response<ResponseResult>, Status> {
        let _inner = request.into_inner();
        let database_url = "lucle.db";
        let mut db_error: String = "".to_string();
        database::drop_database(database_url).unwrap_or_else(|err| {
            tracing::error!("{}", err);
            db_error = err.to_string();
        });
        let reply = ResponseResult { error: db_error };
        Ok(Response::new(reply))
    }

    async fn create_user(
        &self,
        request: Request<User>,
    ) -> Result<Response<ResponseResult>, Status> {
        let inner = request.into_inner();
        let username = inner.username;
        let password = inner.password;
        let email = inner.email;
        let mut db_error: String = "".to_string();
        if EmailAddress::is_valid(&email.clone(), None) {
            user::create_user("lucle.db", username, password, email).unwrap_or_else(|err| {
                tracing::error!("{}", err);
                db_error = err.to_string();
            });
        } else {
            db_error = "email not valid".to_string();
        }
        let reply = ResponseResult { error: db_error };
        Ok(Response::new(reply))
    }

    async fn login(&self, request: Request<User>) -> Result<Response<ResponseResult>, Status> {
        let inner = request.into_inner();
        let username = inner.username;
        let password = inner.password;
        let mut error: String = "".to_string();
        user::login("lucle.db", &username, &password).unwrap_or_else(|err| {
            tracing::error!("12 : {}", err);
            error = err.to_string();
        });
        let reply = ResponseResult { error };
        Ok(Response::new(reply))
    }

    async fn is_created_user(
        &self,
        _request: Request<Database>,
    ) -> Result<Response<ResponseResult>, Status> {
        let mut db_error = "".to_string();
        if user::is_default_user("lucle.db") {
            tracing::info!("ok");
        } else {
            db_error = "test".to_string();
        }
        let reply = ResponseResult { error: db_error };
        Ok(Response::new(reply))
    }

    async fn forgot_password(
        &self,
        request: Request<ResetPassword>,
    ) -> Result<Response<ResponseResult>, Status> {
        let inner = request.into_inner();
        let database_url = "lucle.db";
        let email = inner.email.as_str();
        let mut error: String = "".to_string();
        let mail_exist;
        if EmailAddress::is_valid(email, None) {
            match Backend::for_url(database_url) {
                Backend::Pg => {
                    let conn =
                        &mut PgConnection::establish(database_url).unwrap_or_else(handle_error);
                    mail_exist = users::table
                        .filter(users::dsl::email.eq(email))
                        .select(Users::as_select())
                        .first(conn)
                        .optional();
                }
                Backend::Mysql => {
                    let conn =
                        &mut MysqlConnection::establish(database_url).unwrap_or_else(handle_error);
                    mail_exist = users::table
                        .filter(users::dsl::email.eq(email))
                        .select(Users::as_select())
                        .first(conn)
                        .optional();
                }
                Backend::Sqlite => {
                    let conn =
                        &mut SqliteConnection::establish(database_url).unwrap_or_else(handle_error);
                    mail_exist = users::table
                        .filter(users::dsl::email.eq(email))
                        .select(Users::as_select())
                        .first(conn)
                        .optional();

                    match mail_exist {
                        Ok(Some(val)) => {
                            let token = utils::generate_jwt(val.username, val.email.clone());
                            if diesel::update(
                                users::table.filter(users::dsl::email.eq(val.email.clone())),
                            )
                            .set(users::dsl::reset_token.eq(token))
                            .returning(Users::as_returning())
                            .get_result(conn)
                            .is_ok()
                            {
                                utils::send_mail("a@a.com", &val.email, "test", "hi");
                            }
                        }
                        Ok(None) => error = "Unknow email".to_string(),
                        Err(_err) => error = "Connection failed".to_string(),
                    }
                }
            }
        } else {
            error = "Not a valid email".to_string();
        }
        let reply = ResponseResult { error };
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
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::ServerStreamingEchoStream
        ))
    }
}

#[derive(Debug)]
struct ConnInfo {
    addr: std::net::SocketAddr,
    certificates: Vec<Certificate>,
}

pub async fn start_rpc_server(
    _cert: &mut BufReader<File>,
    _key: &mut BufReader<File>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 50051));

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

    /*utils::generate_ca_cert();

    let api = LucleApi::default();

    let api = LucleServer::new(api);

    let certificate = rustls_pemfile::certs(cert)?
        .into_iter()
        .map(Certificate)
        .collect();
    let key = rustls_pemfile::pkcs8_private_keys(key)?
        .into_iter()
        .map(PrivateKey)
        .next()
        .unwrap();

    let mut tls = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certificate, key)?;
    tls.alpn_protocols = vec![b"h2".to_vec()];

    let cors_layer = CorsLayer::new().allow_origin(Any).allow_headers(Any);

    let svc = Server::builder()
        .accept_http1(true)
        .layer(cors_layer)
        .layer(GrpcWebLayer::new())
        .add_service(api)
        .into_service();

    let mut http = Http::new();
    http.http2_only(true);

    let addr = SocketAddr::from(([0, 0, 0, 0], 50051));
    let listener = TcpListener::bind(addr).await?;
    let tls_acceptor = TlsAcceptor::from(Arc::new(tls));

    tracing::info!("RPCServer listening on {}", addr);

    loop {
        let (conn, addr) = match listener.accept().await {
            Ok(incoming) => incoming,
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
                continue;
            }
        };

        let http = http.clone();
        let tls_acceptor = tls_acceptor.clone();
        let svc = svc.clone();

        tokio::spawn(async move {
            let mut certificates = Vec::new();

            match tls_acceptor
                .accept_with(conn, |info| {
                    if let Some(certs) = info.peer_certificates() {
                        for cert in certs {
                            certificates.push(cert.clone());
                        }
                    }
                })
                .await
            {
                Ok(conn) => {
                    let svc = tower::ServiceBuilder::new()
                        .add_extension(Arc::new(ConnInfo { addr, certificates }))
                        .service(svc);

                    match http.serve_connection(conn, svc).await {
                        Ok(_) => (),
                        Err(err) => tracing::error!("{}", err),
                    }
                }
                Err(err) => {
                    tracing::error!("{}", err)
                }
            }
        });
    }*/
}
