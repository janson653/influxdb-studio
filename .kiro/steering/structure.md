# Project Structure

## Root Directory
```
influxdb-studio/
├── src/                    # Vue 3 frontend source code
├── src-tauri/             # Rust backend source code
├── docs/                  # Project documentation
├── scripts/               # Build and utility scripts
├── public/                # Static assets
└── dist/                  # Build output (generated)
```

## Frontend Structure (`src/`)
```
src/
├── components/            # Vue components organized by feature
│   ├── Common/           # Shared/reusable components
│   ├── Connection/       # Connection management UI
│   ├── Debug/           # Debug and development tools
│   ├── Layout/          # Layout components
│   └── Query/           # Query editor and results
├── stores/              # Pinia state management
├── views/               # Page-level Vue components
├── router/              # Vue Router configuration
├── types/               # TypeScript type definitions
├── utils/               # Utility functions and helpers
├── styles/              # Global CSS and themes
├── App.vue              # Root Vue component
└── main.ts              # Application entry point
```

## Backend Structure (`src-tauri/`)
```
src-tauri/
├── src/
│   ├── commands.rs      # Tauri command handlers
│   ├── models.rs        # Data models and types
│   ├── influxdb.rs      # InfluxDB client logic
│   ├── error.rs         # Error handling
│   └── main.rs          # Rust application entry
├── icons/               # Application icons for all platforms
├── Cargo.toml           # Rust dependencies and metadata
└── tauri.conf.json      # Tauri configuration
```

## Key Conventions

### File Naming
- Vue components: PascalCase (e.g., `ConnectionDialog.vue`)
- TypeScript files: camelCase (e.g., `connectionStore.ts`)
- Rust files: snake_case (e.g., `commands.rs`)
- CSS files: kebab-case (e.g., `dark-theme.css`)

### Component Organization
- Group components by feature/domain in `src/components/`
- Page components go in `src/views/`
- Shared utilities in `src/utils/`
- Type definitions in `src/types/`

### Store Pattern
- Use Pinia with Composition API syntax
- One store per domain (connection, query, etc.)
- Include computed properties and actions in stores
- Persist important state to localStorage

### Tauri Communication
- Frontend calls backend via `invoke()` from `@tauri-apps/api/core`
- All Tauri commands return structured responses with `success` and `data`/`error` fields
- Use `loggedInvoke` wrapper for consistent logging
- Backend commands use snake_case, frontend parameters use camelCase

### Error Handling
- Frontend: try/catch blocks with user-friendly error messages
- Backend: anyhow for error propagation, structured error responses
- Store connection states and error messages in Pinia stores