# Models

Model modules live in `src/entity`. Each module defines a serializable Rust
`Model` with SQLx's `FromRow` derive so query results can be loaded directly
from the local SQLite database. Services use explicit SQLx queries for writes.

## Model catalog

| Module | Table | Main purpose |
| --- | --- | --- |
| `workspace` | `workspace` | Workspaces and the currently opened workspace |
| `connection` | `connection` | Saved database connection definitions |
| `database` | `database` | Databases discovered for a connection |
| `database_object` | `database_object` | Tables or other database objects and metadata |
| `preference` | `preference` | Workspace-scoped JSON preferences |
| `query_history` | `query_history` | Executed queries and their outcomes |
| `saved_query` | `saved_query` | Named, tagged queries |
| `tab` | `tab` | Query editor tabs and cursor state |

## Fields by entity

### `workspace::Model`

Source: `src/entity/workspace.rs`

Fields: `id`, `name`, `is_opened`, `last_opened`, `created_at`, and `updated_at`.
The workspace service uses `is_opened` and `last_opened` to choose the active
workspace.

### `connection::Model`

Source: `src/entity/connection.rs`

Fields: `id`, `workspace_id`, optional `connection_name`, JSON
`connection_config`, and connection/create/update timestamps.

`ConnectionConfig` is a tagged serializable enum with three variants:

```rust
pub enum ConnectionConfig {
    Postgres(PostgresConfig),
    MySql(MySqlConfig),
    Sqlite(SqliteConfig),
}
```

Its functions are:

- `database_type` returns `"postgres"`, `"mysql"`, or `"sqlite"`.
- `into_json` serializes the config for the entity's JSON column.
- `from_json` deserializes JSON back into a typed config.
- `Model::config` deserializes a persisted model's `connection_config`.

`PostgresConfig` contains host, port, database name, username, optional
password, optional SSL mode, and optional extra parameters. `MySqlConfig`
contains the equivalent MySQL fields. `SqliteConfig` contains a file path and
optional extra parameters.

### `database::Model`

Source: `src/entity/database.rs`

Fields: `id`, `connection_id`, `name`, `created_at`, and `updated_at`. It
represents a database associated with a saved connection.

### `database_object::Model`

Source: `src/entity/database_object.rs`

Fields: `id`, `database_id`, `name`, `object_type`, optional `definition`,
optional JSON `metadata`, and `created_at`. It can represent tables, views, or
other database objects discovered from metadata.

### `preference::Model`

Source: `src/entity/preference.rs`

Fields: UUID `key`, `workspace_id`, JSON `value`, and optional create/update
timestamps. The JSON value allows preferences to evolve without a new column for
every setting.

### `query_history::Model`

Source: `src/entity/query_history.rs`

Fields include `connection_id`, `workspace_id`, `query_text`, `executed_at`,
optional `execution_time_ms`, optional `rows_returned`, `status`, optional
`error_message`, and timestamps.

### `saved_query::Model`

Source: `src/entity/saved_query.rs`

Fields include connection/workspace IDs, `title`, `query_text`, optional JSON
`tags`, and create/update timestamps.

### `tab::Model`

Source: `src/entity/tab.rs`

Fields include connection/workspace IDs, `title`, `query_text`,
`cursor_position`, `is_pinned`, and create/update timestamps. This model stores
the editor state needed to restore a query tab.

## Model usage example

Services update models with explicit SQL statements:

```rust
sqlx::query("UPDATE connection SET updated_at = ? WHERE id = ?")
    .bind(chrono::Utc::now().naive_utc())
    .bind(model.id)
    .execute(db)
    .await?;
```

Foreign-key intent is expressed in the SQL schema; the ID fields on the models
provide the corresponding domain links used by application code.
