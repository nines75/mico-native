HOME_PATH=$(pwsh.exe -Command '$env:UserProfile' | tr -d "\r")
HOME_PATH_WSL=$(wslpath "$HOME_PATH")

APP_NAME=mico-native
REG_NAME=mico.native

INSTALL_PATH=$HOME_PATH_WSL/$REG_NAME
MANIFEST_PATH="$HOME_PATH\\$REG_NAME\firefox.json"
REG_PATH="HKCU:\Software\Mozilla\NativeMessagingHosts\\$REG_NAME"
