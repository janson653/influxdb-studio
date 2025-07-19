// 测试正则表达式语法
console.log('测试正则表达式...');

// 测试原始的正则表达式（有问题的）
try {
  const badRegex = /^\s*("([^"]+)"|'([^']+)'|([a-zA-Z0-9_.-]+))/;
  console.log('原始正则表达式语法正确');
  
  // 测试匹配
  const testSql = 'SELECT * FROM measurement LIMIT 10';
  const match = testSql.match(/FROM\s+(.+)/);
  if (match) {
    const afterFrom = match[1];
    const tableMatch = afterFrom.match(badRegex);
    console.log('匹配结果:', tableMatch);
  }
} catch (error) {
  console.error('原始正则表达式错误:', error.message);
}

// 测试修复后的正则表达式
try {
  const goodRegex = /^\s*("([^"]+)"|'([^']+)'|([a-zA-Z0-9_.-]+))/;
  console.log('修复后正则表达式语法正确');
  
  // 测试匹配
  const testSql = 'SELECT * FROM measurement LIMIT 10';
  const match = testSql.match(/FROM\s+(.+)/);
  if (match) {
    const afterFrom = match[1];
    const tableMatch = afterFrom.match(goodRegex);
    console.log('匹配结果:', tableMatch);
  }
} catch (error) {
  console.error('修复后正则表达式错误:', error.message);
}

// 测试更安全的正则表达式
try {
  const safeRegex = /^\s*("([^"]+)"|'([^']+)'|([a-zA-Z0-9_.-]+))/;
  console.log('安全正则表达式语法正确');
  
  // 测试匹配
  const testSql = 'SELECT * FROM measurement LIMIT 10';
  const match = testSql.match(/FROM\s+(.+)/);
  if (match) {
    const afterFrom = match[1];
    const tableMatch = afterFrom.match(safeRegex);
    console.log('匹配结果:', tableMatch);
  }
} catch (error) {
  console.error('安全正则表达式错误:', error.message);
} 