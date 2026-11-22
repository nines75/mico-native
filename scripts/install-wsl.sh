#!/bin/bash

ROOT=$(git rev-parse --show-toplevel)
source "$ROOT/scripts/config.sh"

# ビルド
cargo build --target x86_64-pc-windows-gnu --release || exit 1

# プロセス停止
pwsh.exe -Command "Stop-Process -Name $APP_NAME -ErrorAction SilentlyContinue"

# バイナリ配置
mkdir -p "$INSTALL_PATH"
cp "$ROOT/target/x86_64-pc-windows-gnu/release/$APP_NAME.exe" "$ROOT/firefox.json" "$INSTALL_PATH"

# レジストリ登録
pwsh.exe -Command "New-Item -Path $REG_PATH -Force"
pwsh.exe -Command "Set-ItemProperty -Path $REG_PATH -Name '(default)' -Value '$MANIFEST_PATH'"
