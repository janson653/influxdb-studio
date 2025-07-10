import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

// 连接配置接口
export interface ConnectionConfig {
  id: string
  name: string
  host: string
  port: number
  database?: string
  username?: string
  password?: string
  useSsl: boolean
  timeout: number
}

// 连接状态接口
export interface ConnectionStatus {
  id: string
  status: 'connected' | 'disconnected' | 'connecting' | 'error'
  lastPing?: number
  error?: string
}

// 连接状态管理存储
export const useConnectionStore = defineStore('connection', () => {
  // 状态
  const connections = ref<ConnectionConfig[]>([])
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
  const addConnection = (config: ConnectionConfig) => {
    // 检查是否已存在相同 ID 的连接
    const existingIndex = connections.value.findIndex(c => c.id === config.id)
    if (existingIndex >= 0) {
      connections.value[existingIndex] = config
    } else {
      connections.value.push(config)
    }
    
    // 初始化连接状态
    connectionStatus.value[config.id] = {
      id: config.id,
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

  const updateConnection = (id: string, config: Partial<ConnectionConfig>) => {
    const index = connections.value.findIndex(c => c.id === id)
    if (index >= 0) {
      connections.value[index] = { ...connections.value[index], ...config }
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
      const result = await invoke('test_connection', { config: connection })
      
      // 更新连接状态
      connectionStatus.value[id] = {
        id,
        status: result ? 'connected' : 'error',
        lastPing: Date.now(),
        error: result ? undefined : '连接失败'
      }
      
      return result as boolean
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
    const success = await testConnection(id)
    if (success) {
      setActiveConnection(id)
    }
    return success
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
        connections.value = JSON.parse(saved)
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