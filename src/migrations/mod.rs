use diesel_migrations::{FileBasedMigrations, MigrationError};
use std::{path::PathBuf};

pub fn migrations_dir(migration_dir: Option<String>) -> Result<PathBuf, MigrationError> {
    match migration_dir {
        Some(dir) => Ok(dir.into()),
        None => FileBasedMigrations::find_migrations_directory().map(|p| p.path().to_path_buf()),
    }
}
