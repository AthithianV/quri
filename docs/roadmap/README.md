# Quri Roadmap

This is the implementation checklist for Quri. It starts from the current
baseline—an empty sidebar, a main panel, and an initial connection window—and
turns the architecture plan into small, verifiable milestones.

The rule for this roadmap is simple: every phase should leave Quri in a usable
state. Do not start extension packaging, registry, or marketplace work until
the local application can reliably connect to a database, inspect it, execute a
query, and render the result.

## Current baseline

- [x] Open a GPUI window with the Quri layout.
- [x] Render the title bar, activity bar, left sidebar, right sidebar, and main panel.
- [x] Show an empty connection sidebar.
- [x] Open the add-connection window.
- [ ] Persist a connection created from the UI.
- [ ] Load saved connections when Quri starts.
- [ ] Connect to a user database.

## Phase 0 — Establish the foundation

Goal: make the workspace predictable before adding product features.

- [ ] Confirm the crate boundaries:
  - [ ] `quri-ui` — GPUI views and UI interaction state.
  - [ ] `quri-core` — domain types, application state, services, commands, and events.
  - [ ] `quri-storage` — local SQLite/SQLx persistence.
  - [ ] `quri-extension-api` — database and extension contracts.
  - [ ] `quri-extension-host` — Wasmtime runtime, when extensions are introduced.
- [ ] Make the workspace compile with the intended crates enabled.
- [ ] Add a consistent application error type and error-reporting path.
- [ ] Add structured logging for startup, storage, connection, and query operations.
- [ ] Document the startup flow and dependency direction.
- [ ] Keep GPUI types out of `quri-core` domain and service APIs.
- [ ] Keep PostgreSQL/MySQL/SQLite-specific types out of generic core APIs.

## Phase 1 — Local application state

Goal: start Quri with durable local state and restore the previous workspace.

- [ ] Define stable IDs: `WorkspaceId`, `ConnectionId`, `QueryId`, and `ObjectId`.
- [ ] Define the application domain models for workspaces, saved connections, settings, tabs, and saved queries.
- [ ] Define storage/repository traits in `quri-core`.
- [ ] Move or implement the SQLite repositories in `quri-storage`.
- [ ] Add migrations for:
  - [ ] Workspaces.
  - [ ] Saved connections.
  - [ ] Settings.
  - [ ] Saved queries/query history.
- [ ] Initialize the local database during application startup.
- [ ] Load or create the primary workspace.
- [ ] Replace placeholder `ConnectionListItem` data with records loaded from storage.
- [ ] Refresh the sidebar after creating, editing, or deleting a connection.
- [ ] Show useful loading, empty, and error states in the sidebar.
- [ ] Add tests for repository CRUD and startup restoration.

## Phase 2 — Connection management

Goal: manage saved connection definitions separately from live database sessions.

- [ ] Separate `ConnectionConfig` from the persisted connection model.
- [ ] Define connection status: disconnected, connecting, connected, failed, and closing.
- [ ] Validate connection forms before persistence.
- [ ] Persist connection names and non-secret configuration.
- [ ] Move passwords, tokens, and private keys behind a `SecretStore` interface.
- [ ] Do not store secrets as ordinary plaintext SQLite fields.
- [ ] Add connect, disconnect, reconnect, and test-connection actions.
- [ ] Show connection status and errors in the sidebar.
- [ ] Allow selecting one connection as the active connection.
- [ ] Add integration tests using a supported database fixture where practical.

## Phase 3 — Stable database API

Goal: define the generic database contract before adding more database engines.

- [ ] Define `DatabaseProvider` in `quri-extension-api`.
- [ ] Define generic types for:
  - [ ] `ConnectionConfig`.
  - [ ] `ConnectionHandle` or connection ID.
  - [ ] `Query` and `QueryId`.
  - [ ] `QueryResult` and query rows.
  - [ ] Typed `Value` values.
  - [ ] Database metadata.
  - [ ] Cancellation and streaming.
- [ ] Define generic metadata for databases, schemas, tables, columns, indexes, foreign keys, views, and routines.
- [ ] Define capability discovery for optional operations such as transactions, explain, and streaming.
- [ ] Define database errors without exposing SQLx error types across the API boundary.
- [ ] Add serialization tests for the public API types.
- [ ] Make the first API usable without GPUI or Wasmtime.

## Phase 4 — First database implementation

Goal: complete one end-to-end database workflow before generalizing further.

- [ ] Choose PostgreSQL as the first production provider.
- [ ] Implement the provider behind the generic database API.
- [ ] Keep PostgreSQL-specific SQL and value conversion inside the provider.
- [ ] Connect from the selected saved connection.
- [ ] Introspect the database.
- [ ] Render the database/schema/table tree in the left sidebar.
- [ ] Refresh metadata explicitly and after reconnecting.
- [ ] Handle provider errors without crashing the application.
- [ ] Add a SQLite provider only after the PostgreSQL path is working, if SQLite support is still required at this stage.

## Phase 5 — Query workspace

Goal: turn the empty main panel into a useful query workflow.

- [ ] Create a query tab for the active connection.
- [ ] Add a SQL editor with basic editing and dirty-state tracking.
- [ ] Add execute and cancel actions.
- [ ] Run queries asynchronously without blocking the UI.
- [ ] Stream or incrementally receive rows where the provider supports it.
- [ ] Render columns, typed values, nulls, errors, and affected-row counts.
- [ ] Show query duration and execution status.
- [ ] Preserve query text when switching tabs or connections.
- [ ] Add query history and saved-query actions.
- [ ] Add tests for query success, failure, cancellation, and empty results.

## Phase 6 — Core commands and events

Goal: make application behavior composable and ready for extensions.

- [ ] Add a command registry and command dispatch interface.
- [ ] Register commands for:
  - [ ] New connection.
  - [ ] Connect/disconnect.
  - [ ] Refresh metadata.
  - [ ] New query.
  - [ ] Execute query.
  - [ ] Cancel query.
  - [ ] Save query.
- [ ] Add an event bus with events for workspace, connection, query, metadata, and extension lifecycle changes.
- [ ] Route UI actions through commands instead of directly calling persistence or provider implementations.
- [ ] Use events to refresh dependent views such as the sidebar and result panel.
- [ ] Add keyboard shortcuts and command-palette entries for core commands.

## Phase 7 — Settings, workspace, and product polish

Goal: make the first database client workflow comfortable for daily use.

- [ ] Restore open tabs and selected connection where practical.
- [ ] Add settings storage and settings UI for non-secret preferences.
- [ ] Add workspace switching or clearly defer it to a later milestone.
- [ ] Add confirmation flows for destructive database operations.
- [ ] Improve empty states and first-run guidance.
- [ ] Add loading indicators and cancellation to all long-running operations.
- [ ] Add keyboard navigation to the connection tree and result table.
- [ ] Add error details with a copy action.
- [ ] Add application-level smoke tests for startup, connection creation, and query execution.

## Phase 8 — Extension API and static extraction

Goal: extract the first database provider without changing the user workflow.

- [ ] Finalize the Rust extension API from the working database abstractions.
- [ ] Define the extension lifecycle: discover, validate, load, activate, deactivate, unload, and disable on failure.
- [ ] Define extension manifests and API compatibility versions.
- [ ] Implement a statically linked PostgreSQL extension first.
- [ ] Register database providers through the extension manager.
- [ ] Verify that `quri-core` has no database-vendor branching.
- [ ] Add extension lifecycle events and structured logs.
- [ ] Add an example extension that registers one command.

## Phase 9 — WIT and Wasmtime runtime

Goal: make the extension boundary real only after the static API is proven.

- [ ] Define WIT contracts for types, extensions, databases, commands, events, storage, and UI contributions.
- [ ] Generate host bindings and SDK bindings from WIT.
- [ ] Implement the Wasmtime extension host.
- [ ] Add capability-gated host functions.
- [ ] Isolate extension failures so a crashed extension cannot terminate Quri.
- [ ] Load and activate a minimal example WASM extension.
- [ ] Port the PostgreSQL extension to WASM.
- [ ] Verify query cancellation, metadata, typed values, and errors across the boundary.

## Phase 10 — Developer workflow and distribution

Goal: make extensions buildable and installable by developers and users.

- [ ] Add `quri extension new`.
- [ ] Add `quri extension build`.
- [ ] Add `quri extension dev` with file watching and reload.
- [ ] Add `.qext` packaging.
- [ ] Add local install, uninstall, list, and validation commands.
- [ ] Add checksums and compatibility validation.
- [ ] Add signatures only when the package workflow is stable.
- [ ] Add the initial GitHub-backed registry client.
- [ ] Defer a hosted registry, dependency resolution, and marketplace UI until there is a real extension ecosystem.

## Milestone definition of done

The first major product milestone is complete when this path works reliably:

```text
Start Quri
  → restore local workspace
  → create or select a saved connection
  → connect to PostgreSQL
  → introspect schemas and tables
  → open a query tab
  → execute SQL
  → render typed results
  → cancel a long-running query
  → close and reopen Quri with state restored
```

At that point Quri is a useful database client and the extension/runtime work
has a real, tested contract to build on.

## Explicitly deferred

- [ ] Marketplace and hosted registry.
- [ ] Community extension ecosystem.
- [ ] Complex extension dependency resolution.
- [ ] AI features.
- [ ] Full declarative UI extension framework.
- [ ] Signing infrastructure before packaging and installation are stable.
- [ ] Supporting multiple database engines before the first provider workflow is complete.
