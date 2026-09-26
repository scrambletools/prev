//! Per-document page bookmarks, keyed by the document's canonical path,
//! in the file the settings name (`$XDG_DATA_HOME/prev/bookmarks.toml` by
//! default).

use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{atomic, paths};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bookmark {
    /// Zero-based page index.
    pub page: usize,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
struct DocumentBookmarks {
    path: PathBuf,
    bookmarks: Vec<Bookmark>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BookmarkStore {
    #[serde(default, rename = "document")]
    documents: Vec<DocumentBookmarks>,
}

pub fn default_path() -> Option<PathBuf> {
    paths::locations().map(|locations| locations.bookmarks)
}

impl BookmarkStore {
    /// A missing or unreadable file yields an empty store.
    pub fn load_from(path: &Path) -> Self {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|text| toml::from_str(&text).ok())
            .unwrap_or_default()
    }

    pub fn save_to(&self, path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let text = toml::to_string(self).map_err(io::Error::other)?;
        atomic::write(path, text.as_bytes())
    }

    /// Bookmarks of a document, sorted by page.
    pub fn get(&self, document: &Path) -> &[Bookmark] {
        self.documents
            .iter()
            .find(|entry| entry.path == document)
            .map_or(&[], |entry| entry.bookmarks.as_slice())
    }

    /// Adds a bookmark for `page`, or removes it if one exists. Returns
    /// whether the page is bookmarked afterwards.
    pub fn toggle(&mut self, document: &Path, page: usize, title: impl Into<String>) -> bool {
        let index = match self
            .documents
            .iter()
            .position(|entry| entry.path == document)
        {
            Some(index) => index,
            None => {
                self.documents.push(DocumentBookmarks {
                    path: document.to_path_buf(),
                    bookmarks: Vec::new(),
                });
                self.documents.len() - 1
            }
        };
        let bookmarks = &mut self.documents[index].bookmarks;
        let added = match bookmarks.iter().position(|bookmark| bookmark.page == page) {
            Some(existing) => {
                bookmarks.remove(existing);
                false
            }
            None => {
                bookmarks.push(Bookmark {
                    page,
                    title: title.into(),
                });
                bookmarks.sort_by_key(|bookmark| bookmark.page);
                true
            }
        };
        if self.documents[index].bookmarks.is_empty() {
            self.documents.remove(index);
        }
        added
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_adds_sorts_and_removes() {
        let mut store = BookmarkStore::default();
        let document = Path::new("/docs/spec.pdf");
        assert!(store.toggle(document, 9, "Page 10"));
        assert!(store.toggle(document, 2, "Scope"));
        let pages: Vec<usize> = store
            .get(document)
            .iter()
            .map(|bookmark| bookmark.page)
            .collect();
        assert_eq!(pages, [2, 9]);
        assert!(!store.toggle(document, 9, "ignored"));
        assert!(!store.toggle(document, 2, "ignored"));
        assert!(store.get(document).is_empty());
        assert_eq!(
            store,
            BookmarkStore::default(),
            "empty documents are dropped"
        );
    }

    #[test]
    fn documents_are_separate() {
        let mut store = BookmarkStore::default();
        store.toggle(Path::new("/a.pdf"), 0, "A");
        store.toggle(Path::new("/b.pdf"), 1, "B");
        assert_eq!(store.get(Path::new("/a.pdf"))[0].title, "A");
        assert_eq!(store.get(Path::new("/b.pdf"))[0].page, 1);
        assert!(store.get(Path::new("/c.pdf")).is_empty());
    }

    #[test]
    fn round_trip_and_tolerates_bad_files() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bookmarks.toml");
        assert_eq!(BookmarkStore::load_from(&path), BookmarkStore::default());
        let mut store = BookmarkStore::default();
        store.toggle(Path::new("/docs/with \"quotes\".pdf"), 4, "Five");
        store.save_to(&path).unwrap();
        assert_eq!(BookmarkStore::load_from(&path), store);
        std::fs::write(&path, "not [valid").unwrap();
        assert_eq!(BookmarkStore::load_from(&path), BookmarkStore::default());
    }
}
