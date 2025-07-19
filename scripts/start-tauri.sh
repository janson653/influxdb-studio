#!/bin/bash

# WSL2 环境下启动 Tauri 应用的脚本

echo "🚀 启动 InfluxDB Studio (Tauri 应用)"
echo "📍 当前目录: $(pwd)"

# 清理可能占用端口的进程
echo "🧹 清理端口占用..."
lsof -ti:1420 | xargs -r kill -9 2>/dev/null || true
pkill -f "vite.*1420" 2>/dev/null || true

# 等待端口释放
sleep 2

# 检查端口是否已释放
if lsof -i:1420 >/dev/null 2>&1; then
    echo "❌ 端口 1420 仍被占用，请手动检查"
    lsof -i:1420
    exit 1
fi

echo "✅ 端口 1420 已释放"

# 设置 WSL2 环境变量
export DISPLAY=:0
export LIBGL_ALWAYS_SOFTWARE=1
export MESA_GL_VERSION_OVERRIDE=3.3
export GALLIUM_DRIVER=softpipe
export WEBKIT_DISABLE_COMPOSITING_MODE=1

# 如果是 WSL2 环境，设置额外的环境变量
if grep -qi microsoft /proc/version; then
    echo "🐧 检测到 WSL2 环境，设置相关环境变量..."
    export PULSE_RUNTIME_PATH=/mnt/wslg/runtime
    export XDG_RUNTIME_DIR=/mnt/wslg/runtime
    export WAYLAND_DISPLAY=wayland-0
fi

echo "🔧 环境变量设置完成："
echo "   DISPLAY=$DISPLAY"
echo "   LIBGL_ALWAYS_SOFTWARE=$LIBGL_ALWAYS_SOFTWARE"
echo "   MESA_GL_VERSION_OVERRIDE=$MESA_GL_VERSION_OVERRIDE"

# 启动 Tauri 应用
echo "🚀 启动 Tauri 开发环境..."
pnpm run tauri dev 