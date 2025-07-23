// 测试连接配置
const testConnection = {
  id: 'test_conn_1',
  name: 'Test Connection',
  version: 'v1.x',
  config: {
    host: 'localhost',
    port: 8086,
    useSsl: false,
    timeout: 5000,
    database: 'testdb',
    username: undefined,
    password: undefined
  },
  created_at: Date.now(),
  updated_at: Date.now()
}

console.log('测试连接配置:')
console.log(JSON.stringify(testConnection, null, 2))

// 模拟后端解析
const config = testConnection.config
console.log('\n配置字段:')
console.log('host:', config.host)
console.log('port:', config.port)
console.log('useSsl:', config.useSsl)
console.log('timeout:', config.timeout)
console.log('database:', config.database)
console.log('username:', config.username)
console.log('password:', config.password)

// 检查必需字段
const requiredFields = ['host', 'port', 'database']
const missingFields = requiredFields.filter(field => !config[field])

if (missingFields.length > 0) {
  console.error('缺少必需字段:', missingFields)
} else {
  console.log('配置格式正确')
} 