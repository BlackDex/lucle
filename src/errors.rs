use std::path::PathBuf;

use crate::infer_schema_internals::TableName;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unable to find diesel.toml or Cargo.toml in {0:?} or any parent directories.")]
    ProjectRootNotFound(PathBuf),
    #[error("Encountered an IO error: {0} {}", print_optional_path(.1))]
    IoError(#[source] std::io::Error, Option<PathBuf>),
    #[error("Could not connect to database via `{url}`: {error}")]
    ConnectionError {
        error: diesel::ConnectionError,
        url: String,
    },
    #[error("Invalid argument for table filtering regex: {0}")]
    TableFilterRegexInvalid(#[from] regex::Error),
    #[error(
        "Command would result in changes to `{0}`. \
         Rerun the command locally, and commit the changes."
    )]
    SchemaWouldChange(String),
    #[error("Failed to execute a database query: {0}")]
    QueryError(#[from] diesel::result::Error),
    #[error("Failed to run migrations: {0}")]
    MigrationError(Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("Failed to parse patch file: {0}")]
    DiffyParseError(#[from] diffy::ParsePatchError),
    #[error("Failed to apply path: {0}")]
    DiffyApplyError(#[from] diffy::ApplyError),
    #[error(
        "Diesel only supports tables with primary keys. \
             Table `{0}` has no primary key"
    )]
    NoPrimaryKeyFound(TableName),
    #[error("Failed to format a string: {0}")]
    FmtError(#[from] std::fmt::Error),
    #[error("No table with the name `{0}` exists")]
    NoTableFound(TableName),
    #[error("User not found")]
    UserNotFound,
    #[error("Email not found")]
    EmailNotFound,
    #[error("Not allowed")]
    NotAuthorized,
    #[error("Failed to hash password: {0}")]
    Argon2Error(#[from] argon2::password_hash::Error),
}

/* impl From<io::Error> for DatabaseError {
    fn from(e: io::Error) -> Self {
        IoError(e)
    }
}

impl From<result::Error> for DatabaseError {
    fn from(e: result::Error) -> Self {
        QueryError(e)
    }
}

impl From<result::ConnectionError> for DatabaseError {
    fn from(e: result::ConnectionError) -> Self {
        ConnectionError(e)
    }
}

impl From<Box<dyn Error + Send + Sync + 'static>> for DatabaseError {
    fn from(e: Box<dyn Error + Send + Sync + 'static>) -> Self {
        MigrationError(e)
    }
}

impl From<argon2::password_hash::Error> for DatabaseError {
    fn from(e: argon2::password_hash::Error) -> Self {
        Argon2Error(e)
    }
}

impl Error for DatabaseError {}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ProjectRootNotFound(ref p) => {
                write!(
                    f,
                    "Unable to find diesel.toml or Cargo.toml in {:?} or any parent directories.",
                    p
                )
            }
            IoError(ref error) => f.write_str(
                &error
                    .source()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| error.to_string()),
            ),
            QueryError(ref error) => f.write_str(
                &error
                    .source()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| error.to_string()),
            ),
            ConnectionError(ref error) => f.write_str(
                &error
                    .source()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| error.to_string()),
            ),
            MigrationError(ref error) => {
                write!(f, "Failed to run migrations: {}", error)
            }
            UserNotFound => write!(f, "User not found"),
            EmailNotFound => write!(f, "Email not found"),
            NotAuthorized => write!(f, "You haven't the right role to access here"),
            Argon2Error(ref error) => write!(f, "{}", error),
        }
    }
}

impl PartialEq for DatabaseError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (&ProjectRootNotFound(_), &ProjectRootNotFound(_))
        )
    }
}
*/

fn print_optional_path(path: &Option<PathBuf>) -> String {
    path.as_ref()
        .map(|p| format!(" for `{}`", p.display()))
        .unwrap_or_default()
}
