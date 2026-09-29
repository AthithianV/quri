#![allow(dead_code)]

use super::traits::{QueryColumn, QueryResult, QueryValue, SqlValue};
use anyhow::bail;
use sqlx::{Column, Row, TypeInfo};
use std::collections::BTreeMap;

pub fn quote_identifier(identifier: &str, quote: char) -> anyhow::Result<String> {
    if identifier.is_empty() {
        bail!("identifier cannot be empty");
    }

    let escaped = identifier.replace(quote, &format!("{quote}{quote}"));
    Ok(format!("{quote}{escaped}{quote}"))
}

pub fn qualified_table(schema: Option<&str>, table: &str, quote: char) -> anyhow::Result<String> {
    let table = quote_identifier(table, quote)?;

    match schema {
        Some(schema) if !schema.is_empty() => {
            Ok(format!("{}.{}", quote_identifier(schema, quote)?, table))
        }
        _ => Ok(table),
    }
}

pub fn push_bind<'args, DB>(builder: &mut sqlx::QueryBuilder<'args, DB>, value: &'args SqlValue)
where
    DB: sqlx::Database,
    bool: sqlx::Encode<'args, DB> + sqlx::Type<DB>,
    i64: sqlx::Encode<'args, DB> + sqlx::Type<DB>,
    f64: sqlx::Encode<'args, DB> + sqlx::Type<DB>,
    String: sqlx::Encode<'args, DB> + sqlx::Type<DB>,
    Option<String>: sqlx::Encode<'args, DB> + sqlx::Type<DB>,
{
    match value {
        SqlValue::Null => builder.push_bind(None::<String>),
        SqlValue::Bool(value) => builder.push_bind(*value),
        SqlValue::Integer(value) => builder.push_bind(*value),
        SqlValue::Float(value) => builder.push_bind(*value),
        SqlValue::Text(value) => builder.push_bind(value.clone()),
    };
}

pub fn rows_to_result<R>(rows: Vec<R>, rows_affected: u64) -> QueryResult
where
    R: Row,
    usize: sqlx::ColumnIndex<R>,
    for<'row> Option<String>: sqlx::Decode<'row, R::Database> + sqlx::Type<R::Database>,
    for<'row> Option<i64>: sqlx::Decode<'row, R::Database> + sqlx::Type<R::Database>,
    for<'row> Option<f64>: sqlx::Decode<'row, R::Database> + sqlx::Type<R::Database>,
    for<'row> Option<bool>: sqlx::Decode<'row, R::Database> + sqlx::Type<R::Database>,
{
    let columns = rows
        .first()
        .map(|row| {
            row.columns()
                .iter()
                .map(|column| QueryColumn {
                    name: column.name().to_string(),
                    data_type: column.type_info().name().to_string(),
                })
                .collect()
        })
        .unwrap_or_default();

    let rows = rows
        .iter()
        .map(|row| {
            let mut values = BTreeMap::new();

            for (index, column) in row.columns().iter().enumerate() {
                values.insert(column.name().to_string(), cell_to_value(row, index));
            }

            values
        })
        .collect();

    QueryResult {
        columns,
        rows,
        rows_affected,
    }
}

fn cell_to_value<R>(row: &R, index: usize) -> QueryValue
where
    R: Row,
    usize: sqlx::ColumnIndex<R>,
    for<'row> Option<String>: sqlx::Decode<'row, R::Database> + sqlx::Type<R::Database>,
    for<'row> Option<i64>: sqlx::Decode<'row, R::Database> + sqlx::Type<R::Database>,
    for<'row> Option<f64>: sqlx::Decode<'row, R::Database> + sqlx::Type<R::Database>,
    for<'row> Option<bool>: sqlx::Decode<'row, R::Database> + sqlx::Type<R::Database>,
{
    if let Ok(value) = row.try_get::<Option<String>, _>(index) {
        return value.map(QueryValue::Text).unwrap_or(QueryValue::Null);
    }

    if let Ok(value) = row.try_get::<Option<i64>, _>(index) {
        return value.map(QueryValue::Integer).unwrap_or(QueryValue::Null);
    }

    if let Ok(value) = row.try_get::<Option<f64>, _>(index) {
        return value
            .map(|value| QueryValue::Float(value.to_string()))
            .unwrap_or(QueryValue::Null);
    }

    if let Ok(value) = row.try_get::<Option<bool>, _>(index) {
        return value.map(QueryValue::Bool).unwrap_or(QueryValue::Null);
    }

    QueryValue::Text("<unsupported>".to_string())
}
