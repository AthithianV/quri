#![allow(dead_code)]

use super::{
    sql::{push_bind, qualified_table, quote_identifier, rows_to_result},
    traits::{
        ColumnMetadata, ConnectionHandle, CrudFilter, CrudRow, DatabaseMetadata, DatabasePlugin,
        QueryResult, SchemaMetadata, TableMetadata,
    },
};
use crate::entity::connection::ConnectionConfig;
use anyhow::bail;
use async_trait::async_trait;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    QueryBuilder, Row, Sqlite,
};
use std::str::FromStr;

pub struct SqlitePlugin;

#[async_trait]
impl DatabasePlugin for SqlitePlugin {
    fn db_type(&self) -> &'static str {
        "sqlite"
    }

    async fn connect(&self, config: &ConnectionConfig) -> anyhow::Result<ConnectionHandle> {
        let ConnectionConfig::Sqlite(config) = config else {
            bail!("invalid config for sqlite plugin");
        };

        let options = SqliteConnectOptions::from_str(&config.file_path)?.create_if_missing(true);
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        Ok(ConnectionHandle::Sqlite(pool))
    }

    async fn metadata(&self, handle: &ConnectionHandle) -> anyhow::Result<DatabaseMetadata> {
        let ConnectionHandle::Sqlite(pool) = handle else {
            bail!("invalid sqlite connection handle");
        };

        let tables = sqlx::query(
            r#"
            SELECT name
            FROM sqlite_master
            WHERE type = 'table' AND name NOT LIKE 'sqlite_%'
            ORDER BY name
            "#,
        )
        .fetch_all(pool)
        .await?;

        let mut schema = SchemaMetadata {
            name: "main".to_string(),
            tables: Vec::new(),
        };

        for table in tables {
            let table_name: String = table.get("name");
            let pragma = format!("PRAGMA table_info({})", quote_identifier(&table_name, '"')?);
            let columns = sqlx::query(&pragma).fetch_all(pool).await?;

            schema.tables.push(TableMetadata {
                name: table_name,
                columns: columns
                    .into_iter()
                    .map(|column| ColumnMetadata {
                        name: column.get("name"),
                        data_type: column.get("type"),
                        is_nullable: column.get::<i32, _>("notnull") == 0,
                        ordinal_position: column.get::<i32, _>("cid") + 1,
                    })
                    .collect(),
            });
        }

        Ok(DatabaseMetadata {
            schemas: vec![schema],
        })
    }

    async fn execute_query(
        &self,
        handle: &ConnectionHandle,
        query: &str,
    ) -> anyhow::Result<QueryResult> {
        let ConnectionHandle::Sqlite(pool) = handle else {
            bail!("invalid sqlite connection handle");
        };

        let rows = sqlx::query(query).fetch_all(pool).await?;
        Ok(rows_to_result(rows, 0))
    }

    async fn insert_row(
        &self,
        handle: &ConnectionHandle,
        schema: Option<&str>,
        table: &str,
        row: &CrudRow,
    ) -> anyhow::Result<u64> {
        let ConnectionHandle::Sqlite(pool) = handle else {
            bail!("invalid sqlite connection handle");
        };

        if row.is_empty() {
            bail!("insert row cannot be empty");
        }

        let mut builder = QueryBuilder::<Sqlite>::new("INSERT INTO ");
        builder.push(qualified_table(schema, table, '"')?);
        builder.push(" (");

        let mut columns = row.keys().peekable();
        while let Some(column) = columns.next() {
            builder.push(quote_identifier(column, '"')?);
            if columns.peek().is_some() {
                builder.push(", ");
            }
        }

        builder.push(") VALUES (");
        let mut values = row.values().peekable();
        while let Some(value) = values.next() {
            push_bind(&mut builder, value);
            if values.peek().is_some() {
                builder.push(", ");
            }
        }
        builder.push(")");

        let result = builder.build().execute(pool).await?;
        Ok(result.rows_affected())
    }

    async fn update_rows(
        &self,
        handle: &ConnectionHandle,
        schema: Option<&str>,
        table: &str,
        values: &CrudRow,
        filters: &[CrudFilter],
    ) -> anyhow::Result<u64> {
        let ConnectionHandle::Sqlite(pool) = handle else {
            bail!("invalid sqlite connection handle");
        };

        if values.is_empty() {
            bail!("update values cannot be empty");
        }

        let mut builder = QueryBuilder::<Sqlite>::new("UPDATE ");
        builder.push(qualified_table(schema, table, '"')?);
        builder.push(" SET ");

        let mut values_iter = values.iter().peekable();
        while let Some((column, value)) = values_iter.next() {
            builder.push(quote_identifier(column, '"')?);
            builder.push(" = ");
            push_bind(&mut builder, value);
            if values_iter.peek().is_some() {
                builder.push(", ");
            }
        }

        push_filters(&mut builder, filters)?;

        let result = builder.build().execute(pool).await?;
        Ok(result.rows_affected())
    }

    async fn delete_rows(
        &self,
        handle: &ConnectionHandle,
        schema: Option<&str>,
        table: &str,
        filters: &[CrudFilter],
    ) -> anyhow::Result<u64> {
        let ConnectionHandle::Sqlite(pool) = handle else {
            bail!("invalid sqlite connection handle");
        };

        let mut builder = QueryBuilder::<Sqlite>::new("DELETE FROM ");
        builder.push(qualified_table(schema, table, '"')?);
        push_filters(&mut builder, filters)?;

        let result = builder.build().execute(pool).await?;
        Ok(result.rows_affected())
    }

    async fn select_rows(
        &self,
        handle: &ConnectionHandle,
        schema: Option<&str>,
        table: &str,
        filters: &[CrudFilter],
        limit: Option<u32>,
    ) -> anyhow::Result<QueryResult> {
        let ConnectionHandle::Sqlite(pool) = handle else {
            bail!("invalid sqlite connection handle");
        };

        let mut builder = QueryBuilder::<Sqlite>::new("SELECT * FROM ");
        builder.push(qualified_table(schema, table, '"')?);
        push_filters(&mut builder, filters)?;

        if let Some(limit) = limit {
            builder.push(" LIMIT ");
            builder.push_bind(i64::from(limit));
        }

        let rows = builder.build().fetch_all(pool).await?;
        Ok(rows_to_result(rows, 0))
    }
}

fn push_filters<'args>(
    builder: &mut QueryBuilder<'args, Sqlite>,
    filters: &'args [CrudFilter],
) -> anyhow::Result<()> {
    if filters.is_empty() {
        return Ok(());
    }

    builder.push(" WHERE ");

    let mut filters = filters.iter().peekable();
    while let Some(filter) = filters.next() {
        builder.push(quote_identifier(&filter.column, '"')?);
        builder.push(" = ");
        push_bind(builder, &filter.value);
        if filters.peek().is_some() {
            builder.push(" AND ");
        }
    }

    Ok(())
}
