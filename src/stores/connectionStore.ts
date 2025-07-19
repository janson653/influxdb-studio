import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { 
  ConnectionProfile, 
  ConnectionStatus
} from '../types/influxdb'
import { InfluxDBVersion } from '../types/influxdb'

// 连接状态管理存储
export const useConnectionStore = defineStore('connection', () => {
  // 状态
  const connections = ref<ConnectionProfile[]>([])
  const activeConnection = ref<string | null>(null)
  const connectionStatus = ref<Record<string, ConnectionStatus>>({})

  // 计算属性
  const activeConnectionConfig = computed(() => {
    if (!activeConnection.value) return null
    return connections.value.find(c => c.id === activeConnection.value)
  })

  const connectedConnections = computed(() => {
    return connections.value.filter(c => 
      connectionStatus.value[c.id]?.status === 'connected'
    )
  })

  // 动作
  const addConnection = (profile: ConnectionProfile) => {
    // 检查是否已存在相同 ID 的连接
    const existingIndex = connections.value.findIndex(c => c.id === profile.id)
    if (existingIndex >= 0) {
      connections.value[existingIndex] = {
        ...profile,
        updatedAt: Date.now()
      }
    } else {
      connections.value.push({
        ...profile,
        createdAt: Date.now(),
        updatedAt: Date.now()
      })
    }
    
    // 初始化连接状态
    connectionStatus.value[profile.id] = {
      id: profile.id,
      status: 'disconnected'
    }
    
    // 保存到本地存储
    saveConnections()
  }

  const removeConnection = (id: string) => {
    connections.value = connections.value.filter(c => c.id !== id)
    delete connectionStatus.value[id]
    
    // 如果删除的是当前活跃连接，清空活跃连接
    if (activeConnection.value === id) {
      activeConnection.value = null
    }
    
    // 保存到本地存储
    saveConnections()
  }

  const updateConnection = (id: string, updates: Partial<ConnectionProfile>) => {
    const index = connections.value.findIndex(c => c.id === id)
    if (index >= 0) {
      connections.value[index] = { 
        ...connections.value[index], 
        ...updates,
        updatedAt: Date.now()
      }
      saveConnections()
    }
  }

  const setActiveConnection = (id: string | null) => {
    activeConnection.value = id
  }

  const testConnection = async (id: string): Promise<boolean> => {
    const connection = connections.value.find(c => c.id === id)
    if (!connection) return false

    // 更新连接状态为连接中
    connectionStatus.value[id] = {
      id,
      status: 'connecting'
    }

    try {
      // 调用 Tauri 命令测试连接
      const apiResponse = await invoke('test_connection', { profile: connection }) as any
      
      // 检查 API 响应
      if (!apiResponse.success) {
        throw new Error(apiResponse.error || '连接测试失败')
      }
      
      const result = apiResponse.data as boolean
      
      // 更新连接状态
      connectionStatus.value[id] = {
        id,
        status: result ? 'connected' : 'error',
        lastPing: Date.now(),
        error: result ? undefined : '连接失败'
      }
      
      return result
    } catch (error) {
      // 更新连接状态为错误
      connectionStatus.value[id] = {
        id,
        status: 'error',
        error: error instanceof Error ? error.message : '连接失败'
      }
      return false
    }
  }

  const connectTo = async (id: string): Promise<boolean> => {
    const connection = connections.value.find(c => c.id === id)
    if (!connection) return false

    // 更新连接状态为连接中
    connectionStatus.value[id] = {
      id,
      status: 'connecting'
    }

    try {
      // 调用 Tauri 命令建立连接
      const apiResponse = await invoke('connect_to_database', { profile: connection }) as any
      
      // 检查 API 响应
      if (!apiResponse.success) {
        throw new Error(apiResponse.error || '连接失败')
      }
      
      const backendConnectionId = apiResponse.data as string
      
      // 更新连接状态，保存后端返回的连接ID
      connectionStatus.value[id] = {
        id,
        status: 'connected',
        lastPing: Date.now(),
        backendConnectionId // 保存后端连接ID
      }
      
      setActiveConnection(id)
      return true
    } catch (error) {
      // 更新连接状态为错误
      connectionStatus.value[id] = {
        id,
        status: 'error',
        error: error instanceof Error ? error.message : '连接失败'
      }
      return false
    }
  }

  const disconnectFrom = (id: string) => {
    connectionStatus.value[id] = {
      id,
      status: 'disconnected'
    }
    
    if (activeConnection.value === id) {
      setActiveConnection(null)
    }
  }

  // 本地存储相关
  const saveConnections = () => {
    try {
      localStorage.setItem('influxdb-connections', JSON.stringify(connections.value))
    } catch (error) {
      console.error('保存连接配置失败:', error)
    }
  }

  const loadConnections = () => {
    try {
      const saved = localStorage.getItem('influxdb-connections')
      if (saved) {
        const parsed = JSON.parse(saved)
        // 兼容旧版本数据格式
        if (Array.isArray(parsed)) {
          connections.value = parsed.map(conn => {
            // 如果是旧格式，转换为新格式
            if (!conn.version) {
              // 判断版本并转换
              if (conn.token && conn.org) {
                return {
                  id: conn.id,
                  name: conn.name,
                  version: InfluxDBVersion.V2,
                  config: {
                    host: conn.host,
                    port: conn.port,
                    token: conn.token,
                    org: conn.org,
                    bucket: conn.bucket,
                    useSsl: conn.useSsl || false,
                    timeout: conn.timeout || 5000
                  },
                  createdAt: conn.createdAt || Date.now(),
                  updatedAt: conn.updatedAt || Date.now()
                }
              } else if (conn.database && (conn.username || conn.password)) {
                return {
                  id: conn.id,
                  name: conn.name,
                  version: InfluxDBVersion.V1,
                  config: {
                    host: conn.host,
                    port: conn.port,
                    database: conn.database,
                    username: conn.username,
                    password: conn.password,
                    useSsl: conn.useSsl || false,
                    timeout: conn.timeout || 5000
                  },
                  createdAt: conn.createdAt || Date.now(),
                  updatedAt: conn.updatedAt || Date.now()
                }
              }
            }
            return conn
          })
        }
        
        // 初始化所有连接的状态
        connections.value.forEach(conn => {
          connectionStatus.value[conn.id] = {
            id: conn.id,
            status: 'disconnected'
          }
        })
      }
    } catch (error) {
      console.error('加载连接配置失败:', error)
    }
  }

  // 初始化时加载连接配置
  loadConnections()

  return {
    // 状态
    connections,
    activeConnection,
    connectionStatus,
    
    // 计算属性
    activeConnectionConfig,
    connectedConnections,
    
    // 动作
    addConnection,
    removeConnection,
    updateConnection,
    setActiveConnection,
    testConnection,
    connectTo,
    disconnectFrom,
    saveConnections,
    loadConnections
  }
}) 