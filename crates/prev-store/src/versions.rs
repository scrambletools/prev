//! Saved versions of documents, for "Revert To". Each document gets a
//! folder under `$XDG_DATA_HOME/prev/versions/`, named by a hash of its
//! path, holding copies of earlier contents and an index.

use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::{atomic, paths};

const INDEX: &str = "index.toml";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Version {
    /// File name inside the document's version folder.
    pub file: String,
    /// Seconds since the Unix epoch.
    pub saved_at: u64,
    pub size: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
struct Index {
    path: PathBuf,
    #[serde(default, rename = "version")]
    versions: Vec<Version>,
}

pub struct VersionStore {
    root: PathBuf,
}

/// FNV-1a: stable across Rust releases, unlike the standard hasher.
fn stable_hash(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs())
}

impl VersionStore {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn default_location() -> Option<Self> {
        paths::data_dir().map(|dir| Self::new(dir.join("versions")))
    }

    fn folder(&self, document: &Path) -> PathBuf {
        use std::os::unix::ffi::OsStrExt;
        self.root.join(format!(
            "{:016x}",
            stable_hash(document.as_os_str().as_bytes())
        ))
    }

    fn load_index(&self, document: &Path) -> Index {
        let index = std::fs::read_to_string(self.folder(document).join(INDEX))
            .ok()
            .and_then(|text| toml::from_str::<Index>(&text).ok())
            .filter(|index| index.path == document);
        index.unwrap_or_else(|| Index {
            path: document.to_path_buf(),
            versions: Vec::new(),
        })
    }

    fn save_index(&self, document: &Path, index: &Index) -> io::Result<()> {
        let text = toml::to_string(index).map_err(io::Error::other)?;
        atomic::write(&self.folder(document).join(INDEX), text.as_bytes())
    }

    /// Versions of `document`, newest first.
    pub fn list(&self, document: &Path) -> Vec<Version> {
        let mut versions = self.load_index(document).versions;
        versions.reverse();
        versions
    }

    pub fn contents(&self, document: &Path, version: &Version) -> io::Result<Vec<u8>> {
        std::fs::read(self.folder(document).join(&version.file))
    }

    /// Stores the document's current contents as a new version.
    pub fn keep(&self, document: &Path) -> io::Result<Version> {
        let contents = std::fs::read(document)?;
        let folder = self.folder(document);
        std::fs::create_dir_all(&folder)?;
        let mut index = self.load_index(document);
        let saved_at = now();
        let extension = document
            .extension()
            .map(|ext| format!(".{}", ext.to_string_lossy()))
            .unwrap_or_default();
        let mut sequence = index.versions.len();
        let file = loop {
            let candidate = format!("{saved_at}-{sequence}{extension}");
            if !folder.join(&candidate).exists() {
                break candidate;
            }
            sequence += 1;
        };
        atomic::write(&folder.join(&file), &contents)?;
        let version = Version {
            file,
            saved_at,
            size: contents.len() as u64,
        };
        index.versions.push(version.clone());
        self.save_index(document, &index)?;
        Ok(version)
    }

    /// Replaces the document with `version`, keeping its current contents
    /// as a version first so the revert can itself be undone.
    pub fn restore(&self, document: &Path, version: &Version) -> io::Result<()> {
        let contents = self.contents(document, version)?;
        self.keep(document)?;
        atomic::write(document, &contents)
    }

    /// Deletes every version of `document`, for content that must not
    /// survive anywhere, such as redacted text.
    pub fn forget(&self, document: &Path) -> io::Result<()> {
        let folder = self.folder(document);
        let index = self.load_index(document);
        for version in &index.versions {
            match std::fs::remove_file(folder.join(&version.file)) {
                Err(error) if error.kind() != io::ErrorKind::NotFound => return Err(error),
                _ => {}
            }
        }
        match std::fs::remove_file(folder.join(INDEX)) {
            Err(error) if error.kind() != io::ErrorKind::NotFound => return Err(error),
            _ => {}
        }
        let _ = std::fs::remove_dir(&folder);
        Ok(())
    }

    /// Deletes versions older than `max_age`, then the oldest ones until all
    /// documents together use at most `max_bytes`.
    pub fn prune(&self, max_age: Duration, max_bytes: u64) -> io::Result<()> {
        let Ok(entries) = std::fs::read_dir(&self.root) else {
            return Ok(());
        };
        let cutoff = now().saturating_sub(max_age.as_secs());
        let mut all: Vec<(PathBuf, Index)> = entries
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let text = std::fs::read_to_string(entry.path().join(INDEX)).ok()?;
                Some((entry.path(), toml::from_str::<Index>(&text).ok()?))
            })
            .collect();
        let mut remaining: Vec<(u64, usize, usize)> = Vec::new();
        for (folder_index, (folder, index)) in all.iter_mut().enumerate() {
            index.versions.retain(|version| {
                let keep = version.saved_at >= cutoff;
                if !keep {
                    let _ = std::fs::remove_file(folder.join(&version.file));
                }
                keep
            });
            for (version_index, version) in index.versions.iter().enumerate() {
                remaining.push((version.saved_at, folder_index, version_index));
            }
        }
        let mut total: u64 = all
            .iter()
            .flat_map(|(_, index)| &index.versions)
            .map(|version| version.size)
            .sum();
        remaining.sort();
        let mut dropped: Vec<(usize, usize)> = Vec::new();
        for (_, folder_index, version_index) in remaining {
            if total <= max_bytes {
                break;
            }
            let (folder, index) = &all[folder_index];
            let version = &index.versions[version_index];
            let _ = std::fs::remove_file(folder.join(&version.file));
            total = total.saturating_sub(version.size);
            dropped.push((folder_index, version_index));
        }
        dropped.sort_by(|a, b| b.cmp(a));
        for (folder_index, version_index) in dropped {
            all[folder_index].1.versions.remove(version_index);
        }
        for (folder, index) in &all {
            if index.versions.is_empty() {
                let _ = std::fs::remove_dir_all(folder);
            } else {
                let text = toml::to_string(index).map_err(io::Error::other)?;
                atomic::write(&folder.join(INDEX), text.as_bytes())?;
            }
        }
        Ok(())
    }
}

/// "just now", "5 minutes ago", "3 hours ago", "2 days ago".
pub fn describe_age(saved_at: u64) -> String {
    let seconds = now().saturating_sub(saved_at);
    let (count, unit) = match seconds {
        0..60 => return "just now".into(),
        60..3600 => (seconds / 60, "minute"),
        3600..86_400 => (seconds / 3600, "hour"),
        _ => (seconds / 86_400, "day"),
    };
    format!("{count} {unit}{} ago", if count == 1 { "" } else { "s" })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (tempfile::TempDir, VersionStore, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let store = VersionStore::new(dir.path().join("versions"));
        let document = dir.path().join("photo.jpg");
        std::fs::write(&document, b"first").unwrap();
        (dir, store, document)
    }

    #[test]
    fn keeps_lists_and_restores() {
        let (_dir, store, document) = setup();
        let first = store.keep(&document).unwrap();
        std::fs::write(&document, b"second").unwrap();
        store.keep(&document).unwrap();
        std::fs::write(&document, b"third").unwrap();

        let versions = store.list(&document);
        assert_eq!(versions.len(), 2);
        assert_eq!(
            store.contents(&document, &versions[0]).unwrap(),
            b"second",
            "newest first"
        );
        assert!(first.file.ends_with(".jpg"));

        store.restore(&document, &first).unwrap();
        assert_eq!(std::fs::read(&document).unwrap(), b"first");
        let versions = store.list(&document);
        assert_eq!(
            store.contents(&document, &versions[0]).unwrap(),
            b"third",
            "the revert is undoable"
        );
    }

    #[test]
    fn forgetting_deletes_every_version() {
        let (dir, store, document) = setup();
        let first = store.keep(&document).unwrap();
        store.keep(&document).unwrap();
        store.forget(&document).unwrap();
        assert!(store.list(&document).is_empty());
        assert!(store.contents(&document, &first).is_err());
        assert!(
            !dir.path()
                .join("versions")
                .join(store.folder(&document))
                .exists()
        );
        // Nothing to forget is fine too.
        store.forget(&document).unwrap();
    }

    #[test]
    fn documents_do_not_mix() {
        let (dir, store, document) = setup();
        let other = dir.path().join("other.jpg");
        std::fs::write(&other, b"other").unwrap();
        store.keep(&document).unwrap();
        assert!(store.list(&other).is_empty());
        store.keep(&other).unwrap();
        assert_eq!(store.list(&document).len(), 1);
    }

    #[test]
    fn prunes_by_size_oldest_first() {
        let (_dir, store, document) = setup();
        for contents in [&b"aaaa"[..], b"bbbb", b"cccc"] {
            std::fs::write(&document, contents).unwrap();
            store.keep(&document).unwrap();
        }
        store.prune(Duration::from_secs(3600), 8).unwrap();
        let versions = store.list(&document);
        assert_eq!(versions.len(), 2);
        assert_eq!(
            store.contents(&document, &versions[1]).unwrap(),
            b"bbbb",
            "oldest removed"
        );
        store.prune(Duration::from_secs(3600), 0).unwrap();
        assert!(store.list(&document).is_empty());
    }

    #[test]
    fn ages() {
        assert_eq!(describe_age(now()), "just now");
        assert_eq!(describe_age(now() - 120), "2 minutes ago");
        assert_eq!(describe_age(now() - 3600), "1 hour ago");
        assert_eq!(describe_age(now() - 3 * 86_400), "3 days ago");
    }
}
