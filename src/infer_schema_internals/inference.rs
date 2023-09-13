use crate::database::LucleDBConnection;
use diesel::result::Error::NotFound;
use std::error::Error;

use super::data_structures::ColumnDefinition;
use super::data_structures::*;
use super::table_data::*;

use crate::config::Filtering;
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