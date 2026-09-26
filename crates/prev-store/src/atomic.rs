//! Crash-safe file replacement: write a sibling temp file, fsync, rename.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

static TEMP_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Replaces `path` with `contents` so readers see either the old or the new
/// file, never a partial one. An existing file keeps its permissions.
pub fn write(path: &Path, contents: &[u8]) -> io::Result<()> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let existing_mode = fs::metadata(path)
        .ok()
        .map(|metadata| metadata.permissions());
    let temp_path = temp_sibling(path);

    let result = (|| {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o644)
            .open(&temp_path)?;
        file.write_all(contents)?;
        if let Some(permissions) = existing_mode {
            file.set_permissions(permissions)?;
        }
        file.sync_all()?;
        fs::rename(&temp_path, path)?;
        File::open(parent)?.sync_all()
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temp_path);
    }
    result
}

fn temp_sibling(path: &Path) -> PathBuf {
    let name = path
        .file_name()
        .map(|name| name.to_string_lossy())
        .unwrap_or_default();
    let unique = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    path.with_file_name(format!(".{name}.prev-{}-{unique}.tmp", std::process::id()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn creates_and_replaces() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("doc.pdf");
        write(&path, b"first").unwrap();
        write(&path, b"second").unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"second");
        let leftovers: Vec<_> = fs::read_dir(dir.path()).unwrap().collect();
        assert_eq!(leftovers.len(), 1, "no temp files left behind");
    }

    #[test]
    fn keeps_existing_permissions() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("private.pdf");
        fs::write(&path, b"old").unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
        write(&path, b"new").unwrap();
        assert_eq!(
            fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o600
        );
    }

    #[test]
    fn failure_leaves_original_intact() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("missing-dir").join("doc.pdf");
        assert!(write(&path, b"data").is_err());
        let target = dir.path().join("target");
        fs::create_dir(&target).unwrap();
        assert!(
            write(&target, b"data").is_err(),
            "cannot replace a directory"
        );
        assert!(target.is_dir());
        assert_eq!(
            fs::read_dir(dir.path()).unwrap().count(),
            1,
            "temp file cleaned up"
        );
    }
}
