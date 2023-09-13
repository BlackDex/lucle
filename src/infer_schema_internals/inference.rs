use diesel::{
    backend::Backend,
    deserialize::{self, FromStaticSqlRow, Queryable}
};
use std::fmt;
use super::inference;

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TableName {
    pub sql_name: String,
    pub rust_name: String,
    pub schema: Option<String>,
}

impl TableName {
    pub fn from_name<T: Into<String>>(name: T) -> Self {
        let name = name.into();

        TableName {
            rust_name: inference::rust_name_for_sql_name(&name),
            sql_name: name,
            schema: None,
        }
    }

    pub fn new<T, U>(name: T, schema: U) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        let name = name.into();

        TableName {
            rust_name: inference::rust_name_for_sql_name(&name),
            sql_name: name,
            schema: Some(schema.into()),
        }
    }

    #[cfg(feature = "uses_information_schema")]
    pub fn strip_schema_if_matches(&mut self, schema: &str) {
        if self.schema.as_deref() == Some(schema) {
            self.schema = None;
        }
    }

    pub fn full_sql_name(&self) -> String {
        match self.schema {
            Some(ref schema_name) => format!("{}.{}", schema_name, self.sql_name),
            None => self.sql_name.to_string(),
        }
    }
}

impl<ST, DB> Queryable<ST, DB> for TableName
where
    (String, String): FromStaticSqlRow<ST, DB>,
    DB: Backend,
{
    type Row = (String, String);

    fn build((name, schema): Self::Row) -> deserialize::Result<Self> {
        Ok(TableName::new(name, schema))
    }
}

impl fmt::Display for TableName {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.schema {
            Some(ref schema_name) => write!(out, "{}.{}", schema_name, self.rust_name),
            None => write!(out, "{}", self.rust_name),
        }
    }
}

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