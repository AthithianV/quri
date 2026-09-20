# Components

UI components live in `src/components` and are rendered by GPUI. The root
composition is in `src/window/root_window.rs`.

## Component hierarchy

```text
RootWindow
├── TitleBar
├── MiniSidebar
└── resizable layout
    ├── SideBar
    │   └── ConnectionSideBar
    └── MainPanel
        └── Editor
```

## Layout components

### `TitleBar`

Source: `src/components/layouts/titlebar.rs`

- `TitleBar::new(title)` stores the window title.
- `RenderOnce::render` creates the GPUI component title bar, applies the Quri
  background and border colors, and adds the application menu.

```rust
TitleBar::new("Quri".into())
```

### `MiniSidebar`

Source: `src/components/layouts/mini_sidebar.rs`

- `MiniSidebar::new()` creates the compact navigation rail.
- `RenderOnce::render` displays database, history, and saved-query buttons.

The buttons currently provide the visual navigation shell; their actions are not
connected to application state yet.

### `SideBar`

Source: `src/components/layouts/sidebar.rs`

- `SideBar::new()` creates the main sidebar container.
- `RenderOnce::render` creates a keyed `ConnectionSideBar` state and fills the
  sidebar with it.

The keyed state keeps the connection sidebar instance stable across renders.

### `MainPanel`

Source: `src/components/layouts/main_panel.rs`

- `MainPanel::new(title)` stores the panel title.
- `IntoElement::into_element` creates the card-like editor panel.

The title is currently stored as `_title`; the visible panel contains an editor
initialized with `SELECT * FROM USERS`.

## Connection components

### `ConnectionSideBar`

Source: `src/components/connection/connection_sidebar.rs`

This is a stateful GPUI view that lists saved connections for the opened
workspace.

- `ConnectionSideBar::new` initializes empty state and calls `refresh`.
- `refresh` obtains the global `AppState`, marks the view as loading, and loads
  connections on a GPUI background task.
- `render_connection` converts one connection item into a clickable list row.
- `Render::render` displays loading, error, empty, and populated states.
- `load_connections` reads the app database and opened workspace, then calls
  `ConnectionService::fetch_connections_by_workspace_id`.
- `connection_list_item` converts an entity model into display data and handles
  invalid JSON configuration gracefully.
- `fallback_connection_name` derives a name from the database name or SQLite
  path when no explicit name exists.
- `connection_summary` formats a host/port/database or SQLite path summary.

The refresh and add buttons are wired in the view:

```rust
Button::new("Refresh Connections")
    .on_click(cx.listener(|this, _, _, cx| this.refresh(cx)))
```

Selecting a row currently logs its ID. The add button opens `ConnectionWindow`.

### `ConnectionWindow`

Source: `src/components/connection/connection_window.rs`

This window collects PostgreSQL, MySQL, or SQLite connection settings.

- `DatabaseType::label` returns the display label for a database type.
- `DatabaseTypeOption::new` creates an item for the searchable type selector.
- `SearchableListItem::title` and `value` adapt the item to GPUI Component's
  searchable list API.
- `ConnectionWindow::new` creates all input states and subscribes to database
  type changes. PostgreSQL defaults to port `5432`; MySQL defaults to `3306`.
- `labeled_input` builds a reusable labeled text input.
- `sqlite_path_input` builds a SQLite path input and file picker button.
- `input_value` trims an input value.
- `optional_input_value` turns an empty trimmed value into `None`.
- `connection_config_from_form` validates the port or SQLite path and creates a
  typed `ConnectionConfig`.
- `Render::render` switches between server fields and the SQLite file picker.

The form-to-domain conversion is kept separate from rendering:

```rust
let config = ConnectionWindow::connection_config_from_form(
    DatabaseType::Postgres,
    host,
    port,
    username,
    password,
    database_name,
    String::new(),
)?;
```

The current window renders the form; persistence through
`ConnectionService::create_connection` is the next integration point for its
save action.

## Editor components

### `Editor`

Source: `src/components/editor/editor.rs`

- `Editor::new(content)` stores the initial editor content.
- `RenderOnce::render` creates a keyed GPUI `InputState` configured as a SQL
  code editor with search enabled and whitespace markers disabled.

```rust
Editor::new("SELECT * FROM USERS".into())
```

## Other UI modules

- `src/components/layouts/activity_bar.rs` contains `ActivityBar`, a compact
  status/action bar showing database, history, saved-query, cell, column, sum,
  and AI controls.
- `src/window/root_window.rs` contains `RootWindow::new` and the top-level
  `Render::render` composition.
