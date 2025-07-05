#!/bin/bash

# PDF文件重命名工具启动脚本
# 这个脚本将以GUI模式启动应用程序，不会显示终端窗口

APP_PATH="src-tauri/target/release/bundle/macos/InvoicePilot.app"

if [ -d "$APP_PATH" ]; then
    echo "正在启动PDF文件重命名工具..."
    open "$APP_PATH"
    echo "应用程序已启动！"
else
    echo "错误：找不到应用程序文件"
    echo "请先运行 'pnpm tauri build' 编译应用程序"
    exit 1
fi 