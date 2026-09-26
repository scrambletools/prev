//! Undo and redo for annotation, form and page edits. Changes are undone
//! strictly in order, so the page numbers each one holds are right again
//! by the time it is undone.

use std::sync::Arc;

use prev_pdf::annotation::{Annotation, Removed, StampContent};
use prev_pdf::engine::{CropBox, RemovedPage};
use prev_pdf::geometry::{Rect, Size};
use prev_pdf::pages;
use prev_pdf::worker::{Edit, PageEdit, PageOutcome};

/// A change to the document's pages, with what undoing it needs.
#[derive(Debug, Clone)]
pub enum PageChange {
    Rotated {
        pages: Vec<usize>,
        quarter_turns: i32,
    },
    Reordered(Vec<usize>),
    Removed {
        pages: Vec<usize>,
        /// Set once the document thread has the pages, to put them back.
        removed: Option<Vec<(usize, RemovedPage)>>,
    },
    Inserted {
        at: usize,
        /// Set once the pages are in.
        count: usize,
        source: Insertion,
        /// Set once undone, to put the very same pages back on redo.
        removed: Option<Vec<(usize, RemovedPage)>>,
    },
    Cropped {
        pages: Vec<usize>,
        rect: Rect,
        /// The crop boxes the pages had, once cropped.
        before: Option<Vec<(usize, CropBox)>>,
    },
}

/// Where inserted pages come from.
#[derive(Debug, Clone)]
pub enum Insertion {
    Blank(Size),
    Document(Arc<Vec<u8>>),
}

impl PageChange {
    /// The edit that makes this change (again).
    pub fn forward(&self) -> PageEdit {
        match self {
            PageChange::Rotated {
                pages,
                quarter_turns,
            } => PageEdit::Rotate {
                pages: pages.clone(),
                quarter_turns: *quarter_turns,
            },
            PageChange::Reordered(order) => PageEdit::Reorder(order.clone()),
            PageChange::Removed { pages, .. } => PageEdit::Remove(pages.clone()),
            PageChange::Inserted {
                removed: Some(removed),
                ..
            } => PageEdit::Restore(removed.clone()),
            PageChange::Inserted { at, source, .. } => match source {
                Insertion::Blank(size) => PageEdit::InsertBlank {
                    at: *at,
                    size: *size,
                },
                Insertion::Document(bytes) => PageEdit::Insert {
                    at: *at,
                    bytes: Arc::clone(bytes),
                },
            },
            PageChange::Cropped { pages, rect, .. } => PageEdit::Crop {
                pages: pages.clone(),
                rect: *rect,
            },
        }
    }

    /// The edit that takes this change back, if the document thread has
    /// answered with what that needs.
    pub fn backward(&self) -> Option<PageEdit> {
        Some(match self {
            PageChange::Rotated {
                pages,
                quarter_turns,
            } => PageEdit::Rotate {
                pages: pages.clone(),
                quarter_turns: -*quarter_turns,
            },
            PageChange::Reordered(order) => PageEdit::Reorder(pages::inverse(order)),
            PageChange::Removed { removed, .. } => PageEdit::Restore(removed.clone()?),
            PageChange::Inserted { at, count, .. } if *count > 0 => {
                PageEdit::Remove((*at..at + count).collect())
            }
            PageChange::Inserted { .. } => return None,
            PageChange::Cropped { before, .. } => PageEdit::SetCrop(before.clone()?),
        })
    }

    /// Keeps what the document thread answered, for undoing or redoing.
    fn set_outcome(&mut self, outcome: &PageOutcome) {
        match (self, outcome) {
            (PageChange::Removed { removed, .. }, PageOutcome::Removed(pages))
            | (PageChange::Inserted { removed, .. }, PageOutcome::Removed(pages)) => {
                *removed = Some(pages.clone())
            }
            (PageChange::Inserted { count, .. }, PageOutcome::Inserted { count: added, .. }) => {
                *count = *added
            }
            // Undoing a crop answers with the crop it replaced, which is
            // the crop to redo; only the first answer is kept.
            (PageChange::Cropped { before, .. }, PageOutcome::Cropped(crops))
                if before.is_none() =>
            {
                *before = Some(crops.clone())
            }
            _ => {}
        }
    }
}

/// A step to send to the document thread.
#[derive(Debug, Clone)]
pub enum Step {
    Annotation(usize, Box<Edit>),
    Pages(PageEdit),
}

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
    Pages(PageChange),
}

impl Change {
    /// The step that makes this change (again).
    pub fn forward_step(&self) -> Step {
        match self {
            Change::Pages(change) => Step::Pages(change.forward()),
            Change::Added { page, .. }
            | Change::Removed { page, .. }
            | Change::Updated { page, .. }
            | Change::Field { page, .. } => Step::Annotation(*page, Box::new(self.forward())),
        }
    }

    pub fn backward_step(&self) -> Option<Step> {
        match self {
            Change::Pages(change) => change.backward().map(Step::Pages),
            Change::Added { page, .. }
            | Change::Removed { page, .. }
            | Change::Updated { page, .. }
            | Change::Field { page, .. } => {
                Some(Step::Annotation(*page, Box::new(self.backward())))
            }
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
            Change::Pages(_) => unreachable!("page changes have page edits"),
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
            Change::Pages(_) => unreachable!("page changes have page edits"),
        }
    }

    /// Keeps what a removal returned, for putting the annotation back.
    pub fn set_removed(&mut self, token: Removed) {
        match self {
            Change::Added { removed, .. } | Change::Removed { removed, .. } => {
                *removed = Some(token)
            }
            Change::Updated { .. } | Change::Field { .. } | Change::Pages(_) => {}
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

    /// The step to send to undo the last change. A page change still
    /// waiting for the document thread cannot be undone yet.
    pub fn undo(&mut self) -> Option<Step> {
        let step = self.done.last()?.backward_step()?;
        let change = self.done.pop()?;
        self.undone.push(change);
        Some(step)
    }

    pub fn redo(&mut self) -> Option<Step> {
        let change = self.undone.pop()?;
        let step = change.forward_step();
        self.done.push(change);
        Some(step)
    }

    /// Stores what a page edit answered on the change last moved to
    /// `stack`.
    pub fn page_outcome(&mut self, stack: Stack, outcome: &PageOutcome) {
        let list = match stack {
            Stack::Done => &mut self.done,
            Stack::Undone => &mut self.undone,
        };
        if let Some(Change::Pages(change)) = list.last_mut() {
            change.set_outcome(outcome);
        }
    }

    /// Drops the change last moved to `stack`, whose edit failed.
    pub fn discard(&mut self, stack: Stack) {
        match stack {
            Stack::Done => self.done.pop(),
            Stack::Undone => self.undone.pop(),
        };
    }

    /// Forgets everything, after a change that cannot be undone.
    pub fn clear(&mut self) {
        self.done.clear();
        self.undone.clear();
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
        let Some(Step::Annotation(page, edit)) = history.undo() else {
            panic!("no annotation step");
        };
        assert_eq!(page, 2);
        assert!(matches!(*edit, Edit::Remove(ref id) if id == "a"));
        let Some(Step::Annotation(_, edit)) = history.redo() else {
            panic!("no annotation step");
        };
        assert!(matches!(*edit, Edit::Add(..)), "no token yet, so add again");
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

    #[test]
    fn page_changes_wait_for_their_outcome() {
        let mut history = History::default();
        history.record(Change::Pages(PageChange::Removed {
            pages: vec![1],
            removed: None,
        }));
        assert!(history.undo().is_none(), "nothing to put back yet");
        history.page_outcome(Stack::Done, &PageOutcome::Removed(Vec::new()));
        assert!(matches!(
            history.undo(),
            Some(Step::Pages(PageEdit::Restore(_)))
        ));
        assert!(matches!(
            history.redo(),
            Some(Step::Pages(PageEdit::Remove(ref pages))) if pages == &[1]
        ));
    }

    #[test]
    fn reorders_undo_with_the_inverse() {
        let order = pages::moved_order(4, &[0], 4);
        let change = PageChange::Reordered(order.clone());
        let Some(PageEdit::Reorder(back)) = change.backward() else {
            panic!("no reorder");
        };
        assert_eq!(back, pages::inverse(&order));
    }
}
