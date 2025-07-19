# InfluxDB Studio Flatpak 构建指南

## 概述

本项目支持构建 Flatpak 包，提供更好的 Linux 发行版兼容性和沙盒化运行环境。

## 优势

- **跨发行版兼容**：一次构建，支持所有主流 Linux 发行版
- **沙盒化运行**：提供更好的安全隔离
- **自动依赖管理**：无需手动安装系统依赖
- **用户友好**：简化安装和更新流程

## 本地构建

### 前置要求

1. 安装 Flatpak：
```bash
# Ubuntu/Debian
sudo apt install flatpak

# Fedora
sudo dnf install flatpak

# Arch Linux
sudo pacman -S flatpak
```

2. 安装 Flatpak 构建工具：
```bash
sudo apt install flatpak-builder  # Ubuntu/Debian
sudo dnf install flatpak-builder  # Fedora
sudo pacman -S flatpak-builder    # Arch Linux
```

3. 安装 GNOME Platform 运行时：
```bash
flatpak install org.gnome.Platform//45 org.gnome.Sdk//45
```

### 构建步骤

1. 使用构建脚本（推荐）：
```bash
chmod +x scripts/build-flatpak.sh
./scripts/build-flatpak.sh
```

2. 手动构建：
```bash
# 构建 Tauri 应用
pnpm install
pnpm run build
cd src-tauri
cargo build --release --target x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/influxdb-studio ../flatpak/
cd ..

# 构建 Flatpak 包
cd flatpak
flatpak-builder --force-clean --repo=repo build com.influxdb.studio.yml
flatpak build-bundle repo influxdb-studio.flatpak com.influxdb.studio
cd ..
```

### 安装和测试

```bash
# 安装 Flatpak 包
flatpak install influxdb-studio.flatpak

# 运行应用
flatpak run com.influxdb.studio

# 卸载应用
flatpak uninstall com.influxdb.studio
```

## CI/CD 集成

项目已配置 GitHub Actions 自动构建 Flatpak 包：

- 当推送标签时自动构建
- 仅 Linux 平台构建 Flatpak 包
- 自动上传到 GitHub Releases

## 发布到 Flathub

### 准备工作

1. 确保应用符合 [Flathub 要求](https://github.com/flathub/flathub/wiki/App-Requirements)
2. 完善应用元数据（`com.influxdb.studio.metainfo.xml`）
3. 测试应用在沙盒环境中的运行

### 提交流程

1. Fork [Flathub 仓库](https://github.com/flathub/flathub)
2. 创建应用目录：`apps/com.influxdb.studio/`
3. 复制配置文件到该目录
4. 提交 Pull Request

### 配置文件说明

- `com.influxdb.studio.yml`：应用构建配置
- `com.influxdb.studio.desktop`：桌面入口文件
- `com.influxdb.studio.metainfo.xml`：应用元数据
- `icons/`：应用图标文件

## 权限配置

当前配置的权限包括：

- `--share=network`：网络访问
- `--share=ipc`：进程间通信
- `--socket=fallback-x11`：X11 显示
- `--socket=wayland`：Wayland 显示
- `--filesystem=home`：用户主目录访问
- `--device=dri`：硬件加速
- `--socket=pulseaudio`：音频支持
- `--socket=session-bus`：会话总线
- `--socket=system-bus`：系统总线

## 故障排除

### 常见问题

1. **应用无法启动**：
   - 检查权限配置是否合适
   - 查看 Flatpak 日志：`flatpak logs com.influxdb.studio`

2. **网络连接失败**：
   - 确保包含 `--share=network` 权限
   - 检查防火墙设置

3. **文件访问权限**：
   - 根据需要调整 `--filesystem` 权限
   - 考虑使用 `--filesystem=xdg-documents` 等具体路径

### 调试模式

```bash
# 以调试模式运行
flatpak run --devel com.influxdb.studio

# 查看详细日志
flatpak run --log=verbose com.influxdb.studio
```

## 更新维护

1. **版本更新**：
   - 更新 `com.influxdb.studio.metainfo.xml` 中的版本信息
   - 更新 `com.influxdb.studio.yml` 中的运行时版本（如需要）

2. **依赖更新**：
   - 检查 GNOME Platform 运行时版本兼容性
   - 更新 `finish-args` 中的权限配置

3. **图标更新**：
   - 替换 `icons/` 目录中的图标文件
   - 确保包含所有必要的尺寸（16x16, 32x32, 64x64, 128x128） 