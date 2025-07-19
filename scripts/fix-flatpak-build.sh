#!/bin/bash

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
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

print_info "修复 Flatpak 构建问题..."

# 方案1: 尝试安装 appstream-compose
print_info "方案1: 尝试安装 appstream-compose..."

if command -v apt &> /dev/null; then
    print_info "检测到 apt 包管理器，尝试安装 appstream-compose..."
    sudo apt update && sudo apt install -y appstream-compose
elif command -v dnf &> /dev/null; then
    print_info "检测到 dnf 包管理器，尝试安装 appstream-compose..."
    sudo dnf install -y appstream-compose
elif command -v pacman &> /dev/null; then
    print_info "检测到 pacman 包管理器，尝试安装 appstream-compose..."
    sudo pacman -S appstream-compose
elif command -v zypper &> /dev/null; then
    print_info "检测到 zypper 包管理器，尝试安装 appstream-compose..."
    sudo zypper install appstream-compose
else
    print_warning "无法自动安装 appstream-compose，请手动安装："
    print_warning "Ubuntu/Debian: sudo apt install appstream-compose"
    print_warning "Fedora: sudo dnf install appstream-compose"
    print_warning "Arch: sudo pacman -S appstream-compose"
    print_warning "openSUSE: sudo zypper install appstream-compose"
fi

# 检查是否安装成功
if command -v appstream-compose &> /dev/null; then
    print_info "✅ appstream-compose 安装成功！"
    print_info "现在可以重新运行构建脚本："
    print_info "  ./scripts/build-flatpak.sh"
else
    print_warning "❌ appstream-compose 安装失败，使用替代方案..."
    
    # 方案2: 使用简化构建脚本
    print_info "方案2: 使用简化构建脚本（无需 appstream-compose）..."
    print_info "运行简化构建脚本："
    print_info "  ./scripts/build-flatpak-simple.sh"
    
    # 询问是否立即运行
    read -p "是否立即运行简化构建脚本？(y/n): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        ./scripts/build-flatpak-simple.sh
    fi
fi

print_info "修复完成！" 