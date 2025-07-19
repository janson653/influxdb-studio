// Monaco Editor 配置文件
// 用于优化 worker 加载和性能

// 设置 Monaco Editor 环境
window.MonacoEnvironment = {
  getWorkerUrl: function (moduleId, label) {
    // 根据语言类型返回对应的 worker
    switch (label) {
      case 'sql':
        return '/monaco-editor/min/vs/language/sql/sql.worker.js'
      case 'json':
        return '/monaco-editor/min/vs/language/json/json.worker.js'
      case 'css':
      case 'scss':
      case 'less':
        return '/monaco-editor/min/vs/language/css/css.worker.js'
      case 'html':
      case 'handlebars':
      case 'razor':
        return '/monaco-editor/min/vs/language/html/html.worker.js'
      case 'typescript':
      case 'javascript':
        return '/monaco-editor/min/vs/language/typescript/ts.worker.js'
      default:
        return '/monaco-editor/min/vs/editor/editor.worker.js'
    }
  }
}

// 注意：Monaco Editor 的语言支持会在需要时自动加载
// 不需要手动预加载，这会导致模块解析错误 