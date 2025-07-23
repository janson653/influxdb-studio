#!/bin/bash

# InfluxDB Studio 快速启动脚本
# 用于快速搭建开发环境和启动测试服务

set -e

echo "🚀 InfluxDB Studio 快速启动脚本"
echo "================================"

# 检查必要的工具
check_requirements() {
    echo "📋 检查系统要求..."
    
    # 检查 Node.js
    if ! command -v node &> /dev/null; then
        echo "❌ Node.js 未安装，请先安装 Node.js 18+"
        exit 1
    fi
    
    NODE_VERSION=$(node -v | cut -d'v' -f2 | cut -d'.' -f1)
    if [ "$NODE_VERSION" -lt 18 ]; then
        echo "❌ Node.js 版本过低，需要 18+ 版本"
        exit 1
    fi
    
    echo "✅ Node.js $(node -v)"
    
    # 检查 Rust
    if ! command -v cargo &> /dev/null; then
        echo "❌ Rust 未安装，请先安装 Rust 1.70+"
        echo "   访问 https://rustup.rs/ 安装 Rust"
        exit 1
    fi
    
    echo "✅ Rust $(cargo --version | cut -d' ' -f2)"
    
    # 检查 pnpm
    if ! command -v pnpm &> /dev/null; then
        echo "⚠️  pnpm 未安装，正在安装..."
        npm install -g pnpm
    fi
    
    echo "✅ pnpm $(pnpm --version)"
    
    # 检查 Docker
    if ! command -v docker &> /dev/null; then
        echo "⚠️  Docker 未安装，将跳过测试环境启动"
        DOCKER_AVAILABLE=false
    else
        echo "✅ Docker $(docker --version | cut -d' ' -f3 | cut -d',' -f1)"
        DOCKER_AVAILABLE=true
    fi
}

# 安装依赖
install_dependencies() {
    echo "📦 安装项目依赖..."
    
    # 安装前端依赖
    echo "   安装前端依赖..."
    pnpm install
    
    # 安装 Rust 依赖
    echo "   安装 Rust 依赖..."
    cd src-tauri
    cargo build --release
    cd ..
    
    echo "✅ 依赖安装完成"
}

# 启动测试环境
start_test_environment() {
    if [ "$DOCKER_AVAILABLE" = false ]; then
        echo "⚠️  跳过测试环境启动（Docker 未安装）"
        return
    fi
    
    echo "🐳 启动测试环境..."
    
    # 检查 Docker 服务状态
    if ! docker info &> /dev/null; then
        echo "❌ Docker 服务未启动，请先启动 Docker"
        return
    fi
    
    # 启动 InfluxDB 服务
    echo "   启动 InfluxDB v1.x 服务..."
    docker-compose up influxdb-v1 -d
    
    # 等待服务启动
    echo "   等待服务启动..."
    sleep 10
    
    # 检查服务状态
    if docker-compose ps | grep -q "influxdb-v1-test.*Up"; then
        echo "✅ InfluxDB 服务启动成功"
        echo "   访问地址: http://localhost:8086"
        echo "   测试数据库: testdb"
    else
        echo "❌ InfluxDB 服务启动失败"
        docker-compose logs influxdb-v1
    fi
}

# 显示连接信息
show_connection_info() {
    echo ""
    echo "🔗 连接信息"
    echo "=========="
    echo "主机: localhost"
    echo "端口: 8086"
    echo "数据库: testdb"
    echo "用户名: (留空)"
    echo "密码: (留空)"
    echo ""
    echo "📝 测试查询示例:"
    echo "   SHOW DATABASES"
    echo "   SHOW MEASUREMENTS"
    echo "   SELECT * FROM \"cpu_usage\" LIMIT 10"
    echo ""
}

# 启动开发服务器
start_development() {
    echo "🛠️  启动开发服务器..."
    echo ""
    echo "选择启动模式:"
    echo "1) 前端开发模式 (仅前端)"
    echo "2) Tauri 开发模式 (完整应用)"
    echo "3) 仅构建应用"
    echo ""
    read -p "请选择 (1-3): " choice
    
    case $choice in
        1)
            echo "启动前端开发服务器..."
            pnpm dev
            ;;
        2)
            echo "启动 Tauri 开发模式..."
            pnpm tauri dev
            ;;
        3)
            echo "构建应用..."
            pnpm tauri build
            ;;
        *)
            echo "无效选择，退出"
            exit 1
            ;;
    esac
}

# 清理环境
cleanup() {
    echo "🧹 清理环境..."
    
    if [ "$DOCKER_AVAILABLE" = true ]; then
        echo "   停止 Docker 服务..."
        docker-compose down
    fi
    
    echo "✅ 清理完成"
}

# 主函数
main() {
    case "${1:-}" in
        "cleanup")
            cleanup
            exit 0
            ;;
        "test")
            start_test_environment
            exit 0
            ;;
        "help"|"-h"|"--help")
            echo "用法: $0 [选项]"
            echo ""
            echo "选项:"
            echo "  cleanup    清理测试环境"
            echo "  test       仅启动测试环境"
            echo "  help       显示帮助信息"
            echo ""
            echo "无参数时执行完整启动流程"
            exit 0
            ;;
    esac
    
    check_requirements
    install_dependencies
    start_test_environment
    show_connection_info
    
    echo "🎉 环境搭建完成！"
    echo ""
    echo "下一步操作:"
    echo "1. 启动开发服务器: $0"
    echo "2. 清理环境: $0 cleanup"
    echo "3. 查看帮助: $0 help"
    echo ""
    
    read -p "是否立即启动开发服务器？(y/N): " start_dev
    if [[ $start_dev =~ ^[Yy]$ ]]; then
        start_development
    else
        echo "✅ 环境搭建完成，可以手动启动开发服务器"
        echo "   前端开发: pnpm dev"
        echo "   Tauri 开发: pnpm tauri dev"
    fi
}

# 捕获中断信号
trap cleanup INT

# 执行主函数
main "$@" 