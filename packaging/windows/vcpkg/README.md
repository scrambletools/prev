# libheif for Windows

prev.exe opens HEIC and AVIF through a `heif.dll` beside it, built by
vcpkg from this folder:

```powershell
vcpkg install --x-manifest-root=packaging\windows\vcpkg --x-install-root=vcpkg_installed --triplet x64-windows-prev
```

The DLL is then `vcpkg_installed\x64-windows-prev\bin\heif.dll` (use
`arm64-windows-prev` for ARM64). The triplets link libde265 and aom's
AV1 decoder into it, leave out every encoder, and build release only.
`vcpkg-configuration.json` pins the vcpkg ports, and with them the
versions of libheif, libde265 and aom; move its `baseline` to a newer
vcpkg commit to update them.
