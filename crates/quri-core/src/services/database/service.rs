#![allow(dead_code)]

use super::{
    registry::DatabasePluginRegistry,
    traits::{ConnectionHandle, CrudFilter, CrudRow, DatabaseMetadata, QueryResult},
};
use crate::entity::connection::ConnectionConfig;
use anyhow::Context;

#[derive(Clone)]
pub struct ConnectionService {
    registry: DatabasePluginRegistry,
}

impl ConnectionService {
    pub fn new(registry: DatabasePluginRegistry) -> Self {
        Self { registry }
    }

    pub async fn connect(&self, config: &ConnectionConfig) -> anyhow::Result<ConnectionHandle> {
        self.plugin_for(config)?.connect(config).await
    }

    pub async fn test_connection(&self, config: &ConnectionConfig) -> anyhow::Result<()> {
        self.plugin_for(config)?.test_connection(config).await
    }

    pub async fn metadata(
        &self,
        config: &ConnectionConfig,
        handle: &ConnectionHandle,
    ) -> anyhow::Result<DatabaseMetadata> {
        self.plugin_for(config)?.metadata(handle).await
    }

    pub async fn execute_query(
        &self,
        config: &ConnectionConfig,
        handle: &ConnectionHandle,
        query: &str,
    ) -> anyhow::Result<QueryResult> {
        self.plugin_for(config)?.execute_query(handle, query).await
    }

    pub async fn insert_row(
        &self,
        config: &ConnectionConfig,
        handle: &ConnectionHandle,
        schema: Option<&str>,
        table: &str,
        row: &CrudRow,
    ) -> anyhow::Result<u64> {
        self.plugin_for(config)?
            .insert_row(handle, schema, table, row)
            .await
    }

    pub async fn update_rows(
        &self,
        config: &ConnectionConfig,
        handle: &ConnectionHandle,
        schema: Option<&str>,
        table: &str,
        values: &CrudRow,
        filters: &[CrudFilter],
    ) -> anyhow::Result<u64> {
        self.plugin_for(config)?
            .update_rows(handle, schema, table, values, filters)
            .await
    }

    pub async fn delete_rows(
        &self,
        config: &ConnectionConfig,
        handle: &ConnectionHandle,
        schema: Option<&str>,
        table: &str,
        filters: &[CrudFilter],
    ) -> anyhow::Result<u64> {
        self.plugin_for(config)?
            .delete_rows(handle, schema, table, filters)
            .await
    }

    pub async fn select_rows(
        &self,
        config: &ConnectionConfig,
        handle: &ConnectionHandle,
        schema: Option<&str>,
        table: &str,
        filters: &[CrudFilter],
        limit: Option<u32>,
    ) -> anyhow::Result<QueryResult> {
        self.plugin_for(config)?
            .select_rows(handle, schema, table, filters, limit)
            .await
    }

    fn plugin_for(
        &self,
        config: &ConnectionConfig,
    ) -> anyhow::Result<std::sync::Arc<dyn super::traits::DatabasePlugin>> {
        self.registry
            .get(config.database_type())
            .with_context(|| format!("{} plugin is not available", config.database_type()))
    }
}
