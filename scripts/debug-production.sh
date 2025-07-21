#!/bin/bash

# 生产环境调试脚本
echo "🔍 生产环境调试模式启动"

# 设置环境变量以启用调试
export RUST_LOG=debug
export RUST_BACKTRACE=1
export TAURI_DEBUG=1

# 检查打包文件是否存在
if [ ! -f "src-tauri/target/release/influxdb-studio" ]; then
    echo "❌ 未找到打包文件，请先运行: pnpm tauri build"
    exit 1
fi

echo "🚀 启动生产环境应用..."
echo "📝 调试信息将输出到控制台"
echo "🔧 环境变量:"
echo "   RUST_LOG=$RUST_LOG"
echo "   RUST_BACKTRACE=$RUST_BACKTRACE"
echo "   TAURI_DEBUG=$TAURI_DEBUG"

# 启动应用
./src-tauri/target/release/influxdb-studio 