//! Page editing in the viewer: the sidebar's page selection, page edits
//! sent to the document thread, and keeping every cache in step when page
//! numbers change.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::Arc;

use iced::Task;
use prev_pdf::geometry::Size;
use prev_pdf::pages;
use prev_pdf::worker::{PageEdit, PageOutcome, Restructured};

use super::{PdfMessage, PdfViewer, Request, TileKey};
use crate::pdf::history::{Change, Insertion, PageChange, Stack};

/// How a click on a thumbnail changes the page selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pick {
    /// Only this page, as a plain click.
    Only,
    /// Adds or removes this page, as Ctrl+click.
    Toggle,
    /// Every page from the last one picked, as Shift+click.
    Extend,
}

/// What to do once a page edit is back from the document thread.
#[derive(Debug, Clone)]
pub struct PagesSent {
    edit: PageEdit,
    /// Where the change went in the history; `None` for edits that are not
    /// undone, such as applying redactions.
    stack: Option<Stack>,
    /// Pages to select afterwards, in the new numbering.
    select: Option<Vec<usize>>,
}

/// Where each page was before an edit, by its index after it: `None` for
/// pages the edit added.
fn slots(edit: &PageEdit, outcome: &PageOutcome, count: usize) -> Vec<Option<usize>> {
    let mut slots: Vec<Option<usize>> = (0..count).map(Some).collect();
    match edit {
        PageEdit::Remove(removed) => slots.retain(|page| !removed.contains(&page.unwrap_or(0))),
        PageEdit::Restore(restored) => {
            for (at, _) in restored {
                slots.insert((*at).min(slots.len()), None);
            }
        }
        PageEdit::Reorder(order) => slots = order.iter().copied().map(Some).collect(),
        PageEdit::InsertBlank { at, .. } | PageEdit::Insert { at, .. } => {
            let added = match outcome {
                PageOutcome::Inserted { count, .. } => *count,
                _ => 0,
            };
            let at = (*at).min(slots.len());
            slots.splice(at..at, std::iter::repeat_n(None, added));
        }
        PageEdit::Rotate { .. }
        | PageEdit::Crop { .. }
        | PageEdit::SetCrop(_)
        | PageEdit::ApplyRedactions => {}
    }
    slots
}

/// Pages, by their number before the edit, whose look it changed.
fn changed(edit: &PageEdit, count: usize) -> Vec<usize> {
    match edit {
        PageEdit::Rotate { pages, .. } | PageEdit::Crop { pages, .. } => pages.clone(),
        PageEdit::SetCrop(crops) => crops.iter().map(|(page, _)| *page).collect(),
        PageEdit::ApplyRedactions => (0..count).collect(),
        _ => Vec::new(),
    }
}

impl PdfViewer {
    /// The pages a page edit applies to: those selected in the sidebar,
    /// or the current page.
    pub fn target_pages(&self) -> Vec<usize> {
        if self.selected_pages.is_empty() {
            vec![self.current]
        } else {
            self.selected_pages.iter().copied().collect()
        }
    }

    pub fn pick_page(&mut self, page: usize, pick: Pick) {
        if page >= self.page_count() {
            return;
        }
        match pick {
            Pick::Only => {
                self.selected_pages.clear();
                self.selected_pages.insert(page);
            }
            Pick::Toggle => {
                if !self.selected_pages.remove(&page) {
                    self.selected_pages.insert(page);
                }
                if self.selected_pages.is_empty() {
                    self.selected_pages.insert(page);
                }
            }
            Pick::Extend => {
                let from = self.current;
                let (first, last) = (from.min(page), from.max(page));
                self.selected_pages.extend(first..=last);
            }
        }
    }

    pub fn select_all_pages(&mut self) {
        self.selected_pages = (0..self.page_count()).collect();
    }

    /// Records a page change and makes it.
    pub fn change_pages(&mut self, change: PageChange) -> Task<PdfMessage> {
        let select = match &change {
            PageChange::Rotated { pages, .. } | PageChange::Cropped { pages, .. } => {
                Some(pages.clone())
            }
            _ => None,
        };
        let edit = change.forward();
        self.edit.history.record(Change::Pages(change));
        self.send_pages(edit, Some(Stack::Done), select)
    }

    pub fn rotate_pages(&mut self, quarter_turns: i32) -> Task<PdfMessage> {
        let pages = self.target_pages();
        self.change_pages(PageChange::Rotated {
            pages,
            quarter_turns,
        })
    }

    pub fn remove_pages(&mut self) -> Task<PdfMessage> {
        let pages = self.target_pages();
        if pages.len() >= self.page_count() {
            self.requests.push(Request::Notice(
                "A document needs at least one page.".into(),
            ));
            return Task::none();
        }
        self.change_pages(PageChange::Removed {
            pages,
            removed: None,
        })
    }

    /// Moves `moving` to the gap before page `to` (`page_count` for the
    /// end).
    pub fn move_pages(&mut self, moving: &[usize], to: usize) -> Task<PdfMessage> {
        let order = pages::moved_order(self.page_count(), moving, to);
        if pages::is_identity(&order) {
            return Task::none();
        }
        let went = pages::inverse(&order);
        let mut select: Vec<usize> = moving
            .iter()
            .filter_map(|page| went.get(*page))
            .copied()
            .collect();
        select.sort_unstable();
        self.edit
            .history
            .record(Change::Pages(PageChange::Reordered(order.clone())));
        self.send_pages(PageEdit::Reorder(order), Some(Stack::Done), Some(select))
    }

    /// Where new pages go: after the selected or current page.
    pub fn insertion_point(&self) -> usize {
        self.target_pages().last().map_or(0, |page| page + 1)
    }

    /// Adds an empty page the size of the current one after it.
    pub fn insert_blank_page(&mut self) -> Task<PdfMessage> {
        let size = self
            .info
            .page_sizes
            .get(self.current)
            .copied()
            .unwrap_or(Size::new(612.0, 792.0));
        let at = self.insertion_point();
        self.change_pages(PageChange::Inserted {
            at,
            count: 0,
            source: Insertion::Blank(size),
            removed: None,
        })
    }

    /// Inserts the pages of a PDF at `at`.
    pub fn insert_document(&mut self, at: usize, bytes: Arc<Vec<u8>>) -> Task<PdfMessage> {
        self.change_pages(PageChange::Inserted {
            at: at.min(self.page_count()),
            count: 0,
            source: Insertion::Document(bytes),
            removed: None,
        })
    }

    /// Crops to the rectangular selection: every selected page when the
    /// selection's page is one of them, otherwise that page.
    pub fn crop_to_area(&mut self) -> Task<PdfMessage> {
        let Some((page, rect)) = self.edit.area else {
            self.requests.push(Request::Notice(
                "Choose an area with the rectangular selection tool first.".into(),
            ));
            return Task::none();
        };
        let pages = if self.selected_pages.contains(&page) {
            self.target_pages()
        } else {
            vec![page]
        };
        self.edit.area = None;
        self.change_pages(PageChange::Cropped {
            pages,
            rect,
            before: None,
        })
    }

    /// Applies every redaction mark for good. The history is cleared: what
    /// was removed cannot come back.
    pub fn apply_redactions(&mut self) -> Task<PdfMessage> {
        self.edit.history.clear();
        self.send_pages(PageEdit::ApplyRedactions, None, None)
    }

    pub(super) fn undo_pages(&mut self, edit: PageEdit, stack: Stack) -> Task<PdfMessage> {
        self.send_pages(edit, Some(stack), None)
    }

    fn send_pages(
        &mut self,
        edit: PageEdit,
        stack: Option<Stack>,
        select: Option<Vec<usize>>,
    ) -> Task<PdfMessage> {
        let receiver = self.handle.pages(edit.clone());
        let sent = PagesSent {
            edit,
            stack,
            select,
        };
        Task::perform(receiver, move |result| {
            let result = match result {
                Ok(Ok(restructured)) => Ok(restructured),
                Ok(Err(error)) => Err(error.to_string()),
                Err(_) => Err("the document closed".into()),
            };
            PdfMessage::Restructured(sent.clone(), result)
        })
    }

    pub(super) fn restructured(
        &mut self,
        sent: PagesSent,
        result: Result<Restructured, String>,
    ) -> Task<PdfMessage> {
        let restructured = match result {
            Ok(restructured) => restructured,
            Err(error) => {
                if let Some(stack) = sent.stack {
                    self.edit.history.discard(stack);
                }
                self.requests.push(Request::Notice(format!(
                    "Could not change the pages: {error}"
                )));
                return Task::none();
            }
        };
        if let Some(stack) = sent.stack {
            self.edit.history.page_outcome(stack, &restructured.outcome);
        }
        if let PageOutcome::Redacted(count) = restructured.outcome {
            self.requests.push(Request::Notice(match count {
                0 => "There were no redactions to apply.".to_owned(),
                1 => "Applied 1 redaction.".to_owned(),
                count => format!("Applied {count} redactions."),
            }));
        }
        let count = self.page_count();
        let anchor = self.layout.hit_nearest(self.view.x, self.view.y);
        let slots = slots(&sent.edit, &restructured.outcome, count);
        let changed: HashSet<usize> = changed(&sent.edit, count).into_iter().collect();
        let new_of = self.remap(&slots, &changed);
        self.info = restructured.info;
        let mut follow = sent
            .select
            .as_ref()
            .and_then(|pages| pages.first().copied());
        self.selected_pages = match sent.select {
            Some(pages) => pages.into_iter().collect(),
            None => {
                let added: Vec<usize> = slots
                    .iter()
                    .enumerate()
                    .filter(|(_, slot)| slot.is_none())
                    .map(|(page, _)| page)
                    .collect();
                if added.is_empty() || matches!(sent.edit, PageEdit::ApplyRedactions) {
                    std::mem::take(&mut self.selected_pages)
                } else {
                    // Show what came in.
                    follow = added.first().copied();
                    added.into_iter().collect()
                }
            }
        };
        let count = self.page_count();
        self.selected_pages.retain(|page| *page < count);
        self.current = self.current.min(self.page_count().saturating_sub(1));
        if let Some(first) = self.selected_pages.first() {
            self.current = *first;
        }
        self.relayout();
        let follow = match follow {
            // Show the pages that moved or turned.
            Some(page) => self.go_to(page, None),
            // Otherwise keep the page at the top of the view in view,
            // wherever it went.
            None => {
                if let Some((page, point)) = anchor
                    && let Some(page) = new_of.get(&page)
                {
                    self.keep_in_view(*page, point);
                }
                Task::none()
            }
        };
        self.requests.push(Request::Changed);
        self.requests.push(Request::PagesChanged);
        let search = if self.search.query.trim().is_empty() {
            Task::none()
        } else {
            self.start_search(self.search.query.clone())
        };
        Task::batch([follow, search, self.schedule()])
    }

    fn keep_in_view(&mut self, page: usize, point: prev_pdf::geometry::Point) {
        if self.mode != super::ViewMode::Continuous {
            self.scroll_to(0.0, 0.0);
            return;
        }
        let size = self.info.page_sizes[page];
        let point =
            prev_pdf::geometry::Point::new(point.x.min(size.width), point.y.min(size.height));
        if let Some((x, y)) = self.layout.to_document(page, point) {
            self.scroll_to(x, y);
        }
    }

    /// Moves what is cached for each page to its new number and drops what
    /// no longer shows the page as it is. `slots` holds each new page's
    /// old number; `changed` are old numbers of pages that look different.
    fn remap(
        &mut self,
        slots: &[Option<usize>],
        changed: &HashSet<usize>,
    ) -> HashMap<usize, usize> {
        self.epoch += 1;
        let mut new_of: HashMap<usize, usize> = HashMap::new();
        for (new, old) in slots.iter().enumerate() {
            if let Some(old) = old {
                new_of.insert(*old, new);
            }
        }
        let keep = |old: &usize| -> Option<usize> {
            (!changed.contains(old))
                .then(|| new_of.get(old).copied())
                .flatten()
        };
        fn moved<T>(map: &mut HashMap<usize, T>, keep: impl Fn(&usize) -> Option<usize>) {
            *map = std::mem::take(map)
                .into_iter()
                .filter_map(|(old, value)| Some((keep(&old)?, value)))
                .collect();
        }
        moved(&mut self.displays, keep);
        moved(&mut self.previews, keep);
        moved(&mut self.texts, keep);
        moved(&mut self.links, keep);
        // Generations carry on for changed pages too, so their tiles never
        // reuse a key.
        moved(&mut self.generations, |old| new_of.get(old).copied());
        for page in changed.iter().filter_map(|old| new_of.get(old)) {
            *self.generations.entry(*page).or_insert(0) += 2;
        }
        self.markup.clear();
        let mut tile_bytes = 0;
        self.tiles = std::mem::take(&mut self.tiles)
            .into_iter()
            .filter_map(|(key, entry)| {
                let page = keep(&key.page)?;
                tile_bytes += entry.1;
                Some((TileKey { page, ..key }, entry))
            })
            .collect();
        self.tile_bytes = tile_bytes;
        for (ticket, _) in self.tiles_in_flight.values() {
            ticket.cancel();
        }
        self.tiles_in_flight.clear();
        self.displays_requested.clear();
        self.previews_requested.clear();
        self.thumbnails_waiting.clear();
        self.texts_requested.clear();
        self.links_requested.clear();
        self.markup_requested.clear();
        self.pending_copy = false;
        self.selection = None;
        self.edit.selected = None;
        self.edit.area = None;
        self.edit.drag = None;
        self.edit.lift = None;
        self.edit.text = None;
        self.edit.field = None;
        self.edit.choice = None;
        self.selected_pages = self
            .selected_pages
            .iter()
            .filter_map(|old| new_of.get(old).copied())
            .collect::<BTreeSet<usize>>();
        if let Some(current) = new_of.get(&self.current) {
            self.current = *current;
        }
        new_of
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slots_follow_each_edit() {
        let removed = slots(&PageEdit::Remove(vec![1, 3]), &PageOutcome::Done, 5);
        assert_eq!(removed, [Some(0), Some(2), Some(4)]);
        let inserted = slots(
            &PageEdit::InsertBlank {
                at: 1,
                size: Size::new(1.0, 1.0),
            },
            &PageOutcome::Inserted { at: 1, count: 2 },
            3,
        );
        assert_eq!(inserted, [Some(0), None, None, Some(1), Some(2)]);
        let order = pages::moved_order(3, &[2], 0);
        let reordered = slots(&PageEdit::Reorder(order), &PageOutcome::Done, 3);
        assert_eq!(reordered, [Some(2), Some(0), Some(1)]);
    }
}
