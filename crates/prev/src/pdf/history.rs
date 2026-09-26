//! Undo and redo for annotation and form edits.

use prev_pdf::annotation::{Annotation, Removed, StampContent};
use prev_pdf::worker::Edit;

#[derive(Debug, Clone)]
pub enum Change {
    Added {
        page: usize,
        annotation: Annotation,
        content: Option<StampContent>,
        /// Set once undone, to put back the very same annotation on redo.
        removed: Option<Removed>,
    },
    Removed {
        page: usize,
        annotation: Annotation,
        removed: Option<Removed>,
    },
    Updated {
        page: usize,
        before: Box<Annotation>,
        after: Box<Annotation>,
        content_before: Option<StampContent>,
        content_after: Option<StampContent>,
    },
    Field {
        page: usize,
        id: i32,
        before: String,
        after: String,
    },
}

impl Change {
    pub fn page(&self) -> usize {
        match self {
            Change::Added { page, .. }
            | Change::Removed { page, .. }
            | Change::Updated { page, .. }
            | Change::Field { page, .. } => *page,
        }
    }

    /// The edit that makes this change (again).
    pub fn forward(&self) -> Edit {
        match self {
            Change::Added {
                removed: Some(removed),
                ..
            } => Edit::Restore(removed.clone()),
            Change::Added {
                annotation,
                content,
                ..
            } => Edit::Add(annotation.clone(), content.clone()),
            Change::Removed { annotation, .. } => Edit::Remove(annotation.id.clone()),
            Change::Updated {
                after,
                content_after,
                ..
            } => Edit::Update((**after).clone(), content_after.clone()),
            Change::Field { id, after, .. } => Edit::SetField {
                id: *id,
                value: after.clone(),
            },
        }
    }

    /// The edit that takes this change back.
    pub fn backward(&self) -> Edit {
        match self {
            Change::Added { annotation, .. } => Edit::Remove(annotation.id.clone()),
            Change::Removed {
                removed: Some(removed),
                ..
            } => Edit::Restore(removed.clone()),
            // Not back from the document thread yet: add it again instead.
            Change::Removed { annotation, .. } => Edit::Add(annotation.clone(), None),
            Change::Updated {
                before,
                content_before,
                ..
            } => Edit::Update((**before).clone(), content_before.clone()),
            Change::Field { id, before, .. } => Edit::SetField {
                id: *id,
                value: before.clone(),
            },
        }
    }

    /// Keeps what a removal returned, for putting the annotation back.
    pub fn set_removed(&mut self, token: Removed) {
        match self {
            Change::Added { removed, .. } | Change::Removed { removed, .. } => {
                *removed = Some(token)
            }
            Change::Updated { .. } | Change::Field { .. } => {}
        }
    }
}

#[derive(Debug, Default)]
pub struct History {
    done: Vec<Change>,
    undone: Vec<Change>,
}

/// Which list a change moved to, to route what its edit returns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stack {
    Done,
    Undone,
}

impl History {
    pub fn record(&mut self, change: Change) {
        self.done.push(change);
        self.undone.clear();
    }

    pub fn can_undo(&self) -> bool {
        !self.done.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.undone.is_empty()
    }

    /// The edit to send, and the page, to undo the last change.
    pub fn undo(&mut self) -> Option<(usize, Edit)> {
        let change = self.done.pop()?;
        let step = (change.page(), change.backward());
        self.undone.push(change);
        Some(step)
    }

    pub fn redo(&mut self) -> Option<(usize, Edit)> {
        let change = self.undone.pop()?;
        let step = (change.page(), change.forward());
        self.done.push(change);
        Some(step)
    }

    /// Stores a removal token on the change last moved to `stack`.
    pub fn removed(&mut self, stack: Stack, token: Removed) {
        let list = match stack {
            Stack::Done => &mut self.done,
            Stack::Undone => &mut self.undone,
        };
        if let Some(change) = list.last_mut() {
            change.set_removed(token);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prev_pdf::annotation::Kind;
    use prev_pdf::geometry::Rect;

    fn square() -> Annotation {
        Annotation::new("a", Kind::Square, Rect::new(0.0, 0.0, 10.0, 10.0))
    }

    #[test]
    fn undo_and_redo_walk_the_changes() {
        let mut history = History::default();
        history.record(Change::Added {
            page: 2,
            annotation: square(),
            content: None,
            removed: None,
        });
        assert!(history.can_undo() && !history.can_redo());
        let (page, edit) = history.undo().unwrap();
        assert_eq!(page, 2);
        assert!(matches!(edit, Edit::Remove(ref id) if id == "a"));
        let (_, edit) = history.redo().unwrap();
        assert!(matches!(edit, Edit::Add(..)), "no token yet, so add again");
        assert!(history.can_undo() && !history.can_redo());
    }

    #[test]
    fn new_changes_clear_redo() {
        let mut history = History::default();
        let field = |value: &str| Change::Field {
            page: 0,
            id: 7,
            before: String::new(),
            after: value.into(),
        };
        history.record(field("one"));
        history.undo();
        assert!(history.can_redo());
        history.record(field("two"));
        assert!(!history.can_redo());
    }
}
