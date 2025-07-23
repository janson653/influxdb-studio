import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { useConnectionStore } from './connectionStore';
import { useDatabaseStore } from './databaseStore';

interface Measurement {
  name: string;
  database: string;
  series_count?: number;
  field_keys?: FieldKey[];
  tag_keys?: string[];
}

interface FieldKey {
  name: string;
  field_type: string;
}

interface MeasurementState {
  measurements: Measurement[];
  selectedMeasurement: string | null;
  loading: boolean;
  error: string | null;
}

export const useMeasurementStore = defineStore('measurement', {
  state: (): MeasurementState => ({
    measurements: [],
    selectedMeasurement: null,
    loading: false,
    error: null,
  }),

  getters: {
    getMeasurements: (state) => state.measurements,
    getSelectedMeasurement: (state) => state.selectedMeasurement,
    isLoading: (state) => state.loading,
    getError: (state) => state.error,
  },

  actions: {
    async fetchMeasurements() {
      const connectionStore = useConnectionStore();
      const databaseStore = useDatabaseStore();
      
      const activeConnection = connectionStore.activeConnectionConfig;
      const database = databaseStore.getSelectedDatabase;

      if (!activeConnection || !connectionStore.connectionStatus[activeConnection.id]?.backendConnectionId) {
        this.error = 'No active connection';
        return;
      }

      if (!database) {
        this.error = 'No database selected';
        return;
      }

      return this.fetchMeasurementsForDatabase(database as string);
    },
    
    async fetchMeasurementsForDatabase(database: string) {
      const connectionStore = useConnectionStore();
      const activeConnection = connectionStore.activeConnectionConfig;
      const connectionId = activeConnection?.id ? 
        connectionStore.connectionStatus[activeConnection.id]?.backendConnectionId : null;

      if (!connectionId) {
        this.error = 'No active connection';
        return;
      }

      this.loading = true;
      this.error = null;

      try {
        const response = await invoke<{
          success: boolean;
          data?: string[];
          error?: string;
        }>('get_measurements', {
          connectionId,
          database,
        });

        if (response.success && response.data) {
          this.measurements = response.data.map(name => ({ 
            name, 
            database: database 
          }));
          return this.measurements;
        } else {
          this.error = response.error || 'Failed to fetch measurements';
          return [];
        }
      } catch (error) {
        console.error('Error fetching measurements:', error);
        this.error = error instanceof Error ? error.message : 'Unknown error';
        return [];
      } finally {
        this.loading = false;
      }
    },

    async fetchMeasurementInfo(measurementName: string) {
      const connectionStore = useConnectionStore();
      const databaseStore = useDatabaseStore();
      
      const activeConnection = connectionStore.activeConnectionConfig;
      const connectionId = activeConnection?.id ? 
        connectionStore.connectionStatus[activeConnection.id]?.backendConnectionId : null;
      const database = databaseStore.getSelectedDatabase;

      if (!connectionId) {
        this.error = 'No active connection';
        return null;
      }

      if (!database) {
        this.error = 'No database selected';
        return null;
      }

      this.loading = true;
      this.error = null;

      try {
        const response = await invoke<{
          success: boolean;
          data?: Measurement;
          error?: string;
        }>('get_measurement_info', {
          connectionId,
          database,
          measurement: measurementName,
        });

        if (response.success && response.data) {
          // Update the measurement in the list with the new info
          const index = this.measurements.findIndex(m => m.name === measurementName);
          if (index !== -1) {
            this.measurements[index] = {
              ...this.measurements[index],
              ...response.data,
            };
          }
          return response.data;
        } else {
          this.error = response.error || `Failed to fetch info for measurement ${measurementName}`;
          return null;
        }
      } catch (error) {
        console.error(`Error fetching measurement info for ${measurementName}:`, error);
        this.error = error instanceof Error ? error.message : 'Unknown error';
        return null;
      } finally {
        this.loading = false;
      }
    },

    async createMeasurement(
      measurementName: string, 
      fields: Array<{ name: string; type: string; value: any }>,
      tags?: Array<{ name: string; value: string }>
    ) {
      const connectionStore = useConnectionStore();
      const databaseStore = useDatabaseStore();
      
      const activeConnection = connectionStore.activeConnectionConfig;
      const connectionId = activeConnection?.id ? 
        connectionStore.connectionStatus[activeConnection.id]?.backendConnectionId : null;
      const database = databaseStore.getSelectedDatabase;

      if (!connectionId) {
        this.error = 'No active connection';
        return false;
      }

      if (!database) {
        this.error = 'No database selected';
        return false;
      }

      this.loading = true;
      this.error = null;

      try {
        // Convert fields and tags to the format expected by the backend
        const fieldsFormatted = fields.map(f => [f.name, f.type, f.value]);
        const tagsFormatted = tags?.map(t => [t.name, t.value]);

        const response = await invoke<{
          success: boolean;
          data?: boolean;
          error?: string;
        }>('create_measurement', {
          connectionId,
          database,
          measurement: measurementName,
          fields: fieldsFormatted,
          tags: tagsFormatted,
        });

        if (response.success && response.data) {
          // Add the new measurement to the list
          this.measurements.push({ 
            name: measurementName, 
            database: database as string 
          });
          return true;
        } else {
          this.error = response.error || `Failed to create measurement ${measurementName}`;
          return false;
        }
      } catch (error) {
        console.error(`Error creating measurement ${measurementName}:`, error);
        this.error = error instanceof Error ? error.message : 'Unknown error';
        return false;
      } finally {
        this.loading = false;
      }
    },

    async deleteMeasurement(measurementName: string) {
      const connectionStore = useConnectionStore();
      const databaseStore = useDatabaseStore();
      
      const activeConnection = connectionStore.activeConnectionConfig;
      const connectionId = activeConnection?.id ? 
        connectionStore.connectionStatus[activeConnection.id]?.backendConnectionId : null;
      const database = databaseStore.getSelectedDatabase;

      if (!connectionId) {
        this.error = 'No active connection';
        return false;
      }

      if (!database) {
        this.error = 'No database selected';
        return false;
      }

      this.loading = true;
      this.error = null;

      try {
        const response = await invoke<{
          success: boolean;
          data?: boolean;
          error?: string;
        }>('delete_measurement', {
          connectionId,
          database,
          measurement: measurementName,
        });

        if (response.success && response.data) {
          // Remove the measurement from the list
          this.measurements = this.measurements.filter(m => m.name !== measurementName);
          
          // If the deleted measurement was selected, clear the selection
          if (this.selectedMeasurement === measurementName) {
            this.selectedMeasurement = null;
          }
          
          return true;
        } else {
          this.error = response.error || `Failed to delete measurement ${measurementName}`;
          return false;
        }
      } catch (error) {
        console.error(`Error deleting measurement ${measurementName}:`, error);
        this.error = error instanceof Error ? error.message : 'Unknown error';
        return false;
      } finally {
        this.loading = false;
      }
    },

    setSelectedMeasurement(measurementName: string | null) {
      this.selectedMeasurement = measurementName;
    },

    clearError() {
      this.error = null;
    },
  },
});