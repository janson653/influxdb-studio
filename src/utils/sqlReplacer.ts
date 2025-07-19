/**
 * SQL替换工具类
 * 用于智能替换SQL语句中的表名
 */
export class SqlReplacer {
  /**
   * 替换SQL语句中的表名
   * @param sql 原始SQL语句
   * @param newTableName 新的表名
   * @returns 替换后的SQL语句
   */
  static replaceTableName(sql: string, newTableName: string): string {
    if (!sql || !newTableName) {
      return sql;
    }

    const lines = sql.split('\n');
    const result: string[] = [];

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
  private static replaceFromClause(line: string, newTableName: string): string {
    // 简单替换：将 measurement 替换为新的表名
    // 例如：SELECT * FROM measurement LIMIT 10 -> SELECT * FROM t3 LIMIT 10
    return line.replace(/\bmeasurement\b/gi, newTableName);
  }

  /**
   * 替换INSERT INTO子句中的表名
   */
  private static replaceInsertClause(line: string, newTableName: string): string {
    // 简单替换：将 measurement 替换为新的表名
    // 例如：INSERT INTO measurement,tag_key=tag_value field_key="field_value" -> INSERT INTO t3,tag_key=tag_value field_key="field_value"
    return line.replace(/\bmeasurement\b/gi, newTableName);
  }

  /**
   * 替换UPDATE子句中的表名
   */
  private static replaceUpdateClause(line: string, newTableName: string): string {
    // 简单替换：将 measurement 替换为新的表名
    // 例如：UPDATE measurement SET field_key="new_value" WHERE tag_key="tag_value" -> UPDATE t3 SET field_key="new_value" WHERE tag_key="tag_value"
    return line.replace(/\bmeasurement\b/gi, newTableName);
  }

  /**
   * 替换DELETE FROM子句中的表名
   */
  private static replaceDeleteClause(line: string, newTableName: string): string {
    // 简单替换：将 measurement 替换为新的表名
    // 例如：DELETE FROM measurement WHERE tag_key="tag_value" -> DELETE FROM t3 WHERE tag_key="tag_value"
    return line.replace(/\bmeasurement\b/gi, newTableName);
  }

  /**
   * 替换DROP TABLE子句中的表名
   */
  private static replaceDropClause(line: string, newTableName: string): string {
    // 简单替换：将 measurement 替换为新的表名
    // 例如：DROP TABLE measurement -> DROP TABLE t3
    return line.replace(/\bmeasurement\b/gi, newTableName);
  }

  /**
   * 替换CREATE TABLE子句中的表名
   */
  private static replaceCreateClause(line: string, newTableName: string): string {
    // 简单替换：将 measurement 替换为新的表名
    // 例如：CREATE TABLE measurement (field1 TEXT) -> CREATE TABLE t3 (field1 TEXT)
    return line.replace(/\bmeasurement\b/gi, newTableName);
  }

  /**
   * 检测SQL语句中是否包含表名
   * @param sql SQL语句
   * @param tableName 表名
   * @returns 是否包含该表名
   */
  static containsTableName(sql: string, tableName: string): boolean {
    if (!sql || !tableName) return false;

    const upperSql = sql.toUpperCase();
    // 移除未使用的变量

    // 检查各种模式
    const patterns = [
      `FROM "${tableName}"`,
      `FROM '${tableName}'`,
      `FROM ${tableName}`,
      `INSERT INTO "${tableName}"`,
      `INSERT INTO '${tableName}'`,
      `INSERT INTO ${tableName}`,
      `UPDATE "${tableName}"`,
      `UPDATE '${tableName}'`,
      `UPDATE ${tableName}`,
      `DELETE FROM "${tableName}"`,
      `DELETE FROM '${tableName}'`,
      `DELETE FROM ${tableName}`,
      `DROP TABLE "${tableName}"`,
      `DROP TABLE '${tableName}'`,
      `DROP TABLE ${tableName}`,
      `CREATE TABLE "${tableName}"`,
      `CREATE TABLE '${tableName}'`,
      `CREATE TABLE ${tableName}`
    ];

    return patterns.some(pattern => upperSql.includes(pattern.toUpperCase()));
  }

  /**
   * 检测SQL语句中是否包含任何表名
   * @param sql SQL语句
   * @returns 是否包含表名
   */
  static hasAnyTableName(sql: string): boolean {
    if (!sql) return false;

    // 简单检测：是否包含 measurement 关键字
    return /\bmeasurement\b/gi.test(sql);
  }

  /**
   * 获取SQL语句中的第一个表名
   * @param sql SQL语句
   * @returns 表名，如果没有找到则返回null
   */
  static getFirstTableName(sql: string): string | null {
    if (!sql) return null;

    // 简单提取：查找 measurement 关键字
    const match = sql.match(/\bmeasurement\b/gi);
    return match ? 'measurement' : null;
  }


} 