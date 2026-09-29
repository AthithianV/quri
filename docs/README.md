# Quri documentation

Quri is a lightweight database client built with Rust, GPUI, and SQLx.
The application is organized around a small UI layer, persistent SQLx models, application state, and database services.

## Crate responsibilities

Quri is split into focused crates so the user interface, application state, persistent data, and extension system can evolve independently.

### `quri-core`

- The application and domain layer.
- It is responsible for shared application state, active workspaces, open database connection pools, domain models, application services, and interfaces used by infrastructure such as storage, database providers, and extensions.

### `quri-storage`

- The persistence and database-access layer.
- It owns stored entities such as workspaces, connections, tabs, settings, and database objects; initializes the local SQLite database; runs persistence services; and provides database provider abstractions for SQLite, PostgreSQL, and MySQL, including metadata, query, and CRUD operations.

### `quri-ui`

- The desktop presentation layer built with GPUI.
- It starts the application, creates the root window, applies the theme, loads assets, and provides the connection views, editor, layout, sidebar, title bar, and other user-interface components.

### `quri-extension-api`

- The public contract for Quri extensions.
- It is intended to contain the shared types, serialization formats, and asynchronous interfaces that an extension uses to communicate with Quri.

### `quri-extension-host`

- The extension runtime boundary. It is intended to discover, load, manage, and communicate with extensions through `quri-extension-api`, while keeping extension lifecycle and execution details outside the core and UI layers.
