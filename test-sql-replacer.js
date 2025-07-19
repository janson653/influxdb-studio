// 简单的SqlReplacer测试脚本
// 直接包含SqlReplacer实现进行测试

/**
 * SQL替换工具类
 * 用于智能替换SQL语句中的表名
 */
class SqlReplacer {
  /**
   * 替换SQL语句中的表名
   * @param sql 原始SQL语句
   * @param newTableName 新的表名
   * @returns 替换后的SQL语句
   */
  static replaceTableName(sql, newTableName) {
    if (!sql || !newTableName) {
      return sql;
    }

    const lines = sql.split('\n');
    const result = [];

    for (const line of lines) {
      const upperLine = line.toUpperCase();
      let newLine = line;

      // 处理 SELECT ... FROM table 模式
      if (upperLine.includes('FROM')) {
        newLine = this.replaceFromClause(line, newTableName);
      }
      // 处理 INSERT INTO table 模式
      else if (upperLine.includes('INSERT INTO')) {
        newLine = this.replaceInsertClause(line, newTableName);
      }
      // 处理 UPDATE table 模式
      else if (upperLine.includes('UPDATE')) {
        newLine = this.replaceUpdateClause(line, newTableName);
      }
      // 处理 DELETE FROM table 模式
      else if (upperLine.includes('DELETE FROM')) {
        newLine = this.replaceDeleteClause(line, newTableName);
      }
      // 处理 DROP TABLE table 模式
      else if (upperLine.includes('DROP TABLE')) {
        newLine = this.replaceDropClause(line, newTableName);
      }
      // 处理 CREATE TABLE table 模式
      else if (upperLine.includes('CREATE TABLE')) {
        newLine = this.replaceCreateClause(line, newTableName);
      }

      result.push(newLine);
    }

    return result.join('\n');
  }

  /**
   * 替换FROM子句中的表名
   */
  static replaceFromClause(line, newTableName) {
    const upperLine = line.toUpperCase();
    const fromIndex = upperLine.indexOf('FROM');
    if (fromIndex === -1) return line;

    // 找到FROM后的内容
    const afterFrom = line.substring(fromIndex + 4);
    
    // 使用正则表达式来匹配表名
    const tableNameRegex = /^\s*("([^"]+)"|'([^']+)'|([a-zA-Z_][a-zA-Z0-9_]*))/;
    const match = afterFrom.match(tableNameRegex);
    
    if (!match) return line;

    // 计算表名的实际位置
    const tableNameInAfterFrom = match[1];
    const tableNameStartInAfterFrom = afterFrom.indexOf(tableNameInAfterFrom);
    const tableNameStart = fromIndex + 4 + tableNameStartInAfterFrom;
    const tableNameEnd = tableNameStart + tableNameInAfterFrom.length;

    // 替换表名，统一使用双引号包围
    const newTableNameWithQuotes = `"${newTableName}"`;
    return line.substring(0, tableNameStart) + newTableNameWithQuotes + line.substring(tableNameEnd);
  }

  /**
   * 替换INSERT INTO子句中的表名
   */
  static replaceInsertClause(line, newTableName) {
    const upperLine = line.toUpperCase();
    const insertIntoIndex = upperLine.indexOf('INSERT INTO');
    if (insertIntoIndex === -1) return line;

    // 找到INSERT INTO后的内容
    const afterInsertInto = line.substring(insertIntoIndex + 11);
    
    // 使用正则表达式来匹配表名
    const tableNameRegex = /^\s*("([^"]+)"|'([^']+)'|([a-zA-Z_][a-zA-Z0-9_]*))/;
    const match = afterInsertInto.match(tableNameRegex);
    
    if (!match) return line;

    // 计算表名的实际位置
    const tableNameInAfterInsertInto = match[1];
    const tableNameStartInAfterInsertInto = afterInsertInto.indexOf(tableNameInAfterInsertInto);
    const tableNameStart = insertIntoIndex + 11 + tableNameStartInAfterInsertInto;
    const tableNameEnd = tableNameStart + tableNameInAfterInsertInto.length;

    // 替换表名，统一使用双引号包围
    const newTableNameWithQuotes = `"${newTableName}"`;
    return line.substring(0, tableNameStart) + newTableNameWithQuotes + line.substring(tableNameEnd);
  }

  /**
   * 替换UPDATE子句中的表名
   */
  static replaceUpdateClause(line, newTableName) {
    const upperLine = line.toUpperCase();
    const updateIndex = upperLine.indexOf('UPDATE');
    if (updateIndex === -1) return line;

    // 找到UPDATE后的内容
    const afterUpdate = line.substring(updateIndex + 6);
    
    // 使用正则表达式来匹配表名
    const tableNameRegex = /^\s*("([^"]+)"|'([^']+)'|([a-zA-Z_][a-zA-Z0-9_]*))/;
    const match = afterUpdate.match(tableNameRegex);
    
    if (!match) return line;

    // 计算表名的实际位置
    const tableNameInAfterUpdate = match[1];
    const tableNameStartInAfterUpdate = afterUpdate.indexOf(tableNameInAfterUpdate);
    const tableNameStart = updateIndex + 6 + tableNameStartInAfterUpdate;
    const tableNameEnd = tableNameStart + tableNameInAfterUpdate.length;

    // 替换表名，统一使用双引号包围
    const newTableNameWithQuotes = `"${newTableName}"`;
    return line.substring(0, tableNameStart) + newTableNameWithQuotes + line.substring(tableNameEnd);
  }

  /**
   * 替换DELETE FROM子句中的表名
   */
  static replaceDeleteClause(line, newTableName) {
    const upperLine = line.toUpperCase();
    const deleteFromIndex = upperLine.indexOf('DELETE FROM');
    if (deleteFromIndex === -1) return line;

    // 找到DELETE FROM后的第一个表名
    const afterDeleteFrom = line.substring(deleteFromIndex + 11).trim();
    
    // 使用正则表达式来匹配表名，包含前导空格
    const tableNameRegex = /^\s*("([^"]+)"|'([^']+)'|([a-zA-Z_][a-zA-Z0-9_]*))/;
    const match = afterDeleteFrom.match(tableNameRegex);
    
    if (!match) return line;

    let tableName;
    let tableNameStart;
    let tableNameEnd;

    if (match[1].startsWith('"') && match[1].endsWith('"')) {
      tableName = match[2];
      tableNameStart = deleteFromIndex + 11 + afterDeleteFrom.indexOf(match[1]);
      tableNameEnd = tableNameStart + match[1].length;
    } else if (match[1].startsWith("'") && match[1].endsWith("'")) {
      tableName = match[3];
      tableNameStart = deleteFromIndex + 11 + afterDeleteFrom.indexOf(match[1]);
      tableNameEnd = tableNameStart + match[1].length;
    } else {
      tableName = match[4];
      tableNameStart = deleteFromIndex + 11 + afterDeleteFrom.indexOf(match[1]);
      tableNameEnd = tableNameStart + match[1].length;
    }

    // 替换表名，统一使用双引号包围
    const newTableNameWithQuotes = `"${newTableName}"`;
    return line.substring(0, tableNameStart) + newTableNameWithQuotes + line.substring(tableNameEnd);
  }

  /**
   * 替换DROP TABLE子句中的表名
   */
  static replaceDropClause(line, newTableName) {
    const upperLine = line.toUpperCase();
    const dropTableIndex = upperLine.indexOf('DROP TABLE');
    if (dropTableIndex === -1) return line;

    // 找到DROP TABLE后的第一个表名
    const afterDropTable = line.substring(dropTableIndex + 10).trim();
    
    // 使用正则表达式来匹配表名，包含前导空格
    const tableNameRegex = /^\s*("([^"]+)"|'([^']+)'|([a-zA-Z_][a-zA-Z0-9_]*))/;
    const match = afterDropTable.match(tableNameRegex);
    
    if (!match) return line;

    let tableName;
    let tableNameStart;
    let tableNameEnd;

    if (match[1].startsWith('"') && match[1].endsWith('"')) {
      tableName = match[2];
      tableNameStart = dropTableIndex + 10 + afterDropTable.indexOf(match[1]);
      tableNameEnd = tableNameStart + match[1].length;
    } else if (match[1].startsWith("'") && match[1].endsWith("'")) {
      tableName = match[3];
      tableNameStart = dropTableIndex + 10 + afterDropTable.indexOf(match[1]);
      tableNameEnd = tableNameStart + match[1].length;
    } else {
      tableName = match[4];
      tableNameStart = dropTableIndex + 10 + afterDropTable.indexOf(match[1]);
      tableNameEnd = tableNameStart + match[1].length;
    }

    // 替换表名，统一使用双引号包围
    const newTableNameWithQuotes = `"${newTableName}"`;
    return line.substring(0, tableNameStart) + newTableNameWithQuotes + line.substring(tableNameEnd);
  }

  /**
   * 替换CREATE TABLE子句中的表名
   */
  static replaceCreateClause(line, newTableName) {
    const upperLine = line.toUpperCase();
    const createTableIndex = upperLine.indexOf('CREATE TABLE');
    if (createTableIndex === -1) return line;

    // 找到CREATE TABLE后的第一个表名
    const afterCreateTable = line.substring(createTableIndex + 12).trim();
    
    // 使用正则表达式来匹配表名，包含前导空格
    const tableNameRegex = /^\s*("([^"]+)"|'([^']+)'|([a-zA-Z_][a-zA-Z0-9_]*))/;
    const match = afterCreateTable.match(tableNameRegex);
    
    if (!match) return line;

    let tableName;
    let tableNameStart;
    let tableNameEnd;

    if (match[1].startsWith('"') && match[1].endsWith('"')) {
      tableName = match[2];
      tableNameStart = createTableIndex + 12 + afterCreateTable.indexOf(match[1]);
      tableNameEnd = tableNameStart + match[1].length;
    } else if (match[1].startsWith("'") && match[1].endsWith("'")) {
      tableName = match[3];
      tableNameStart = createTableIndex + 12 + afterCreateTable.indexOf(match[1]);
      tableNameEnd = tableNameStart + match[1].length;
    } else {
      tableName = match[4];
      tableNameStart = createTableIndex + 12 + afterCreateTable.indexOf(match[1]);
      tableNameEnd = tableNameStart + match[1].length;
    }

    // 替换表名，统一使用双引号包围
    const newTableNameWithQuotes = `"${newTableName}"`;
    return line.substring(0, tableNameStart) + newTableNameWithQuotes + line.substring(tableNameEnd);
  }

  /**
   * 检测SQL语句中是否包含任何表名
   */
  static hasAnyTableName(sql) {
    if (!sql) return false;

    const upperSql = sql.toUpperCase();
    
    // 检查是否包含FROM、INSERT INTO、UPDATE、DELETE FROM、DROP TABLE、CREATE TABLE等关键字
    const hasFrom = upperSql.includes('FROM');
    const hasInsertInto = upperSql.includes('INSERT INTO');
    const hasUpdate = upperSql.includes('UPDATE');
    const hasDeleteFrom = upperSql.includes('DELETE FROM');
    const hasDropTable = upperSql.includes('DROP TABLE');
    const hasCreateTable = upperSql.includes('CREATE TABLE');

    return hasFrom || hasInsertInto || hasUpdate || hasDeleteFrom || hasDropTable || hasCreateTable;
  }

  /**
   * 获取SQL语句中的第一个表名
   */
  static getFirstTableName(sql) {
    if (!sql) return null;

    const lines = sql.split('\n');
    
    for (const line of lines) {
      const upperLine = line.toUpperCase();
      
      if (upperLine.includes('FROM')) {
        const match = this.extractTableNameFromFromClause(line);
        if (match) return match;
      } else if (upperLine.includes('INSERT INTO')) {
        const match = this.extractTableNameFromInsertClause(line);
        if (match) return match;
      } else if (upperLine.includes('UPDATE')) {
        const match = this.extractTableNameFromUpdateClause(line);
        if (match) return match;
      } else if (upperLine.includes('DELETE FROM')) {
        const match = this.extractTableNameFromDeleteClause(line);
        if (match) return match;
      } else if (upperLine.includes('DROP TABLE')) {
        const match = this.extractTableNameFromDropClause(line);
        if (match) return match;
      } else if (upperLine.includes('CREATE TABLE')) {
        const match = this.extractTableNameFromCreateClause(line);
        if (match) return match;
      }
    }
    
    return null;
  }

  /**
   * 从FROM子句中提取表名
   */
  static extractTableNameFromFromClause(line) {
    const upperLine = line.toUpperCase();
    const fromIndex = upperLine.indexOf('FROM');
    if (fromIndex === -1) return null;

    const afterFrom = line.substring(fromIndex + 4).trim();
    const tableNameRegex = /^("([^"]+)"|'([^']+)'|([a-zA-Z_][a-zA-Z0-9_]*))/;
    const match = afterFrom.match(tableNameRegex);
    
    if (!match) return null;

    if (match[1].startsWith('"') && match[1].endsWith('"')) {
      return match[2];
    } else if (match[1].startsWith("'") && match[1].endsWith("'")) {
      return match[3];
    } else {
      return match[4];
    }
  }

  /**
   * 从INSERT INTO子句中提取表名
   */
  static extractTableNameFromInsertClause(line) {
    const upperLine = line.toUpperCase();
    const insertIntoIndex = upperLine.indexOf('INSERT INTO');
    if (insertIntoIndex === -1) return null;

    const afterInsertInto = line.substring(insertIntoIndex + 11).trim();
    const tableNameRegex = /^("([^"]+)"|'([^']+)'|([a-zA-Z_][a-zA-Z0-9_]*))/;
    const match = afterInsertInto.match(tableNameRegex);
    
    if (!match) return null;

    if (match[1].startsWith('"') && match[1].endsWith('"')) {
      return match[2];
    } else if (match[1].startsWith("'") && match[1].endsWith("'")) {
      return match[3];
    } else {
      return match[4];
    }
  }

  /**
   * 从UPDATE子句中提取表名
   */
  static extractTableNameFromUpdateClause(line) {
    const upperLine = line.toUpperCase();
    const updateIndex = upperLine.indexOf('UPDATE');
    if (updateIndex === -1) return null;

    const afterUpdate = line.substring(updateIndex + 6).trim();
    const tableNameRegex = /^("([^"]+)"|'([^']+)'|([a-zA-Z_][a-zA-Z0-9_]*))/;
    const match = afterUpdate.match(tableNameRegex);
    
    if (!match) return null;

    if (match[1].startsWith('"') && match[1].endsWith('"')) {
      return match[2];
    } else if (match[1].startsWith("'") && match[1].endsWith("'")) {
      return match[3];
    } else {
      return match[4];
    }
  }

  /**
   * 从DELETE FROM子句中提取表名
   */
  static extractTableNameFromDeleteClause(line) {
    const upperLine = line.toUpperCase();
    const deleteFromIndex = upperLine.indexOf('DELETE FROM');
    if (deleteFromIndex === -1) return null;

    const afterDeleteFrom = line.substring(deleteFromIndex + 11).trim();
    const tableNameRegex = /^("([^"]+)"|'([^']+)'|([a-zA-Z_][a-zA-Z0-9_]*))/;
    const match = afterDeleteFrom.match(tableNameRegex);
    
    if (!match) return null;

    if (match[1].startsWith('"') && match[1].endsWith('"')) {
      return match[2];
    } else if (match[1].startsWith("'") && match[1].endsWith("'")) {
      return match[3];
    } else {
      return match[4];
    }
  }

  /**
   * 从DROP TABLE子句中提取表名
   */
  static extractTableNameFromDropClause(line) {
    const upperLine = line.toUpperCase();
    const dropTableIndex = upperLine.indexOf('DROP TABLE');
    if (dropTableIndex === -1) return null;

    const afterDropTable = line.substring(dropTableIndex + 10).trim();
    const tableNameRegex = /^("([^"]+)"|'([^']+)'|([a-zA-Z_][a-zA-Z0-9_]*))/;
    const match = afterDropTable.match(tableNameRegex);
    
    if (!match) return null;

    if (match[1].startsWith('"') && match[1].endsWith('"')) {
      return match[2];
    } else if (match[1].startsWith("'") && match[1].endsWith("'")) {
      return match[3];
    } else {
      return match[4];
    }
  }

  /**
   * 从CREATE TABLE子句中提取表名
   */
  static extractTableNameFromCreateClause(line) {
    const upperLine = line.toUpperCase();
    const createTableIndex = upperLine.indexOf('CREATE TABLE');
    if (createTableIndex === -1) return null;

    const afterCreateTable = line.substring(createTableIndex + 12).trim();
    const tableNameRegex = /^("([^"]+)"|'([^']+)'|([a-zA-Z_][a-zA-Z0-9_]*))/;
    const match = afterCreateTable.match(tableNameRegex);
    
    if (!match) return null;

    if (match[1].startsWith('"') && match[1].endsWith('"')) {
      return match[2];
    } else if (match[1].startsWith("'") && match[1].endsWith("'")) {
      return match[3];
    } else {
      return match[4];
    }
  }
}

console.log('🧪 测试 SqlReplacer 功能...\n')

// 测试用例
const testCases = [
  {
    name: '替换无引号表名',
    sql: 'SELECT * FROM measurement LIMIT 10',
    newTableName: 'cpu_usage',
    expected: 'SELECT * FROM "cpu_usage" LIMIT 10'
  },
  {
    name: '替换双引号表名',
    sql: 'SELECT * FROM "old_table" LIMIT 10',
    newTableName: 'new_table',
    expected: 'SELECT * FROM "new_table" LIMIT 10'
  },
  {
    name: '替换单引号表名',
    sql: "SELECT * FROM 'old_table' LIMIT 10",
    newTableName: 'new_table',
    expected: 'SELECT * FROM "new_table" LIMIT 10'
  },
  {
    name: '处理复杂SQL',
    sql: 'SELECT field1, field2 FROM measurement WHERE time > now() - 1h ORDER BY time DESC LIMIT 100',
    newTableName: 'cpu_metrics',
    expected: 'SELECT field1, field2 FROM "cpu_metrics" WHERE time > now() - 1h ORDER BY time DESC LIMIT 100'
  },
  {
    name: '处理INSERT语句',
    sql: 'INSERT INTO old_table (field1, field2) VALUES (1, 2)',
    newTableName: 'new_table',
    expected: 'INSERT INTO "new_table" (field1, field2) VALUES (1, 2)'
  },
  {
    name: '处理UPDATE语句',
    sql: 'UPDATE old_table SET field1 = 1 WHERE id = 1',
    newTableName: 'new_table',
    expected: 'UPDATE "new_table" SET field1 = 1 WHERE id = 1'
  }
]

// 运行测试
let passed = 0
let total = testCases.length

testCases.forEach((testCase, index) => {
  console.log(`📝 测试 ${index + 1}: ${testCase.name}`)
  console.log(`   输入SQL: ${testCase.sql}`)
  console.log(`   新表名: ${testCase.newTableName}`)
  
  try {
    const result = SqlReplacer.replaceTableName(testCase.sql, testCase.newTableName)
    console.log(`   输出SQL: ${result}`)
    console.log(`   期望SQL: ${testCase.expected}`)
    
    if (result === testCase.expected) {
      console.log('   ✅ 通过\n')
      passed++
    } else {
      console.log('   ❌ 失败\n')
    }
  } catch (error) {
    console.log(`   ❌ 错误: ${error.message}\n`)
  }
})

// 测试其他功能
console.log('🔍 测试其他功能...\n')

const testSql = 'SELECT * FROM measurement LIMIT 10'
console.log(`测试SQL: ${testSql}`)
console.log(`包含表名: ${SqlReplacer.hasAnyTableName(testSql)}`)
console.log(`第一个表名: ${SqlReplacer.getFirstTableName(testSql)}`)

console.log(`\n📊 测试结果: ${passed}/${total} 通过`) 