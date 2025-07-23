# Design Document

## Overview

This design document outlines the implementation approach for enhancing InfluxDB Studio with database and measurement management capabilities, along with a UI redesign for a geek-style interface. The implementation will leverage the existing Tauri + Vue 3 architecture, with backend commands in Rust and frontend components in Vue 3 with TypeScript.

## Architecture

The feature will follow the existing application architecture:

1. **Frontend (Vue 3 + TypeScript)**
   - New Vue components for database and measurement management
   - Enhanced UI components with geek-style theming
   - Pinia store extensions for state management
   - Vue Router integration for navigation

2. **Backend (Rust + Tauri)**
   - New Tauri commands for database and measurement operations
   - Integration with InfluxDB client libraries
   - Error handling and response formatting

3. **Communication**
   - Tauri's invoke system for frontend-backend communication
   - Structured responses with success/error indicators

## Components and Interfaces

### Backend Components

1. **Database Management Module (`src-tauri/src/database.rs`)**
   - Functions for database operations:
     - `list_databases`: Retrieve all databases from InfluxDB
     - `create_database`: Create a new database
     - `delete_database`: Delete an existing database

2. **Measurement Management Module (`src-tauri/src/measurement.rs`)**
   - Functions for measurement operations:
     - `list_measurements`: Retrieve all measurements in a database
     - `create_measurement`: Create a new measurement
     - `delete_measurement`: Delete an existing measurement

3. **Command Registration (`src-tauri/src/commands.rs`)**
   - Register new Tauri commands for database and measurement operations
   - Handle error cases and format responses

### Frontend Components

1. **Database Explorer Enhancement (`src/views/DatabaseExplorer.vue`)**
   - Extended to include database management controls
   - Integration with measurement management

2. **Database Management Components**
   - `DatabaseList.vue`: Display and manage databases
   - `CreateDatabaseDialog.vue`: Form for creating new databases
   - `DeleteConfirmationDialog.vue`: Confirmation for deletion operations

3. **Measurement Management Components**
   - `MeasurementList.vue`: Display and manage measurements
   - `CreateMeasurementDialog.vue`: Form for creating new measurements

4. **UI Components for Geek-Style Interface**
   - `GeekTheme.css`: CSS variables and theme definitions
   - Enhanced versions of existing UI components with geek-style aesthetics

5. **Store Extensions**
   - `databaseStore.ts`: State management for databases
   - `measurementStore.ts`: State management for measurements

## Data Models

### Database Model
```typescript
interface Database {
  name: string;
  // Additional metadata if available from InfluxDB
  retention_policies?: RetentionPolicy[];
  created_at?: string;
}

interface RetentionPolicy {
  name: string;
  duration: string;
  shardGroupDuration: string;
  replicaN: number;
  default: boolean;
}
```

### Measurement Model
```typescript
interface Measurement {
  name: string;
  database: string;
  // Additional metadata if available
  series_count?: number;
  field_keys?: FieldKey[];
  tag_keys?: string[];
}

interface FieldKey {
  name: string;
  type: string;
}
```

### Command Request/Response Models

```typescript
// Database Operations
interface ListDatabasesResponse {
  success: boolean;
  data?: Database[];
  error?: string;
}

interface CreateDatabaseRequest {
  name: string;
  retention_policy?: {
    name: string;
    duration: string;
    replication: number;
    default: boolean;
  };
}

interface DeleteDatabaseRequest {
  name: string;
}

// Measurement Operations
interface ListMeasurementsRequest {
  database: string;
}

interface ListMeasurementsResponse {
  success: boolean;
  data?: Measurement[];
  error?: string;
}

interface CreateMeasurementRequest {
  database: string;
  name: string;
  fields: {
    name: string;
    type: string;
    value: any;
  }[];
  tags?: {
    name: string;
    value: string;
  }[];
}

interface DeleteMeasurementRequest {
  database: string;
  name: string;
}
```

## Error Handling

1. **Backend Error Handling**
   - Use Rust's `Result` type for error propagation
   - Convert errors to structured JSON responses
   - Include error codes and user-friendly messages

2. **Frontend Error Handling**
   - Try/catch blocks around Tauri command invocations
   - Display error notifications using Element Plus components
   - Provide retry mechanisms for failed operations

## UI Design

### DataGrip-Inspired Geek-Style Interface

The UI design will take inspiration from JetBrains DataGrip, which is known for its professional, developer-friendly interface that balances information density with usability.

1. **Color Scheme**
   - Dark background (#2B2B2B as primary, #3C3F41 for panels)
   - High contrast text (#A9B7C6 for primary text, #BBB529 for keywords)
   - Accent colors for important elements (#499C54 for success, #CC7832 for keywords, #6897BB for numbers)
   - Syntax highlighting similar to Darcula theme

2. **Typography**
   - JetBrains Mono or similar monospace fonts for data and code
   - Sans-serif fonts (like Segoe UI or SF Pro) for UI elements
   - Compact line height and spacing for data-dense areas
   - Clear hierarchy with font weights and sizes

3. **Layout**
   - Multi-panel interface with resizable splits
   - Tree view navigation panel on the left
   - Main content area with tabs for different views
   - Bottom panel for logs, messages, and query results
   - Status bar with connection information

4. **Interactive Elements**
   - Tool windows that can be pinned, unpinned, or hidden
   - Context menus for common operations
   - Double-click to edit items in place
   - Drag and drop for reorganizing elements
   - Keyboard shortcut indicators on buttons and in tooltips

5. **Data Visualization**
   - Syntax highlighting for query results and scripts
   - Tree views for database objects with expandable nodes
   - Tabular data with sortable columns
   - Visual query execution plan diagrams

### Component Mockups

```
+------------------------------------------------------+
| InfluxDB Studio                                [_][X]|
+------------------------------------------------------+
| File | Edit | View | Tools | Help                    |
+------------------------------------------------------+
| DB |                                                 |
+----+                                                 |
| ⊞ localhost:8086                                     |
|   ├─ 📁 Databases                                    |
|   │  ├─ 📊 my_database                               |
|   │  │  ├─ 📈 Measurements                           |
|   │  │  │  ├─ cpu_metrics                            |
|   │  │  │  ├─ memory_usage                           |
|   │  │  │  └─ network_stats                          |
|   │  │  └─ 🏷️ Tags                                   |
|   │  ├─ 📊 another_db                                |
|   │  └─ 📊 test_timeseries                           |
|   └─ 🔧 System                                       |
+----+-----------------+-------------------------------+
| Structure | Properties | Data | DDL | Dependencies   |
+----+-----------------+-------------------------------+
| Name         | Type   | Fields                       |
+----+-----------------+-------------------------------+
| cpu_metrics  | Meas.  | usage(float), cores(int)     |
| memory_usage | Meas.  | total(float), used(float)    |
| network_stats| Meas.  | rx(float), tx(float)         |
+----+-----------------+-------------------------------+
|                                                      |
| Console | Messages | Query Results                   |
+------------------------------------------------------+
| > SELECT * FROM cpu_metrics LIMIT 10                 |
| time                  | usage  | cores | host        |
| 2023-07-23T10:15:00Z | 45.2   | 8     | server01    |
| 2023-07-23T10:15:10Z | 47.8   | 8     | server01    |
+------------------------------------------------------+
| Connected to localhost:8086 | my_database | 3 items  |
+------------------------------------------------------+
```

### Key UI Features

1. **Database Navigator**
   - Hierarchical tree view of databases and their objects
   - Context menu for common operations (create, delete, refresh)
   - Icons to distinguish different object types
   - Filter/search functionality to quickly find objects

2. **Object Inspector**
   - Tabbed interface showing different aspects of selected objects
   - Structure view showing fields and tags
   - Properties panel for metadata
   - Data preview with pagination
   - DDL (Data Definition Language) view showing creation scripts

3. **Query Interface**
   - SQL editor with syntax highlighting and auto-completion
   - Query history with ability to rerun previous queries
   - Results displayed in tabular format with sorting
   - Export options for query results

4. **Toolbar and Action Buttons**
   - Quick access buttons for common operations
   - Visual indicators for connection status
   - Refresh button to update object lists
   - Create/delete buttons with appropriate icons

## Testing Strategy

1. **Unit Testing**
   - Test backend Rust functions for database and measurement operations
   - Test frontend store actions and mutations
   - Mock InfluxDB responses for testing

2. **Integration Testing**
   - Test frontend-backend communication
   - Verify proper handling of success and error cases

3. **UI Testing**
   - Test component rendering and interactions
   - Verify theme application and styling

4. **End-to-End Testing**
   - Test complete workflows with a test InfluxDB instance
   - Verify database and measurement operations work as expected

## Implementation Considerations

1. **InfluxDB Version Compatibility**
   - Ensure compatibility with InfluxDB v1.x
   - Design for future compatibility with v2.x and v3.x

2. **Performance**
   - Implement pagination for large database and measurement lists
   - Use efficient queries to minimize load on InfluxDB server

3. **Security**
   - Validate user inputs to prevent injection attacks
   - Confirm destructive operations (delete) with user

4. **Accessibility**
   - Ensure keyboard navigation works for all operations
   - Maintain sufficient color contrast despite dark theme
   - Provide text alternatives for visual indicators

5. **Internationalization**
   - Extract UI strings for potential future translation
   - Use locale-aware formatting for dates and numbers