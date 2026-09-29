# Quri Design Plan

## 1. Product architecture

```text
                         ┌──────────────────────────┐
                         │          QURI APP        │
                         │                          │
                         │  GPUI                    │
                         │  ├── Workspace           │
                         │  ├── Sidebar             │
                         │  ├── Tabs                │
                         │  ├── Editors             │
                         │  └── Main Panels         │
                         │                          │
                         └────────────┬─────────────┘
                                      │
                         ┌────────────▼─────────────┐
                         │       QURI CORE          │
                         │                          │
                         │  Extension Manager       │
                         │  Command System          │
                         │  Event System            │
                         │  Workspace               │
                         │  Settings                │
                         │  Secrets                 │
                         │  Storage                 │
                         └────────────┬─────────────┘
                                      │
                         ┌────────────▼─────────────┐
                         │   STABLE EXTENSION API   │
                         │                          │
                         │ WIT / Component Model    │
                         │                          │
                         │ Extension                │
                         │ Database                 │
                         │ UI                       │
                         │ Commands                 │
                         │ Events                   │
                         │ Storage                  │
                         └────────────┬─────────────┘
                                      │
                         ┌────────────▼─────────────┐
                         │    EXTENSION HOST        │
                         │        Wasmtime          │
                         └────────────┬─────────────┘
                                      │
               ┌──────────────────────┼──────────────────────┐
               ▼                      ▼                      ▼
        postgres.wasm            mysql.wasm             sqlite.wasm
```

Quri Core will remain database-agnostic; database-specific behavior belongs to extensions.

---

# 2. Divide the system into 5 layers

### Layer 1 — Application

Responsible for:

- GPUI
- windows
- workspace
- tabs
- layout
- theme
- keybindings
- command palette

### Layer 2 — Core

Responsible for:

- application state
- extension manager
- extension lifecycle
- commands
- events
- settings
- logging
- notifications
- storage
- secrets
- connection management

### Layer 3 — Stable API

This is the most important layer.

```text
quri-extension-api
        │
        ├── Extension API
        ├── Database API
        ├── UI API
        ├── Command API
        ├── Event API
        ├── Storage API
        └── Types
```

### Layer 4 — Runtime

```text
quri-extension-host
        │
        └── Wasmtime
             │
             └── WASM Components
```

### Layer 5 — Extensions

```text
extensions/
├── postgres
├── mysql
├── sqlite
├── er-diagram
├── sql-formatter
└── ...
```

---

# 3. Repository design

```text
quri/
│
├── Cargo.toml
│
├── crates/
│   ├── quri/
│   ├── quri-core/
│   ├── quri-ui/
│   ├── quri-extension-api/
│   ├── quri-extension-host/
│   ├── quri-extension-manager/
│   ├── quri-storage/
│   ├── quri-secrets/
│   ├── quri-registry/
│   ├── quri-sdk/
│   └── quri-cli/
│
├── wit/
│   ├── extension.wit
│   ├── types.wit
│   ├── database.wit
│   ├── ui.wit
│   ├── commands.wit
│   ├── events.wit
│   └── storage.wit
│
├── extensions/
│   └── postgres/
│
├── examples/
│   └── hello-extension/
│
└── tests/
    └── integration/
```

---

# 4. Design the API first

Before implementing WASM, define the domain model.

## Core types

```rust
ExtensionId
ExtensionMetadata
ConnectionId
QueryId
ObjectId
```

## Database

```rust
DatabaseProvider
Connection
ConnectionConfig
Query
QueryResult
QueryStream
DatabaseMetadata
Database
Schema
Table
Column
Index
ForeignKey
View
Function
Procedure
Trigger
Sequence
```

## Values

```rust
enum Value {
    Null,
    Bool(bool),
    Int64(i64),
    UInt64(u64),
    Float64(f64),
    Decimal(String),
    String(String),
    Bytes(Vec<u8>),
    Date(String),
    Time(String),
    Timestamp(String),
    Json(String),
    Uuid(String),

    Unsupported {
        type_name: String,
        display: String,
    },
}
```

The source design specifically calls out this typed-value model because databases expose substantially more than strings.

---

# 5. Database API

```rust
trait DatabaseProvider {
    fn connect(
        &self,
        config: ConnectionConfig,
    ) -> Result<ConnectionId>;

    fn disconnect(
        &self,
        connection: ConnectionId,
    ) -> Result<()>;

    fn execute(
        &self,
        connection: ConnectionId,
        query: Query,
    ) -> Result<QueryResult>;

    fn cancel(
        &self,
        query: QueryId,
    ) -> Result<()>;

    fn introspect(
        &self,
        connection: ConnectionId,
    ) -> Result<DatabaseMetadata>;
}
```

Then evolve it with:

```text
Transactions
Streaming
Cancellation
Prepared statements
Explain
Catalog
Optional capabilities
```

---

# 6. Query execution pipeline

Design this before building the UI.

```text
                 Quri UI
                    │
                    ▼
              Query Editor
                    │
                    ▼
              Query Service
                    │
                    ▼
             Extension API
                    │
                    ▼
          Extension Host / WASM
                    │
                    ▼
           Database Provider
                    │
                    ▼
                Database
```

For results:

```text
Database
   │
   ▼
Query Stream
   │
   ├── Row
   ├── Row
   ├── Row
   └── ...
        │
        ▼
    Result Model
        │
        ▼
      GPUI Table
```

Streaming and cancellation should be designed into this pipeline rather than retrofitted later.

---

# 7. Generic database metadata model

```text
DatabaseMetadata
│
├── databases
├── schemas
│   ├── tables
│   │   ├── columns
│   │   ├── indexes
│   │   └── foreign keys
│   │
│   ├── views
│   ├── functions
│   ├── procedures
│   ├── triggers
│   └── sequences
```

Then:

```text
PostgreSQL ──┐
MySQL ───────┤
SQLite ──────┼──► DatabaseMetadata
DuckDB ──────┤
MongoDB ─────┘
                  │
                  ▼
             Quri Sidebar
                  │
                  ▼
             Other Extensions
```

This is one of the most valuable architectural decisions in the document because tools such as ER diagrams can consume generic `DatabaseMetadata` without understanding PostgreSQL specifically.

---

# 8. Extension lifecycle

Define this as a state machine.

```text
Installed
   │
   ▼
Discovered
   │
   ▼
Validated
   │
   ▼
Loaded
   │
   ▼
Activated
   │
   ▼
Running
   │
   ├────── error ──────► Disabled
   │
   ▼
Deactivated
   │
   ▼
Unloaded
```

Each transition should have:

```rust
ExtensionEvent
```

and structured logging.

Most importantly:

```text
Extension crash
      ↓
Extension disabled
      ↓
Quri continues running
```

The WASM boundary should enforce this isolation.

---

# 9. Capability model

```text
Extension → Computer
```

Instead:

```text
Extension
   │
   ├── UI capability
   ├── Database capability
   ├── Storage capability
   ├── Secret capability
   ├── Network capability
   ├── Filesystem capability
   └── Process capability
```

Default permissions should be restrictive.

For example:

```toml
[capabilities]
database = true
network = false
filesystem = false
process = false
storage = true
```

Then permissions can be narrowed further.

The source architecture explicitly treats each host interaction as a capability rather than granting extensions unrestricted machine access.

---

# 10. Manifest design

Create the manifest schema early.

```toml
id = "postgres"
name = "PostgreSQL"
version = "0.1.0"

description = "PostgreSQL support for Quri"
license = "MIT"

[quri]
api = "1"

[capabilities]
database = true
network = true
storage = true

[contributes]
database = true
commands = true
panels = true

[permissions]
filesystem = false
process = false
```

The important thing is:

```toml
[quri]
api = "1"
```

rather than coupling compatibility directly to a particular Quri application version.

---

# 11. UI architecture

GPUI is not exposed directly.

Instead:

```text
Extension
    │
    ▼
Quri UI API
    │
    ▼
UI Host
    │
    ▼
GPUI
```

Quri uses declarative UI:

```text
Panel
 ├── Header
 │    ├── Text
 │    └── Button
 │
 └── Table
      ├── Column
      └── Rows
```

Initial components:

```text
Panel
Text
Button
Input
Table
Tree
Menu
Dialog
Icon
```

This gives the ability to change GPUI internals later without breaking extensions. The source explicitly recommends an abstraction between extension authors and GPUI and suggests declarative UI for v1.

---

# 12. Command system

```rust
Command {
    id: "postgres.refresh-schema",
    title: "Refresh Schema",
}
```

Commands are usable from:

```text
Command Palette
Keyboard shortcuts
Menus
Toolbar
Context menus
Extension UI
```

Architecture:

```text
Extension
    │
    ▼
Command Registry
    │
    ├── Command Palette
    ├── Menus
    ├── Keybindings
    └── Toolbar
```

---

# 13. Event system

A central event bus is defined.

```text
ConnectionOpened
ConnectionClosed

QueryStarted
QueryCompleted
QueryFailed
QueryCancelled

SchemaChanged
SelectionChanged

WorkspaceChanged

ExtensionActivated
ExtensionDeactivated
```

Then extensions can compose.

Example:

```text
Postgres
   │
   │ QueryCompleted
   ▼
Query Profiler
   │
   ▼
Performance Analysis
```

This follows the proposed event-driven extension composition model.

---

# 14. Extension SDK

The developer should never deal directly with:

```text
Wasmtime
WIT
Component
Linker
Store
Instance
```

Instead:

```rust
use quri_sdk::prelude::*;

pub struct PostgresExtension;

impl Extension for PostgresExtension {
    fn activate(&mut self, ctx: ExtensionContext) {
        ctx.register_database(PostgresDriver);

        ctx.register_command(
            Command::new(
                "postgres.new-query",
                "New PostgreSQL Query",
            )
        );
    }
}
```

The SDK should be the ergonomic Rust layer over the WIT contract.

---

# 15. WIT should be the actual contract

Structure:

```text
wit/
├── extension.wit
├── types.wit
├── database.wit
├── ui.wit
├── commands.wit
├── events.wit
└── storage.wit
```

Conceptually:

```text
                WIT
                 │
       ┌─────────┴─────────┐
       ▼                   ▼
 Quri Host Bindings    SDK Bindings
       │                   │
       ▼                   ▼
   Wasmtime             Extension
```

Rust traits are an SDK implementation convenience.

**WIT is the ABI/API contract.**

That's an important distinction in the architecture document.

---

# 16. PostgreSQL migration strategy

Don't rewrite everything at once.

### Current

```text
Quri
 │
 └── PostgreSQL implementation
```

### Step 1

```text
Quri Core
 │
 ▼
DatabaseProvider
 │
 ▼
PostgreSQL implementation
```

Still statically linked.

### Step 2

```text
Quri
 │
 ▼
quri-sdk
 │
 ▼
PostgreSQL extension
```

### Step 3

```text
Quri
 │
 ▼
Extension Host
 │
 ▼
postgres.wasm
 │
 ▼
PostgreSQL
```

---

# 17. CLI design

```bash
quri extension new <name>

quri extension build

quri extension dev

quri extension package

quri extension install <path>

quri extension uninstall <id>

quri extension list

quri extension search <query>
```

Eventually:

```bash
quri extension publish
```

Development flow:

```text
quri extension new postgres
             │
             ▼
         Rust project
             │
             ▼
       quri extension dev
             │
             ▼
         Build WASM
             │
             ▼
        Start Quri
             │
             ▼
        Hot reload
```

The source proposes essentially this CLI workflow.

---

# 18. Hot reload architecture

```text
Rust source
     │
     ▼
File watcher
     │
     ▼
cargo build
     │
     ▼
WASM component
     │
     ▼
Unload component
     │
     ▼
Load component
     │
     ▼
Re-register contributions
     │
     ▼
Refresh UI
```

Target:

```text
~0.5–2 seconds
```

---

# 19. Package format

Use:

```text
.qext
```

Internally:

```text
postgres.qext
│
├── manifest.toml
├── extension.wasm
├── icon.svg
├── README.md
├── checksums
└── signature
```

Signature/provenance can initially be optional for local development and become required for published ecosystem packages later.

---

# 20. Registry architecture

A marketplace is not available initially.

### V1

```text
GitHub
  │
  ▼
quri-registry.json
  │
  ▼
Quri Registry Client
```

### Later

```text
registry.quri.dev
       │
       ├── Search
       ├── Versions
       ├── Downloads
       ├── Compatibility
       ├── Signatures
       └── Metadata
```

This keeps the initial project focused on the platform rather than marketplace infrastructure.

---

# 21. Development roadmap

The huge roadmap is simplified into **10 engineering milestones**.

| Milestone | Goal                  | Output                            |
| --------- | --------------------- | --------------------------------- |
| M1        | Domain model          | Rust API types                    |
| M2        | PostgreSQL extraction | `DatabaseProvider` implementation |
| M3        | SDK                   | `quri-sdk`                        |
| M4        | WIT                   | Stable extension contract         |
| M5        | Extension Host        | Wasmtime runtime                  |
| M6        | WASM PostgreSQL       | `postgres.wasm`                   |
| M7        | CLI                   | `quri extension ...`              |
| M8        | Hot reload            | `quri extension dev`              |
| M9        | Packaging             | `.qext`                           |
| M10       | Registry              | Install/search extensions         |

---

# 22. Steps to build

The **first implementation sprint** should deliberately avoid:

```text
❌ Marketplace
❌ Registry service
❌ Community extensions
❌ Complex dependency resolver
❌ Full UI extension framework
❌ AI extension
❌ Signing infrastructure
```

Instead build:

```text
                    SPRINT 1

              quri-extension-api
                       │
          ┌────────────┼────────────┐
          ▼            ▼            ▼
      Database       Types       Commands
          │
          ▼
   DatabaseProvider
          │
          ▼
   PostgreSQL adapter
          │
          ▼
       Quri Core
```

The source itself recommends starting with the extension API and extracting PostgreSQL before introducing WASM.

---

# 23. The first concrete folder to design

I'd make this the immediate target:

```text
crates/quri-extension-api/
│
├── Cargo.toml
│
└── src/
    ├── lib.rs
    │
    ├── extension.rs
    ├── metadata.rs
    │
    ├── database/
    │   ├── mod.rs
    │   ├── provider.rs
    │   ├── connection.rs
    │   ├── query.rs
    │   ├── result.rs
    │   ├── value.rs
    │   └── metadata.rs
    │
    ├── commands.rs
    ├── events.rs
    ├── ui.rs
    ├── storage.rs
    └── errors.rs
```

And separately:

```text
wit/
├── extension.wit
├── types.wit
├── database.wit
├── commands.wit
├── events.wit
├── ui.wit
└── storage.wit
```

---

# 24. Definition of Done for the first milestone

I would not consider the architecture ready until this works:

```text
Existing Quri
      │
      ▼
DatabaseProvider
      │
      ▼
PostgreSQL implementation
      │
      ▼
Connect
      │
      ▼
Introspect
      │
      ▼
Show database tree
      │
      ▼
Execute SQL
      │
      ▼
Stream QueryResult
      │
      ▼
Render typed values
      │
      ▼
Cancel query
```

And the critical constraint:

> **Quri Core should have no `Postgres`, `MySQL`, `SQLite`, etc. branching.**

That is the architectural test that the abstraction is actually working.

## Overall design sequence

```text
                 ┌──────────────────┐
                 │  1. Domain Model │
                 └────────┬─────────┘
                          ▼
                 ┌───────────────────┐
                 │ 2. Stable Rust API│
                 └────────┬──────────┘
                          ▼
                 ┌──────────────────┐
                 │ 3. Extract PG    │
                 └────────┬─────────┘
                          ▼
                 ┌──────────────────┐
                 │ 4. quri-sdk      │
                 └────────┬─────────┘
                          ▼
                 ┌──────────────────┐
                 │ 5. WIT Contract  │
                 └────────┬─────────┘
                          ▼
                 ┌──────────────────┐
                 │ 6. Wasmtime Host │
                 └────────┬─────────┘
                          ▼
                 ┌──────────────────┐
                 │ 7. postgres.wasm │
                 └────────┬─────────┘
                          ▼
                 ┌──────────────────┐
                 │ 8. quri CLI      │
                 └────────┬─────────┘
                          ▼
                 ┌──────────────────┐
                 │ 9. .qext Package │
                 └────────┬─────────┘
                          ▼
                 ┌──────────────────┐
                 │ 10. Registry     │
                 └────────┬─────────┘
                          ▼
                 ┌────────────────────┐
                 │ Community Ecosystem│
                 └────────────────────┘
```
