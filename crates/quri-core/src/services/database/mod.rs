#![allow(dead_code, unused_imports)]

pub mod mysql;
pub mod postgres;
pub mod registry;
pub mod service;
mod sql;
pub mod sqlite;
pub mod traits;

pub use registry::{create_plugin_registry, DatabasePluginRegistry};
pub use service::ConnectionService;
pub use traits::{
    ColumnMetadata, ConnectionHandle, CrudFilter, CrudRow, DatabaseMetadata, DatabasePlugin,
    QueryColumn, QueryResult, QueryValue, SchemaMetadata, SqlValue, TableMetadata,
};
