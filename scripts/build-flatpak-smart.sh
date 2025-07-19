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
    
    if ! command -v flatpak &> /dev/null; then
        print_error "Flatpak 未安装，请先安装 Flatpak"
        exit 1
    fi
    
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo 未安装，请先安装 Rust"
        exit 1
    fi
    
    if ! command -v pnpm &> /dev/null; then
        print_error "pnpm 未安装，请先安装 pnpm"
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
    
    # 检查 Flatpak 运行时
    if ! flatpak list | grep -q "org.gnome.Platform"; then
        print_info "安装 GNOME Platform 运行时..."
        # 尝试安装最新稳定版本
        flatpak install flathub org.gnome.Platform//47 org.gnome.Sdk//47 -y || \
        flatpak install flathub org.gnome.Platform//45 org.gnome.Sdk//45 -y || \
        flatpak install flathub org.gnome.Platform//44 org.gnome.Sdk//44 -y
    fi
    
    # 检查 appstream-compose 是否可用
    if command -v appstream-compose &> /dev/null; then
        print_info "✅ appstream-compose 可用，使用标准构建流程"
        flatpak-builder --force-clean --repo=repo build com.influxdb.studio.yml
    else
        print_warning "❌ appstream-compose 不可用，使用备用配置"
        
        # 创建临时配置文件
        cp com.influxdb.studio.yml com.influxdb.studio.backup.yml
        
        # 修改配置文件避免使用 appstream-compose
        sed -i 's|- install -D com.influxdb.studio.metainfo.xml /app/share/metainfo/|- install -D com.influxdb.studio.metainfo.xml /app/share/metainfo/com.influxdb.studio.appdata.xml|' com.influxdb.studio.yml
        
        # 构建
        flatpak-builder --force-clean --repo=repo build com.influxdb.studio.yml
        
        # 恢复原配置
        mv com.influxdb.studio.backup.yml com.influxdb.studio.yml
    fi
    
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
    print_info "开始智能构建 InfluxDB Studio Flatpak 包..."
    
    check_dependencies
    build_tauri_app
    build_flatpak_smart
    
    print_info "构建完成！"
    print_info "Flatpak 包位置: flatpak/influxdb-studio.flatpak"
    print_info "安装命令: flatpak install influxdb-studio.flatpak"
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
        echo "  无参数: 智能构建 Flatpak 包"
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