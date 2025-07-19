# Flatpak 构建指南

## 概述

InfluxDB Studio 支持通过 Flatpak 在 Linux 系统上分发。Flatpak 提供了沙盒化的应用环境，确保应用在不同 Linux 发行版上的一致性。

## 构建环境要求

### 系统依赖

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install flatpak flatpak-builder appstream-compose

# CentOS/RHEL
sudo yum install flatpak flatpak-builder

# Arch Linux
sudo pacman -S flatpak flatpak-builder appstream-compose
```

### Flatpak 运行时

```bash
# 添加 Flathub 仓库
flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

# 安装 GNOME Platform 运行时
flatpak install flathub org.gnome.Platform//47 org.gnome.Sdk//47
```

## 构建配置

### manifest 文件

项目使用 `flatpak/org.influxdb.studio.yml` 作为构建清单：

```yaml
app-id: org.influxdb.studio
runtime: org.gnome.Platform
runtime-version: '47'
sdk: org.gnome.Sdk
command: influxdb-studio
finish-args:
  - --share=network
  - --share=ipc
  - --socket=fallback-x11
  - --socket=wayland
  - --filesystem=home
  - --device=dri
modules:
  - name: influxdb-studio
    buildsystem: simple
    build-commands:
      - cp -r src-tauri/target/release/bundle/flatpak/* /app/
      - cp -r dist/* /app/
    sources:
      - type: dir
        path: .
```

### 应用元数据

`flatpak/org.influxdb.studio.appdata.xml` 包含应用元数据：

```xml
<?xml version="1.0" encoding="UTF-8"?>
<component type="desktop-application">
  <id>org.influxdb.studio</id>
  <name>InfluxDB Studio</name>
  <summary>InfluxDB Desktop Client</summary>
  <description>
    <p>
      A modern desktop client for InfluxDB, built with Tauri and Vue.js.
      Supports InfluxDB v1.x, v2.x, and v3.x with a beautiful, intuitive interface.
    </p>
  </description>
  <url type="homepage">https://github.com/your-username/influxdb-studio</url>
  <url type="bugtracker">https://github.com/your-username/influxdb-studio/issues</url>
  <launchable type="desktop-id">org.influxdb.studio.desktop</launchable>
  <releases>
    <release version="0.1.0" date="2024-01-01"/>
  </releases>
</component>
```

### 桌面文件

`flatpak/org.influxdb.studio.desktop` 定义桌面集成：

```ini
[Desktop Entry]
Name=InfluxDB Studio
Comment=InfluxDB Desktop Client
Exec=influxdb-studio
Icon=org.influxdb.studio
Type=Application
Categories=Development;Database;
StartupWMClass=influxdb-studio
```

## 构建流程

### 本地构建

```bash
# 进入项目根目录
cd influxdb-studio

# 构建前端
pnpm build

# 构建 Rust 后端
cd src-tauri
cargo build --release
cd ..

# 构建 Flatpak 包
flatpak-builder --force-clean --repo=flatpak/repo flatpak/build flatpak/org.influxdb.studio.yml

# 创建 Flatpak 包
flatpak build-bundle flatpak/repo influxdb-studio.flatpak org.influxdb.studio
```

### 自动化构建

项目使用 GitHub Actions 进行自动化构建：

```yaml
# .github/workflows/flatpak.yml
name: Flatpak Build

on:
  push:
    tags: ['v*']
  pull_request:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        gnome-version: [47, 46, 45, 44]
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Flatpak
        uses: flatpak/flatpak-github-actions/flatpak-builder@v6
        with:
          bundle: influxdb-studio-gnome${{ matrix.gnome-version }}.flatpak
          cache-key: flatpak-builder-${{ matrix.gnome-version }}
          manifest-path: flatpak/org.influxdb.studio.yml
```

## 智能构建特性

### 环境检测

构建脚本会自动检测系统环境：

```bash
#!/bin/bash
# 检查 appstream-compose 可用性
if command -v appstream-compose &> /dev/null; then
  echo "✅ 使用标准构建流程"
  BUILD_METHOD="standard"
else
  echo "⚠️  使用备用构建流程"
  BUILD_METHOD="fallback"
fi
```

### 多版本支持

支持多个 GNOME Platform 版本：

```bash
# 智能安装 GNOME Platform 运行时
flatpak install flathub org.gnome.Platform//47 org.gnome.Sdk//47 -y || \
flatpak install flathub org.gnome.Platform//46 org.gnome.Sdk//46 -y || \
flatpak install flathub org.gnome.Platform//45 org.gnome.Sdk//45 -y || \
flatpak install flathub org.gnome.Platform//44 org.gnome.Sdk//44 -y
```

### 备用构建方案

当 `appstream-compose` 不可用时，使用备用方案：

```yaml
# 备用构建配置
- name: Fallback Build
  if: steps.check-appstream.outputs.available != 'true'
  run: |
    # 备份原始配置文件
    cp flatpak/org.influxdb.studio.yml flatpak/org.influxdb.studio.yml.backup
    
    # 修改配置避免使用 appstream-compose
    sed -i '/appstream-compose/d' flatpak/org.influxdb.studio.yml
    
    # 执行构建
    flatpak-builder --force-clean --repo=flatpak/repo flatpak/build flatpak/org.influxdb.studio.yml
    
    # 恢复原始配置
    mv flatpak/org.influxdb.studio.yml.backup flatpak/org.influxdb.studio.yml
```

## 安装和分发

### 本地安装

```bash
# 安装 Flatpak 包
flatpak install influxdb-studio.flatpak

# 运行应用
flatpak run org.influxdb.studio
```

### 远程仓库分发

```bash
# 创建远程仓库
flatpak build-repo flatpak/repo

# 添加远程仓库
flatpak remote-add --if-not-exists influxdb-studio flatpak/repo

# 安装应用
flatpak install influxdb-studio org.influxdb.studio
```

### Flathub 分发

1. 准备应用元数据
2. 提交到 Flathub 审核
3. 通过审核后自动分发

## 故障排除

### 常见问题

#### appstream-compose 缺失

**问题**: 构建时提示 `appstream-compose` 不可用

**解决方案**:
```bash
# 安装 appstream-compose
sudo apt install appstream-compose

# 或使用备用构建方案
# 构建脚本会自动检测并使用备用方案
```

#### GNOME Platform 版本不兼容

**问题**: 构建失败，提示运行时版本不兼容

**解决方案**:
```bash
# 尝试安装不同版本的 GNOME Platform
flatpak install flathub org.gnome.Platform//47 org.gnome.Sdk//47 -y || \
flatpak install flathub org.gnome.Platform//46 org.gnome.Sdk//46 -y || \
flatpak install flathub org.gnome.Platform//45 org.gnome.Sdk//45 -y || \
flatpak install flathub org.gnome.Platform//44 org.gnome.Sdk//44 -y
```

#### 权限问题

**问题**: 应用无法访问网络或文件系统

**解决方案**:
```yaml
# 在 manifest 文件中添加必要的权限
finish-args:
  - --share=network      # 网络访问
  - --share=ipc         # 进程间通信
  - --filesystem=home   # 主目录访问
  - --socket=fallback-x11  # X11 显示
  - --socket=wayland    # Wayland 显示
```

### 调试技巧

#### 构建日志

```bash
# 查看详细构建日志
flatpak-builder --verbose --force-clean flatpak/build flatpak/org.influxdb.studio.yml

# 查看构建环境
flatpak-builder --run flatpak/build flatpak/org.influxdb.studio.yml bash
```

#### 运行时调试

```bash
# 在 Flatpak 环境中运行调试
flatpak run --command=bash org.influxdb.studio

# 查看应用日志
journalctl --user -f -u org.influxdb.studio
```

#### 权限检查

```bash
# 检查应用权限
flatpak info org.influxdb.studio

# 查看沙盒状态
flatpak run --sandbox org.influxdb.studio
```

## 最佳实践

### 构建优化

1. **缓存策略**: 使用 Flatpak 构建缓存
2. **并行构建**: 利用多核 CPU 加速构建
3. **增量构建**: 避免不必要的重新构建

### 安全考虑

1. **最小权限**: 只请求必要的权限
2. **沙盒化**: 充分利用 Flatpak 沙盒
3. **更新机制**: 实现安全的自动更新

### 用户体验

1. **桌面集成**: 提供完整的桌面集成
2. **图标和主题**: 支持系统图标和主题
3. **文件关联**: 支持相关文件类型

## 发布流程

### 版本管理

1. 更新版本号
2. 更新应用元数据
3. 创建 Git 标签
4. 触发自动化构建

### 质量保证

1. **功能测试**: 验证所有功能正常工作
2. **兼容性测试**: 测试不同 Linux 发行版
3. **性能测试**: 确保性能符合要求
4. **安全测试**: 验证安全配置

### 分发策略

1. **GitHub Releases**: 提供直接下载
2. **Flathub**: 通过官方应用商店分发
3. **自定义仓库**: 维护自己的分发仓库

## 维护和更新

### 定期更新

1. **依赖更新**: 定期更新 Flatpak 运行时
2. **安全补丁**: 及时应用安全更新
3. **功能增强**: 添加新功能和改进

### 用户支持

1. **文档维护**: 保持文档最新
2. **问题跟踪**: 及时响应用户反馈
3. **社区建设**: 建立用户社区 