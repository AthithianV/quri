PRAGMA foreign_keys = ON;

-- ============================================================
-- Documents
-- Files that the application knows about / has opened.
-- ============================================================

CREATE TABLE documents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    name TEXT NOT NULL,
    path TEXT NOT NULL UNIQUE,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_documents_updated_at
    ON documents(updated_at);


-- ============================================================
-- Tabs
-- Persistent state for opened tabs.
-- A document can be opened in a tab.
-- ============================================================

CREATE TABLE tabs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    document_id INTEGER NOT NULL,

    position INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 0,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (document_id)
        REFERENCES documents(id)
        ON DELETE CASCADE
);

CREATE INDEX idx_tabs_position
    ON tabs(position);


-- ============================================================
-- Recent Files
-- Files recently opened by the user.
-- ============================================================

CREATE TABLE recent_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    document_id INTEGER NOT NULL,

    opened_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (document_id)
        REFERENCES documents(id)
        ON DELETE CASCADE
);

CREATE INDEX idx_recent_files_opened_at
    ON recent_files(opened_at);


-- ============================================================
-- Settings
-- Generic application settings.
-- ============================================================

CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);


-- ============================================================
-- Workspaces
-- Saved workspace state and the currently opened workspace.
-- ============================================================

CREATE TABLE workspace (
    id BLOB PRIMARY KEY,

    name TEXT NOT NULL,
    is_opened BOOLEAN,
    last_opened DATETIME,
    created_at DATETIME,
    updated_at DATETIME
);

INSERT INTO workspace (id, name, is_opened, last_opened, created_at, updated_at)
SELECT X'00000000000040008000000000000001', 'Primary', 1,
       CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
WHERE NOT EXISTS (
    SELECT 1 FROM workspace WHERE name = 'Primary'
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
    query_text TEXT NOT NULL,
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
-- Saved Queries
-- Named, tagged queries.
-- ============================================================

CREATE TABLE saved_query (
    id BLOB PRIMARY KEY,

    connection_id BLOB NOT NULL,
    workspace_id BLOB NOT NULL,
    title TEXT NOT NULL,
    query_text TEXT NOT NULL,
    tags TEXT,
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
    updated_at DATETIME,

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

CREATE TABLE preference (
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
-- Databases
-- Databases discovered for a saved connection.
-- ============================================================

CREATE TABLE database (
    id BLOB PRIMARY KEY,

    connection_id BLOB NOT NULL,
    name TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (connection_id)
        REFERENCES connection(id)
        ON DELETE CASCADE
);


-- ============================================================
-- Database Objects
-- Generalized schema cache for tables, views, functions, etc.
-- ============================================================

CREATE TABLE database_object (
    id BLOB PRIMARY KEY,

    database_id BLOB NOT NULL,
    name TEXT NOT NULL,
    object_type TEXT NOT NULL,
    definition TEXT,
    metadata BLOB,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (database_id)
        REFERENCES database(id)
        ON DELETE CASCADE
);

CREATE INDEX "idx-database-object-name-type"
    ON database_object(name, object_type);
