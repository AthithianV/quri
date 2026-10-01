PRAGMA foreign_keys = ON;

-- ============================================================
-- Workspaces
-- Saved workspace state and the currently opened workspace.
-- ============================================================
CREATE TABLE workspace (
    id BLOB PRIMARY KEY,

    name TEXT NOT NULL,
    is_active BOOLEAN,
    is_opened BOOLEAN,
    last_opened DATETIME,
    created_at DATETIME,
    updated_at DATETIME
);


-- ============================================================
-- Connections
-- Saved database connection definitions.
-- ============================================================
CREATE TABLE connection (
    id BLOB PRIMARY KEY,

    workspace_id BLOB NOT NULL,
    connection_name TEXT,
    connection_config TEXT NOT NULL,
    last_connected_at DATETIME,
    last_introspected_at DATETIME,
    created_at DATETIME,
    updated_at DATETIME,

    FOREIGN KEY (workspace_id)
        REFERENCES workspace(id)
        ON DELETE CASCADE
);

-- ============================================================
-- Query Tabs
-- Persistent SQL editor tabs.
-- ============================================================
CREATE TABLE tab (
    id BLOB PRIMARY KEY,

    connection_id BLOB NOT NULL,
    workspace_id BLOB NOT NULL,
    title TEXT NOT NULL,
    cursor_position INTEGER DEFAULT 0,
    is_pinned BOOLEAN DEFAULT 0,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,

    FOREIGN KEY (connection_id)
        REFERENCES connection(id)
        ON DELETE CASCADE,
    FOREIGN KEY (workspace_id)
        REFERENCES workspace(id)
        ON DELETE CASCADE
);


-- ============================================================
-- Settings
-- Generic application settings.
-- ============================================================
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- ============================================================
-- Saved Queries
-- Named, tagged queries.
-- ============================================================
CREATE TABLE saved_query (
    id BLOB PRIMARY KEY,

    connection_id BLOB NOT NULL,
    workspace_id BLOB NOT NULL,
    title TEXT NOT NULL,
    file_path TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,

    FOREIGN KEY (connection_id)
        REFERENCES connection(id)
        ON DELETE CASCADE,
    FOREIGN KEY (workspace_id)
        REFERENCES workspace(id)
        ON DELETE CASCADE
);

-- ============================================================
-- Query History
-- Executed queries and their outcomes.
-- ============================================================
CREATE TABLE query_history (
    id BLOB PRIMARY KEY,

    connection_id BLOB NOT NULL,
    workspace_id BLOB NOT NULL,
    query_text TEXT NOT NULL,
    executed_at DATETIME NOT NULL,
    execution_time_ms INTEGER,
    rows_returned INTEGER,
    status TEXT NOT NULL,
    error_message TEXT,
    created_at DATETIME,

    FOREIGN KEY (connection_id)
        REFERENCES connection(id)
        ON DELETE CASCADE,

    FOREIGN KEY (workspace_id)
        REFERENCES workspace(id)
        ON DELETE CASCADE
);


-- ============================================================
-- Preferences
-- Workspace-scoped JSON preferences.
-- ============================================================
CREATE TABLE workspace_preferences (
    key BLOB PRIMARY KEY,

    workspace_id BLOB NOT NULL,
    value TEXT NOT NULL,
    created_at DATETIME,
    updated_at DATETIME,

    FOREIGN KEY (workspace_id)
        REFERENCES workspace(id)
        ON DELETE CASCADE
);

-- ============================================================
-- Database Metadata
-- Cached metadata from a remote database connection.
-- ============================================================
CREATE TABLE databases (
    id BLOB PRIMARY KEY,

    connection_id BLOB NOT NULL,
    provider TEXT NOT NULL,
    name TEXT,
    fetched_at DATETIME NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (connection_id)
        REFERENCES connection(id)
        ON DELETE CASCADE
);

CREATE INDEX idx_database_connection
    ON database(connection_id, fetched_at DESC);

-- Generalized schema tree for tables, views, routines, and other objects.
CREATE TABLE database_object (
    id BLOB PRIMARY KEY,

    database_id BLOB NOT NULL,
    parent_id BLOB,
    schema_name TEXT,
    name TEXT NOT NULL,
    qualified_name TEXT,
    object_type TEXT NOT NULL,
    definition TEXT,
    metadata TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (database_id)
        REFERENCES databases(id)
        ON DELETE CASCADE,
    FOREIGN KEY (parent_id)
        REFERENCES database_object(id)
        ON DELETE CASCADE
);

CREATE INDEX idx_database_object_snapshot
    ON database_object(snapshot_id, object_type, name);

CREATE INDEX idx_database_object_parent
    ON database_object(parent_id);


-- Object of noSQL databases
CREATE TABLE object_field (
    id BLOB PRIMARY KEY,

    object_id BLOB NOT NULL,
    field_path TEXT NOT NULL,
    inferred_types TEXT,
    is_required BOOLEAN,
    metadata TEXT,

    FOREIGN KEY (object_id)
        REFERENCES database_object(id)
        ON DELETE CASCADE
);


-- Columns belonging to table or view objects.
CREATE TABLE table_column (
    id BLOB PRIMARY KEY,

    object_id BLOB NOT NULL,
    name TEXT NOT NULL,
    ordinal_position INTEGER NOT NULL,
    data_type TEXT NOT NULL,
    is_nullable BOOLEAN NOT NULL DEFAULT 1,
    default_value TEXT,
    is_primary_key BOOLEAN NOT NULL DEFAULT 0,
    metadata TEXT,

    FOREIGN KEY (object_id)
        REFERENCES database_object(id)
        ON DELETE CASCADE,

    UNIQUE (object_id, name),
    UNIQUE (object_id, ordinal_position)
);

CREATE INDEX idx_table_column_object
    ON table_column(object_id, ordinal_position);

-- Indexes belonging to table objects.
CREATE TABLE table_index (
    id BLOB PRIMARY KEY,

    object_id BLOB NOT NULL,
    name TEXT NOT NULL,
    is_unique BOOLEAN NOT NULL DEFAULT 0,
    is_primary BOOLEAN NOT NULL DEFAULT 0,
    index_type TEXT,
    definition TEXT,
    metadata TEXT,

    FOREIGN KEY (object_id)
        REFERENCES database_object(id)
        ON DELETE CASCADE,

    UNIQUE (object_id, name)
);

CREATE INDEX idx_table_index_object
    ON table_index(object_id);

-- Foreign-key constraints belonging to table objects.
CREATE TABLE foreign_key (
    id BLOB PRIMARY KEY,

    object_id BLOB NOT NULL,
    constraint_name TEXT,
    column_name TEXT NOT NULL,
    referenced_object_id BLOB,
    referenced_schema_name TEXT,
    referenced_table_name TEXT NOT NULL,
    referenced_column_name TEXT NOT NULL,
    on_update TEXT,
    on_delete TEXT,
    metadata TEXT,

    FOREIGN KEY (object_id)
        REFERENCES database_object(id)
        ON DELETE CASCADE,
    FOREIGN KEY (referenced_object_id)
        REFERENCES database_object(id)
        ON DELETE SET NULL
);

CREATE INDEX idx_foreign_key_object
    ON foreign_key(object_id);

-- Functions and stored procedures.
CREATE TABLE routine (
    id BLOB PRIMARY KEY,

    snapshot_id BLOB NOT NULL,
    schema_name TEXT,
    name TEXT NOT NULL,
    routine_type TEXT NOT NULL,
    return_type TEXT,
    definition TEXT,
    metadata TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (snapshot_id)
        REFERENCES metadata_snapshot(id)
        ON DELETE CASCADE
);

CREATE INDEX idx_routine_snapshot
    ON routine(snapshot_id, schema_name, name);

-- Triggers associated with tables or views.
CREATE TABLE "trigger" (
    id BLOB PRIMARY KEY,

    snapshot_id BLOB NOT NULL,
    object_id BLOB,
    schema_name TEXT,
    name TEXT NOT NULL,
    event TEXT,
    timing TEXT,
    definition TEXT,
    metadata TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (snapshot_id)
        REFERENCES metadata_snapshot(id)
        ON DELETE CASCADE,
    FOREIGN KEY (object_id)
        REFERENCES database_object(id)
        ON DELETE SET NULL
);

CREATE INDEX idx_trigger_snapshot
    ON "trigger"(snapshot_id, schema_name, name);

-- Sequences exposed by the database provider.
CREATE TABLE "sequence" (
    id BLOB PRIMARY KEY,

    snapshot_id BLOB NOT NULL,
    schema_name TEXT,
    name TEXT NOT NULL,
    data_type TEXT,
    start_value TEXT,
    min_value TEXT,
    max_value TEXT,
    increment_value TEXT,
    metadata TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (snapshot_id)
        REFERENCES metadata_snapshot(id)
        ON DELETE CASCADE
);

CREATE INDEX idx_sequence_snapshot
    ON "sequence"(snapshot_id, schema_name, name);
