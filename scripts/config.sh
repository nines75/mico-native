USER_PROFILE=$(powershell.exe -Command '$env:UserProfile' | tr -d "\r")
USER_PROFILE_WSL=$(wslpath $USER_PROFILE)

INSTALL_PATH=$USER_PROFILE_WSL/mico.native
MANIFEST_PATH="$USER_PROFILE\mico.native\mico.native.json"
REG_PATH="HKCU:\Software\Mozilla\NativeMessagingHosts\mico.native"
