# What should go into quri-core:

### AppState

- active workspace
- active/open connections
- running queries
- service registries
- command and event registries

### Domain models

- workspace
- saved connection
- query
- query result
- database metadata
- typed database values
- IDs and capabilities

### Application services

- workspace management
- connection management
- query execution orchestration
- settings management
- extension lifecycle management
- Commands and events
- QueryStarted, QueryCompleted, ConnectionOpened, etc.
- command registration and dispatch

### Traits/interfaces for infrastructure

- AppStorage
- SecretStore
- DatabaseProvider
- ExtensionHost

### Core errors and logging/notification events.
