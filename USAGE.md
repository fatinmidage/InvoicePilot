# InvoicePilot - 使用说明

## 🚀 如何启动应用程序

### ✅ 推荐方式

#### 方法1: 使用启动脚本（最简单）

```bash
./launch_app.sh
```

#### 方法2: 在Finder中双击

1. 打开Finder
2. 导航到: `src-tauri/target/release/bundle/macos/`
3. 双击 `InvoicePilot.app`

#### 方法3: 使用终端命令

```bash
open src-tauri/target/release/bundle/macos/InvoicePilot.app
```

#### 方法4: 安装DMG文件

```bash
# 双击安装DMG文件
open src-tauri/target/release/bundle/dmg/InvoicePilot_0.1.0_aarch64.dmg
```

### ❌ 避免的方式

**不要直接运行可执行文件，这会显示终端窗口：**

```bash
# ❌ 不要这样做
./src-tauri/target/release/InvoicePilot
```

## 🔧 开发相关

### 编译应用程序

```bash
pnpm tauri build
```

### 开发模式运行

```bash
pnpm tauri dev
```

## 📁 文件结构

```shell
InvoicePilot/
├── launch_app.sh                    # 启动脚本
├── src-tauri/target/release/
│   ├── InvoicePilot                # 可执行文件（不要直接运行）
│   └── bundle/
│       ├── macos/
│       │   └── InvoicePilot.app    # macOS应用程序（推荐）
│       └── dmg/
│           └── InvoicePilot_0.1.0_aarch64.dmg  # 安装包
```

## 🎯 总结

- **GUI模式**: 双击 `.app` 文件或使用启动脚本
- **避免终端窗口**: 不要直接运行可执行文件
- **分发**: 使用 `.dmg` 文件进行分发

如果您仍然看到终端窗口，请确保您使用的是上述推荐的启动方式之一。
