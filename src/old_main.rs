use rocket::{get, routes};
use rocket::http::{Method};
use rocket::fs::NamedFile;
use rocket::error::Error;
use rocket::fs::{relative};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use std::path::{Path, PathBuf};

//mod diesel_sqlite;
mod rpc;
          
#[get("/<_..>", rank = 2)]
async fn index() -> Option<NamedFile> {
  NamedFile::open(Path::new(relative!("web/dist")).join("index.html"))
        .await
        .ok()
}

#[get("/<file..>", rank = 1)]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(relative!("web/dist")).join("assets/").join(file))
        .await
        .ok()
}

fn cors_options() -> CorsOptions {
   let allowed_origins = AllowedOrigins::all();

    rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
}

#[rocket::main]
async fn main() -> Result<(), Error> {
  let _ = rocket::build()
        .mount("/assets", routes![static_files])
	.mount("/", routes![index])
	.mount("/", rocket_cors::catch_all_options_routes())
        .manage(cors_options().to_cors().expect("To not fail"))
//	.attach(diesel_sqlite::stage())
	.launch()
        .await?;

    Ok(())
}
