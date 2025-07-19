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

print_debug() {
    echo -e "${BLUE}[DEBUG]${NC} $1"
}

print_info "🔧 修复 Tauri 开发环境..."

# 检查当前目录
if [ ! -f "package.json" ] || [ ! -d "src-tauri" ]; then
    print_error "请在项目根目录运行此脚本"
    exit 1
fi

print_info "📍 当前目录: $(pwd)"

# 方案1: 清理 Flatpak 构建目录
if [ -d "flatpak/build" ]; then
    print_info "🧹 清理 Flatpak 构建目录..."
    rm -rf flatpak/build
    print_info "✅ Flatpak 构建目录已清理"
fi

if [ -d "flatpak/repo" ]; then
    print_info "🧹 清理 Flatpak 仓库目录..."
    rm -rf flatpak/repo
    print_info "✅ Flatpak 仓库目录已清理"
fi

if [ -d "flatpak/.flatpak-builder" ]; then
    print_info "🧹 清理 Flatpak 构建缓存..."
    rm -rf flatpak/.flatpak-builder
    print_info "✅ Flatpak 构建缓存已清理"
fi

# 方案2: 检查并清理其他可能的符号链接问题
print_info "🔍 检查项目中的符号链接..."

# 查找项目中的符号链接
SYMLINKS=$(find . -type l -not -path "./node_modules/*" -not -path "./.git/*" 2>/dev/null || true)

if [ -n "$SYMLINKS" ]; then
    print_warning "发现符号链接："
    echo "$SYMLINKS"
    
    read -p "是否删除这些符号链接？(y/n): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "$SYMLINKS" | xargs rm -f
        print_info "✅ 符号链接已删除"
    fi
else
    print_info "✅ 未发现可疑的符号链接"
fi

# 方案3: 清理 Vite 缓存
print_info "🧹 清理 Vite 缓存..."
rm -rf node_modules/.vite 2>/dev/null || true
print_info "✅ Vite 缓存已清理"

# 方案4: 检查端口占用
print_info "🔍 检查端口占用..."
if lsof -ti:1420 > /dev/null 2>&1; then
    print_warning "端口 1420 被占用，正在释放..."
    lsof -ti:1420 | xargs kill -9
    print_info "✅ 端口 1420 已释放"
else
    print_info "✅ 端口 1420 可用"
fi

# 方案5: 创建 .gitignore 规则防止未来问题
print_info "📝 更新 .gitignore 规则..."

# 检查是否已有相关规则
if ! grep -q "flatpak/build" .gitignore 2>/dev/null; then
    echo "" >> .gitignore
    echo "# Flatpak build artifacts" >> .gitignore
    echo "flatpak/build/" >> .gitignore
    echo "flatpak/repo/" >> .gitignore
    echo "flatpak/.flatpak-builder/" >> .gitignore
    echo "flatpak/*.flatpak" >> .gitignore
    print_info "✅ 已添加 Flatpak 构建文件到 .gitignore"
fi

# 方案6: 创建 Vite 配置优化
print_info "🔧 优化 Vite 配置..."

# 检查 vite.config.ts 是否存在
if [ -f "vite.config.ts" ]; then
    # 检查是否已有 server.watch 配置
    if ! grep -q "server.*watch" vite.config.ts; then
        print_info "📝 添加 Vite 文件监视器配置..."
        
        # 备份原配置
        cp vite.config.ts vite.config.ts.backup
        
        # 添加配置（这里只是示例，实际需要根据现有配置调整）
        print_warning "请手动在 vite.config.ts 中添加以下配置："
        echo "server: {"
        echo "  watch: {"
        echo "    ignored: ['**/flatpak/**', '**/node_modules/**']"
        echo "  }"
        echo "}"
    else
        print_info "✅ Vite 配置已包含文件监视器设置"
    fi
fi

print_info "🎉 修复完成！"
print_info "现在可以尝试运行: ./start-tauri.sh"

# 询问是否立即启动
read -p "是否立即启动 Tauri 开发环境？(y/n): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    print_info "🚀 启动 Tauri 开发环境..."
    ./start-tauri.sh
fi 