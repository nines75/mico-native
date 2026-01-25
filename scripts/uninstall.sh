#!/bin/bash

ROOT="$(git rev-parse --show-toplevel)"
source $ROOT/scripts/config.sh

rm -rf $INSTALL_PATH
powershell.exe -ExecutionPolicy Bypass -File $ROOT/scripts/unregister.ps1
