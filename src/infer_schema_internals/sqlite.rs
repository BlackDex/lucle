use std::error::Error;

use diesel::deserialize::{self, Queryable};
use diesel::dsl::sql;
use diesel::row::NamedRow;
use diesel::sql_types::{Bool, Text};
use diesel::sqlite::Sqlite;
use diesel::*;

use super::data_structures::*;
use super::table_data::TableName;
use crate::print_schema::ColumnSorting;

table! {
    sqlite_master (name) {
        name -> VarChar,
    }
}

table! {
    pragma_foreign_key_list {
        id -> Integer,
        seq -> Integer,
        _table -> VarChar,
        from -> VarChar,
        to -> Nullable<VarChar>,
        on_update -> VarChar,
        on_delete -> VarChar,
        _match -> VarChar,
    }
}

pub fn load_table_names(
    connection: &mut SqliteConnection,
    schema_name: Option<&str>,
) -> Result<Vec<TableName>, Box<dyn Error + Send + Sync + 'static>> {
    use self::sqlite_master::dsl::*;

    if schema_name.is_some() {
        return Err("sqlite cannot infer schema for databases other than the \
                    main database"
            .into());
    }

    Ok(sqlite_master
        .select(name)
        .filter(name.not_like("\\_\\_%").escape('\\'))
        .filter(name.not_like("sqlite%"))
        .filter(sql::<sql_types::Bool>("type='table'"))
        .order(name)
        .load::<String>(connection)?
        .into_iter()
        .map(TableName::from_name)
        .collect())
}

pub fn get_table_data(
    conn: &mut SqliteConnection,
    table: &TableName,
    column_sorting: &ColumnSorting,
) -> QueryResult<Vec<ColumnInformation>> {
    let sqlite_version = get_sqlite_version(conn);
    let query = if sqlite_version >= SqliteVersion::new(3, 26, 0) {
        /*
         * To get generated columns we need to use TABLE_XINFO
         * This would return hidden columns as well, but those would need to be created at runtime
         * therefore they aren't an issue.
         */
        format!("PRAGMA TABLE_XINFO('{}')", &table.sql_name)
    } else {
        format!("PRAGMA TABLE_INFO('{}')", &table.sql_name)
    };

    // See: https://github.com/diesel-rs/diesel/issues/3579 as to why we use a direct
    // `sql_query` with `QueryableByName` instead of using `sql::<pragma_table_info::SqlType>`.
    let mut result = sql_query(query).load::<ColumnInformation>(conn)?;
    match column_sorting {
        ColumnSorting::OrdinalPosition => {}
        ColumnSorting::Name => {
            result.sort_by(|a: &ColumnInformation, b: &ColumnInformation| {
                a.column_name.partial_cmp(&b.column_name).unwrap()
            });
        }
    };
    Ok(result)
}

pub fn load_foreign_key_constraints(
    connection: &mut SqliteConnection,
    schema_name: Option<&str>,
) -> Result<Vec<ForeignKeyConstraint>, Box<dyn Error + Send + Sync + 'static>> {
    let tables = load_table_names(connection, schema_name)?;
    let rows = tables
        .into_iter()
        .map(|child_table| {
            let query = format!("PRAGMA FOREIGN_KEY_LIST('{}')", child_table.sql_name);
            sql::<pragma_foreign_key_list::SqlType>(&query)
                .load::<ForeignKeyListRow>(connection)?
                .into_iter()
                .map(|row| {
                    let parent_table = TableName::from_name(row.parent_table);
                    let primary_key = if let Some(primary_key) = row.primary_key {
                        vec![primary_key]
                    } else {
                        get_primary_keys(connection, &parent_table)?
                    };
                    Ok(ForeignKeyConstraint {
                        child_table: child_table.clone(),
                        parent_table,
                        foreign_key_columns: vec![row.foreign_key.clone()],
                        foreign_key_columns_rust: vec![row.foreign_key.clone()],
                        primary_key_columns: primary_key,
                    })
                })
                .collect::<Result<_, _>>()
        })
        .collect::<QueryResult<Vec<Vec<_>>>>()?;
    Ok(rows.into_iter().flatten().collect())
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct SqliteVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl SqliteVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> SqliteVersion {
        SqliteVersion {
            major,
            minor,
            patch,
        }
    }
}

fn get_sqlite_version(conn: &mut SqliteConnection) -> SqliteVersion {
    let query = "SELECT sqlite_version()";
    let result = sql::<sql_types::Text>(query).load::<String>(conn).unwrap();
    let parts = result[0]
        .split('.')
        .map(|part| part.parse().unwrap())
        .collect::<Vec<u32>>();
    assert_eq!(parts.len(), 3);
    SqliteVersion::new(parts[0], parts[1], parts[2])
}

const SQLITE_ROWID_ALIASES: &[&str] = &["rowid", "oid", "_rowid_"];

pub fn get_primary_keys(
    conn: &mut SqliteConnection,
    table: &TableName,
) -> QueryResult<Vec<String>> {
    let sqlite_version = get_sqlite_version(conn);
    let query = if sqlite_version >= SqliteVersion::new(3, 26, 0) {
        format!("PRAGMA TABLE_XINFO('{}')", &table.sql_name)
    } else {
        format!("PRAGMA TABLE_INFO('{}')", &table.sql_name)
    };
    let results = sql_query(query).load::<PrimaryKeyInformation>(conn)?;
    let mut collected: Vec<String> = results
        .iter()
        .filter_map(|i| {
            if i.primary_key {
                Some(i.name.clone())
            } else {
                None
            }
        })
        .collect();
    // SQLite tables without "WITHOUT ROWID" always have aliases for the implicit PRIMARY KEY "rowid" and its aliases
    // unless the user defines a column with those names, then the name in question refers to the created column
    // https://www.sqlite.org/rowidtable.html
    if collected.is_empty() {
        for alias in SQLITE_ROWID_ALIASES {
            if results.iter().any(|v| &v.name.as_str() == alias) {
                continue;
            }

            // only add one alias as the primary key
            collected.push(alias.to_string());
            break;
        }
        // if it is still empty at this point, then a "diesel requires a primary key" error will be given
    }
    Ok(collected)
}

impl QueryableByName<Sqlite> for ColumnInformation {
    fn build<'a>(row: &impl NamedRow<'a, Sqlite>) -> deserialize::Result<Self> {
        let column_name = NamedRow::get::<Text, String>(row, "name")?;
        let type_name = NamedRow::get::<Text, String>(row, "type")?;
        let notnull = NamedRow::get::<Bool, bool>(row, "notnull")?;

        Ok(Self::new(
            column_name,
            type_name,
            None,
            !notnull,
            None,
            None,
        ))
    }
}

struct PrimaryKeyInformation {
    name: String,
    primary_key: bool,
}

impl QueryableByName<Sqlite> for PrimaryKeyInformation {
    fn build<'a>(row: &impl NamedRow<'a, Sqlite>) -> deserialize::Result<Self> {
        let name = NamedRow::get::<Text, String>(row, "name")?;
        let primary_key = NamedRow::get::<Bool, bool>(row, "pk")?;

        Ok(Self { name, primary_key })
    }
}

#[derive(Queryable)]
struct ForeignKeyListRow {
    _id: i32,
    _seq: i32,
    parent_table: String,
    foreign_key: String,
    primary_key: Option<String>,
    _on_update: String,
    _on_delete: String,
    _match: String,
}

pub fn determine_column_type(
    attr: &ColumnInformation,
) -> Result<ColumnType, Box<dyn Error + Send + Sync + 'static>> {
    let mut type_name = attr.type_name.to_lowercase();
    if type_name == "generated always" {
        type_name.clear();
    }

    let path = if is_bool(&type_name) {
        String::from("Bool")
    } else if is_smallint(&type_name) {
        String::from("SmallInt")
    } else if is_bigint(&type_name) {
        String::from("BigInt")
    } else if type_name.contains("int") {
        String::from("Integer")
    } else if is_text(&type_name) {
        String::from("Text")
    } else if is_binary(&type_name) {
        String::from("Binary")
    } else if is_float(&type_name) {
        String::from("Float")
    } else if is_double(&type_name) {
        String::from("Double")
    } else if type_name == "datetime" || type_name == "timestamp" {
        String::from("Timestamp")
    } else if type_name == "date" {
        String::from("Date")
    } else if type_name == "time" {
        String::from("Time")
    } else {
        return Err(format!("Unsupported type: {type_name}").into());
    };

    Ok(ColumnType {
        schema: None,
        rust_name: path.clone(),
        sql_name: path,
        is_array: false,
        is_nullable: attr.nullable,
        is_unsigned: false,
        max_length: attr.max_length,
    })
}

fn is_text(type_name: &str) -> bool {
    type_name.contains("char") || type_name.contains("clob") || type_name.contains("text")
}

fn is_binary(type_name: &str) -> bool {
    type_name.contains("blob") || type_name.contains("binary") || type_name.is_empty()
}

fn is_bool(type_name: &str) -> bool {
    type_name == "boolean"
        || type_name == "bool"
        || type_name.contains("tiny") && type_name.contains("int")
}

fn is_smallint(type_name: &str) -> bool {
    type_name == "int2" || type_name.contains("small") && type_name.contains("int")
}

fn is_bigint(type_name: &str) -> bool {
    type_name == "int8" || type_name.contains("big") && type_name.contains("int")
}

fn is_float(type_name: &str) -> bool {
    type_name.contains("float") || type_name.contains("real")
}

fn is_double(type_name: &str) -> bool {
    type_name.contains("double") || type_name.contains("num") || type_name.contains("dec")
}
