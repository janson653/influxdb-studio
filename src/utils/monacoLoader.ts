// Monaco Editor 加载器工具
// 用于统一管理 Monaco Editor 的加载和配置

let monacoInstance: any = null
let isLoading = false
let loadPromise: Promise<any> | null = null

export interface MonacoLoadOptions {
  forceReload?: boolean
  timeout?: number
}

/**
 * 加载 Monaco Editor
 * @param options 加载选项
 * @returns Monaco Editor 实例
 */
export async function loadMonaco(options: MonacoLoadOptions = {}): Promise<any> {
  const { forceReload = false, timeout = 10000 } = options

  // 如果已经加载且不强制重载，直接返回
  if (monacoInstance && !forceReload) {
    return monacoInstance
  }

  // 如果正在加载，返回现有的 Promise
  if (isLoading && loadPromise) {
    return loadPromise
  }

  // 创建加载 Promise
  loadPromise = new Promise((resolve, reject) => {
    isLoading = true

    // 设置超时
    const timeoutId = setTimeout(() => {
      isLoading = false
      loadPromise = null
      reject(new Error('Monaco Editor 加载超时'))
    }, timeout)

    // 动态导入 Monaco Editor
    import('monaco-editor')
      .then((monacoModule) => {
        clearTimeout(timeoutId)
        
        monacoInstance = monacoModule.default || monacoModule
        
        // 验证 Monaco Editor 是否正确加载
        if (!monacoInstance || !monacoInstance.editor) {
          throw new Error('Monaco Editor 加载失败')
        }

        // 配置 Monaco Editor
        configureMonaco(monacoInstance)
        
        isLoading = false
        loadPromise = null
        resolve(monacoInstance)
      })
      .catch((error) => {
        clearTimeout(timeoutId)
        isLoading = false
        loadPromise = null
        console.error('Monaco Editor 加载错误:', error)
        reject(error)
      })
  })

  return loadPromise
}

/**
 * 配置 Monaco Editor
 * @param monaco Monaco Editor 实例
 */
function configureMonaco(monaco: any) {
  // 设置 Monaco Editor 环境
  if (typeof window !== 'undefined') {
    (window as any).MonacoEnvironment = {
      getWorkerUrl: function (_moduleId: string, label: string) {
        // 使用内联 worker 来避免跨域问题
        const workerScript = getWorkerScript(label)
        const blob = new Blob([workerScript], { type: 'application/javascript' })
        return URL.createObjectURL(blob)
      }
    }
  }

  // 配置编辑器默认选项
  monaco.editor.defineTheme('vs-dark', {
    base: 'vs-dark',
    inherit: true,
    rules: [],
    colors: {}
  })
}

/**
 * 获取 Worker 脚本内容
 * @param label 语言标签
 * @returns Worker 脚本内容
 */
function getWorkerScript(label: string): string {
  // 根据语言类型返回对应的 worker 脚本
  switch (label) {
    case 'sql':
      return `
        importScripts('/monaco-editor/min/vs/language/sql/sql.worker.js');
      `
    case 'json':
      return `
        importScripts('/monaco-editor/min/vs/language/json/json.worker.js');
      `
    case 'typescript':
    case 'javascript':
      return `
        importScripts('/monaco-editor/min/vs/language/typescript/ts.worker.js');
      `
    default:
      return `
        importScripts('/monaco-editor/min/vs/editor/editor.worker.js');
      `
  }
}

/**
 * 获取已加载的 Monaco Editor 实例
 * @returns Monaco Editor 实例或 null
 */
export function getMonacoInstance(): any {
  return monacoInstance
}

/**
 * 检查 Monaco Editor 是否已加载
 * @returns 是否已加载
 */
export function isMonacoLoaded(): boolean {
  return monacoInstance !== null
}

/**
 * 重置 Monaco Editor 实例
 */
export function resetMonaco(): void {
  monacoInstance = null
  isLoading = false
  loadPromise = null
} 