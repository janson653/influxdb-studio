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

print_info "检查 Flatpak 构建环境..."

# 检查 flatpak-builder 版本
if command -v flatpak-builder &> /dev/null; then
    print_info "flatpak-builder 已安装"
    print_debug "版本信息："
    flatpak-builder --version
else
    print_error "flatpak-builder 未安装"
    exit 1
fi

# 检查 flatpak 版本
if command -v flatpak &> /dev/null; then
    print_info "flatpak 已安装"
    print_debug "版本信息："
    flatpak --version
else
    print_error "flatpak 未安装"
    exit 1
fi

# 检查 appstream-compose
if command -v appstream-compose &> /dev/null; then
    print_info "✅ appstream-compose 已安装"
    print_debug "版本信息："
    appstream-compose --version
else
    print_warning "❌ appstream-compose 未安装"
fi

# 检查可用的 flatpak-builder 选项
print_info "检查 flatpak-builder 可用选项..."
print_debug "帮助信息："
flatpak-builder --help | head -20

print_info "检查完成！" 