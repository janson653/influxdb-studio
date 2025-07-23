import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { useConnectionStore } from './connectionStore';

interface Database {
  name: string;
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

interface DatabaseState {
  databases: Database[];
  selectedDatabase: string | null;
  loading: boolean;
  error: string | null;
}

export const useDatabaseStore = defineStore('database', {
  state: (): DatabaseState => ({
    databases: [],
    selectedDatabase: null,
    loading: false,
    error: null,
  }),

  getters: {
    getDatabases: (state) => state.databases,
    getSelectedDatabase: (state) => state.selectedDatabase,
    isLoading: (state) => state.loading,
    getError: (state) => state.error,
    isConnected: () => {
      const connectionStore = useConnectionStore();
      return connectionStore.isConnected;
    },
  },

  actions: {
    async fetchDatabases() {
      const connectionStore = useConnectionStore();
      const activeConnection = connectionStore.activeConnectionConfig;
      
      if (!activeConnection?.id) {
        this.error = 'No active connection';
        return;
      }
      
      const connectionStatus = connectionStore.connectionStatus[activeConnection.id];
      const connectionId = connectionStatus?.backendConnectionId;

      if (!connectionId) {
        this.error = 'No backend connection ID';
        console.error('Missing backend connection ID for connection:', activeConnection.id);
        return;
      }

      this.loading = true;
      this.error = null;

      try {
        const response = await invoke<{
          success: boolean;
          data?: string[];
          error?: string;
        }>('get_databases', {
          connectionId,
        });

        if (response.success && response.data) {
          this.databases = response.data.map(name => ({ name }));
        } else {
          this.error = response.error || 'Failed to fetch databases';
        }
      } catch (error) {
        console.error('Error fetching databases:', error);
        this.error = error instanceof Error ? error.message : 'Unknown error';
      } finally {
        this.loading = false;
      }
    },

    async fetchDatabaseInfo(databaseName: string) {
      const connectionStore = useConnectionStore();
      const activeConnection = connectionStore.activeConnectionConfig;
      
      if (!activeConnection?.id) {
        this.error = 'No active connection';
        return null;
      }
      
      const connectionStatus = connectionStore.connectionStatus[activeConnection.id];
      const connectionId = connectionStatus?.backendConnectionId;

      if (!connectionId) {
        this.error = 'No backend connection ID';
        console.error('Missing backend connection ID for connection:', activeConnection.id);
        return null;
      }

      this.loading = true;
      this.error = null;

      try {
        const response = await invoke<{
          success: boolean;
          data?: DatabaseInfo;
          error?: string;
        }>('get_database_info', {
          connectionId,
          database: databaseName,
        });

        if (response.success && response.data) {
          // Update the database in the list with the new info
          const index = this.databases.findIndex(db => db.name === databaseName);
          if (index !== -1) {
            this.databases[index] = {
              ...this.databases[index],
              ...response.data,
            };
          }
          return response.data;
        } else {
          this.error = response.error || `Failed to fetch info for database ${databaseName}`;
          return null;
        }
      } catch (error) {
        console.error(`Error fetching database info for ${databaseName}:`, error);
        this.error = error instanceof Error ? error.message : 'Unknown error';
        return null;
      } finally {
        this.loading = false;
      }
    },

    async createDatabase(databaseName: string, retentionPolicy?: RetentionPolicy) {
      const connectionStore = useConnectionStore();
      const activeConnection = connectionStore.activeConnectionConfig;
      
      if (!activeConnection?.id) {
        this.error = 'No active connection';
        return false;
      }
      
      const connectionStatus = connectionStore.connectionStatus[activeConnection.id];
      const connectionId = connectionStatus?.backendConnectionId;

      if (!connectionId) {
        this.error = 'No backend connection ID';
        console.error('Missing backend connection ID for connection:', activeConnection.id);
        return false;
      }

      this.loading = true;
      this.error = null;

      try {
        const response = await invoke<{
          success: boolean;
          data?: boolean;
          error?: string;
        }>('create_database', {
          connectionId,
          database: databaseName,
          retentionPolicy,
        });

        if (response.success && response.data) {
          // Add the new database to the list
          this.databases.push({ name: databaseName });
          return true;
        } else {
          this.error = response.error || `Failed to create database ${databaseName}`;
          return false;
        }
      } catch (error) {
        console.error(`Error creating database ${databaseName}:`, error);
        this.error = error instanceof Error ? error.message : 'Unknown error';
        return false;
      } finally {
        this.loading = false;
      }
    },

    async deleteDatabase(databaseName: string) {
      const connectionStore = useConnectionStore();
      const activeConnection = connectionStore.activeConnectionConfig;
      
      if (!activeConnection?.id) {
        this.error = 'No active connection';
        return false;
      }
      
      const connectionStatus = connectionStore.connectionStatus[activeConnection.id];
      const connectionId = connectionStatus?.backendConnectionId;

      if (!connectionId) {
        this.error = 'No backend connection ID';
        console.error('Missing backend connection ID for connection:', activeConnection.id);
        return false;
      }

      this.loading = true;
      this.error = null;

      try {
        const response = await invoke<{
          success: boolean;
          data?: boolean;
          error?: string;
        }>('drop_database', {
          connectionId,
          database: databaseName,
        });

        if (response.success && response.data) {
          // Remove the database from the list
          this.databases = this.databases.filter(db => db.name !== databaseName);
          
          // If the deleted database was selected, clear the selection
          if (this.selectedDatabase === databaseName) {
            this.selectedDatabase = null;
          }
          
          return true;
        } else {
          this.error = response.error || `Failed to delete database ${databaseName}`;
          return false;
        }
      } catch (error) {
        console.error(`Error deleting database ${databaseName}:`, error);
        this.error = error instanceof Error ? error.message : 'Unknown error';
        return false;
      } finally {
        this.loading = false;
      }
    },

    setSelectedDatabase(databaseName: string | null) {
      this.selectedDatabase = databaseName;
    },

    clearError() {
      this.error = null;
    },
  },
});

// Types for API responses
interface DatabaseInfo {
  name: string;
  retention_policies?: RetentionPolicy[];
  created_at?: string;
}