#!/bin/bash

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

print_header() {
    echo -e "${BLUE}=== $1 ===${NC}"
}

# 检查 Flatpak 安装
check_flatpak_installation() {
    print_header "检查 Flatpak 安装"
    
    if command -v flatpak &> /dev/null; then
        print_info "Flatpak 已安装: $(flatpak --version)"
    else
        print_error "Flatpak 未安装"
        return 1
    fi
    
    if command -v flatpak-builder &> /dev/null; then
        print_info "flatpak-builder 已安装"
    else
        print_warning "flatpak-builder 未安装"
    fi
}

# 检查远程仓库
check_remotes() {
    print_header "检查远程仓库"
    
    echo "当前远程仓库列表："
    flatpak remote-list
    
    if flatpak remote-list | grep -q "flathub"; then
        print_info "Flathub 远程仓库已配置"
    else
        print_warning "Flathub 远程仓库未配置"
        print_info "建议运行: flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo"
    fi
}

# 检查已安装的运行时
check_runtimes() {
    print_header "检查已安装的运行时"
    
    echo "已安装的运行时："
    flatpak list --runtime
    
    if flatpak list --runtime | grep -q "org.gnome.Platform"; then
        print_info "GNOME Platform 运行时已安装"
        flatpak list --runtime | grep "org.gnome.Platform"
    else
        print_warning "GNOME Platform 运行时未安装"
    fi
    
    if flatpak list --runtime | grep -q "org.gnome.Sdk"; then
        print_info "GNOME SDK 运行时已安装"
        flatpak list --runtime | grep "org.gnome.Sdk"
    else
        print_warning "GNOME SDK 运行时未安装"
    fi
}

# 检查可用的运行时版本
check_available_runtimes() {
    print_header "检查可用的运行时版本"
    
    if flatpak remote-list | grep -q "flathub"; then
        echo "Flathub 中可用的 GNOME Platform 版本："
        flatpak remote-ls flathub | grep "org.gnome.Platform" | head -10
        
        echo "Flathub 中可用的 GNOME SDK 版本："
        flatpak remote-ls flathub | grep "org.gnome.Sdk" | head -10
    else
        print_warning "无法检查可用版本，Flathub 远程仓库未配置"
    fi
}

# 检查系统环境
check_system_environment() {
    print_header "检查系统环境"
    
    echo "操作系统: $(lsb_release -d 2>/dev/null | cut -f2 || echo 'Unknown')"
    echo "内核版本: $(uname -r)"
    echo "架构: $(uname -m)"
    echo "XDG_DATA_DIRS: $XDG_DATA_DIRS"
    
    # 检查 Flatpak 导出目录
    if [ -d "/var/lib/flatpak/exports/share" ]; then
        print_info "系统级 Flatpak 导出目录存在"
    else
        print_warning "系统级 Flatpak 导出目录不存在"
    fi
    
    if [ -d "$HOME/.local/share/flatpak/exports/share" ]; then
        print_info "用户级 Flatpak 导出目录存在"
    else
        print_warning "用户级 Flatpak 导出目录不存在"
    fi
}

# 提供修复建议
provide_fixes() {
    print_header "修复建议"
    
    echo "如果遇到问题，请按以下步骤操作："
    echo ""
    echo "1. 添加 Flathub 远程仓库："
    echo "   flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo"
    echo ""
    echo "2. 更新远程仓库："
    echo "   flatpak update"
    echo ""
    echo "3. 安装 GNOME Platform 运行时："
    echo "   flatpak install flathub org.gnome.Platform//44 org.gnome.Sdk//44"
    echo ""
    echo "4. 如果上述版本不可用，尝试其他版本："
    echo "   flatpak install flathub org.gnome.Platform//43 org.gnome.Sdk//43"
    echo "   flatpak install flathub org.gnome.Platform//42 org.gnome.Sdk//42"
    echo ""
    echo "5. 重启会话以更新 XDG_DATA_DIRS："
    echo "   logout 然后重新登录"
}

# 主函数
main() {
    print_header "Flatpak 环境诊断"
    
    check_flatpak_installation || exit 1
    check_remotes
    check_runtimes
    check_available_runtimes
    check_system_environment
    provide_fixes
    
    print_header "诊断完成"
}

main "$@" 