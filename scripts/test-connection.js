#!/usr/bin/env node

/**
 * InfluxDB 连接测试脚本
 * 用于验证 InfluxDB 服务是否正常运行
 */

const http = require('http');
const { URL } = require('url');

// 配置
const config = {
  host: 'localhost',
  port: 8086,
  database: 'testdb',
  timeout: 5000
};

// 颜色输出
const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  magenta: '\x1b[35m',
  cyan: '\x1b[36m'
};

function log(message, color = 'reset') {
  console.log(`${colors[color]}${message}${colors.reset}`);
}

function logSuccess(message) {
  log(`✅ ${message}`, 'green');
}

function logError(message) {
  log(`❌ ${message}`, 'red');
}

function logWarning(message) {
  log(`⚠️  ${message}`, 'yellow');
}

function logInfo(message) {
  log(`ℹ️  ${message}`, 'blue');
}

// HTTP 请求工具
function makeRequest(path, method = 'GET', data = null) {
  return new Promise((resolve, reject) => {
    const url = new URL(`http://${config.host}:${config.port}${path}`);
    
    const options = {
      hostname: url.hostname,
      port: url.port,
      path: url.pathname + url.search,
      method: method,
      timeout: config.timeout,
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded'
      }
    };

    if (data) {
      options.headers['Content-Length'] = Buffer.byteLength(data);
    }

    const req = http.request(options, (res) => {
      let responseData = '';
      
      res.on('data', (chunk) => {
        responseData += chunk;
      });
      
      res.on('end', () => {
        resolve({
          statusCode: res.statusCode,
          headers: res.headers,
          data: responseData
        });
      });
    });

    req.on('error', (error) => {
      reject(error);
    });

    req.on('timeout', () => {
      req.destroy();
      reject(new Error('Request timeout'));
    });

    if (data) {
      req.write(data);
    }
    
    req.end();
  });
}

// 测试函数
async function testPing() {
  logInfo('测试 InfluxDB 服务连通性...');
  
  try {
    const response = await makeRequest('/ping');
    if (response.statusCode === 204) {
      logSuccess('InfluxDB 服务响应正常');
      return true;
    } else {
      logError(`服务响应异常: ${response.statusCode}`);
      return false;
    }
  } catch (error) {
    logError(`连接失败: ${error.message}`);
    return false;
  }
}

async function testQuery(query) {
  logInfo(`执行查询: ${query}`);
  
  try {
    const data = `q=${encodeURIComponent(query)}&db=${encodeURIComponent(config.database)}`;
    const response = await makeRequest('/query', 'POST', data);
    
    if (response.statusCode === 200) {
      const result = JSON.parse(response.data);
      if (result.results && result.results[0]) {
        const queryResult = result.results[0];
        if (queryResult.error) {
          logError(`查询错误: ${queryResult.error}`);
          return false;
        } else {
          logSuccess('查询执行成功');
          if (queryResult.series) {
            logInfo(`返回 ${queryResult.series.length} 个系列`);
            queryResult.series.forEach((series, index) => {
              logInfo(`系列 ${index + 1}: ${series.name || 'unnamed'}`);
              if (series.values) {
                logInfo(`  数据点: ${series.values.length}`);
              }
            });
          }
          return true;
        }
      }
    } else {
      logError(`HTTP 错误: ${response.statusCode}`);
      return false;
    }
  } catch (error) {
    logError(`查询失败: ${error.message}`);
    return false;
  }
}

async function testWrite() {
  logInfo('测试数据写入...');
  
  const testData = `cpu_usage,host=test_server,region=test value=0.5 ${Date.now() * 1000000}`;
  const data = `db=${encodeURIComponent(config.database)}&precision=ns`;
  
  try {
    const response = await makeRequest('/write', 'POST', data + '&' + testData);
    
    if (response.statusCode === 204) {
      logSuccess('数据写入成功');
      return true;
    } else {
      logError(`写入失败: ${response.statusCode}`);
      return false;
    }
  } catch (error) {
    logError(`写入错误: ${error.message}`);
    return false;
  }
}

async function runTests() {
  log('🚀 InfluxDB 连接测试', 'bright');
  log('====================', 'bright');
  
  // 测试 1: 服务连通性
  const pingSuccess = await testPing();
  if (!pingSuccess) {
    logError('服务连通性测试失败，停止后续测试');
    process.exit(1);
  }
  
  // 测试 2: 基本查询
  const queries = [
    'SHOW DATABASES',
    'SHOW MEASUREMENTS',
    'SELECT * FROM "cpu_usage" LIMIT 5'
  ];
  
  for (const query of queries) {
    const success = await testQuery(query);
    if (!success) {
      logWarning(`查询测试失败: ${query}`);
    }
  }
  
  // 测试 3: 数据写入
  const writeSuccess = await testWrite();
  if (writeSuccess) {
    // 验证写入的数据
    await testQuery('SELECT * FROM "cpu_usage" WHERE host = \'test_server\' LIMIT 1');
  }
  
  log('🎉 测试完成', 'bright');
}

// 命令行参数处理
const args = process.argv.slice(2);
if (args.includes('--help') || args.includes('-h')) {
  log('InfluxDB 连接测试脚本', 'bright');
  log('');
  log('用法: node test-connection.js [选项]', 'cyan');
  log('');
  log('选项:', 'yellow');
  log('  --host <host>      InfluxDB 主机地址 (默认: localhost)', 'cyan');
  log('  --port <port>      InfluxDB 端口 (默认: 8086)', 'cyan');
  log('  --database <db>    数据库名称 (默认: testdb)', 'cyan');
  log('  --timeout <ms>     超时时间 (默认: 5000ms)', 'cyan');
  log('  --help, -h         显示帮助信息', 'cyan');
  log('');
  log('示例:', 'yellow');
  log('  node test-connection.js', 'cyan');
  log('  node test-connection.js --host 192.168.1.100 --port 8086', 'cyan');
  process.exit(0);
}

// 解析命令行参数
for (let i = 0; i < args.length; i += 2) {
  const key = args[i];
  const value = args[i + 1];
  
  switch (key) {
    case '--host':
      config.host = value;
      break;
    case '--port':
      config.port = parseInt(value);
      break;
    case '--database':
      config.database = value;
      break;
    case '--timeout':
      config.timeout = parseInt(value);
      break;
  }
}

// 显示配置
log('📋 测试配置:', 'bright');
log(`   主机: ${config.host}`, 'cyan');
log(`   端口: ${config.port}`, 'cyan');
log(`   数据库: ${config.database}`, 'cyan');
log(`   超时: ${config.timeout}ms`, 'cyan');
log('');

// 运行测试
runTests().catch((error) => {
  logError(`测试执行失败: ${error.message}`);
  process.exit(1);
}); 