# mupdf-sys builds MuPDF with the v143 toolset when it finds Visual Studio
# 2022 and falls back to v142 for any other version. v142 cannot build
# ARM64 against current Windows SDKs (winnt.h needs its _CountOneBits64
# intrinsic), and runners now come with Visual Studio 2026. This picks the
# toolset of the newest Visual Studio installed, through the variable
# mupdf-sys reads, for the steps after it in a GitHub Actions job.
$ErrorActionPreference = 'Stop'
$vswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe'
$version = & $vswhere -latest -products * -property installationVersion
if (-not $version) { throw 'vswhere found no Visual Studio' }
$toolset = switch ([int]($version.Split('.')[0])) {
    17 { 'v143' }
    18 { 'v145' }
    default { throw "Visual Studio $version has no known MSVC toolset; add it here" }
}
Write-Host "Visual Studio $version, MSVC toolset $toolset"
"MUPDF_MSVC_PLATFORM_TOOLSET=$toolset" | Out-File -Append -Encoding utf8 $env:GITHUB_ENV
