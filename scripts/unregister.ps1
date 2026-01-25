$Name = "mico.native"
$RegPath = "HKCU:\Software\Mozilla\NativeMessagingHosts\$Name"

if (Test-Path $RegPath) {
    Remove-Item -Path $RegPath -Recurse
} 