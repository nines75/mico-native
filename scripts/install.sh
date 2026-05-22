#!/bin/bash

ROOT="$(git rev-parse --show-toplevel)"
source $ROOT/scripts/config.sh

cargo build --target x86_64-pc-windows-gnu --release || exit 1

powershell.exe -ExecutionPolicy Bypass -File $ROOT/scripts/stop.ps1

mkdir -p $INSTALL_PATH
cp $ROOT/target/x86_64-pc-windows-gnu/release/mico-native.exe $ROOT/mico.native.json $INSTALL_PATH

powershell.exe -ExecutionPolicy Bypass -File $ROOT/scripts/register.ps1
