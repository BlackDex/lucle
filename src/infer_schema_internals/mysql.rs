use diesel::deserialize::{self, FromStaticSqlRow, Queryable};
use diesel::mysql::{Mysql, MysqlConnection};
use diesel::*;
use heck::ToUpperCamelCase;
use std::collections::HashMap;
use std::{borrow::Cow, error::Error};

use super::data_structures::*;
use super::information_schema::DefaultSchema;
use super::table_data::TableName;
use crate::print_schema::ColumnSorting;

diesel::define_sql_function! {
    #[sql_name = "NULLIF"]
    fn null_if_text(lhs: sql_types::Text, rhs: sql_types::Text) -> sql_types::Nullable<sql_types::Text>
}

pub fn load_foreign_key_constraints(
    connection: &mut MysqlConnection,
    schema_name: Option<&str>,
) -> QueryResult<Vec<ForeignKeyConstraint>> {
    use self::information_schema::key_column_usage as kcu;
    use self::information_schema::table_constraints as tc;

    let default_schema = Mysql::default_schema(connection)?;
    let schema_name = match schema_name {
        Some(name) => name,
        None => &default_schema,
    };

    let constraints = tc::table
        .filter(tc::constraint_type.eq("FOREIGN KEY"))
        .filter(tc::table_schema.eq(schema_name))
        .filter(kcu::referenced_column_name.is_not_null())
        .inner_join(
            kcu::table.on(tc::constraint_schema
                .eq(kcu::constraint_schema)
                .and(tc::constraint_name.eq(kcu::constraint_name))),
        )
        .select((
            (kcu::table_name, kcu::table_schema),
            (kcu::referenced_table_name, kcu::referenced_table_schema),
            kcu::column_name,
            kcu::referenced_column_name,
            kcu::constraint_name,
        ))
        .load::<(TableName, TableName, String, String, String)>(connection)?
        .into_iter()
        .fold(
            HashMap::new(),
            |mut acc, (child_table, parent_table, foreign_key, primary_key, fk_constraint_name)| {
                let entry = acc
                    .entry(fk_constraint_name)
                    .or_insert_with(|| (child_table, parent_table, Vec::new(), Vec::new()));
                entry.2.push(foreign_key);
                entry.3.push(primary_key);
                acc
            },
        )
        .into_values()
        .map(
            |(mut child_table, mut parent_table, foreign_key_columns, primary_key_columns)| {
                child_table.strip_schema_if_matches(&default_schema);
                parent_table.strip_schema_if_matches(&default_schema);

                ForeignKeyConstraint {
                    child_table,
                    parent_table,
                    primary_key_columns,
                    foreign_key_columns_rust: foreign_key_columns.clone(),
                    foreign_key_columns,
                }
            },
        )
        .collect();
    Ok(constraints)
}

pub fn get_table_data(
    conn: &mut MysqlConnection,
    table: &TableName,
    column_sorting: &ColumnSorting,
) -> QueryResult<Vec<ColumnInformation>> {
    use self::information_schema::columns::dsl::*;

    let schema_name = match table.schema {
        Some(ref name) => Cow::Borrowed(name),
        None => Cow::Owned(Mysql::default_schema(conn)?),
    };

    let type_schema = None::<String>.into_sql();
    let query = columns
        .select((
            column_name,
            column_type,
            type_schema,
            __is_nullable,
            character_maximum_length,
            // MySQL comments are not nullable and are empty strings if not set
            null_if_text(column_comment, ""),
        ))
        .filter(table_name.eq(&table.sql_name))
        .filter(table_schema.eq(schema_name));
    let mut table_columns: Vec<ColumnInformation> = match column_sorting {
        ColumnSorting::OrdinalPosition => query.order(ordinal_position).load(conn)?,
        ColumnSorting::Name => query.order(column_name).load(conn)?,
    };
    for c in &mut table_columns {
        if c.max_length.is_some() && !c.type_name.contains('(') {
            // Mysql returns something in character_maximum_length regardless
            // of whether it's specified at field creation time
            // In addition there is typically a shared limitation at row level,
            // so it's typically not even the real max.
            // This basically means no max.
            // https://dev.mysql.com/doc/refman/8.0/en/column-count-limit.html
            // https://chartio.com/resources/tutorials/understanding-strorage-sizes-for-mysql-text-data-types/
            c.max_length = None;
        }
    }
    Ok(table_columns)
}

pub fn determine_column_type(
    attr: &ColumnInformation,
) -> Result<ColumnType, Box<dyn Error + Send + Sync + 'static>> {
    let tpe = determine_type_name(&attr.type_name)?;
    let unsigned = determine_unsigned(&attr.type_name);

    Ok(ColumnType {
        schema: None,
        sql_name: tpe.trim().to_string(),
        rust_name: tpe.trim().to_upper_camel_case(),
        is_array: false,
        is_nullable: attr.nullable,
        is_unsigned: unsigned,
        max_length: attr.max_length,
    })
}

fn determine_type_name(
    sql_type_name: &str,
) -> Result<String, Box<dyn Error + Send + Sync + 'static>> {
    let result = if sql_type_name == "tinyint(1)" {
        "bool"
    } else if sql_type_name.starts_with("int") {
        "integer"
    } else if let Some(idx) = sql_type_name.find('(') {
        &sql_type_name[..idx]
    } else {
        sql_type_name
    };

    if determine_unsigned(result) {
        Ok(result
            .to_lowercase()
            .replace("unsigned", "")
            .trim()
            .to_owned())
    } else if result.contains(' ') {
        Err(format!("unrecognized type {result:?}").into())
    } else {
        Ok(result.to_owned())
    }
}

fn determine_unsigned(sql_type_name: &str) -> bool {
    sql_type_name.to_lowercase().contains("unsigned")
}

pub fn get_table_comment(
    conn: &mut MysqlConnection,
    table: &TableName,
) -> QueryResult<Option<String>> {
    use self::information_schema::tables::dsl::*;

    let schema_name = match table.schema {
        Some(ref name) => Cow::Borrowed(name),
        None => Cow::Owned(Mysql::default_schema(conn)?),
    };

    let comment: String = tables
        .select(table_comment)
        .filter(table_name.eq(&table.sql_name))
        .filter(table_schema.eq(schema_name))
        .get_result(conn)?;

    if comment.is_empty() {
        Ok(None)
    } else {
        Ok(Some(comment))
    }
}

impl<ST> Queryable<ST, Mysql> for ColumnInformation
where
    (
        String,
        String,
        Option<String>,
        String,
        Option<u64>,
        Option<String>,
    ): FromStaticSqlRow<ST, Mysql>,
{
    type Row = (
        String,
        String,
        Option<String>,
        String,
        Option<u64>,
        Option<String>,
    );

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(ColumnInformation::new(
            row.0,
            row.1,
            row.2,
            row.3 == "YES",
            row.4,
            row.5,
        ))
    }
}

mod information_schema {
    use diesel::prelude::{allow_tables_to_appear_in_same_query, table};

    table! {
        information_schema.tables (table_schema, table_name) {
            table_schema -> VarChar,
            table_name -> VarChar,
            table_comment -> VarChar,
        }
    }

    table! {
        information_schema.table_constraints (constraint_schema, constraint_name) {
            table_schema -> VarChar,
            constraint_schema -> VarChar,
            constraint_name -> VarChar,
            constraint_type -> VarChar,
        }
    }

    table! {
        information_schema.key_column_usage (constraint_schema, constraint_name) {
            constraint_schema -> VarChar,
            constraint_name -> VarChar,
            table_schema -> VarChar,
            table_name -> VarChar,
            column_name -> VarChar,
            referenced_table_schema -> VarChar,
            referenced_table_name -> VarChar,
            referenced_column_name -> VarChar,
        }
    }

    table! {
        information_schema.columns (table_schema, table_name, column_name) {
            table_schema -> VarChar,
            table_name -> VarChar,
            column_name -> VarChar,
            #[sql_name = "is_nullable"]
            __is_nullable -> VarChar,
            character_maximum_length -> Nullable<Unsigned<BigInt>>,
            ordinal_position -> Unsigned<BigInt>,
            udt_name -> VarChar,
            udt_schema -> VarChar,
            column_type -> VarChar,
            column_comment -> VarChar,
        }
    }

    allow_tables_to_appear_in_same_query!(table_constraints, key_column_usage);
}
