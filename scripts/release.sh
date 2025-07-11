#!/bin/bash

# InfluxDB Studio 发布脚本
# 使用方法: ./scripts/release.sh v0.1.0

set -e

# 检查参数
if [ $# -eq 0 ]; then
    echo "❌ 错误: 请提供版本号"
    echo "使用方法: $0 v0.1.0"
    exit 1
fi

VERSION=$1

# 验证版本号格式
if [[ ! $VERSION =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "❌ 错误: 版本号格式不正确"
    echo "正确格式: v1.0.0"
    exit 1
fi

echo "🚀 准备发布 InfluxDB Studio $VERSION"

# 检查是否有未提交的更改
if [ -n "$(git status --porcelain)" ]; then
    echo "❌ 错误: 有未提交的更改，请先提交所有更改"
    git status
    exit 1
fi

# 更新版本号
echo "📝 更新版本号..."

# 更新 package.json
sed -i "s/\"version\": \".*\"/\"version\": \"${VERSION#v}\"/" package.json

# 更新 src-tauri/Cargo.toml
sed -i "s/version = \".*\"/version = \"${VERSION#v}\"/" src-tauri/Cargo.toml

# 更新 src-tauri/tauri.conf.json
sed -i "s/\"version\": \".*\"/\"version\": \"${VERSION#v}\"/" src-tauri/tauri.conf.json

echo "✅ 版本号已更新为 ${VERSION#v}"

# 提交更改
echo "📦 提交版本更新..."
git add package.json src-tauri/Cargo.toml src-tauri/tauri.conf.json
git commit -m "chore: bump version to $VERSION"

# 创建标签
echo "🏷️  创建标签..."
git tag $VERSION

# 推送到远程
echo "🚀 推送到远程仓库..."
git push origin main
git push origin $VERSION

echo "✨ 发布流程已启动！"
echo "📋 接下来的步骤:"
echo "   1. 访问 GitHub Actions 页面查看构建进度"
echo "   2. 构建完成后，检查 GitHub Releases 页面"
echo "   3. 测试下载的安装包"
echo ""
echo "🔗 GitHub Actions: https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:/]\(.*\)\.git/\1/')/actions"
echo "🔗 Releases: https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:/]\(.*\)\.git/\1/')/releases" 