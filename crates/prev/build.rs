//! Rebuilds prev when its translations change, and records the commit
//! and time it is built from for its version line. On Windows, also puts
//! prev's icon and version details into prev.exe, for Explorer, the
//! taskbar and Alt+Tab; other systems take their icons from the desktop
//! entry.

/// The commit prev is built from, with `+` when the working tree has
/// changes: from git, or from a `BUILD` file beside the workspace for
/// copies of the source without `.git`.
fn commit() -> String {
    let git = |args: &[&str]| {
        std::process::Command::new("git")
            .args(args)
            .current_dir("../..")
            .output()
            .ok()
            .filter(|output| output.status.success())
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
    };
    match git(&["rev-parse", "--short=7", "HEAD"]) {
        Some(hash) => {
            let dirty = git(&["status", "--porcelain", "--untracked-files=no"])
                .is_some_and(|status| !status.is_empty());
            format!("{hash}{}", if dirty { "+" } else { "" })
        }
        None => std::fs::read_to_string("../../BUILD")
            .map(|text| text.trim().to_owned())
            .unwrap_or_else(|_| "unknown".to_owned()),
    }
}

/// The time of the build, in UTC, as "YYYY-MM-DD HH:MM".
fn built_at() -> String {
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |since| since.as_secs() as i64);
    let (days, rest) = (seconds.div_euclid(86_400), seconds.rem_euclid(86_400));
    // Days since 1970 to a civil date (Howard Hinnant's algorithm).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02} {:02}:{:02}",
        rest / 3600,
        rest % 3600 / 60
    )
}

fn main() {
    println!("cargo:rustc-env=PREV_COMMIT={}", commit());
    println!("cargo:rustc-env=PREV_BUILT={}", built_at());
    // So the commit and time follow every change, not only these files.
    for path in [
        "../../.git/HEAD",
        "../../.git/index",
        "../../BUILD",
        "../../crates",
        "../../vendor",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
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
