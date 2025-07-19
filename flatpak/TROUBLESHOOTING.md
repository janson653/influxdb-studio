# Flatpak 构建故障排除

## 常见问题

### 1. appstream-compose 错误

**错误信息**：
```
bwrap: execvp appstream-compose: No such file or directory
Error: ERROR: appstream-compose failed: Child process exited with code 1
```

**原因**：系统中缺少 `appstream-compose` 工具，这是 Flatpak 构建过程中处理 AppStream 元数据所需的工具。

**解决方案**：

#### 方案一：安装 appstream-compose（推荐）

根据你的 Linux 发行版运行以下命令：

**Ubuntu/Debian**：
```bash
sudo apt update
sudo apt install appstream-compose
```

**Fedora**：
```bash
sudo dnf install appstream-compose
```

**Arch Linux**：
```bash
sudo pacman -S appstream-compose
```

**openSUSE**：
```bash
sudo zypper install appstream-compose
```

#### 方案二：使用自动修复脚本

运行项目提供的自动修复脚本：
```bash
./scripts/fix-flatpak-build.sh
```

这个脚本会自动检测你的系统并尝试安装 `appstream-compose`。

#### 方案三：使用简化构建脚本

如果无法安装 `appstream-compose`，可以使用简化版本的构建脚本：
```bash
./scripts/build-flatpak-simple.sh
```

这个脚本直接复制 metainfo 文件，避免使用 `appstream-compose`。

#### 方案四：使用智能构建脚本（推荐）

使用智能构建脚本，它会自动检测环境并选择最佳策略：
```bash
./scripts/build-flatpak-smart.sh
```

这个脚本会：
- 自动检测 `appstream-compose` 是否可用
- 如果可用，使用标准构建流程
- 如果不可用，自动修改配置使用备用方案

#### 方案五：手动修改配置

修改 `flatpak/com.influxdb.studio.yml` 文件，将：
```yaml
- install -D com.influxdb.studio.metainfo.xml /app/share/metainfo/
```

改为：
```yaml
- install -D com.influxdb.studio.metainfo.xml /app/share/metainfo/com.influxdb.studio.appdata.xml
```

## 其他常见问题

### 2. 权限问题

如果遇到权限错误，确保：
- 脚本有执行权限：`chmod +x scripts/*.sh`
- 有足够的磁盘空间
- 有 sudo 权限（用于安装依赖）

### 3. 网络问题

如果下载 Flatpak 运行时失败：
```bash
# 检查网络连接
ping flathub.org

# 重新配置 Flathub 仓库
flatpak remote-remove flathub
flatpak remote-add flathub https://flathub.org/repo/flathub.flatpakrepo
flatpak update
```

### 4. 运行时版本问题

如果遇到运行时版本不兼容：
```bash
# 查看可用的运行时版本
flatpak remote-ls flathub | grep org.gnome.Platform

# 安装特定版本
flatpak install flathub org.gnome.Platform//45 org.gnome.Sdk//45
```

## 构建流程

1. **检查依赖**：确保所有必要的工具已安装
2. **构建 Tauri 应用**：编译 Rust 后端和前端
3. **构建 Flatpak 包**：创建 Flatpak 应用程序包
4. **清理**：清理临时构建文件

## 验证构建结果

构建成功后，你应该看到：
- `flatpak/influxdb-studio.flatpak` 文件
- 文件大小应该在 10-50MB 之间

安装和测试：
```bash
# 安装 Flatpak 包
flatpak install influxdb-studio.flatpak

# 运行应用
flatpak run com.influxdb.studio

# 卸载应用
flatpak uninstall com.influxdb.studio
```

## 获取帮助

如果遇到其他问题：
1. 查看构建日志中的详细错误信息
2. 检查 Flatpak 文档：https://docs.flatpak.org/
3. 在项目 Issues 中报告问题 