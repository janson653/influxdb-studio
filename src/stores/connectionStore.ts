import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { 
  ConnectionProfile, 
  ConnectionStatus
} from '../types/influxdb'
import { InfluxDBVersion } from '../types/influxdb'

/**
 * A logging wrapper around the Tauri invoke function.
 * @param command The name of the Rust command to invoke.
 * @param args The arguments to pass to the command.
 * @returns The result of the API call.
 */
async function loggedInvoke(command: string, args?: any): Promise<any> {
  console.log(`[FE] Invoking command '${command}' with args:`, args)
  try {
    const response = await invoke(command, args)
    console.log(`[FE] Command '${command}' succeeded with response:`, response)
    
    const apiResponse = response as any
    if (apiResponse && apiResponse.success === false) {
      console.error(`[FE] Command '${command}' returned a business error:`, apiResponse.error)
      // Still resolve with the business error response to be handled by the caller
      return apiResponse
    }
    
    return response
  } catch (error) {
    console.error(`[FE] Command '${command}' failed with error:`, error)
    // Rethrow the error to be caught by the caller's catch block
    throw error
  }
}


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
    console.log(`[Store] Adding or updating connection: ${profile.name} (${profile.id})`)
    // 检查是否已存在相同 ID 的连接
    const existingIndex = connections.value.findIndex(c => c.id === profile.id)
    if (existingIndex >= 0) {
      connections.value[existingIndex] = {
        ...profile,
        updated_at: Date.now()
      }
    } else {
      connections.value.push({
        ...profile,
        created_at: Date.now(),
        updated_at: Date.now()
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
    console.log(`[Store] Removing connection: ${id}`)
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
        updated_at: Date.now()
      }
      saveConnections()
    }
  }

  const setActiveConnection = (id: string | null) => {
    console.log(`[Store] Setting active connection to: ${id}`)
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
      const apiResponse = await loggedInvoke('test_connection', { profile: connection })
      
      // 检查 API 响应
      if (!apiResponse.success) {
        // 后端返回了业务错误
        const errorMessage = apiResponse.error || '连接测试失败，未提供额外信息。'
        connectionStatus.value[id] = {
          id,
          status: 'error',
          error: errorMessage
        }
        return false
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
      // 调用本身失败 (e.g., network error, Rust panic)
      connectionStatus.value[id] = {
        id,
        status: 'error',
        error: error instanceof Error ? error.message : String(error)
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
      const apiResponse = await loggedInvoke('connect_to_database', { profile: connection })
      
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
      // 改进错误处理
      let errorMessage = '连接失败'
      
      if (error instanceof Error) {
        if (error.message.includes('Connection refused')) {
          errorMessage = '无法连接到 InfluxDB 服务器，请检查：\n1. InfluxDB 服务是否已启动\n2. 主机地址和端口是否正确\n3. 防火墙设置是否允许连接'
        } else if (error.message.includes('timeout')) {
          errorMessage = '连接超时，请检查网络连接和服务器状态'
        } else if (error.message.includes('Network')) {
          errorMessage = '网络连接失败，请检查网络设置'
        } else {
          errorMessage = error.message
        }
      }
      
      // 更新连接状态为错误
      connectionStatus.value[id] = {
        id,
        status: 'error',
        error: errorMessage
      }
      return false
    }
  }

  const disconnectFrom = (id: string) => {
    console.log(`[Store] Disconnecting from: ${id}`)
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
        const parsed = JSON.parse(saved) as any[]
        
        if (Array.isArray(parsed)) {
          connections.value = parsed.map(conn => {
            const now = Date.now()
            // 创建一个保证结构完整的新对象，而不是修补旧对象
            const newConn = {
              ...conn,
              created_at: conn.created_at || conn.createdAt || now,
              updated_at: conn.updated_at || conn.updatedAt || now,
            }

            // 兼容旧版本数据格式
            if (!newConn.version) {
              // 判断版本并转换
              if (newConn.token && newConn.org) {
                return {
                  id: newConn.id,
                  name: newConn.name,
                  version: InfluxDBVersion.V2,
                  config: {
                    host: newConn.host,
                    port: newConn.port,
                    token: newConn.token,
                    org: newConn.org,
                    bucket: newConn.bucket,
                    useSsl: newConn.useSsl || false,
                    timeout: newConn.timeout || 5000
                  },
                  createdAt: newConn.createdAt,
                  updatedAt: newConn.updatedAt
                }
              } else if (newConn.database) { // 稍微放宽 v1 的判断条件
                return {
                  id: newConn.id,
                  name: newConn.name,
                  version: InfluxDBVersion.V1,
                  config: {
                    host: newConn.host,
                    port: newConn.port,
                    database: newConn.database,
                    username: newConn.username,
                    password: newConn.password,
                    useSsl: newConn.useSsl || false,
                    timeout: newConn.timeout || 5000
                  },
                  createdAt: newConn.createdAt,
                  updatedAt: newConn.updatedAt
                }
              }
            }
            return newConn
          })
        }
        
        // 初始化所有连接的状态
        connections.value.forEach(conn => {
          if (conn) { // 确保 conn 不为 null 或 undefined
            connectionStatus.value[conn.id] = {
              id: conn.id,
              status: 'disconnected'
            }
          }
        })
      }
    } catch (error) {
      console.error('加载连接配置失败:', error)
    }
  }

  // 初始化时加载连接配置
  console.log('[Store] Initializing connection store and loading connections.')
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