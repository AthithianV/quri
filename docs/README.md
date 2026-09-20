# Quri documentation

Quri is a lightweight database client built with Rust, GPUI, and SQLx.
The application is organized around a small UI layer, persistent SQLx models,
application state, and database services.

## Documentation map

- [Components](components.md) — GPUI windows and reusable UI elements in `src/components`.
- [Models](models.md) — shared runtime state and database operation DTOs.
- [Services](services.md) — persistence services and the database plugin abstraction.
- [Models](entities.md) — SQLx models mapped to the local application database.

## Runtime flow

```text
main
 ├─ initializes the local SQLx database
 ├─ loads or creates the opened workspace
 ├─ stores both in AppState
 └─ opens RootWindow
      ├─ TitleBar
      ├─ MiniSidebar
      ├─ SideBar -> ConnectionSideBar
      └─ MainPanel -> Editor
```

The local database stores Quri data such as workspaces and saved connections.
Connections to user databases are handled separately through the database plugin
services described in [Services](services.md).

## Source conventions

- UI types generally implement `Render`, `RenderOnce`, `IntoElement`, or
  `IntoElement`-related GPUI traits.
- Database records are SQLx `FromRow` models with explicit SQL queries.
- Long-running or database operations are asynchronous and return `Result`.
- `AppState` is registered as a GPUI global and is shared through `Arc` plus
  asynchronous read/write locks.
