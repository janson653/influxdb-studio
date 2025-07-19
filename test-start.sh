#!/bin/bash

echo "测试 Tauri 启动环境..."

# 清理 Flatpak 构建目录
if [ -d "flatpak/build" ]; then
    echo "清理 flatpak/build..."
    rm -rf flatpak/build
fi

if [ -d "flatpak/repo" ]; then
    echo "清理 flatpak/repo..."
    rm -rf flatpak/repo
fi

# 清理 Vite 缓存
if [ -d "node_modules/.vite" ]; then
    echo "清理 Vite 缓存..."
    rm -rf node_modules/.vite
fi

# 检查端口
if lsof -ti:1420 > /dev/null 2>&1; then
    echo "释放端口 1420..."
    lsof -ti:1420 | xargs kill -9
fi

echo "环境清理完成，现在可以启动 Tauri" 