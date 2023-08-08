use super::query_helper;
use crate::database_errors::{DatabaseError, DatabaseResult};
use chrono::Utc;
use diesel::{
    backend::Backend as DieselBackend, dsl::select, dsl::sql, mysql::MysqlConnection,
    pg::PgConnection, result, sql_types::Bool, sqlite::SqliteConnection, Connection, RunQueryDsl,
};
use diesel_migrations::{FileBasedMigrations, HarnessWithOutput, MigrationError, MigrationHarness};
use std::{
    env,
    error::Error,
    fmt::Display,
    fs,
    io::Write,
    path::{Path, PathBuf},
};
use url::Url;

pub static TIMESTAMP_FORMAT: &str = "%Y-%m-%d-%H%M%S";

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
#[derive(diesel::MultiConnection)]
pub enum InferConnection {
    Pg(PgConnection),
    Sqlite(SqliteConnection),
    Mysql(MysqlConnection),
}

impl InferConnection {
    pub fn from_matches(database_url: &str) -> Self {
        Self::establish(&database_url)
            .unwrap_or_else(|err| handle_error_with_database_url(&database_url, err))
    }
}

pub fn setup_database(database_url: &str, migrations_dir: &Path) -> DatabaseResult<()> {
    create_database_if_needed(&database_url)?;
    create_default_migration_if_needed(&database_url, migrations_dir)?;
    create_schema_table_and_run_migrations_if_needed(&database_url, migrations_dir)?;
    run_generate_migration_command(
        "allo".to_string(),
        None,
        None,
        Some("sql".to_string()),
        Some(true),
    )?;
    Ok(())
}

fn create_database_if_needed(database_url: &str) -> DatabaseResult<()> {
    match Backend::for_url(database_url) {
        Backend::Pg => {
            if PgConnection::establish(database_url).is_err() {
                let (database, postgres_url) = change_database_of_url(database_url, "postgres");
                let mut conn = PgConnection::establish(&postgres_url)?;
                query_helper::create_database(&database).execute(&mut conn)?;
            }
        }
        Backend::Sqlite => {
            let path = path_from_sqlite_url(database_url)?;
            if !path.exists() {
                SqliteConnection::establish(database_url)?;
            }
        }
        Backend::Mysql => {
            if MysqlConnection::establish(database_url).is_err() {
                let (database, mysql_url) =
                    change_database_of_url(database_url, "information_schema");
                let mut conn = MysqlConnection::establish(&mysql_url)?;
                query_helper::create_database(&database).execute(&mut conn)?;
            }
        }
    }
    tracing::info!("Creating database: {}", database_url);

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
            let mut conn = InferConnection::establish(database_url)?;
            run_migrations_with_output(&mut conn, migrations)?;
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
    let dir = match migrations_dir(migration_path) {
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
                    eprintln!(
                        "WARNING: Unable to delete existing `migrations/.gitkeep`:\n{}",
                        err
                    )
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
    tracing::info!("Creating migrations directory at: {}", path.display());
    fs::create_dir(path)?;
    fs::File::create(path.join(".keep"))?;
    Ok(path.to_owned())
}

fn find_project_root() -> DatabaseResult<PathBuf> {
    let current_dir = env::current_dir()?;
    search_for_directory_containing_file(&current_dir, "diesel.toml")
        .or_else(|_| search_for_directory_containing_file(&current_dir, "Cargo.toml"))
}

fn migrations_dir(path: Option<String>) -> Result<PathBuf, MigrationError> {
    match path {
        Some(dir) => Ok(dir.into()),
        None => FileBasedMigrations::find_migrations_directory().map(|p| p.path().to_path_buf()),
    }
}

fn create_table(migration_dir: PathBuf) {
    let up_path = migration_dir.join("up.sql");

    tracing::info!("Creating {}", migration_dir.join("up.sql").display());
    let mut up = fs::File::create(up_path).unwrap();
    up.write_all(
        b"CREATE TABLE posts (
        id SERIAL PRIMARY KEY,
        title VARCHAR NOT NULL,
        body TEXT NOT NULL,
        published BOOLEAN NOT NULL DEFAULT FALSE
      )",
    )
    .unwrap();
}

pub fn handle_error<E: Error, T>(error: E) -> T {
    println!("{error}");
    ::std::process::exit(1);
}

pub fn handle_error_with_database_url<E: Error, T>(database_url: &str, error: E) -> T {
    eprintln!("Could not connect to database via `{database_url}`: {error}");
    ::std::process::exit(1);
}

//migration
fn run_generate_migration_command(
    migration_name: String,
    migration_folder: Option<String>,
    migration_ver: Option<String>,
    migration_format: Option<String>,
    with_down_file: Option<bool>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let version = migration_version(migration_ver);
    let versioned_name = format!("{}_{}", version, migration_name);
    let migration_dir = migrations_dir(migration_folder)
        .unwrap_or_else(handle_error)
        .join(versioned_name);
    fs::create_dir(&migration_dir).unwrap();

    let down_file;
    if let Some(_i) = with_down_file {
        down_file = true;
    } else {
        down_file = false;
    }
    match migration_format.as_deref() {
        Some("sql") => generate_sql_migration(&migration_dir, down_file),
        Some(x) => return Err(format!("Unrecognized migration format `{}`", x).into()),
        None => tracing::info!("MIGRATION_FORMAT has a default value"),
    }

    Ok(())
}

fn generate_sql_migration(path: &Path, with_down: bool) {
    let migration_dir_relative =
        convert_absolute_path_to_relative(path, &env::current_dir().unwrap());

    create_table(migration_dir_relative.clone());

    if with_down {
        let down_path = path.join("down.sql");
        tracing::info!(
            "Creating {}",
            migration_dir_relative.join("down.sql").display()
        );
        let mut down = fs::File::create(down_path).unwrap();
        down.write_all(b"-- This file should undo anything in `up.sql`")
            .unwrap();
    }
}

fn convert_absolute_path_to_relative(target_path: &Path, mut current_path: &Path) -> PathBuf {
    let mut result = PathBuf::new();

    while !target_path.starts_with(current_path) {
        result.push("..");
        match current_path.parent() {
            Some(parent) => current_path = parent,
            None => return target_path.into(),
        }
    }

    result.join(target_path.strip_prefix(current_path).unwrap())
}

fn migration_version<'a>(version: Option<String>) -> Box<dyn Display + 'a> {
    match version {
        Some(version) => Box::new(version) as Box<dyn Display>,
        None => Box::new(Utc::now().format(TIMESTAMP_FORMAT)),
    }
}
