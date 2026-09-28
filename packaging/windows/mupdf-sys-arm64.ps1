# mupdf-sys 0.8 builds MuPDF with MSBuild only for x86 and x64, though
# MuPDF's Visual Studio solution has an ARM64 platform too. This lets it
# build for ARM64. Run it after `cargo fetch` and before building for
# aarch64-pc-windows-msvc; running it again is fine.
$ErrorActionPreference = 'Stop'
$cargoHome = if ($env:CARGO_HOME) { $env:CARGO_HOME } else { Join-Path $HOME '.cargo' }
$crates = @(Get-ChildItem -Directory (Join-Path $cargoHome 'registry\src\*\mupdf-sys-0.8.0'))
if ($crates.Count -eq 0) { throw 'mupdf-sys 0.8.0 is not in the cargo registry; run cargo fetch first' }

# Each edit: the file, the text to replace and its replacement.
$edits = @(
    # The build script: the ARM64 platform, and its library folder, which
    # MSBuild names after the platform for everything but Win32.
    @('msbuild.rs', '"x86_64" => "x64",', "`"x86_64`" => `"x64`",`n            `"aarch64`" => `"ARM64`","),
    @('msbuild.rs', 'if platform == "x64" {', 'if platform != "Win32" {'),
    @('msbuild.rs', 'platform/win32/x64/{configuration}', 'platform/win32/{platform}/{configuration}'),
    # bin2coff, which embeds MuPDF's fonts and other resources, puts each
    # one's 4-byte size straight after its data; ARM64 cannot load it from
    # an unaligned place, so the data is padded to a multiple of 4.
    @('mupdf\scripts\bin2coff.c', 'size_t size, alloc_size;', 'size_t size, real_size, alloc_size;'),
    @('mupdf\scripts\bin2coff.c', "size = (size_t)ftell(fd);`n", "size = (size_t)ftell(fd);`n`treal_size = size;`n`tsize = (size + 3) & ~(size_t)3;`n"),
    @('mupdf\scripts\bin2coff.c', '1, size, fd) != size)', '1, real_size, fd) != real_size)'),
    @('mupdf\scripts\bin2coff.c', '*data_size = (SIZE_TYPE)size;', '*data_size = (SIZE_TYPE)real_size;')
)
foreach ($crate in $crates) {
    foreach ($name in ($edits | ForEach-Object { $_[0] } | Select-Object -Unique)) {
        $path = Join-Path $crate.FullName $name
        $text = [IO.File]::ReadAllText($path)
        # MuPDF's sources use either line ending.
        $crlf = $text.Contains("`r`n")
        if ($crlf) { $text = $text.Replace("`r`n", "`n") }
        foreach ($edit in ($edits | Where-Object { $_[0] -eq $name })) {
            if ($text.Contains($edit[2])) { continue }
            if (-not $text.Contains($edit[1])) { throw "$path has changed; update this patch" }
            $text = $text.Replace($edit[1], $edit[2])
        }
        if ($crlf) { $text = $text.Replace("`n", "`r`n") }
        [IO.File]::WriteAllText($path, $text)
        Write-Host "patched $path"
    }
}
