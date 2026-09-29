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
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
    ConnectOptions, MySql, QueryBuilder, Row,
};

pub struct MySqlPlugin;

#[async_trait]
impl DatabasePlugin for MySqlPlugin {
    fn db_type(&self) -> &'static str {
        "mysql"
    }

    async fn connect(&self, config: &ConnectionConfig) -> anyhow::Result<ConnectionHandle> {
        let ConnectionConfig::MySql(config) = config else {
            bail!("invalid config for mysql plugin");
        };

        let mut options = MySqlConnectOptions::new()
            .host(&config.host)
            .port(config.port)
            .database(&config.database_name)
            .username(&config.username);

        if let Some(password) = &config.password {
            options = options.password(password);
        }

        options = options.disable_statement_logging();

        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;
        Ok(ConnectionHandle::MySql(pool))
    }

    async fn metadata(&self, handle: &ConnectionHandle) -> anyhow::Result<DatabaseMetadata> {
        let ConnectionHandle::MySql(pool) = handle else {
            bail!("invalid mysql connection handle");
        };

        let rows = sqlx::query(
            r#"
            SELECT table_schema, table_name, column_name, data_type, is_nullable, ordinal_position
            FROM information_schema.columns
            WHERE table_schema NOT IN ('information_schema', 'mysql', 'performance_schema', 'sys')
            ORDER BY table_schema, table_name, ordinal_position
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(build_metadata(rows))
    }

    async fn execute_query(
        &self,
        handle: &ConnectionHandle,
        query: &str,
    ) -> anyhow::Result<QueryResult> {
        let ConnectionHandle::MySql(pool) = handle else {
            bail!("invalid mysql connection handle");
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
        let ConnectionHandle::MySql(pool) = handle else {
            bail!("invalid mysql connection handle");
        };

        if row.is_empty() {
            bail!("insert row cannot be empty");
        }

        let mut builder = QueryBuilder::<MySql>::new("INSERT INTO ");
        builder.push(qualified_table(schema, table, '`')?);
        builder.push(" (");

        let mut columns = row.keys().peekable();
        while let Some(column) = columns.next() {
            builder.push(quote_identifier(column, '`')?);
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
        let ConnectionHandle::MySql(pool) = handle else {
            bail!("invalid mysql connection handle");
        };

        if values.is_empty() {
            bail!("update values cannot be empty");
        }

        let mut builder = QueryBuilder::<MySql>::new("UPDATE ");
        builder.push(qualified_table(schema, table, '`')?);
        builder.push(" SET ");

        let mut values_iter = values.iter().peekable();
        while let Some((column, value)) = values_iter.next() {
            builder.push(quote_identifier(column, '`')?);
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
        let ConnectionHandle::MySql(pool) = handle else {
            bail!("invalid mysql connection handle");
        };

        let mut builder = QueryBuilder::<MySql>::new("DELETE FROM ");
        builder.push(qualified_table(schema, table, '`')?);
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
        let ConnectionHandle::MySql(pool) = handle else {
            bail!("invalid mysql connection handle");
        };

        let mut builder = QueryBuilder::<MySql>::new("SELECT * FROM ");
        builder.push(qualified_table(schema, table, '`')?);
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
    builder: &mut QueryBuilder<'args, MySql>,
    filters: &'args [CrudFilter],
) -> anyhow::Result<()> {
    if filters.is_empty() {
        return Ok(());
    }

    builder.push(" WHERE ");

    let mut filters = filters.iter().peekable();
    while let Some(filter) = filters.next() {
        builder.push(quote_identifier(&filter.column, '`')?);
        builder.push(" = ");
        push_bind(builder, &filter.value);
        if filters.peek().is_some() {
            builder.push(" AND ");
        }
    }

    Ok(())
}

fn build_metadata(rows: Vec<sqlx::mysql::MySqlRow>) -> DatabaseMetadata {
    let mut schemas = Vec::<SchemaMetadata>::new();

    for row in rows {
        let schema_name: String = row.get("table_schema");
        let table_name: String = row.get("table_name");
        let column = ColumnMetadata {
            name: row.get("column_name"),
            data_type: row.get("data_type"),
            is_nullable: row.get::<String, _>("is_nullable") == "YES",
            ordinal_position: row.get("ordinal_position"),
        };

        let schema_index = schemas
            .iter()
            .position(|schema| schema.name == schema_name)
            .unwrap_or_else(|| {
                schemas.push(SchemaMetadata {
                    name: schema_name.clone(),
                    tables: Vec::new(),
                });
                schemas.len() - 1
            });

        let table_index = schemas[schema_index]
            .tables
            .iter()
            .position(|table| table.name == table_name)
            .unwrap_or_else(|| {
                schemas[schema_index].tables.push(TableMetadata {
                    name: table_name.clone(),
                    columns: Vec::new(),
                });
                schemas[schema_index].tables.len() - 1
            });

        schemas[schema_index].tables[table_index]
            .columns
            .push(column);
    }

    DatabaseMetadata { schemas }
}
