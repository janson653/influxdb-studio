#!/bin/bash

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_info "🚀 启动 InfluxDB Studio (Tauri 应用)"
print_info "📍 当前目录: $(pwd)"

# 检查项目结构
if [ ! -f "package.json" ] || [ ! -d "src-tauri" ]; then
    print_error "请在项目根目录运行此脚本"
    exit 1
fi

# 预检查：清理可能导致问题的目录
print_info "🔍 预检查环境..."

# 检查并清理 Flatpak 构建目录
if [ -d "flatpak/build" ]; then
    print_warning "发现 Flatpak 构建目录，正在清理..."
    rm -rf flatpak/build
    print_info "✅ Flatpak 构建目录已清理"
fi

if [ -d "flatpak/repo" ]; then
    print_warning "发现 Flatpak 仓库目录，正在清理..."
    rm -rf flatpak/repo
    print_info "✅ Flatpak 仓库目录已清理"
fi

# 清理 Vite 缓存
if [ -d "node_modules/.vite" ]; then
    print_info "🧹 清理 Vite 缓存..."
    rm -rf node_modules/.vite
    print_info "✅ Vite 缓存已清理"
fi

# 清理端口占用
print_info "🧹 清理端口占用..."
if lsof -ti:1420 > /dev/null 2>&1; then
    print_warning "端口 1420 被占用，正在释放..."
    lsof -ti:1420 | xargs kill -9
    sleep 1
    print_info "✅ 端口 1420 已释放"
else
    print_info "✅ 端口 1420 可用"
fi

# 设置环境变量
print_info "🔧 设置环境变量..."

# 检测 WSL2 环境
if grep -q Microsoft /proc/version 2>/dev/null; then
    print_info "🐧 检测到 WSL2 环境，设置相关环境变量..."
    export DISPLAY=:0
    export LIBGL_ALWAYS_SOFTWARE=1
    export MESA_GL_VERSION_OVERRIDE=3.3
fi

# 设置 Node.js 环境
export NODE_ENV=development

print_info "🔧 环境变量设置完成："
echo "   DISPLAY=$DISPLAY"
echo "   LIBGL_ALWAYS_SOFTWARE=$LIBGL_ALWAYS_SOFTWARE"
echo "   MESA_GL_VERSION_OVERRIDE=$MESA_GL_VERSION_OVERRIDE"
echo "   NODE_ENV=$NODE_ENV"

# 启动开发服务器
print_info "🚀 启动 Tauri 开发环境..."

# 使用 exec 确保信号正确传递
exec pnpm tauri dev 