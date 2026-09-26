//! The document thread and render pool.

mod common;

use std::sync::Arc;

use futures::StreamExt;
use futures::executor::block_on;
use prev_pdf::engine::{Bitmap, Engine, LinkTarget, PageDisplay};
use prev_pdf::geometry::{PixelRect, Quad, Size};
use prev_pdf::mupdf_engine::MupdfEngine;
use prev_pdf::text::TextLayout;
use prev_pdf::worker::{DocumentHandle, Opened, RenderPool, SearchEvent, Ticket, flatten};

fn engine() -> Arc<dyn Engine> {
    Arc::new(MupdfEngine)
}

fn fixture(dir: &tempfile::TempDir, bytes: &[u8]) -> std::path::PathBuf {
    let path = dir.path().join("fixture.pdf");
    std::fs::write(&path, bytes).unwrap();
    path
}

fn open_ready(dir: &tempfile::TempDir) -> (DocumentHandle, prev_pdf::worker::DocumentInfo) {
    let (handle, opened) =
        DocumentHandle::open(engine(), fixture(dir, &common::navigation_fixture()));
    match flatten(block_on(opened)).unwrap() {
        Opened::Ready(info) => (handle, info),
        Opened::NeedsPassword => panic!("fixture is not encrypted"),
    }
}

#[test]
fn opens_with_layout_info() {
    let dir = tempfile::tempdir().unwrap();
    let (handle, info) = open_ready(&dir);
    assert_eq!(
        info.page_sizes,
        vec![
            Size::new(612.0, 792.0),
            Size::new(842.0, 595.0),
            Size::new(792.0, 612.0)
        ]
    );
    assert_eq!(info.page_labels[1].as_deref(), Some("ii"));
    assert!(info.outline.is_empty(), "loaded separately");
    assert_eq!(flatten(block_on(handle.outline())).unwrap().len(), 2);
    let links = flatten(block_on(handle.links(0))).unwrap();
    assert!(
        links
            .iter()
            .any(|link| matches!(link.target, LinkTarget::Page { index: 1, .. }))
    );
}

#[test]
fn renders_on_the_pool() {
    let dir = tempfile::tempdir().unwrap();
    let (handle, _) = open_ready(&dir);
    let pool = RenderPool::new(2);
    let display = flatten(block_on(handle.display(0))).unwrap();
    let area = PixelRect {
        x: 0,
        y: 0,
        width: 306,
        height: 396,
    };
    let bitmap = flatten(block_on(pool.render(
        Arc::clone(&display),
        0.5,
        area,
        0,
        Ticket::new(),
    )))
    .unwrap();
    assert_eq!((bitmap.width, bitmap.height), (306, 396));
    let text = flatten(block_on(pool.text(display, 0, Ticket::new()))).unwrap();
    assert_eq!(text.lines.len(), 2);
}

#[test]
fn cancelled_jobs_resolve_as_cancelled() {
    let dir = tempfile::tempdir().unwrap();
    let (handle, _) = open_ready(&dir);
    let pool = RenderPool::new(1);
    let display = flatten(block_on(handle.display(0))).unwrap();
    let ticket = Ticket::new();
    ticket.cancel();
    let area = PixelRect {
        x: 0,
        y: 0,
        width: 10,
        height: 10,
    };
    assert!(block_on(pool.render(display, 1.0, area, 0, ticket)).is_err());
}

type Gate = Arc<(std::sync::Mutex<bool>, std::sync::Condvar)>;

/// A page whose render waits until the gate opens, then records its name.
struct GatedPage {
    name: &'static str,
    gate: Gate,
    rendered: Arc<std::sync::Mutex<Vec<&'static str>>>,
}

impl PageDisplay for GatedPage {
    fn size(&self) -> Size {
        Size::new(1.0, 1.0)
    }

    fn render(&self, _: f32, _: PixelRect) -> prev_pdf::engine::Result<Bitmap> {
        let (open, changed) = &*self.gate;
        let mut open = open.lock().unwrap();
        while !*open {
            open = changed.wait(open).unwrap();
        }
        drop(open);
        self.rendered.lock().unwrap().push(self.name);
        Ok(Bitmap {
            width: 1,
            height: 1,
            pixels: vec![255; 4],
        })
    }

    fn text(&self) -> prev_pdf::engine::Result<TextLayout> {
        Ok(TextLayout::default())
    }

    fn search(&self, _: &str) -> prev_pdf::engine::Result<Vec<Quad>> {
        Ok(Vec::new())
    }
}

#[test]
fn higher_priority_jobs_run_first() {
    let pool = RenderPool::new(1);
    let gate: Gate = Arc::new((std::sync::Mutex::new(false), std::sync::Condvar::new()));
    let rendered = Arc::new(std::sync::Mutex::new(Vec::new()));
    let page = |name| {
        Arc::new(GatedPage {
            name,
            gate: Arc::clone(&gate),
            rendered: Arc::clone(&rendered),
        })
    };
    let area = PixelRect {
        x: 0,
        y: 0,
        width: 1,
        height: 1,
    };

    let blocker = pool.render(page("blocker"), 1.0, area, 0, Ticket::new());
    // Wait until the only worker is inside the blocker, so the next two queue.
    while !pool.is_idle_queue() {
        std::thread::yield_now();
    }
    let low = pool.render(page("low"), 1.0, area, 9, Ticket::new());
    let high = pool.render(page("high"), 1.0, area, 1, Ticket::new());
    *gate.0.lock().unwrap() = true;
    gate.1.notify_all();
    for receiver in [blocker, low, high] {
        block_on(receiver).unwrap().unwrap();
    }
    assert_eq!(*rendered.lock().unwrap(), vec!["blocker", "high", "low"]);
}

#[test]
fn search_streams_matches_per_page() {
    let dir = tempfile::tempdir().unwrap();
    let (handle, _) = open_ready(&dir);
    let events: Vec<SearchEvent> = block_on(handle.search("page".into(), Ticket::new()).collect());
    let pages: Vec<usize> = events
        .iter()
        .filter_map(|event| match event {
            SearchEvent::Matches { page, .. } => Some(*page),
            _ => None,
        })
        .collect();
    assert_eq!(pages, vec![1, 2]);
    assert_eq!(events.last(), Some(&SearchEvent::Finished));
    assert!(events.contains(&SearchEvent::Progress {
        searched: 3,
        total: 3
    }));
}

#[test]
fn a_new_search_replaces_the_old_one() {
    let dir = tempfile::tempdir().unwrap();
    let (handle, _) = open_ready(&dir);
    let first = handle.search("page".into(), Ticket::new());
    let second = handle.search("hello".into(), Ticket::new());
    let second_events: Vec<SearchEvent> = block_on(second.collect());
    assert!(matches!(
        second_events.first(),
        Some(SearchEvent::Matches { page: 0, .. })
    ));
    let first_events: Vec<SearchEvent> = block_on(first.collect());
    assert!(
        !first_events.contains(&SearchEvent::Finished),
        "replaced search never finishes"
    );
}

#[test]
fn password_flow() {
    use mupdf::pdf::{PdfDocument, PdfWriteOptions, document::Encryption};
    let source = PdfDocument::from_bytes(&common::navigation_fixture()).unwrap();
    let mut options = PdfWriteOptions::default();
    options
        .set_encryption(Encryption::Aes256)
        .set_user_password("secret")
        .set_owner_password("owner");
    let mut bytes = Vec::new();
    source.write_to_with_options(&mut bytes, options).unwrap();

    let dir = tempfile::tempdir().unwrap();
    let (handle, opened) = DocumentHandle::open(engine(), fixture(&dir, &bytes));
    assert_eq!(flatten(block_on(opened)).unwrap(), Opened::NeedsPassword);
    assert_eq!(
        flatten(block_on(handle.authenticate("nope".into()))).unwrap(),
        None
    );
    let info = flatten(block_on(handle.authenticate("secret".into())))
        .unwrap()
        .unwrap();
    assert_eq!(info.page_sizes.len(), 3);
}

#[test]
fn open_errors_are_reported() {
    let dir = tempfile::tempdir().unwrap();
    let (_handle, opened) = DocumentHandle::open(engine(), dir.path().join("missing.pdf"));
    assert!(flatten(block_on(opened)).is_err());
}
