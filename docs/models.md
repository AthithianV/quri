# Models and shared runtime types

This document covers non-ORM models used to move data between the UI, state,
and database execution services. ORM records are documented separately in
[Entities](entities.md).

## `AppState`

Source: `src/state/app_state.rs`

`AppState` is the shared GPUI global. It wraps `AppStateInner` in an `Arc`; the
inner fields use `tokio::sync::RwLock` so asynchronous tasks can safely access
the app database, opened workspace, and active connection pools.

```rust
pub struct AppStateInner {
    pub app_db: RwLock<Option<DbConn>>,
    pub opened_workspace: RwLock<Option<workspace::Model>>,
    pub connection_pools: RwLock<HashMap<i64, ConnectionPool>>,
}
```

Functions:

- `AppState::new` creates empty shared state.
- `add_connection_pool` inserts or replaces a pool by connection ID.
- `remove_connection_pool` removes a pool.
- `get_connection_pool` reads and clones a pool.
- `get_app_db_connection` returns the local app database or an initialization
  error.
- `set_app_db_connection` stores the local app database.
- `get_opened_workspace` returns the current workspace or an initialization
  error.
- `set_opened_workspace` stores the current workspace.

`DbType` identifies PostgreSQL, MySQL, or SQLite. `DbPoolType` stores the
corresponding SQLx pool. `ConnectionPool` groups a pool with its connection ID,
display name, and database type; `ConnectionPool::new` is its constructor.

## Database operation models

Source: `src/services/database/traits.rs`

These serializable types provide a database-agnostic API:

- `ConnectionHandle` wraps a SQLx pool for PostgreSQL, MySQL, or SQLite.
- `DatabaseMetadata` contains schemas.
- `SchemaMetadata` contains tables.
- `TableMetadata` contains columns.
- `ColumnMetadata` describes a column name, type, nullability, and ordinal.
- `QueryResult` contains columns, rows, and affected-row count.
- `QueryColumn` describes one returned column.
- `QueryValue` represents values returned by a query. Its float variant is a
  string to preserve database formatting/precision.
- `SqlValue` represents values sent to CRUD operations. Its float variant is
  an `f64`.
- `CrudRow` is a `BTreeMap<String, SqlValue>`.
- `CrudFilter` pairs a column name with a `SqlValue` predicate value.

Example result shape:

```rust
QueryResult {
    columns: vec![QueryColumn {
        name: "id".into(),
        data_type: "INTEGER".into(),
    }],
    rows: vec![],
    rows_affected: 0,
}
```

## Connection input models

Source: `src/services/connection.rs`

- `CreateConnection` contains workspace ID, optional display name, typed
  `ConnectionConfig`, and optional last-connected time.
- `UpdateConnection` uses `Option<Option<T>>` for nullable fields, allowing a
  caller to distinguish “leave unchanged” from “set this field to null”.

These models are consumed by the persistence methods documented in
[Services](services.md).
