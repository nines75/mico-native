#!/bin/bash

ROOT="$(git rev-parse --show-toplevel)"
source $ROOT/scripts/config.sh

# プロセス停止
powershell.exe -Command "Stop-Process -Name 'mico-native' -ErrorAction SilentlyContinue"

# バイナリ削除
rm -rf $INSTALL_PATH

# レジストリ削除
powershell.exe -Command "if (Test-Path $REG_PATH) { Remove-Item -Path $REG_PATH -Recurse }"
