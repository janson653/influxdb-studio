// InfluxDB 版本枚举
export enum InfluxDBVersion {
  V1 = 'v1.x',
  V2 = 'v2.x',
  V3 = 'v3.x'
}

// InfluxDB v1.x 配置接口
export interface InfluxDBV1Config {
  host: string
  port: number
  database: string
  username?: string
  password?: string
  useSsl: boolean
  timeout: number
}

// InfluxDB v2.x 配置接口
export interface InfluxDBV2Config {
  host: string
  port: number
  token: string
  org: string
  bucket?: string
  useSsl: boolean
  timeout: number
}

// 统一的连接配置接口
export interface ConnectionProfile {
  id: string
  name: string
  version: InfluxDBVersion
  config: InfluxDBV1Config | InfluxDBV2Config
  createdAt: number
  updatedAt: number
}

// 连接状态接口
export interface ConnectionStatus {
  id: string
  status: 'connected' | 'disconnected' | 'connecting' | 'error'
  lastPing?: number
  error?: string
  backendConnectionId?: string
}

// 类型守卫函数
export function isV1Config(config: any): config is InfluxDBV1Config {
  return config && typeof config.database === 'string' && 
         (config.username !== undefined || config.password !== undefined)
}

export function isV2Config(config: any): config is InfluxDBV2Config {
  return config && typeof config.token === 'string' && 
         typeof config.org === 'string'
}

// 获取配置的 URL
export function getConnectionUrl(config: InfluxDBV1Config | InfluxDBV2Config): string {
  const protocol = config.useSsl ? 'https' : 'http'
  return `${protocol}://${config.host}:${config.port}`
} 