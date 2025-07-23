# Product Overview

InfluxDB Studio is a cross-platform desktop client for InfluxDB built with Tauri + Vue 3. It provides a modern interface for connecting to and querying InfluxDB instances.

## Key Features
- Cross-platform support (Windows, macOS, Linux)
- InfluxDB v1.x support (v2.x and v3.x in development)
- Modern UI with Vue 3 + Element Plus
- Monaco Editor for query syntax highlighting
- Connection management with multiple profiles
- Query history and result visualization

## Target Users
Database administrators and developers working with InfluxDB time-series databases who need a desktop GUI client for database management and querying.

## Architecture
Hybrid desktop application using Tauri framework:
- Frontend: Vue 3 + TypeScript + Element Plus
- Backend: Rust with Tauri for native system integration
- Communication: Tauri's invoke system for frontend-backend calls