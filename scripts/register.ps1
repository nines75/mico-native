$Name = "mico.native"
$ManifestPath = Join-Path $env:USERPROFILE "$Name\$Name.json"
$RegPath = "HKCU:\Software\Mozilla\NativeMessagingHosts\$Name"

New-Item -Path $RegPath -Force # Force: 再帰的にフォルダを作り、存在した場合上書き
Set-ItemProperty -Path $RegPath -Name "(default)" -Value $ManifestPath