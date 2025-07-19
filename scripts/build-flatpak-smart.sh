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

# 检查依赖
check_dependencies() {
    print_info "检查构建依赖..."
    
    local missing_deps=()
    
    command -v flatpak &> /dev/null || missing_deps+=("flatpak")
    command -v cargo &> /dev/null || missing_deps+=("cargo")
    command -v pnpm &> /dev/null || missing_deps+=("pnpm")
    
    if [ ${#missing_deps[@]} -ne 0 ]; then
        print_error "缺少依赖: ${missing_deps[*]}"
        print_info "请安装缺少的依赖后重试"
        exit 1
    fi
    
    print_info "所有依赖检查通过"
}

# 构建 Tauri 应用
build_tauri_app() {
    print_info "构建 Tauri 应用..."
    
    # 安装前端依赖
    pnpm install
    
    # 构建前端
    pnpm run build
    
    # 构建 Tauri 应用
    cd src-tauri
    cargo build --release --target x86_64-unknown-linux-gnu
    
    # 复制构建产物
    cp target/x86_64-unknown-linux-gnu/release/influxdb-studio ../flatpak/
    cd ..
    
    print_info "Tauri 应用构建完成"
}

# 智能构建 Flatpak 包
build_flatpak_smart() {
    print_info "智能构建 Flatpak 包..."
    
    cd flatpak
    
    # 检查并配置 Flathub 远程仓库
    if ! flatpak remote-list | grep -q "flathub"; then
        print_info "添加 Flathub 远程仓库..."
        flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
        flatpak update
    fi
    
    # 安装 GNOME Platform 运行时 (使用最新的47版本)
    if ! flatpak list | grep -q "org.gnome.Platform//47"; then
        print_info "安装 GNOME Platform 47 运行时..."
        flatpak install flathub org.gnome.Platform//47 org.gnome.Sdk//47 -y
    fi
    
    # 构建 Flatpak 包
    print_info "构建 Flatpak 包..."
    flatpak-builder --force-clean --repo=repo build com.influxdb.studio.yml
    
    # 创建 Flatpak 包文件
    flatpak build-bundle repo influxdb-studio.flatpak com.influxdb.studio
    
    cd ..
    
    print_info "Flatpak 包构建完成: flatpak/influxdb-studio.flatpak"
}

# 清理构建文件
cleanup() {
    print_info "清理构建文件..."
    rm -rf flatpak/build
    rm -rf flatpak/repo
    print_info "清理完成"
}

# 主函数
main() {
    print_info "开始构建 InfluxDB Studio Flatpak 包..."
    
    check_dependencies
    build_tauri_app
    build_flatpak_smart
    
    print_info "✅ 构建完成！"
    print_info "📦 包文件: flatpak/influxdb-studio.flatpak"
    print_info "🔧 安装: flatpak install influxdb-studio.flatpak"
}

# 处理命令行参数
case "${1:-}" in
    "clean")
        cleanup
        ;;
    "help"|"-h"|"--help")
        echo "用法: $0 [clean|help]"
        echo "  clean: 清理构建文件"
        echo "  help:  显示帮助信息"
        echo "  无参数: 构建 Flatpak 包"
        ;;
    "")
        main
        ;;
    *)
        print_error "未知参数: $1"
        echo "使用 '$0 help' 查看帮助"
        exit 1
        ;;
esac 