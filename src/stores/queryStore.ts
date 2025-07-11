import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// 查询结果接口
export interface QueryResult {
  series: Series[]
  error?: string
  execution_time: number  // 修改为与后端匹配的字段名
}

// 系列数据接口
export interface Series {
  name: string
  columns: string[]
  values: any[][]
  tags?: Record<string, string>
}

// 查询历史接口
export interface QueryHistory {
  id: string
  query: string
  database: string
  timestamp: Date
  executionTime: number
  resultCount: number
  connectionId: string
}

// 查询状态管理存储
export const useQueryStore = defineStore('query', () => {
  // 状态
  const queryHistory = ref<QueryHistory[]>([])
  const currentQuery = ref('')
  const queryResults = ref<QueryResult | null>(null)
  const isExecuting = ref(false)
  const selectedDatabase = ref('')

  // 计算属性
  const recentQueries = computed(() => {
    return queryHistory.value.slice(0, 10) // 最近10条查询
  })

  const queryCount = computed(() => {
    return queryHistory.value.length
  })

  // 动作
  const executeQuery = async (query: string, database: string, connectionId: string): Promise<QueryResult | null> => {
    if (!query.trim() || !database || !connectionId) {
      throw new Error('查询、数据库和连接ID不能为空')
    }

    isExecuting.value = true
    const startTime = Date.now()

    try {
      // 调用 Tauri 命令执行查询
      const apiResponse = await invoke('execute_query', {
        query,
        database,
        connectionId // 使用 camelCase 参数名
      }) as any

      // 检查 API 响应
      if (!apiResponse.success) {
        throw new Error(apiResponse.error || '查询执行失败')
      }

      const result = apiResponse.data as QueryResult
      
      // 计算执行时间
      const executionTime = Date.now() - startTime

      // 保存到查询历史
      const historyItem: QueryHistory = {
        id: Date.now().toString(),
        query,
        database,
        timestamp: new Date(),
        executionTime,
        resultCount: result.series?.reduce((total, series) => total + series.values.length, 0) || 0,
        connectionId
      }

      saveToHistory(historyItem)

      // 更新当前查询和结果
      currentQuery.value = query
      queryResults.value = result
      selectedDatabase.value = database

      return result
    } catch (error) {
      const errorResult: QueryResult = {
        series: [],
        error: error instanceof Error ? error.message : '查询执行失败',
        execution_time: Date.now() - startTime
      }
      
      queryResults.value = errorResult
      throw error
    } finally {
      isExecuting.value = false
    }
  }

  const saveToHistory = (query: QueryHistory) => {
    // 避免重复的查询
    const existingIndex = queryHistory.value.findIndex(
      h => h.query === query.query && h.database === query.database
    )
    
    if (existingIndex >= 0) {
      // 更新现有记录的时间戳
      queryHistory.value[existingIndex] = query
    } else {
      // 添加到历史记录开头
      queryHistory.value.unshift(query)
    }

    // 限制历史记录数量
    if (queryHistory.value.length > 100) {
      queryHistory.value = queryHistory.value.slice(0, 100)
    }

    // 保存到本地存储
    saveQueryHistory()
  }

  const clearResults = () => {
    queryResults.value = null
  }

  const clearHistory = () => {
    queryHistory.value = []
    saveQueryHistory()
  }

  const removeFromHistory = (id: string) => {
    queryHistory.value = queryHistory.value.filter(h => h.id !== id)
    saveQueryHistory()
  }

  const setCurrentQuery = (query: string) => {
    currentQuery.value = query
  }

  const setSelectedDatabase = (database: string) => {
    selectedDatabase.value = database
  }

  // 本地存储相关
  const saveQueryHistory = () => {
    try {
      localStorage.setItem('influxdb-query-history', JSON.stringify(queryHistory.value))
    } catch (error) {
      console.error('保存查询历史失败:', error)
    }
  }

  const loadQueryHistory = () => {
    try {
      const saved = localStorage.getItem('influxdb-query-history')
      if (saved) {
        const parsed = JSON.parse(saved)
        // 转换时间戳字符串为 Date 对象
        queryHistory.value = parsed.map((item: any) => ({
          ...item,
          timestamp: new Date(item.timestamp)
        }))
      }
    } catch (error) {
      console.error('加载查询历史失败:', error)
    }
  }

  // 初始化时加载查询历史
  loadQueryHistory()

  return {
    // 状态
    queryHistory,
    currentQuery,
    queryResults,
    isExecuting,
    selectedDatabase,
    
    // 计算属性
    recentQueries,
    queryCount,
    
    // 动作
    executeQuery,
    saveToHistory,
    clearResults,
    clearHistory,
    removeFromHistory,
    setCurrentQuery,
    setSelectedDatabase,
    saveQueryHistory,
    loadQueryHistory
  }
}) 