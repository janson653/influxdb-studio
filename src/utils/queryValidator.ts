import { InfluxDBVersion } from '../types/influxdb'

export interface QueryValidationResult {
  isValid: boolean
  error?: string
  suggestion?: string
  correctedQuery?: string
}

export class QueryValidator {
  /**
   * 验证 InfluxDB 查询语法
   */
  static validateQuery(query: string, version: InfluxDBVersion): QueryValidationResult {
    const trimmedQuery = query.trim()
    
    // 检查是否为空
    if (!trimmedQuery) {
      return {
        isValid: false,
        error: '查询语句不能为空'
      }
    }
    
    // 目前只支持 v1.x 版本
    if (version === InfluxDBVersion.V1) {
      return this.validateInfluxQL(trimmedQuery)
    } else {
      return {
        isValid: false,
        error: '目前只支持 InfluxDB v1.x，v2.x 和 v3.x 支持正在开发中'
      }
    }
  }
  
  /**
   * 验证 InfluxQL 语法
   */
  private static validateInfluxQL(query: string): QueryValidationResult {
    const upperQuery = query.toUpperCase()
    
    // 检查 INSERT 语句
    if (upperQuery.startsWith('INSERT')) {
      return this.validateInfluxQLInsert(query)
    }
    
    // 检查 SELECT 语句
    if (upperQuery.startsWith('SELECT')) {
      return this.validateInfluxQLSelect(query)
    }
    
    // 检查 SHOW 语句
    if (upperQuery.startsWith('SHOW')) {
      return this.validateInfluxQLShow(query)
    }
    
    // 检查 CREATE 语句
    if (upperQuery.startsWith('CREATE')) {
      return this.validateInfluxQLCreate(query)
    }
    
    // 检查 DELETE 语句
    if (upperQuery.startsWith('DELETE')) {
      return this.validateInfluxQLDelete(query)
    }
    
    return {
      isValid: true
    }
  }
  
  /**
   * 验证 InfluxQL INSERT 语句
   */
  private static validateInfluxQLInsert(query: string): QueryValidationResult {
    const upperQuery = query.toUpperCase()
    
    // 检查基本 INSERT 语法
    if (!upperQuery.includes('INSERT')) {
      return {
        isValid: false,
        error: 'INSERT 语句必须以 INSERT 开头',
        suggestion: '请检查 INSERT 关键字是否正确'
      }
    }
    
    // 注释掉 INTO 关键字检查，允许更灵活的 INSERT 语法
    // if (!upperQuery.includes('INTO')) {
    //   // 尝试自动修复 - 添加 INTO 和默认数据库
    //   const correctedQuery = query.replace(/^INSERT\s+/i, 'INSERT INTO "your_database" ')
    //   return {
    //     isValid: false,
    //     error: 'INSERT 语句缺少 INTO 关键字和数据库名称',
    //     suggestion: '请使用格式: INSERT INTO "database_name" measurement,tag_key=tag_value field_key="field_value"',
    //     correctedQuery
    //   }
    // }
    
    // 检查数据库名称格式 - 支持带INTO和不带INTO的语法
    let dbMatch = query.match(/INSERT\s+INTO\s+"([^"]+)"\s+(.+)/i)
    if (!dbMatch) {
      // 尝试匹配没有引号的数据库名（带INTO）
      dbMatch = query.match(/INSERT\s+INTO\s+(\S+)\s+(.+)/i)
    }
    if (!dbMatch) {
      // 尝试匹配不带INTO的语法：INSERT "database" measurement...
      dbMatch = query.match(/INSERT\s+"([^"]+)"\s+(.+)/i)
    }
    if (!dbMatch) {
      // 尝试匹配不带INTO和引号的语法：INSERT database measurement...
      dbMatch = query.match(/INSERT\s+(\S+)\s+(.+)/i)
    }
    if (!dbMatch) {
      return {
        isValid: false,
        error: 'INSERT 语句格式错误',
        suggestion: '请使用格式: INSERT [INTO] [database_name] measurement,tag_key=tag_value field_key="field_value"'
      }
    }
    
    // 检查是否有测量值名称和字段 - 支持多种INSERT语法
    let dataMatch = query.match(/INSERT\s+INTO\s+[^,\s]+,\s*([^,\s]+)/i)
    if (!dataMatch) {
      // 尝试匹配不带逗号的格式（带INTO）
      dataMatch = query.match(/INSERT\s+INTO\s+[^,\s]+\s+([^,\s]+)/i)
    }
    if (!dataMatch) {
      // 尝试匹配不带INTO的格式：INSERT "database" measurement,...
      dataMatch = query.match(/INSERT\s+[^,\s]+,\s*([^,\s]+)/i)
    }
    if (!dataMatch) {
      // 尝试匹配不带INTO和逗号的格式：INSERT "database" measurement ...
      dataMatch = query.match(/INSERT\s+[^,\s]+\s+([^,\s]+)/i)
    }
    if (!dataMatch) {
      return {
        isValid: false,
        error: 'INSERT 语句缺少测量值名称',
        suggestion: '请指定测量值名称，格式: INSERT [INTO] [database] measurement_name,tag_key=tag_value field_key="field_value"'
      }
    }
    
    // 检查是否有字段值
    if (!query.includes('=')) {
      return {
        isValid: false,
        error: 'INSERT 语句缺少字段值',
        suggestion: '请添加字段值，格式: measurement,tag_key=tag_value field_key="field_value"'
      }
    }
    
    // 检查是否有空格分隔标签和字段 - 支持更灵活的格式
    const parts = query.split(/\s+/)
    if (parts.length < 3) {
      return {
        isValid: false,
        error: 'INSERT 语句格式不完整',
        suggestion: '请确保包含测量值、标签和字段，格式: INSERT [INTO] [database] measurement,tag_key=tag_value field_key="field_value"'
      }
    }
    
    return {
      isValid: true
    }
  }
  
  /**
   * 验证 InfluxQL SELECT 语句
   */
  private static validateInfluxQLSelect(query: string): QueryValidationResult {
    const upperQuery = query.toUpperCase()
    
    if (!upperQuery.includes('FROM')) {
      return {
        isValid: false,
        error: 'SELECT 语句缺少 FROM 子句',
        suggestion: '请添加 FROM 子句，格式: SELECT * FROM measurement'
      }
    }
    
    return {
      isValid: true
    }
  }
  
  /**
   * 验证 InfluxQL SHOW 语句
   */
  private static validateInfluxQLShow(query: string): QueryValidationResult {
    const upperQuery = query.toUpperCase()
    
    if (!upperQuery.includes('SHOW DATABASES') && 
        !upperQuery.includes('SHOW MEASUREMENTS') &&
        !upperQuery.includes('SHOW RETENTION POLICIES') &&
        !upperQuery.includes('SHOW TAG KEYS') &&
        !upperQuery.includes('SHOW FIELD KEYS')) {
      return {
        isValid: false,
        error: '不支持的 SHOW 语句',
        suggestion: '支持的 SHOW 语句: SHOW DATABASES, SHOW MEASUREMENTS, SHOW RETENTION POLICIES, SHOW TAG KEYS, SHOW FIELD KEYS'
      }
    }
    
    return {
      isValid: true
    }
  }
  
  /**
   * 验证 InfluxQL CREATE 语句
   */
  private static validateInfluxQLCreate(query: string): QueryValidationResult {
    const upperQuery = query.toUpperCase()
    
    if (!upperQuery.includes('CREATE DATABASE')) {
      return {
        isValid: false,
        error: '不支持的 CREATE 语句',
        suggestion: '目前只支持 CREATE DATABASE 语句'
      }
    }
    
    return {
      isValid: true
    }
  }
  
  /**
   * 验证 InfluxQL DELETE 语句
   */
  private static validateInfluxQLDelete(query: string): QueryValidationResult {
    const upperQuery = query.toUpperCase()
    
    if (!upperQuery.includes('FROM')) {
      return {
        isValid: false,
        error: 'DELETE 语句缺少 FROM 子句',
        suggestion: '请添加 FROM 子句，格式: DELETE FROM measurement'
      }
    }
    
    return {
      isValid: true
    }
  }
  
  /**
   * 验证 Flux 语法
   */
  private static validateFlux(query: string): QueryValidationResult {
    // Flux 语法比较复杂，这里只做基本检查
    if (query.includes('from(') || query.includes('import ')) {
      return {
        isValid: true
      }
    }
    
    return {
      isValid: false,
      error: 'Flux 查询语法不正确',
      suggestion: 'Flux 查询应该以 from() 或 import 开头，例如: from(bucket: "my-bucket") |> range(start: -1h)'
    }
  }
  
  /**
   * 获取查询示例
   */
  static getQueryExamples(version: InfluxDBVersion): string[] {
    if (version === InfluxDBVersion.V1) {
      return [
        'SELECT * FROM "measurement" LIMIT 10',
        'INSERT INTO "testdb" cpu,host=server01 value=0.64',
        'INSERT INTO "testdb" memory,host=server01,region=us-west value=0.32',
        'SHOW DATABASES',
        'SHOW MEASUREMENTS',
        'CREATE DATABASE "new_database"'
      ]
    } else {
      return [
        '目前只支持 InfluxDB v1.x 查询示例',
        'v2.x 和 v3.x 支持正在开发中'
      ]
    }
  }
} 