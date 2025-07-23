# Technology Stack

## Frontend Stack
- **Framework**: Vue 3 with Composition API
- **Language**: TypeScript with strict mode enabled
- **UI Library**: Element Plus for components
- **State Management**: Pinia stores
- **Data Fetching**: @tanstack/vue-query
- **Code Editor**: Monaco Editor with SQL syntax highlighting
- **Build Tool**: Vite with ES modules
- **Routing**: Vue Router 4

## Backend Stack
- **Framework**: Tauri 2.0 (Rust-based)
- **Language**: Rust 2021 edition
- **HTTP Client**: reqwest with JSON support
- **Async Runtime**: Tokio
- **Serialization**: serde with derive features
- **InfluxDB Clients**: influxdb (v1.x), influxdb2 (v2.x)
- **Logging**: tracing + tracing-subscriber
- **Error Handling**: anyhow
- **Date/Time**: chrono with serde support
- **UUID Generation**: uuid with v4 features
- **Tauri Plugins**: tauri-plugin-shell

## Development Tools
- **Package Manager**: pnpm (preferred) or npm
- **Linting**: ESLint with TypeScript and Vue plugins
- **Formatting**: Prettier
- **Type Checking**: vue-tsc

## Common Commands

### Development
```bash
# Install dependencies
pnpm install

# Start development server
pnpm tauri dev

# WSL development (if needed)
pnpm run tauri:dev:wsl
```

### Building
```bash
# Build frontend only
pnpm run build

# Build complete application
pnpm tauri build

# Type check
vue-tsc
```

### Version Management
```bash
# Check version consistency
pnpm run version:check

# Sync versions across files
pnpm run version:sync
```

## Configuration Notes
- Vite dev server runs on port 1420 (fixed for Tauri)
- Monaco Editor is pre-bundled for performance
- Tauri commands use camelCase for parameters
- All async operations use proper error handling with try/catch