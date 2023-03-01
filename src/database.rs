use super::query_helper;
use crate::database_errors::{DatabaseError, DatabaseResult};
use crate::migrations;
use diesel::{
    backend::Backend as DieselBackend, dsl::select, dsl::sql, mysql::MysqlConnection,
    pg::PgConnection, result, sql_types::Bool, sqlite::SqliteConnection, Connection, RunQueryDsl,
};
use diesel_migrations::{FileBasedMigrations, HarnessWithOutput, MigrationHarness};
use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};
use url::Url;

pub enum Backend {
    Pg,
    Sqlite,
    Mysql,
}

impl Backend {
    pub fn for_url(database_url: &str) -> Self {
        let mut available_schemes: Vec<&str> = Vec::new();
        match database_url {
            _ if database_url.starts_with("postgres://")
                || database_url.starts_with("postgresql://") =>
            {
                available_schemes.push("`postgres://`");
                Backend::Pg
            }
            _ if database_url.starts_with("mysql://") => {
                available_schemes.push("`mysql://`");
                Backend::Mysql
            }
            _ => Backend::Sqlite,
        }
    }
}

pub enum InferConnection {
    Pg(PgConnection),
    Sqlite(SqliteConnection),
    Mysql(MysqlConnection),
}

impl InferConnection {
    pub fn establish(database_url: &str) -> DatabaseResult<Self> {
        match Backend::for_url(database_url) {
            Backend::Pg => PgConnection::establish(database_url).map(InferConnection::Pg),
            Backend::Sqlite => {
                SqliteConnection::establish(database_url).map(InferConnection::Sqlite)
            }
            Backend::Mysql => MysqlConnection::establish(database_url).map(InferConnection::Mysql),
        }
        .map_err(Into::into)
    }
}

macro_rules! call_with_conn {
    (
        $database_url:expr,
        $($func:ident)::+ ($($args:expr),*)
    ) => {
        match crate::database::InferConnection::establish(&$database_url)
            .unwrap_or_else(|err| {handle_error_with_database_url(&$database_url, err)})
        {
            crate::database::InferConnection::Pg(ref mut conn) => $($func)::+ (conn, $($args),*),
            crate::database::InferConnection::Sqlite(ref mut conn) => $($func)::+ (conn, $($args),*),
            crate::database::InferConnection::Mysql(ref mut conn) => $($func)::+ (conn, $($args),*),
        }
    };
}

pub fn setup_database(database_url: &str, migrations_dir: &Path) -> DatabaseResult<()> {
    create_database_if_needed(&database_url)?;
    create_default_migration_if_needed(&database_url, migrations_dir)?;
    create_schema_table_and_run_migrations_if_needed(&database_url, migrations_dir)?;
    Ok(())
}

fn create_database_if_needed(database_url: &str) -> DatabaseResult<()> {
    match Backend::for_url(database_url) {
        Backend::Pg => {
            if PgConnection::establish(database_url).is_err() {
                let (database, postgres_url) = change_database_of_url(database_url, "postgres");
                tracing::info!("Creating database: {}", database);
                let mut conn = PgConnection::establish(&postgres_url)?;
                query_helper::create_database(&database).execute(&mut conn)?;
            }
        }
        Backend::Sqlite => {
            let path = path_from_sqlite_url(database_url)?;
            if !path.exists() {
                tracing::info!("Creating database: {}", database_url);
                SqliteConnection::establish(database_url)?;
            }
        }
        Backend::Mysql => {
            if MysqlConnection::establish(database_url).is_err() {
                let (database, mysql_url) =
                    change_database_of_url(database_url, "information_schema");
                tracing::info!("Creating database: {}", database);
                let mut conn = MysqlConnection::establish(&mysql_url)?;
                query_helper::create_database(&database).execute(&mut conn)?;
            }
        }
    }

    Ok(())
}

fn schema_table_exists(database_url: &str) -> DatabaseResult<bool> {
    match InferConnection::establish(database_url).unwrap() {
        InferConnection::Pg(mut conn) => select(sql::<Bool>(
            "EXISTS \
             (SELECT 1 \
             FROM information_schema.tables \
             WHERE table_name = '__diesel_schema_migrations')",
        ))
        .get_result(&mut conn),
        InferConnection::Sqlite(mut conn) => select(sql::<Bool>(
            "EXISTS \
             (SELECT 1 \
             FROM sqlite_master \
             WHERE type = 'table' \
             AND name = '__diesel_schema_migrations')",
        ))
        .get_result(&mut conn),
        InferConnection::Mysql(mut conn) => select(sql::<Bool>(
            "EXISTS \
                    (SELECT 1 \
                     FROM information_schema.tables \
                     WHERE table_name = '__diesel_schema_migrations'
                     AND table_schema = DATABASE())",
        ))
        .get_result(&mut conn),
    }
    .map_err(Into::into)
}

fn database_url(database_url: Option<String>) -> String {
    match database_url {
        Some(path) => path,
        None => handle_error(DatabaseError::DatabaseUrlMissing),
    }
}

fn create_default_migration_if_needed(
    database_url: &str,
    migrations_dir: &Path,
) -> DatabaseResult<()> {
    let initial_migration_path = migrations_dir.join("diesel_initial_setup");
    if initial_migration_path.exists() {
        return Ok(());
    }

    match Backend::for_url(database_url) {
        Backend::Pg => {
            fs::create_dir_all(&initial_migration_path)?;
            //          let mut up_sql = File::create(initial_migration_path.join("up.sql"))?;
            //          up_sql.write_all(include_bytes!("setup_sql/postgres/initial_setup/up.sql"))?;
            //          let mut down_sql = File::create(initial_migration_path.join("down.sql"))?;
            //          down_sql.write_all(include_bytes!("setup_sql/postgres/initial_setup/down.sql"))?;
        }
        _ => {}
    }

    Ok(())
}

fn create_schema_table_and_run_migrations_if_needed(
    database_url: &str,
    migrations_dir: &Path,
) -> DatabaseResult<()> {
    if !schema_table_exists(database_url).unwrap_or_else(handle_error) {
        let migrations =
            FileBasedMigrations::from_path(migrations_dir).unwrap_or_else(handle_error);
        call_with_conn!(database_url, run_migrations_with_output(migrations))?;
    };
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

pub fn create_migrations_dir(migration_path: Option<String>) -> DatabaseResult<PathBuf> {
    let dir = match self::migrations::migrations_dir(migration_path) {
        Ok(dir) => dir,
        Err(_) => find_project_root()
            .unwrap_or_else(handle_error)
            .join("migrations"),
    };

    if dir.exists() {
        // This is a cleanup code for migrating from an
        // older version of diesel_cli that set a `.gitkeep` instead of a `.keep` file.
        // TODO: remove this after a few releases
        if let Ok(read_dir) = fs::read_dir(&dir) {
            if let Some(dir_entry) =
                read_dir
                    .filter_map(|entry| entry.ok())
                    .find(|entry| match entry.file_type() {
                        Ok(file_type) => file_type.is_file() && entry.file_name() == ".gitkeep",
                        Err(_) => false,
                    })
            {
                fs::remove_file(dir_entry.path()).unwrap_or_else(|err| {
                    eprintln!("WARNING: Unable to delete existing `migrations/.gitkeep`:\n{err}")
                });
            }
        }
    } else {
        create_migrations_directory(&dir)?;
    }

    Ok(dir)
}

fn search_for_directory_containing_file(path: &Path, file: &str) -> DatabaseResult<PathBuf> {
    let toml_path = path.join(file);
    if toml_path.is_file() {
        Ok(path.to_owned())
    } else {
        path.parent()
            .map(|p| search_for_directory_containing_file(p, file))
            .unwrap_or_else(|| Err(DatabaseError::ProjectRootNotFound(path.into())))
            .map_err(|_| DatabaseError::ProjectRootNotFound(path.into()))
    }
}

fn run_migrations_with_output<Conn, DB>(
    conn: &mut Conn,
    migrations: FileBasedMigrations,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>>
where
    Conn: MigrationHarness<DB> + Connection<Backend = DB> + 'static,
    DB: DieselBackend,
{
    HarnessWithOutput::write_to_stdout(conn)
        .run_pending_migrations(migrations)
        .map(|_| ())
}

fn create_migrations_directory(path: &Path) -> DatabaseResult<PathBuf> {
    println!("Creating migrations directory at: {}", path.display());
    fs::create_dir(path)?;
    fs::File::create(path.join(".keep"))?;
    Ok(path.to_owned())
}

fn find_project_root() -> DatabaseResult<PathBuf> {
    let current_dir = env::current_dir()?;
    search_for_directory_containing_file(&current_dir, "diesel.toml")
        .or_else(|_| search_for_directory_containing_file(&current_dir, "Cargo.toml"))
}

pub fn handle_error<E: Error, T>(error: E) -> T {
    println!("{error}");
    ::std::process::exit(1);
}

pub fn handle_error_with_database_url<E: Error, T>(database_url: &str, error: E) -> T {
    eprintln!("Could not connect to database via `{database_url}`: {error}");
    ::std::process::exit(1);
}
