# prev's libheif: one heif.dll with libde265 and aom's decoder linked in,
# release builds only, and no encoders, which prev never uses.
set(VCPKG_TARGET_ARCHITECTURE arm64)
set(VCPKG_CRT_LINKAGE dynamic)
set(VCPKG_LIBRARY_LINKAGE static)
set(VCPKG_BUILD_TYPE release)

if(PORT STREQUAL "libheif")
    set(VCPKG_LIBRARY_LINKAGE dynamic)
    set(VCPKG_CMAKE_CONFIGURE_OPTIONS -DWITH_AOM_ENCODER=OFF)
elseif(PORT STREQUAL "aom")
    set(VCPKG_CMAKE_CONFIGURE_OPTIONS -DCONFIG_AV1_ENCODER=0)
endif()
