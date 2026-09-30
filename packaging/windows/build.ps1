# Packages a Windows build of prev: the MSI installer and a portable zip.
#
#   packaging\windows\build.ps1 -Version 1.1.0 -Source dist -Out out [-Arch arm64]
#
# Source holds prev.exe and heif.dll, which has libheif's decoders linked
# in (packaging/windows/vcpkg). Needs WiX 5 (dotnet tool install --global
# wix --version 5.0.2).
param(
    [Parameter(Mandatory)] [string] $Version,
    [Parameter(Mandatory)] [string] $Source,
    [Parameter(Mandatory)] [string] $Out,
    [ValidateSet('x64', 'arm64')] [string] $Arch = 'x64'
)
$ErrorActionPreference = 'Stop'
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
New-Item -ItemType Directory -Force $Out | Out-Null

$msi = Join-Path $Out "prev-$Version-$Arch.msi"
wix build (Join-Path $here 'prev.wxs') -arch $Arch -d "Version=$Version" -d "Source=$((Resolve-Path $Source).Path)" -o $msi
if ($LASTEXITCODE -ne 0) { throw "wix build failed" }
# WiX's debug symbols are no use to people installing prev.
Remove-Item (Join-Path $Out '*.wixpdb') -ErrorAction SilentlyContinue

$zip = Join-Path $Out "prev-$Version-$Arch-windows.zip"
$stage = Join-Path ([System.IO.Path]::GetTempPath()) "prev-$Version-$Arch"
Remove-Item -Recurse -Force $stage -ErrorAction SilentlyContinue
New-Item -ItemType Directory $stage | Out-Null
Copy-Item (Join-Path $Source '*') $stage
Copy-Item (Join-Path $here '..\..\LICENSE') $stage
Copy-Item (Join-Path $here '..\..\docs\THIRD-PARTY.md') $stage
Compress-Archive -Path (Join-Path $stage '*') -DestinationPath $zip -Force

Get-ChildItem $Out
