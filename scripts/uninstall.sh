#!/bin/bash

ROOT="$(git rev-parse --show-toplevel)"
source $ROOT/scripts/config.sh

powershell.exe -ExecutionPolicy Bypass -File $ROOT/scripts/stop.ps1

rm -rf $INSTALL_PATH
powershell.exe -ExecutionPolicy Bypass -File $ROOT/scripts/unregister.ps1
