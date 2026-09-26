//! The signature library: images of the user's signatures, kept for reuse
//! in `$XDG_DATA_HOME/prev/signatures/`, one PNG each plus an index with
//! their descriptions, in the order they were made.

use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::{atomic, paths};

const INDEX: &str = "index.toml";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature {
    /// File name of the PNG inside the library folder.
    pub file: String,
    /// What the user called it, such as "Full name" or "Initials".
    pub description: String,
    /// Seconds since the Unix epoch.
    pub created: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
struct Index {
    #[serde(default, rename = "signature")]
    signatures: Vec<Signature>,
}

pub struct SignatureStore {
    folder: PathBuf,
}

impl SignatureStore {
    pub fn new(folder: PathBuf) -> Self {
        Self { folder }
    }

    pub fn default_location() -> Option<Self> {
        paths::data_dir().map(|dir| Self::new(dir.join("signatures")))
    }

    fn index(&self) -> Index {
        std::fs::read_to_string(self.folder.join(INDEX))
            .ok()
            .and_then(|text| toml::from_str(&text).ok())
            .unwrap_or_default()
    }

    fn save_index(&self, index: &Index) -> io::Result<()> {
        let text = toml::to_string(index).map_err(io::Error::other)?;
        atomic::write(&self.folder.join(INDEX), text.as_bytes())
    }

    /// Signatures whose image is present, oldest first.
    pub fn list(&self) -> Vec<Signature> {
        self.index()
            .signatures
            .into_iter()
            .filter(|signature| self.folder.join(&signature.file).is_file())
            .collect()
    }

    pub fn path_of(&self, signature: &Signature) -> PathBuf {
        self.folder.join(&signature.file)
    }

    pub fn read(&self, signature: &Signature) -> io::Result<Vec<u8>> {
        std::fs::read(self.path_of(signature))
    }

    /// Stores a PNG under `description`.
    pub fn add(&self, description: &str, png: &[u8]) -> io::Result<Signature> {
        std::fs::create_dir_all(&self.folder)?;
        let mut index = self.index();
        let created = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |elapsed| elapsed.as_secs());
        let mut sequence = index.signatures.len();
        let file = loop {
            let candidate = format!("{created}-{sequence}.png");
            if !self.folder.join(&candidate).exists() {
                break candidate;
            }
            sequence += 1;
        };
        atomic::write(&self.folder.join(&file), png)?;
        let signature = Signature {
            file,
            description: description.trim().to_owned(),
            created,
        };
        index.signatures.push(signature.clone());
        self.save_index(&index)?;
        Ok(signature)
    }

    pub fn remove(&self, signature: &Signature) -> io::Result<()> {
        let mut index = self.index();
        index.signatures.retain(|kept| kept.file != signature.file);
        self.save_index(&index)?;
        match std::fs::remove_file(self.folder.join(&signature.file)) {
            Err(error) if error.kind() != io::ErrorKind::NotFound => Err(error),
            _ => Ok(()),
        }
    }

    pub fn folder(&self) -> &Path {
        &self.folder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_lists_and_removes() {
        let dir = tempfile::tempdir().unwrap();
        let store = SignatureStore::new(dir.path().join("signatures"));
        assert!(store.list().is_empty());
        let full = store.add(" Full name ", b"png one").unwrap();
        let initials = store.add("Initials", b"png two").unwrap();
        let listed = store.list();
        assert_eq!(listed, vec![full.clone(), initials.clone()]);
        assert_eq!(listed[0].description, "Full name");
        assert_eq!(store.read(&initials).unwrap(), b"png two");
        store.remove(&full).unwrap();
        assert_eq!(store.list(), vec![initials]);
    }

    #[test]
    fn missing_images_are_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let store = SignatureStore::new(dir.path().to_path_buf());
        let signature = store.add("Gone", b"png").unwrap();
        std::fs::remove_file(store.path_of(&signature)).unwrap();
        assert!(store.list().is_empty());
    }
}
