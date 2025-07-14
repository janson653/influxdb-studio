#!/bin/bash

# GitHub Actions 测试脚本
# 用于本地验证构建流程

set -e

echo "🔍 检查 GitHub Actions 配置..."

# 检查必要的文件
if [ ! -f ".github/workflows/build.yml" ]; then
    echo "❌ 缺少 build.yml 工作流文件"
    exit 1
fi

if [ ! -f ".github/workflows/test.yml" ]; then
    echo "❌ 缺少 test.yml 工作流文件"
    exit 1
fi

echo "✅ 工作流文件存在"

# 检查 Tauri 配置
if [ ! -f "src-tauri/tauri.conf.json" ]; then
    echo "❌ 缺少 Tauri 配置文件"
    exit 1
fi

echo "✅ Tauri 配置文件存在"

# 检查前端配置
if [ ! -f "package.json" ]; then
    echo "❌ 缺少 package.json"
    exit 1
fi

if [ ! -f "src-tauri/Cargo.toml" ]; then
    echo "❌ 缺少 Cargo.toml"
    exit 1
fi

echo "✅ 项目配置文件完整"

# 检查依赖
echo "📦 检查依赖安装..."

if ! command -v pnpm &> /dev/null; then
    echo "❌ pnpm 未安装"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "❌ Rust 未安装"
    exit 1
fi

echo "✅ 依赖检查通过"

# 模拟构建流程
echo "🔨 模拟构建流程..."

echo "1. 安装前端依赖..."
pnpm install

echo "2. 构建前端..."
pnpm run build

echo "3. 检查 Rust 代码..."
cd src-tauri
cargo check
cargo clippy -- -D warnings

echo "4. 运行测试..."
cargo test

cd ..

echo "✅ 本地构建测试通过！"

# 检查 GitHub 权限配置
echo "🔐 检查 GitHub Actions 权限配置..."

if grep -q "permissions:" .github/workflows/build.yml; then
    echo "✅ 权限配置已添加"
else
    echo "❌ 缺少权限配置"
    exit 1
fi

if grep -q "contents: write" .github/workflows/build.yml; then
    echo "✅ contents 权限已配置"
else
    echo "❌ 缺少 contents 权限"
    exit 1
fi

echo "🎉 所有检查通过！GitHub Actions 应该可以正常运行了。"

echo "📋 下一步操作："
echo "   1. 提交更改到 GitHub"
echo "   2. 创建标签触发发布: git tag v0.1.1 && git push origin v0.1.1"
echo "   3. 查看 GitHub Actions 运行结果" 