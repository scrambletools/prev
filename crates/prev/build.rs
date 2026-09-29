//! Rebuilds prev when its translations change. On Windows, also puts
//! prev's icon and version details into prev.exe, for Explorer, the
//! taskbar and Alt+Tab; other systems take their icons from the desktop
//! entry.

fn main() {
    println!("cargo:rerun-if-changed=../../packaging/windows/prev.ico");
    // The translations are embedded when prev compiles; a new or changed
    // language file must rebuild it.
    println!("cargo:rerun-if-changed=../../i18n");
    let windows = std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows");
    let msvc = std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc");
    if !(windows && msvc) {
        return;
    }
    let mut resource = winresource::WindowsResource::new();
    resource
        .set_icon("../../packaging/windows/prev.ico")
        .set("ProductName", "prev")
        .set("FileDescription", "prev, a document and image viewer")
        .set("CompanyName", "Scramble Tools")
        .set(
            "LegalCopyright",
            "Scramble Tools. Free software under the GNU AGPL-3.0",
        )
        .set("OriginalFilename", "prev.exe");
    if let Err(error) = resource.compile() {
        panic!("could not embed prev's icon: {error}");
    }
}
