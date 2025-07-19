import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },

  // 优化配置
  optimizeDeps: {
    include: [
      'monaco-editor',
      'monaco-editor/esm/vs/editor/editor.api',
      'monaco-editor/esm/vs/editor/editor.main',
      'monaco-editor/esm/vs/basic-languages/sql/sql.contribution',
      'monaco-editor/esm/vs/basic-languages/javascript/javascript.contribution',
      'monaco-editor/esm/vs/basic-languages/typescript/typescript.contribution',
      'monaco-editor/esm/vs/basic-languages/json/json.contribution'
    ],
    exclude: ['@tauri-apps/api']
  },

  // 构建配置
  build: {
    rollupOptions: {
      external: ['@tauri-apps/api'],
      output: {
        manualChunks: {
          'monaco-editor': ['monaco-editor']
        }
      }
    }
  },

  // 解决 Monaco Editor 的 worker 问题
  worker: {
    format: 'es'
  }
}) 