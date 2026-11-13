USER_PROFILE=$(powershell.exe -Command '$env:UserProfile' | tr -d "\r")
USER_PROFILE_WSL=$(wslpath $USER_PROFILE)

APP_NAME=mico-native
REG_NAME=mico.native

INSTALL_PATH=$USER_PROFILE_WSL/$REG_NAME
MANIFEST_PATH="$USER_PROFILE\\$REG_NAME\firefox.json"
REG_PATH="HKCU:\Software\Mozilla\NativeMessagingHosts\\$REG_NAME"
