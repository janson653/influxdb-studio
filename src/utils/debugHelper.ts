/**
 * 调试辅助工具
 * 用于在生产环境中收集和显示调试信息
 */

// 为 import.meta.env 添加类型声明
declare global {
  interface ImportMeta {
    env: {
      DEV: boolean
      PROD: boolean
      MODE: string
      BASE_URL: string
      [key: string]: any
    }
  }
}

export interface DebugInfo {
  timestamp: string
  level: 'info' | 'warn' | 'error'
  message: string
  data?: any
}

class DebugHelper {
  private debugLogs: DebugInfo[] = []
  private maxLogs = 100

  /**
   * 添加调试日志
   */
  log(level: DebugInfo['level'], message: string, data?: any) {
    const logEntry: DebugInfo = {
      timestamp: new Date().toISOString(),
      level,
      message,
      data
    }

    this.debugLogs.push(logEntry)

    // 限制日志数量
    if (this.debugLogs.length > this.maxLogs) {
      this.debugLogs = this.debugLogs.slice(-this.maxLogs)
    }

    // 在开发环境中输出到控制台
    if (import.meta.env.DEV) {
      console.log(`[DEBUG] ${message}`, data)
    }
  }

  /**
   * 获取所有调试日志
   */
  getLogs(): DebugInfo[] {
    return [...this.debugLogs]
  }

  /**
   * 清空调试日志
   */
  clear() {
    this.debugLogs = []
  }

  /**
   * 导出调试信息
   */
  exportDebugInfo() {
    return {
      logs: this.debugLogs,
      userAgent: navigator.userAgent,
      timestamp: new Date().toISOString(),
      url: window.location.href
    }
  }

  /**
   * 检查连接状态
   */
  checkConnectionStatus(connectionStore: any) {
    const status = {
      hasActiveConnection: !!connectionStore.activeConnectionConfig,
      connectionStatus: connectionStore.connectionStatus,
      connections: connectionStore.connections
    }

    this.log('info', '连接状态检查', status)
    return status
  }

  /**
   * 检查数据库树状态
   */
  checkDatabaseTreeStatus(databaseTreeData: any[]) {
    const status = {
      treeDataLength: databaseTreeData.length,
      treeData: databaseTreeData.map(db => ({
        name: db.name,
        type: db.type,
        childrenCount: db.children?.length || 0
      }))
    }

    this.log('info', '数据库树状态检查', status)
    return status
  }
}

// 创建全局实例
export const debugHelper = new DebugHelper()

// 在开发环境中暴露到全局对象
if (import.meta.env.DEV) {
  ;(window as any).debugHelper = debugHelper
} 