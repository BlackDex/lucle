use super::data_structures::*;
use super::TableName;
use std::borrow::Cow;
use super::information_schema::DefaultSchema;
use diesel::connection::DefaultLoadingMode;
use diesel::sql_types::{self, Array, Text};
use diesel::RunQueryDsl;
use diesel::expression::AsExpression;
use diesel::dsl::AsExprOf;
use diesel::prelude::*;
use diesel::pg::Pg;
use diesel::QueryResult;
use diesel::PgConnection;

pub fn load_foreign_key_constraints(
    connection: &mut PgConnection,
    schema_name: Option<&str>,
) -> QueryResult<Vec<ForeignKeyConstraint>> {
    #[derive(QueryableByName)]
    struct ForeignKeyList {
        #[diesel(sql_type = Text)]
        self_schema: String,
        #[diesel(sql_type = Text)]
        self_table: String,
        #[diesel(sql_type = Array<Text>)]
        self_columns: Vec<String>,
        #[diesel(sql_type = Text)]
        foreign_schema: String,
        #[diesel(sql_type = Text)]
        foreign_table: String,
        #[diesel(sql_type = Array<Text>)]
        foreign_columns: Vec<String>,
    }

    let default_schema = Pg::default_schema(connection)?;
    let schema_name = schema_name.unwrap_or(&default_schema);

    diesel::sql_query(include_str!("load_foreign_keys.sql"))
        .bind::<Text, _>(schema_name)
        .load_iter::<ForeignKeyList, DefaultLoadingMode>(connection)?
        .map(|f| {
            let f = f?;
            let mut child_table = TableName::new(f.self_table, f.self_schema);
            child_table.strip_schema_if_matches(&default_schema);
            let mut parent_table = TableName::new(f.foreign_table, f.foreign_schema);
            parent_table.strip_schema_if_matches(&default_schema);

            let foreign_key_columns_rust = f
                .self_columns
                .iter()
                .map(|s| super::inference::rust_name_for_sql_name(s))
                .collect();
            Ok(ForeignKeyConstraint {
                child_table,
                parent_table,
                foreign_key_columns: f.self_columns,
                foreign_key_columns_rust,
                primary_key_columns: f.foreign_columns,
            })
        })
        .collect()
}

diesel::postfix_operator!(Regclass, "::regclass", sql_types::Oid, backend: Pg);

fn regclass(table: &TableName) -> Regclass<AsExprOf<String, sql_types::Text>> {
    let table_name = match table.schema {
        Some(ref schema_name) => format!("\"{}\".\"{}\"", schema_name, table.sql_name),
        None => format!("\"{}\"", table.sql_name),
    };

    Regclass::new(<String as AsExpression<sql_types::Text>>::as_expression(
        table_name,
    ))
}

diesel::sql_function!(fn col_description(table: sql_types::Oid, column_number: sql_types::BigInt) -> sql_types::Nullable<sql_types::Text>);

pub fn get_table_comment(
    conn: &mut PgConnection,
    table: &TableName,
) -> QueryResult<Option<String>> {
    diesel::select(obj_description(regclass(table), "pg_class")).get_result(conn)
}

pub fn get_table_data(
    conn: &mut PgConnection,
    table: &TableName,
    column_sorting: &ColumnSorting,
) -> QueryResult<Vec<ColumnInformation>> {
    use self::information_schema::columns::dsl::*;

    let schema_name = match table.schema {
        Some(ref name) => Cow::Borrowed(name),
        None => Cow::Owned(Pg::default_schema(conn)?),
    };

    let query = columns
        .select((
            column_name,
            udt_name,
            udt_schema.nullable(),
            __is_nullable,
            character_maximum_length,
            col_description(regclass(table), ordinal_position),
        ))
        .filter(table_name.eq(&table.sql_name))
        .filter(table_schema.eq(schema_name));
    match column_sorting {
        ColumnSorting::OrdinalPosition => query.order(ordinal_position).load(conn),
        ColumnSorting::Name => query.order(column_name).load(conn),
    }
}
