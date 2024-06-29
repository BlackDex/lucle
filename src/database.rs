use super::query_helper;
use crate::config::Config;
use crate::print_schema;
use chrono::Utc;
use diesel::{
    backend::Backend as DieselBackend, connection::InstrumentationEvent, dsl::select, dsl::sql,
    result, sql_types::Bool, sqlite::SqliteConnection, Connection, ExpressionMethods,
    MysqlConnection, OptionalExtension, PgConnection, QueryDsl, QueryResult, RunQueryDsl,
};
use diesel_migrations::{FileBasedMigrations, MigrationError, MigrationHarness};
use std::{
    env,
    error::Error,
    fmt::Display,
    fs,
    io::{Read, Write},
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

    pub(crate) fn for_connection(connection: &LucleDBConnection) -> Backend {
        match connection {
            LucleDBConnection::Pg(_) => Self::Pg,
            LucleDBConnection::Sqlite(_) => Self::Sqlite,
            LucleDBConnection::Mysql(_) => Self::Mysql,
        }
    }
}

#[derive(diesel::MultiConnection)]
pub enum LucleDBConnection {
    Pg(PgConnection),
    Sqlite(SqliteConnection),
    Mysql(MysqlConnection),
}

impl LucleDBConnection {
    pub fn from_matches(database_url: String) -> Result<Self, crate::errors::Error> {
        Self::from_url(database_url)
    }

    pub fn from_url(database_url: String) -> Result<LucleDBConnection, crate::errors::Error> {
        let result = match Backend::for_url(&database_url) {
            Backend::Pg => PgConnection::establish(&database_url).map(Self::Pg),
            Backend::Mysql => MysqlConnection::establish(&database_url).map(Self::Mysql),
            Backend::Sqlite => SqliteConnection::establish(&database_url).map(Self::Sqlite),
        };

        let mut conn = result.map_err(|err| crate::errors::Error::ConnectionError {
            error: err,
            url: database_url,
        })?;

        conn.set_instrumentation(|event: InstrumentationEvent<'_>| {
            if let InstrumentationEvent::FinishQuery { query, error, .. } = event {
                if let Some(err) = error {
                    tracing::error!(?query, ?err, "Failed to execute query");
                } else {
                    tracing::debug!(?query);
                }
            }
        });
        Ok(conn)
    }
}

pub fn setup_database(
    database_url: &str,
    migrations_dir: &Path,
) -> Result<(), crate::errors::Error> {
    create_database(database_url)?;
    create_default_migration(database_url, migrations_dir)?;
    create_schema_table_and_run_migrations(database_url, migrations_dir)?;
    run_generate_migration_command(
        "allo".to_string(),
        None,
        None,
        Some("sql".to_string()),
        Some(true),
    )?;
    do_migrations(
        database_url.to_string(),
        Some(migrations_dir.display().to_string()),
    )?;
    Ok(())
}

fn create_database(database_url: &str) -> Result<(), crate::errors::Error> {
    match Backend::for_url(database_url) {
        Backend::Pg => {
            if PgConnection::establish(database_url).is_err() {
                let (database, postgres_url) = change_database_of_url(database_url, "postgres");
                let mut conn = PgConnection::establish(&postgres_url).map_err(|e| {
                    crate::errors::Error::ConnectionError {
                        error: e,
                        url: postgres_url,
                    }
                })?;
                query_helper::create_database(&database).execute(&mut conn)?;
            }
        }
        Backend::Sqlite => {
            let path = path_from_sqlite_url(database_url)?;
            if !path.exists() {
                SqliteConnection::establish(database_url).map_err(|error| {
                    crate::errors::Error::ConnectionError {
                        error,
                        url: database_url.to_owned(),
                    }
                })?;
            }
        }
        Backend::Mysql => {
            if MysqlConnection::establish(database_url).is_err() {
                let (database, mysql_url) =
                    change_database_of_url(database_url, "information_schema");
                let mut conn = MysqlConnection::establish(&mysql_url).map_err(|e| {
                    crate::errors::Error::ConnectionError {
                        error: e,
                        url: mysql_url,
                    }
                })?;
                query_helper::create_database(&database).execute(&mut conn)?;
            }
        }
    }
    tracing::info!("Creating database: {}", database_url);

    Ok(())
}

fn schema_table_exists(database_url: &str) -> Result<bool, crate::errors::Error> {
    match LucleDBConnection::establish(database_url).unwrap() {
        LucleDBConnection::Pg(mut conn) => select(sql::<Bool>(
            "EXISTS \
             (SELECT 1 \
             FROM information_schema.tables \
             WHERE table_name = '__diesel_schema_migrations')",
        ))
        .get_result(&mut conn),
        LucleDBConnection::Sqlite(mut conn) => select(sql::<Bool>(
            "EXISTS \
             (SELECT 1 \
             FROM sqlite_master \
             WHERE type = 'table' \
             AND name = '__diesel_schema_migrations')",
        ))
        .get_result(&mut conn),
        LucleDBConnection::Mysql(mut conn) => select(sql::<Bool>(
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

fn create_default_migration(
    database_url: &str,
    migrations_dir: &Path,
) -> Result<(), crate::errors::Error> {
    let initial_migration_path = migrations_dir.join("diesel_initial_setup");
    if initial_migration_path.exists() {
        return Ok(());
    }

    if let Backend::Pg = Backend::for_url(database_url) {
        fs::create_dir_all(&initial_migration_path)
            .map_err(|e| crate::errors::Error::IoError(e, Some(initial_migration_path.clone())))?;
        fs::create_dir_all(&initial_migration_path)
            .map_err(|e| crate::errors::Error::IoError(e, Some(initial_migration_path.clone())))?;
        let up_sql_file = initial_migration_path.join("up.sql");
        std::fs::write(
            &up_sql_file,
            include_bytes!("setup_sql/postgres/initial_setup/up.sql"),
        )
        .map_err(|e| crate::errors::Error::IoError(e, Some(up_sql_file.clone())))?;
        let down_sql_file = initial_migration_path.join("down.sql");
        std::fs::write(
            &down_sql_file,
            include_bytes!("setup_sql/postgres/initial_setup/down.sql"),
        )
        .map_err(|e| crate::errors::Error::IoError(e, Some(down_sql_file.clone())))?;
    }
    Ok(())
}

//TODO: move this into schema
diesel::table! {
    pg_database (datname) {
        datname -> Text,
        datistemplate -> Bool,
    }
}

diesel::table! {
    information_schema.schemata (schema_name) {
        schema_name -> Text,
    }
}

pub fn drop_database(database_url: &str) -> Result<(), crate::errors::Error> {
    match Backend::for_url(database_url) {
        Backend::Pg => {
            let (database, postgres_url) = change_database_of_url(database_url, "postgres");
            let mut conn = PgConnection::establish(&postgres_url).map_err(|e| {
                crate::errors::Error::ConnectionError {
                    error: e,
                    url: postgres_url,
                }
            })?;
            if pg_database_exists(&mut conn, &database)? {
                println!("Dropping database: {database}");
                query_helper::drop_database(&database)
                    .if_exists()
                    .execute(&mut conn)?;
            }
        }
        Backend::Sqlite => {
            if Path::new(database_url).exists() {
                println!("Dropping database: {database_url}");
                std::fs::remove_file(database_url).map_err(|e| {
                    crate::errors::Error::IoError(e, Some(std::path::PathBuf::from(database_url)))
                })?;
            }
        }
        Backend::Mysql => {
            let (database, mysql_url) = change_database_of_url(database_url, "information_schema");
            let mut conn = MysqlConnection::establish(&mysql_url).map_err(|e| {
                crate::errors::Error::ConnectionError {
                    error: e,
                    url: mysql_url,
                }
            })?;
            if mysql_database_exists(&mut conn, &database)? {
                println!("Dropping database: {database}");
                query_helper::drop_database(&database)
                    .if_exists()
                    .execute(&mut conn)?;
            }
        }
    }
    Ok(())
}

fn pg_database_exists(conn: &mut PgConnection, database_name: &str) -> QueryResult<bool> {
    use self::pg_database::dsl::*;

    pg_database
        .select(datname)
        .filter(datname.eq(database_name))
        .filter(datistemplate.eq(false))
        .get_result::<String>(conn)
        .optional()
        .map(|x| x.is_some())
}

fn mysql_database_exists(conn: &mut MysqlConnection, database_name: &str) -> QueryResult<bool> {
    use self::schemata::dsl::*;

    schemata
        .select(schema_name)
        .filter(schema_name.eq(database_name))
        .get_result::<String>(conn)
        .optional()
        .map(|x| x.is_some())
}

fn create_schema_table_and_run_migrations(
    database_url: &str,
    migrations_dir: &Path,
) -> Result<(), crate::errors::Error> {
    if !schema_table_exists(database_url).unwrap_or_else(handle_error) {
        let migrations =
            FileBasedMigrations::from_path(migrations_dir).unwrap_or_else(handle_error);
        let mut conn = LucleDBConnection::from_url(database_url.to_string())?;
        run_migrations(&mut conn, migrations).map_err(crate::errors::Error::MigrationError)?;
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

fn path_from_sqlite_url(database_url: &str) -> Result<std::path::PathBuf, crate::errors::Error> {
    if database_url.starts_with("file:/") {
        match Url::parse(database_url) {
            Ok(url) if url.scheme() == "file" => {
                Ok(url
                    .to_file_path()
                    .map_err(|_err| crate::errors::Error::ConnectionError {
                        error: result::ConnectionError::InvalidConnectionUrl(String::from(
                            database_url,
                        )),
                        url: database_url.into(),
                    })?)
            }
            _ => Err(crate::errors::Error::ConnectionError {
                error: result::ConnectionError::InvalidConnectionUrl(String::from(database_url)),
                url: database_url.into(),
            }),
        }
    } else {
        // assume it's a bare path
        Ok(::std::path::PathBuf::from(database_url))
    }
}

pub fn create_migrations_dir(
    migration_path: Option<String>,
) -> Result<PathBuf, crate::errors::Error> {
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

fn search_for_directory_containing_file(
    path: &Path,
    file: &str,
) -> Result<PathBuf, crate::errors::Error> {
    let toml_path = path.join(file);
    if toml_path.is_file() {
        Ok(path.to_owned())
    } else {
        path.parent()
            .map(|p| search_for_directory_containing_file(p, file))
            .unwrap_or_else(|| Err(crate::errors::Error::ProjectRootNotFound(path.into())))
            .map_err(|_| crate::errors::Error::ProjectRootNotFound(path.into()))
    }
}

fn run_migrations<Conn, DB>(
    conn: &mut Conn,
    migrations: FileBasedMigrations,
) -> Result<(), crate::errors::Error>
where
    Conn: MigrationHarness<DB> + Connection<Backend = DB> + 'static,
    DB: DieselBackend,
{
    tracing::info!("Running migration");

    conn.run_pending_migrations(migrations)
        .map(|_| ())
        .map_err(crate::errors::Error::MigrationError)
}

fn create_migrations_directory(path: &Path) -> Result<PathBuf, crate::errors::Error> {
    tracing::info!("Creating migrations directory at: {}", path.display());
    fs::create_dir_all(path)
        .map_err(|e| crate::errors::Error::IoError(e, Some(path.to_owned())))?;
    let keep_path = path.join("keep");
    fs::File::create(&keep_path).map_err(|e| crate::errors::Error::IoError(e, Some(keep_path)))?;
    Ok(path.to_owned())
}

pub fn find_project_root() -> Result<PathBuf, crate::errors::Error> {
    let current_dir = env::current_dir().map_err(|e| crate::errors::Error::IoError(e, None))?;
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
        b"CREATE TABLE users (
        id INT PRIMARY KEY AUTOINCREMENT,
        username TEXT NOT NULL,
        password TEXT NOT NULL,
        email TEXT NOT NULL,
        created_at  TIMSTAMPNOT NULL,
        modified_at TIMESTAMP NOT NULL,
        role TEXT,
	reset_token TEXT
      )",
    )
    .unwrap();
}

pub fn handle_error<E: Error, T>(error: E) -> T {
    tracing::error!("{}", error);
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
) -> Result<(), crate::errors::Error> {
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

fn do_migrations(
    database_url: String,
    migration_dir: Option<String>,
) -> Result<(), crate::errors::Error> {
    let mut conn = LucleDBConnection::from_matches(database_url.clone());
    let dir = migrations_dir(migration_dir).unwrap_or_else(handle_error);
    let dir = FileBasedMigrations::from_path(dir).unwrap_or_else(handle_error);
    run_migrations(&mut conn, dir)?;
    regenerate_schema(database_url, None)?;

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

fn regenerate_schema(
    database_url: String,
    locked_schema: Option<bool>,
) -> Result<(), crate::errors::Error> {
    let config = Config::read()?.print_schema;
    for config in config.all_configs.values() {
        if let Some(ref path) = config.file {
            let mut connection = LucleDBConnection::from_matches(database_url)?;
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| crate::errors::Error::IoError(e, Some(parent.to_owned())))?;
            }

            if let Some(locked_schema) = locked_schema {
                if locked_schema {
                    let mut buf = Vec::new();
                    print_schema::run_print_schema(&mut connection, config, &mut buf)?;

                    let old_buf = std::fs::read(path)
                        .map_err(|e| crate::errors::Error::IoError(e, Some(path.to_owned())))?;

                    if buf != old_buf {
                        return Err(crate::errors::Error::SchemaWouldChange(
                            path.display().to_string(),
                        ));
                    }
                }
            } else {
                let schema = print_schema::output_schema(&mut connection, &config)?;
                std::fs::write(path, schema.as_bytes())
                    .map_err(|e| crate::errors::Error::IoError(e, Some(path.to_owned())))?;
            }
        }
    }

    Ok(())
}
