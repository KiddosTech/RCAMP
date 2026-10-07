$ErrorActionPreference = 'Stop'
$repo = 'KiddosTech/RCAMP'
$version = if ($env:RCAMP_VERSION) { $env:RCAMP_VERSION } else { (Invoke-RestMethod "https://api.github.com/repos/$repo/releases/latest").tag_name }
$url = "https://github.com/$repo/releases/download/$version/rcamp-windows-x86_64.zip"
$root = Join-Path $env:TEMP 'rcamp-install'
$archive = Join-Path $root 'rcamp.zip'
$bin = Join-Path $env:LOCALAPPDATA 'RCAMP\bin'
New-Item -ItemType Directory -Force $root, $bin | Out-Null
Invoke-WebRequest -Uri $url -OutFile $archive
Expand-Archive -Path $archive -DestinationPath $root -Force
Copy-Item (Join-Path $root 'rcamp.exe') (Join-Path $bin 'rcamp.exe') -Force
$userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
if (-not (($userPath -split ';') -contains $bin)) { [Environment]::SetEnvironmentVariable('Path', "$userPath;$bin", 'User') }
Write-Host "RCAMP/CLI $version installed in $bin"
Write-Host 'Open a new terminal, then run: rcamp'
