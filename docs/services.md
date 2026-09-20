# Services

Services live in `src/services`. They keep database access and persistence out
of GPUI rendering code.

## Workspace service

Source: `src/services/workspace.rs`

`WorkspaceService::get_or_create_opened_workspace(db)` is the startup entry
point for local workspace state:

1. Repairs the legacy primary workspace ID when needed.
2. Returns the most recently opened workspace marked `is_opened = true`.
3. Reopens the workspace named `Primary` if it exists.
4. Creates the primary workspace with a stable UUID if no workspace exists.

Private helpers:

- `mark_workspace_opened` updates open and last-opened timestamps.
- `repair_primary_workspace_id` repairs a text-form primary ID in SQLite.

## Saved connection service

Source: `src/services/connection.rs`

`ConnectionService` persists Quri's saved connection records.

- `create_connection(db, input)` creates a UUID, serializes the typed config to
  JSON, and inserts the record.
- `update_connection(db, id, input)` applies only supplied fields, supports
  explicitly clearing nullable fields, updates the timestamp, and returns
  `None` when the ID does not exist.
- `delete_connection(db, id)` deletes by UUID and returns whether a row was
  affected.
- `fetch_connection_by_id(db, id)` returns one connection if present.
- `fetch_connections_by_workspace_id(db, workspace_id)` returns connections
  ordered by name and then creation time.
- `connection_config_to_json` converts serialization errors into `DbErr` for
  consistent service error handling.

Typical read flow:

```rust
let connections = ConnectionService::fetch_connections_by_workspace_id(
    &db,
    workspace.id,
).await?;

let config = connections[0].config()?;
```

## Database plugin architecture

Sources: `src/services/database/{traits,registry,service}.rs`

The database service uses a plugin interface so PostgreSQL, MySQL, and SQLite
share one application API.

### `DatabasePlugin`

The trait defines:

- `db_type` — stable plugin key.
- `connect` — creates a typed SQLx pool wrapped in `ConnectionHandle`.
- `test_connection` — connects and discards the handle by default.
- `metadata` — reads schemas, tables, and columns.
- `execute_query` — executes arbitrary SQL and returns `QueryResult`.
- `insert_row` — inserts one `CrudRow`.
- `update_rows` — updates rows matching `CrudFilter` values.
- `delete_rows` — deletes rows matching filters.
- `select_rows` — selects rows with optional limit and filters.

Implementations are in:

- `src/services/database/postgres.rs` — `PostgresPlugin`
- `src/services/database/mysql.rs` — `MySqlPlugin`
- `src/services/database/sqlite.rs` — `SqlitePlugin`

### `DatabasePluginRegistry`

Source: `src/services/database/registry.rs`

- `new` creates an empty registry.
- `register` inserts any `DatabasePlugin` under its `db_type`.
- `get` returns a cloned `Arc` to a registered plugin.
- `available_plugins` returns sorted plugin keys.
- `create_plugin_registry` registers PostgreSQL, MySQL, and SQLite.

### Database `ConnectionService`

Source: `src/services/database/service.rs`

This service is a thin dispatcher. `new` stores a registry, while `connect`,
`test_connection`, `metadata`, `execute_query`, `insert_row`, `update_rows`,
`delete_rows`, and `select_rows` resolve the plugin from the typed config and
forward the call.

`plugin_for` is the central validation point. If a config's database type is
not registered, it returns an error such as `sqlite plugin is not available`.

```rust
let registry = create_plugin_registry();
let service = ConnectionService::new(registry);
let handle = service.connect(&config).await?;
let metadata = service.metadata(&config, &handle).await?;
```

## SQL helpers

Source: `src/services/database/sql.rs`

- `quote_identifier` safely quotes an identifier for a database dialect.
- `qualified_table` builds an optional schema plus table reference.
- `push_bind` adds a `SqlValue` to a SQLx query builder as a bound parameter.
- `rows_to_result` converts SQLx rows into the common `QueryResult` format.

These helpers centralize identifier quoting and value binding so the individual
database plugins can focus on dialect-specific SQL and row decoding.
