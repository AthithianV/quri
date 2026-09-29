#![allow(dead_code)]

use crate::entity::connection::ConnectionConfig;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone)]
pub enum ConnectionHandle {
    Postgres(sqlx::Pool<sqlx::Postgres>),
    MySql(sqlx::Pool<sqlx::MySql>),
    Sqlite(sqlx::Pool<sqlx::Sqlite>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatabaseMetadata {
    pub schemas: Vec<SchemaMetadata>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaMetadata {
    pub name: String,
    pub tables: Vec<TableMetadata>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableMetadata {
    pub name: String,
    pub columns: Vec<ColumnMetadata>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColumnMetadata {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub ordinal_position: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryResult {
    pub columns: Vec<QueryColumn>,
    pub rows: Vec<BTreeMap<String, QueryValue>>,
    pub rows_affected: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryColumn {
    pub name: String,
    pub data_type: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryValue {
    Null,
    Bool(bool),
    Integer(i64),
    Float(String),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SqlValue {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    Text(String),
}

pub type CrudRow = BTreeMap<String, SqlValue>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CrudFilter {
    pub column: String,
    pub value: SqlValue,
}

#[async_trait]
pub trait DatabasePlugin: Send + Sync {
    fn db_type(&self) -> &'static str;

    async fn connect(&self, config: &ConnectionConfig) -> anyhow::Result<ConnectionHandle>;

    async fn test_connection(&self, config: &ConnectionConfig) -> anyhow::Result<()> {
        self.connect(config).await.map(|_| ())
    }

    async fn metadata(&self, handle: &ConnectionHandle) -> anyhow::Result<DatabaseMetadata>;

    async fn execute_query(
        &self,
        handle: &ConnectionHandle,
        query: &str,
    ) -> anyhow::Result<QueryResult>;

    async fn insert_row(
        &self,
        handle: &ConnectionHandle,
        schema: Option<&str>,
        table: &str,
        row: &CrudRow,
    ) -> anyhow::Result<u64>;

    async fn update_rows(
        &self,
        handle: &ConnectionHandle,
        schema: Option<&str>,
        table: &str,
        values: &CrudRow,
        filters: &[CrudFilter],
    ) -> anyhow::Result<u64>;

    async fn delete_rows(
        &self,
        handle: &ConnectionHandle,
        schema: Option<&str>,
        table: &str,
        filters: &[CrudFilter],
    ) -> anyhow::Result<u64>;

    async fn select_rows(
        &self,
        handle: &ConnectionHandle,
        schema: Option<&str>,
        table: &str,
        filters: &[CrudFilter],
        limit: Option<u32>,
    ) -> anyhow::Result<QueryResult>;
}
