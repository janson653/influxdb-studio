/**
 * 简单的事件总线
 * 用于组件间通信
 */
type EventCallback = (...args: any[]) => void;

class EventBus {
  private events: Map<string, EventCallback[]> = new Map();

  /**
   * 监听事件
   * @param event 事件名称
   * @param callback 回调函数
   */
  on(event: string, callback: EventCallback): void {
    if (!this.events.has(event)) {
      this.events.set(event, []);
    }
    this.events.get(event)!.push(callback);
  }

  /**
   * 移除事件监听
   * @param event 事件名称
   * @param callback 回调函数
   */
  off(event: string, callback: EventCallback): void {
    if (!this.events.has(event)) return;
    
    const callbacks = this.events.get(event)!;
    const index = callbacks.indexOf(callback);
    if (index > -1) {
      callbacks.splice(index, 1);
    }
  }

  /**
   * 触发事件
   * @param event 事件名称
   * @param args 参数
   */
  emit(event: string, ...args: any[]): void {
    if (!this.events.has(event)) return;
    
    const callbacks = this.events.get(event)!;
    callbacks.forEach(callback => {
      try {
        callback(...args);
      } catch (error) {
        console.error(`Event callback error for event '${event}':`, error);
      }
    });
  }

  /**
   * 清除所有事件监听
   */
  clear(): void {
    this.events.clear();
  }
}

// 创建全局事件总线实例
export const eventBus = new EventBus();

// 预定义的事件类型
export const Events = {
  // 双击表名事件
  DOUBLE_CLICK_TABLE: 'double-click-table',
  // 替换SQL表名事件
  REPLACE_SQL_TABLE: 'replace-sql-table',
  // 查询编辑器更新事件
  QUERY_EDITOR_UPDATE: 'query-editor-update'
} as const; 