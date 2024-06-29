use crate::database::LucleDBConnection;
use diesel::result::Error::NotFound;
use std::error::Error;

use super::data_structures::ColumnDefinition;
use super::data_structures::*;
use super::table_data::*;

use crate::config::{Filtering, PrintSchema};
use crate::print_schema::{ColumnSorting, DocConfig};

static RESERVED_NAMES: &[&str] = &[
    "abstract",
    "alignof",
    "as",
    "become",
    "box",
    "break",
    "const",
    "continue",
    "crate",
    "do",
    "else",
    "enum",
    "extern",
    "false",
    "final",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "macro",
    "match",
    "mod",
    "move",
    "mut",
    "offsetof",
    "override",
    "priv",
    "proc",
    "pub",
    "pure",
    "ref",
    "return",
    "Self",
    "self",
    "sizeof",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "typeof",
    "unsafe",
    "unsized",
    "use",
    "virtual",
    "where",
    "while",
    "yield",
    "bool",
    "columns",
    "is_nullable",
];

fn is_reserved_name(name: &str) -> bool {
    RESERVED_NAMES.contains(&name)
        || (
            // Names ending in an underscore are not considered reserved so that we
            // can always just append an underscore to generate an unreserved name.
            name.starts_with("__") && !name.ends_with('_')
        )
}

fn contains_unmappable_chars(name: &str) -> bool {
    // Rust identifier names are restricted to [a-zA-Z0-9_].
    !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn rust_name_for_sql_name(sql_name: &str) -> String {
    if is_reserved_name(sql_name) {
        format!("{sql_name}_")
    } else if contains_unmappable_chars(sql_name) {
        // Map each non-alphanumeric character ([^a-zA-Z0-9]) to an underscore.
        let mut rust_name: String = sql_name
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
            .collect();

        // Iteratively remove adjoining underscores ("__").
        let mut last_len = rust_name.len();
        'remove_adjoining: loop {
            rust_name = rust_name.replace("__", "_");
            if rust_name.len() == last_len {
                // No more underscore pairs left.
                break 'remove_adjoining;
            }
            last_len = rust_name.len();
        }

        rust_name
    } else {
        sql_name.to_string()
    }
}

pub fn filter_table_names(table_names: Vec<TableName>, table_filter: &Filtering) -> Vec<TableName> {
    table_names
        .into_iter()
        .filter(|t| !table_filter.should_ignore_table(t))
        .collect::<_>()
}

pub fn load_table_names(
    connection: &mut LucleDBConnection,
    schema_name: Option<&str>,
) -> Result<Vec<TableName>, crate::errors::Error> {
    let tables = match connection {
        LucleDBConnection::Sqlite(ref mut c) => super::sqlite::load_table_names(c, schema_name),
        LucleDBConnection::Pg(ref mut c) => {
            super::information_schema::load_table_names(c, schema_name)
        }
        LucleDBConnection::Mysql(ref mut c) => {
            super::information_schema::load_table_names(c, schema_name)
        }
    }?;
    tracing::info!(?tables, "Loaded tables");
    Ok(tables)
}

pub(crate) fn get_primary_keys(
    conn: &mut LucleDBConnection,
    table: &TableName,
) -> Result<Vec<String>, crate::errors::Error> {
    let primary_keys: Vec<String> = match *conn {
        LucleDBConnection::Sqlite(ref mut c) => super::sqlite::get_primary_keys(c, table),
        LucleDBConnection::Pg(ref mut c) => super::information_schema::get_primary_keys(c, table),
        LucleDBConnection::Mysql(ref mut c) => {
            super::information_schema::get_primary_keys(c, table)
        }
    }?;
    if primary_keys.is_empty() {
        Err(crate::errors::Error::NoPrimaryKeyFound(table.clone()))
    } else {
        tracing::info!(?primary_keys, "Load primary keys for table {table}");
        Ok(primary_keys)
    }
}

fn get_table_comment(
    conn: &mut LucleDBConnection,
    table: &TableName,
) -> Result<Option<String>, crate::errors::Error> {
    let table_comment = match *conn {
        LucleDBConnection::Sqlite(_) => Ok(None),
        LucleDBConnection::Pg(ref mut c) => super::pg::get_table_comment(c, table),
        LucleDBConnection::Mysql(ref mut c) => super::mysql::get_table_comment(c, table),
    };
    if let Err(NotFound) = table_comment {
        Err(crate::errors::Error::NoTableFound(table.clone()))
    } else {
        let table_comment = table_comment?;
        tracing::info!(?table_comment, "Load table comments for {table}");
        Ok(table_comment)
    }
}

fn get_column_information(
    conn: &mut LucleDBConnection,
    table: &TableName,
    column_sorting: &ColumnSorting,
) -> Result<Vec<ColumnInformation>, crate::errors::Error> {
    let column_info = match *conn {
        LucleDBConnection::Sqlite(ref mut c) => {
            super::sqlite::get_table_data(c, table, column_sorting)
        }
        LucleDBConnection::Pg(ref mut c) => super::pg::get_table_data(c, table, column_sorting),
        LucleDBConnection::Mysql(ref mut c) => {
            super::mysql::get_table_data(c, table, column_sorting)
        }
    };
    if let Err(NotFound) = column_info {
        Err(crate::errors::Error::NoTableFound(table.clone()))
    } else {
        let column_info = column_info?;
        tracing::info!(?column_info, "Load column information for table {table}");
        Ok(column_info)
    }
}

fn determine_column_type(
    attr: &ColumnInformation,
    conn: &mut LucleDBConnection,
) -> Result<ColumnType, Box<dyn Error + Send + Sync + 'static>> {
    match *conn {
        LucleDBConnection::Sqlite(_) => super::sqlite::determine_column_type(attr),
        LucleDBConnection::Pg(ref mut conn) => {
            use crate::infer_schema_internals::information_schema::DefaultSchema;

            super::pg::determine_column_type(attr, diesel::pg::Pg::default_schema(conn)?)
        }
        LucleDBConnection::Mysql(_) => super::mysql::determine_column_type(attr),
    }
}

#[tracing::instrument(skip(connection))]
pub fn load_foreign_key_constraints(
    connection: &mut LucleDBConnection,
    schema_name: Option<&str>,
) -> Result<Vec<ForeignKeyConstraint>, crate::errors::Error> {
    let constraints = match connection {
        LucleDBConnection::Sqlite(ref mut c) => {
            super::sqlite::load_foreign_key_constraints(c, schema_name)
        }
        LucleDBConnection::Pg(ref mut c) => {
            super::pg::load_foreign_key_constraints(c, schema_name).map_err(Into::into)
        }
        LucleDBConnection::Mysql(ref mut c) => {
            super::mysql::load_foreign_key_constraints(c, schema_name).map_err(Into::into)
        }
    };

    constraints.map(|mut ct| {
        ct.sort();
        ct.iter_mut().for_each(|foreign_key_constraint| {
            for name in &mut foreign_key_constraint.foreign_key_columns_rust {
                if is_reserved_name(name) {
                    *name = format!("{name}_");
                }
            }
        });
        tracing::info!(?ct, "Loaded foreign key constraints");
        ct
    })
}

#[tracing::instrument(skip(connection))]
pub fn load_table_data(
    connection: &mut LucleDBConnection,
    name: TableName,
    config: &PrintSchema,
) -> Result<TableData, crate::errors::Error> {
    // No point in loading table comments if they are not going to be displayed
    let table_comment = match config.with_docs {
        DocConfig::NoDocComments => None,
        DocConfig::OnlyDatabaseComments
        | DocConfig::DatabaseCommentsFallbackToAutoGeneratedDocComment => {
            get_table_comment(connection, &name)?
        }
    };

    let primary_key = get_primary_keys(connection, &name)?;

    let column_data = get_column_information(connection, &name, &config.column_sorting)?
        .into_iter()
        .map(|c| {
            let ty = determine_column_type(&c, connection, &name, &primary_key, config)?;

            let ColumnInformation {
                column_name,
                comment,
                ..
            } = c;
            let rust_name = rust_name_for_sql_name(&column_name);

            Ok(ColumnDefinition {
                sql_name: column_name,
                ty,
                rust_name,
                comment,
            })
        })
        .collect::<Result<_, crate::errors::Error>>()?;

    let primary_key = primary_key
        .iter()
        .map(|k| rust_name_for_sql_name(k))
        .collect::<Vec<_>>();

    Ok(TableData {
        name,
        primary_key,
        column_data,
        comment: table_comment,
    })
}
