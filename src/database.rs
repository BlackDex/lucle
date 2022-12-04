use std::path::Path;
use diesel::{
  pg::PgConnection,
  sqlite::SqliteConnection,
  mysql::MysqlConnection,
  Connection,
  result,
  RunQueryDsl,
};
use url::{Url};
use crate::database_errors::DatabaseResult;
use super::query_helper;

pub enum Backend {
    Pg,
    Sqlite,
    Mysql,
}

impl Backend {
    pub fn for_url(database_url: &str) -> Self {
	println!("{}", database_url);
	let mut available_schemes: Vec<&str> = Vec::new();
        match database_url {
            _ if database_url.starts_with("postgres://")
                || database_url.starts_with("postgresql://") =>
            {
		available_schemes.push("`postgres://`");
    	        Backend::Pg
            }
            _ if database_url.starts_with("mysql://") =>
            {
		available_schemes.push("`mysql://`");
                Backend::Mysql
            }
            _ => Backend::Sqlite
        }
    }
}

pub fn setup_database(database_url: &str/*, migrations_dir: &Path*/) -> DatabaseResult<()> {
    println!("allo");
    create_database_if_needed(&database_url)?;
//    create_default_migration_if_needed(&database_url, migrations_dir)?;
//    create_schema_table_and_run_migrations_if_needed(&database_url, migrations_dir)?;
    Ok(())
}

fn create_database_if_needed(database_url: &str) -> DatabaseResult<()> {
    println!("12 {}", database_url);
    match Backend::for_url(database_url) {
        Backend::Pg => {
            if PgConnection::establish(database_url).is_err() {
                let (database, postgres_url) = change_database_of_url(database_url, "postgres");
                println!("Creating database: {}", database);
                let mut conn = PgConnection::establish(&postgres_url)?;
                query_helper::create_database(&database).execute(&mut conn)?;
            }
        }
        Backend::Sqlite => {
            let path = path_from_sqlite_url(database_url)?;
            if !path.exists() {
                println!("Creating database: {}", database_url);
                SqliteConnection::establish(database_url)?;
            }
        }
        Backend::Mysql => {
            if MysqlConnection::establish(database_url).is_err() {
                let (database, mysql_url) =
                    change_database_of_url(database_url, "information_schema");
                println!("Creating database: {}", database);
                let mut conn = MysqlConnection::establish(&mysql_url)?;
                query_helper::create_database(&database).execute(&mut conn)?;
            }
        }
    }

    Ok(())
}

fn change_database_of_url(database_url: &str, default_database: &str) -> (String, String) {
    let base = Url::parse(database_url).unwrap();
    let database = base.path_segments().unwrap().last().unwrap().to_owned();
    let mut new_url = base.join(default_database).unwrap();
    new_url.set_query(base.query());
    (database, new_url.into())
}

fn path_from_sqlite_url(database_url: &str) -> DatabaseResult<::std::path::PathBuf> {
    if database_url.starts_with("file:/") {
        // looks like a file URL
        match Url::parse(database_url) {
            Ok(url) if url.scheme() == "file" => Ok(url.to_file_path().map_err(|_err| {
                result::ConnectionError::InvalidConnectionUrl(String::from(database_url))
            })?),
            _ => {
                // invalid URL or scheme
                Err(
                    result::ConnectionError::InvalidConnectionUrl(String::from(database_url))
                        .into(),
                )
            }
        }
    } else {
        // assume it's a bare path
        Ok(::std::path::PathBuf::from(database_url))
    }
}
