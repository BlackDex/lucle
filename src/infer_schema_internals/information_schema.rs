use std::borrow::Cow;
use std::error::Error;

use diesel::backend::Backend;
use diesel::connection::LoadConnection;
use diesel::deserialize::FromSql;
use diesel::dsl::*;
use diesel::expression::QueryMetadata;
use diesel::mysql::Mysql;
use diesel::pg::Pg;
use diesel::query_builder::QueryFragment;
use diesel::*;

use self::information_schema::{key_column_usage, table_constraints, tables};
use super::inference;
use super::table_data::TableName;

pub trait DefaultSchema: Backend {
    fn default_schema<C>(conn: &mut C) -> QueryResult<String>
    where
        C: LoadConnection<Backend = Self>,
        String: FromSql<sql_types::Text, C::Backend>;
}

impl DefaultSchema for Pg {
    fn default_schema<C>(_conn: &mut C) -> QueryResult<String> {
        Ok("public".into())
    }
}

define_sql_function!(fn database() -> VarChar);

impl DefaultSchema for Mysql {
    fn default_schema<C>(conn: &mut C) -> QueryResult<String>
    where
        C: LoadConnection<Backend = Self>,
        String: FromSql<sql_types::Text, C::Backend>,
    {
        select(database()).get_result(conn)
    }
}

pub fn get_primary_keys<'a, Conn>(conn: &mut Conn, table: &'a TableName) -> QueryResult<Vec<String>>
where
    Conn: LoadConnection,
    Conn::Backend: DefaultSchema,
    String: FromSql<sql_types::Text, Conn::Backend>,
    Order<
        Filter<
            Filter<
                Filter<
                    Select<key_column_usage::table, key_column_usage::column_name>,
                    EqAny<
                        key_column_usage::constraint_name,
                        Filter<
                            Select<table_constraints::table, table_constraints::constraint_name>,
                            Eq<table_constraints::constraint_type, &'static str>,
                        >,
                    >,
                >,
                Eq<key_column_usage::table_name, &'a String>,
            >,
            Eq<key_column_usage::table_schema, Cow<'a, String>>,
        >,
        key_column_usage::ordinal_position,
    >: QueryFragment<Conn::Backend>,
    Conn::Backend: QueryMetadata<sql_types::Text> + 'static,
{
    use self::information_schema::key_column_usage::dsl::*;
    use self::information_schema::table_constraints::constraint_type;

    let pk_query = table_constraints::table
        .select(table_constraints::constraint_name)
        .filter(constraint_type.eq("PRIMARY KEY"));

    let schema_name = match table.schema {
        Some(ref name) => Cow::Borrowed(name),
        None => Cow::Owned(Conn::Backend::default_schema(conn)?),
    };

    key_column_usage
        .select(column_name)
        .filter(constraint_name.eq_any(pk_query))
        .filter(table_name.eq(&table.sql_name))
        .filter(table_schema.eq(schema_name))
        .order(ordinal_position)
        .load(conn)
}

pub fn load_table_names<'a, Conn>(
    connection: &mut Conn,
    schema_name: Option<&'a str>,
) -> Result<Vec<TableName>, Box<dyn Error + Send + Sync + 'static>>
where
    Conn: LoadConnection,
    Conn::Backend: DefaultSchema + 'static,
    String: FromSql<sql_types::Text, Conn::Backend>,
    Filter<
        Filter<
            Filter<
                Select<tables::table, tables::table_name>,
                Eq<tables::table_schema, Cow<'a, str>>,
            >,
            NotLike<tables::table_name, &'static str>,
        >,
        Like<tables::table_type, &'static str>,
    >: QueryFragment<Conn::Backend>,
    Conn::Backend: QueryMetadata<sql_types::Text>,
{
    use self::information_schema::tables::dsl::*;

    let default_schema = Conn::Backend::default_schema(connection)?;
    let db_schema_name = schema_name
        .map(Cow::Borrowed)
        .unwrap_or_else(|| Cow::Owned(default_schema.clone()));

    let mut table_names = tables
        .select(table_name)
        .filter(table_schema.eq(db_schema_name))
        .filter(table_name.not_like("\\_\\_%"))
        .filter(table_type.like("BASE TABLE"))
        .load::<String>(connection)?;
    table_names.sort_unstable();
    Ok(table_names
        .into_iter()
        .map(|name| TableName {
            rust_name: inference::rust_name_for_sql_name(&name),
            sql_name: name,
            schema: schema_name
                .filter(|&schema| schema != default_schema)
                .map(|schema| schema.to_owned()),
        })
        .collect())
}

#[allow(clippy::module_inception)]
pub mod information_schema {
    use diesel::prelude::{allow_tables_to_appear_in_same_query, table};

    table! {
        information_schema.tables (table_schema, table_name) {
            table_schema -> VarChar,
            table_name -> VarChar,
            table_type -> VarChar,
        }
    }

    table! {
        information_schema.key_column_usage (table_schema, table_name, column_name, constraint_name) {
            table_schema -> VarChar,
            table_name -> VarChar,
            column_name -> VarChar,
            constraint_schema -> VarChar,
            constraint_name -> VarChar,
            ordinal_position -> BigInt,
        }
    }

    table! {
        information_schema.table_constraints (table_schema, table_name, constraint_name) {
            table_schema -> VarChar,
            table_name -> VarChar,
            constraint_schema -> VarChar,
            constraint_name -> VarChar,
            constraint_type -> VarChar,
        }
    }

    table! {
        information_schema.referential_constraints (constraint_schema, constraint_name) {
            constraint_schema -> VarChar,
            constraint_name -> VarChar,
            unique_constraint_schema -> Nullable<VarChar>,
            unique_constraint_name -> Nullable<VarChar>,
        }
    }

    allow_tables_to_appear_in_same_query!(table_constraints, referential_constraints);
    allow_tables_to_appear_in_same_query!(key_column_usage, table_constraints);
}
