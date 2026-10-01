# mupdf-sys runs bindgen for MuPDF's bindings, and the mupdf crate needs
# max_align_t from them. clang's own stddef.h declares it for C; MSVC's
# does not. A fresh Windows build (VS 2026, LLVM 20) left it out of the
# bindings until clang's header folder came first, which this does,
# through the variable bindgen reads, for the steps after it in a GitHub
# Actions job.
$ErrorActionPreference = 'Stop'
$clang = Join-Path $env:LIBCLANG_PATH 'clang.exe'
$resource = (& $clang -print-resource-dir | Out-String).Trim()
if (-not $resource) { throw "$clang gave no resource directory" }
$include = Join-Path $resource 'include'
if (-not (Test-Path (Join-Path $include 'stddef.h'))) { throw "no stddef.h in $include" }
Write-Host "bindgen reads clang's headers from $include first"
"BINDGEN_EXTRA_CLANG_ARGS=-isystem `"$include`"" | Out-File -Append -Encoding utf8 $env:GITHUB_ENV
