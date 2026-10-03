//! Threads around an engine: one document thread per open document, which
//! owns the (non-`Send`) document, and a shared render pool that turns page
//! displays into bitmaps and text layouts. Results arrive as futures.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

use futures_channel::{mpsc as stream, oneshot};

use crate::annotation::{Annotation, Field, Removed, StampContent};
use crate::engine::{
    Bitmap, CropBox, Document, Engine, Error, ExportOptions, Lifted, Link, Metadata, OutlineItem,
    PageDisplay, RemovedPage, Result,
};
use crate::geometry::{PixelRect, Quad, Rect, Size};
use crate::text::TextLayout;

const DISPLAY_CACHE_PAGES: usize = 48;

/// Marks work as no longer wanted. Cancelled jobs are skipped and their
/// futures resolve to `Err(Canceled)`.
#[derive(Debug, Clone, Default)]
pub struct Ticket(Arc<AtomicBool>);

impl Ticket {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cancel(&self) {
        self.0.store(true, Ordering::Relaxed);
    }

    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

pub use oneshot::Canceled;

type Job = Box<dyn FnOnce() + Send>;

struct QueuedJob {
    priority: u32,
    sequence: u64,
    ticket: Ticket,
    job: Job,
}

impl PartialEq for QueuedJob {
    fn eq(&self, other: &Self) -> bool {
        (self.priority, self.sequence) == (other.priority, other.sequence)
    }
}

impl Eq for QueuedJob {}

impl PartialOrd for QueuedJob {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueuedJob {
    /// Lowest priority value first, then oldest first.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Reverse((self.priority, self.sequence)).cmp(&Reverse((other.priority, other.sequence)))
    }
}

/// Worker threads shared by all documents.
pub struct RenderPool {
    queue: Mutex<BinaryHeap<QueuedJob>>,
    available: Condvar,
    sequence: AtomicU64,
}

impl RenderPool {
    pub fn new(threads: usize) -> Arc<Self> {
        let pool = Arc::new(Self {
            queue: Mutex::new(BinaryHeap::new()),
            available: Condvar::new(),
            sequence: AtomicU64::new(0),
        });
        for index in 0..threads.max(1) {
            let pool = Arc::clone(&pool);
            std::thread::Builder::new()
                .name(format!("prev-render-{index}"))
                .spawn(move || pool.run())
                .expect("spawn render thread");
        }
        pool
    }

    /// One thread fewer than the machine has, at most six.
    pub fn default_threads() -> usize {
        std::thread::available_parallelism()
            .map_or(2, |count| count.get().saturating_sub(1))
            .clamp(1, 6)
    }

    /// Whether no job is waiting for a worker.
    pub fn is_idle_queue(&self) -> bool {
        self.queue.lock().unwrap().is_empty()
    }

    fn run(&self) {
        loop {
            let queued = {
                let mut queue = self.queue.lock().unwrap();
                loop {
                    match queue.pop() {
                        Some(queued) if queued.ticket.is_cancelled() => continue,
                        Some(queued) => break queued,
                        None => queue = self.available.wait(queue).unwrap(),
                    }
                }
            };
            (queued.job)();
        }
    }

    fn submit<T: Send + 'static>(
        &self,
        priority: u32,
        ticket: Ticket,
        work: impl FnOnce() -> T + Send + 'static,
    ) -> oneshot::Receiver<T> {
        let (sender, receiver) = oneshot::channel();
        let job_ticket = ticket.clone();
        let job: Job = Box::new(move || {
            if !job_ticket.is_cancelled() {
                let _ = sender.send(work());
            }
        });
        let sequence = self.sequence.fetch_add(1, Ordering::Relaxed);
        self.queue.lock().unwrap().push(QueuedJob {
            priority,
            sequence,
            ticket,
            job,
        });
        self.available.notify_one();
        receiver
    }

    /// Renders `area` of a page at `scale`. Lower `priority` runs first.
    pub fn render(
        &self,
        display: Arc<dyn PageDisplay>,
        scale: f32,
        area: PixelRect,
        priority: u32,
        ticket: Ticket,
    ) -> oneshot::Receiver<Result<Bitmap>> {
        self.submit(priority, ticket, move || display.render(scale, area))
    }

    pub fn text(
        &self,
        display: Arc<dyn PageDisplay>,
        priority: u32,
        ticket: Ticket,
    ) -> oneshot::Receiver<Result<TextLayout>> {
        self.submit(priority, ticket, move || display.text())
    }
}

/// What the UI needs to lay out a document.
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentInfo {
    pub page_sizes: Vec<Size>,
    pub page_labels: Vec<Option<String>>,
    pub title: Option<String>,
    /// Empty until loaded with `DocumentHandle::outline`, which can be slow
    /// for large documents.
    pub outline: Vec<OutlineItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Opened {
    Ready(DocumentInfo),
    NeedsPassword,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchEvent {
    /// Matches on one page; pages without matches are skipped.
    Matches {
        page: usize,
        quads: Vec<Quad>,
    },
    /// Pages searched so far, out of the total.
    Progress {
        searched: usize,
        total: usize,
    },
    Finished,
}

/// A change to a page's annotations or form fields.
#[derive(Debug, Clone)]
pub enum Edit {
    Add(Annotation, Option<StampContent>),
    Update(Annotation, Option<StampContent>),
    Remove(String),
    Restore(Removed),
    SetField { id: i32, value: String },
}

/// A page after an edit: everything the UI shows of it, rebuilt.
#[derive(Clone)]
pub struct Edited {
    pub page: usize,
    pub display: Arc<dyn PageDisplay>,
    pub annotations: Vec<Annotation>,
    pub fields: Vec<Field>,
    /// Set when the edit removed an annotation, to undo it with.
    pub removed: Option<Removed>,
}

impl std::fmt::Debug for Edited {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Edited(page {})", self.page)
    }
}

/// A change to the document's pages. Page numbers are those before the
/// change.
#[derive(Debug, Clone)]
pub enum PageEdit {
    /// Turns pages clockwise by quarter turns; negative turns go back.
    Rotate {
        pages: Vec<usize>,
        quarter_turns: i32,
    },
    /// Takes pages out; the outcome holds them, to put back.
    Remove(Vec<usize>),
    /// Puts removed pages back, each at its index, lowest first.
    Restore(Vec<(usize, RemovedPage)>),
    /// Puts pages in a new order: `order[i]` is the page that goes to `i`.
    Reorder(Vec<usize>),
    InsertBlank {
        at: usize,
        size: Size,
    },
    /// Inserts every page of a PDF.
    Insert {
        at: usize,
        bytes: Arc<Vec<u8>>,
    },
    /// Crops pages to a rect in page space.
    Crop {
        pages: Vec<usize>,
        rect: Rect,
    },
    /// Gives pages back the crop boxes they had.
    SetCrop(Vec<(usize, CropBox)>),
    /// Applies every redaction mark. Cannot be undone.
    ApplyRedactions,
}

/// What a page edit returned, to undo it with.
#[derive(Debug, Clone, PartialEq)]
pub enum PageOutcome {
    Done,
    /// The removed pages with the index each had, lowest first.
    Removed(Vec<(usize, RemovedPage)>),
    Inserted {
        at: usize,
        count: usize,
    },
    /// The crop boxes the pages had.
    Cropped(Vec<(usize, CropBox)>),
    /// How many redaction marks were applied.
    Redacted(usize),
}

/// The document after a page edit.
#[derive(Debug, Clone)]
pub struct Restructured {
    pub info: DocumentInfo,
    pub outcome: PageOutcome,
}

/// Writes a saved document's bytes to its file.
pub type Writer = Box<dyn FnOnce(&[u8]) -> std::result::Result<(), String> + Send>;

/// Annotations of every page that has any.
pub type DocumentAnnotations = Vec<(usize, Vec<Annotation>)>;

/// A page's annotations and form fields.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PageMarkup {
    pub annotations: Vec<Annotation>,
    pub fields: Vec<Field>,
}

enum Request {
    Markup(usize, oneshot::Sender<Result<PageMarkup>>),
    AllAnnotations(oneshot::Sender<Result<DocumentAnnotations>>),
    Edit(usize, Box<Edit>, oneshot::Sender<Result<Edited>>),
    Save(Writer, oneshot::Sender<Result<()>>),
    Pages(PageEdit, oneshot::Sender<Result<Restructured>>),
    Extract(Vec<usize>, oneshot::Sender<Result<Vec<u8>>>),
    Lift(usize, String, oneshot::Sender<Result<Lifted>>),
    Draft(
        usize,
        Box<Annotation>,
        bool,
        oneshot::Sender<Result<Arc<dyn PageDisplay>>>,
    ),
    AnnotationLayer(usize, oneshot::Sender<Result<Arc<dyn PageDisplay>>>),
    Metadata(oneshot::Sender<Metadata>),
    Export(ExportOptions, Writer, oneshot::Sender<Result<()>>),
    Authenticate(String, oneshot::Sender<Result<Option<DocumentInfo>>>),
    Display(usize, oneshot::Sender<Result<Arc<dyn PageDisplay>>>),
    Links(usize, oneshot::Sender<Result<Vec<Link>>>),
    Outline(oneshot::Sender<Result<Vec<OutlineItem>>>),
    Search {
        needle: String,
        ticket: Ticket,
        events: stream::UnboundedSender<SearchEvent>,
    },
}

/// A cheap, cloneable handle to a document thread. The thread ends when the
/// last handle is dropped.
#[derive(Clone)]
pub struct DocumentHandle {
    requests: mpsc::Sender<Request>,
}

impl std::fmt::Debug for DocumentHandle {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("DocumentHandle")
    }
}

impl DocumentHandle {
    /// Starts a document thread and opens `path` on it.
    pub fn open(
        engine: Arc<dyn Engine>,
        path: PathBuf,
    ) -> (Self, oneshot::Receiver<Result<Opened>>) {
        let (requests, receiver) = mpsc::channel();
        let (opened_sender, opened) = oneshot::channel();
        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        std::thread::Builder::new()
            .name(format!("prev-doc-{name}"))
            .spawn(move || {
                let document = match engine.open(&path) {
                    Ok(document) => document,
                    Err(error) => {
                        let _ = opened_sender.send(Err(error));
                        return;
                    }
                };
                let mut thread = DocumentThread::new(document, engine, path);
                let _ = opened_sender.send(if thread.document.needs_password() {
                    Ok(Opened::NeedsPassword)
                } else {
                    thread.info().map(Opened::Ready)
                });
                thread.run(receiver);
            })
            .expect("spawn document thread");
        (Self { requests }, opened)
    }

    fn request<T>(&self, make: impl FnOnce(oneshot::Sender<T>) -> Request) -> oneshot::Receiver<T> {
        let (sender, receiver) = oneshot::channel();
        let _ = self.requests.send(make(sender));
        receiver
    }

    /// Resolves to `Ok(None)` when the password is wrong.
    pub fn authenticate(
        &self,
        password: String,
    ) -> oneshot::Receiver<Result<Option<DocumentInfo>>> {
        self.request(|sender| Request::Authenticate(password, sender))
    }

    pub fn display(&self, page: usize) -> oneshot::Receiver<Result<Arc<dyn PageDisplay>>> {
        self.request(|sender| Request::Display(page, sender))
    }

    pub fn links(&self, page: usize) -> oneshot::Receiver<Result<Vec<Link>>> {
        self.request(|sender| Request::Links(page, sender))
    }

    pub fn outline(&self) -> oneshot::Receiver<Result<Vec<OutlineItem>>> {
        self.request(Request::Outline)
    }

    pub fn markup(&self, page: usize) -> oneshot::Receiver<Result<PageMarkup>> {
        self.request(|sender| Request::Markup(page, sender))
    }

    /// Annotations on every page that has any.
    pub fn all_annotations(&self) -> oneshot::Receiver<Result<DocumentAnnotations>> {
        self.request(Request::AllAnnotations)
    }

    /// Applies an edit and re-parses the page.
    pub fn edit(&self, page: usize, edit: Edit) -> oneshot::Receiver<Result<Edited>> {
        self.request(|sender| Request::Edit(page, Box::new(edit), sender))
    }

    /// Saves every edit so far: `write` puts the bytes in place, then the
    /// document is opened again from its file, so later saves build on
    /// what is on disk.
    pub fn save(&self, write: Writer) -> oneshot::Receiver<Result<()>> {
        self.request(|sender| Request::Save(write, sender))
    }

    /// Changes the pages. Every page's display and markup must be fetched
    /// again afterwards.
    pub fn pages(&self, edit: PageEdit) -> oneshot::Receiver<Result<Restructured>> {
        self.request(|sender| Request::Pages(edit, sender))
    }

    /// The document information: title, author, dates and so on.
    pub fn metadata(&self) -> oneshot::Receiver<Metadata> {
        self.request(Request::Metadata)
    }

    /// The page split for moving annotation `id` on screen.
    pub fn lift(&self, page: usize, id: String) -> oneshot::Receiver<Result<Lifted>> {
        self.request(|sender| Request::Lift(page, id, sender))
    }

    /// Annotation `annotation.id` as `annotation` has it, alone on a
    /// transparent page, for showing it while its handles are dragged.
    /// `fresh` starts from the annotation as the document has it.
    pub fn draft(
        &self,
        page: usize,
        annotation: Annotation,
        fresh: bool,
    ) -> oneshot::Receiver<Result<Arc<dyn PageDisplay>>> {
        self.request(|sender| Request::Draft(page, Box::new(annotation), fresh, sender))
    }

    /// The annotations of `page` alone, on a transparent page.
    pub fn annotation_layer(&self, page: usize) -> oneshot::Receiver<Result<Arc<dyn PageDisplay>>> {
        self.request(|sender| Request::AnnotationLayer(page, sender))
    }

    /// A new PDF with copies of `pages`, in that order.
    pub fn extract(&self, pages: Vec<usize>) -> oneshot::Receiver<Result<Vec<u8>>> {
        self.request(|sender| Request::Extract(pages, sender))
    }

    /// Writes the document, rewritten with `options`, through `write`.
    pub fn export(&self, options: ExportOptions, write: Writer) -> oneshot::Receiver<Result<()>> {
        self.request(|sender| Request::Export(options, write, sender))
    }

    /// Streams matches page by page from the first page. Cancelling the
    /// ticket stops the search.
    pub fn search(&self, needle: String, ticket: Ticket) -> stream::UnboundedReceiver<SearchEvent> {
        let (events, receiver) = stream::unbounded();
        let _ = self.requests.send(Request::Search {
            needle,
            ticket,
            events,
        });
        receiver
    }
}

struct ActiveSearch {
    needle: String,
    ticket: Ticket,
    events: stream::UnboundedSender<SearchEvent>,
    next_page: usize,
    total: usize,
}

struct DocumentThread {
    document: Box<dyn Document>,
    engine: Arc<dyn Engine>,
    path: PathBuf,
    /// The password that unlocked the document, to open it again.
    password: Option<String>,
    displays: HashMap<usize, Arc<dyn PageDisplay>>,
    recent: VecDeque<usize>,
    search: Option<ActiveSearch>,
}

impl DocumentThread {
    fn new(document: Box<dyn Document>, engine: Arc<dyn Engine>, path: PathBuf) -> Self {
        Self {
            document,
            engine,
            path,
            password: None,
            displays: HashMap::new(),
            recent: VecDeque::new(),
            search: None,
        }
    }

    fn info(&self) -> Result<DocumentInfo> {
        let count = self.document.page_count()?;
        let page_sizes = self.document.page_sizes()?;
        Ok(DocumentInfo {
            page_sizes,
            page_labels: (0..count)
                .map(|index| self.document.page_label(index))
                .collect(),
            title: self.document.title(),
            outline: Vec::new(),
        })
    }

    fn display(&mut self, page: usize) -> Result<Arc<dyn PageDisplay>> {
        if let Some(display) = self.displays.get(&page) {
            let display = Arc::clone(display);
            self.recent.retain(|recent| *recent != page);
            self.recent.push_back(page);
            return Ok(display);
        }
        let display = self.document.display(page)?;
        self.displays.insert(page, Arc::clone(&display));
        self.recent.push_back(page);
        while self.recent.len() > DISPLAY_CACHE_PAGES {
            if let Some(oldest) = self.recent.pop_front() {
                self.displays.remove(&oldest);
            }
        }
        Ok(display)
    }

    fn markup(&self, page: usize) -> Result<PageMarkup> {
        Ok(PageMarkup {
            annotations: self.document.annotations(page)?,
            fields: self.document.fields(page)?,
        })
    }

    fn edit(&mut self, page: usize, edit: Edit) -> Result<Edited> {
        let mut removed = None;
        match &edit {
            Edit::Add(annotation, content) => {
                self.document
                    .add_annotation(page, annotation, content.as_ref())?
            }
            Edit::Update(annotation, content) => {
                self.document
                    .update_annotation(page, annotation, content.as_ref())?
            }
            Edit::Remove(id) => removed = Some(self.document.remove_annotation(page, id)?),
            Edit::Restore(removed) => self.document.restore_annotation(page, removed)?,
            Edit::SetField { id, value } => self.document.set_field(page, *id, value)?,
        }
        // The cached display shows the page as it was.
        self.displays.remove(&page);
        self.recent.retain(|recent| *recent != page);
        let display = self.display(page)?;
        let PageMarkup {
            annotations,
            fields,
        } = self.markup(page)?;
        Ok(Edited {
            page,
            display,
            annotations,
            fields,
            removed,
        })
    }

    /// MuPDF's incremental saves assume the output is appended to the file
    /// it opened, so after writing, reopen from that file.
    fn save(&mut self, write: Writer) -> Result<()> {
        if !self.document.has_changes() {
            return Ok(());
        }
        let bytes = self.document.save()?;
        write(&bytes).map_err(Error::Write)?;
        let mut reopened = self.engine.open(&self.path)?;
        if let Some(password) = &self.password
            && !reopened.authenticate(password)
        {
            return Err(Error::SavedUnreadable);
        }
        self.document = reopened;
        Ok(())
    }

    fn page_edit(&mut self, edit: PageEdit) -> Result<Restructured> {
        // Page numbers change, so nothing cached by number holds.
        self.displays.clear();
        self.recent.clear();
        if let Some(search) = self.search.take() {
            search.ticket.cancel();
        }
        let outcome = match edit {
            PageEdit::Rotate {
                pages,
                quarter_turns,
            } => {
                for page in pages {
                    self.document.rotate_page(page, quarter_turns)?;
                }
                PageOutcome::Done
            }
            PageEdit::Remove(mut pages) => {
                pages.sort_unstable();
                pages.dedup();
                if pages.len() >= self.document.page_count()? {
                    return Err(Error::Engine("a document needs at least one page".into()));
                }
                let mut removed = Vec::new();
                for page in pages.iter().rev() {
                    removed.push((*page, self.document.remove_page(*page)?));
                }
                removed.reverse();
                PageOutcome::Removed(removed)
            }
            PageEdit::Restore(pages) => {
                for (at, page) in &pages {
                    self.document.restore_page(*at, page)?;
                }
                PageOutcome::Done
            }
            PageEdit::Reorder(order) => {
                if order.len() != self.document.page_count()? {
                    return Err(Error::Engine(
                        "the page order does not match the document".into(),
                    ));
                }
                for (from, to) in crate::pages::moves(&order) {
                    self.document.move_page(from, to)?;
                }
                PageOutcome::Done
            }
            PageEdit::InsertBlank { at, size } => {
                self.document.insert_blank_page(at, size)?;
                PageOutcome::Inserted { at, count: 1 }
            }
            PageEdit::Insert { at, bytes } => {
                let count = self.document.insert_document(at, &bytes)?;
                PageOutcome::Inserted { at, count }
            }
            PageEdit::Crop { pages, rect } => {
                let mut before = Vec::new();
                for page in pages {
                    before.push((page, self.document.crop_page(page, rect)?));
                }
                PageOutcome::Cropped(before)
            }
            PageEdit::SetCrop(crops) => {
                let mut before = Vec::new();
                for (page, crop) in crops {
                    before.push((page, self.document.set_crop_box(page, &crop)?));
                }
                PageOutcome::Cropped(before)
            }
            PageEdit::ApplyRedactions => PageOutcome::Redacted(self.document.apply_redactions()?),
        };
        Ok(Restructured {
            info: self.info()?,
            outcome,
        })
    }

    fn export(&mut self, options: &ExportOptions, write: Writer) -> Result<()> {
        let bytes = self.document.export(options)?;
        write(&bytes).map_err(Error::Engine)
    }

    fn run(&mut self, requests: mpsc::Receiver<Request>) {
        loop {
            // Serve requests first; search one page whenever the queue is idle.
            let request = if self.search.is_some() {
                match requests.recv_timeout(Duration::ZERO) {
                    Ok(request) => Some(request),
                    Err(RecvTimeoutError::Timeout) => None,
                    Err(RecvTimeoutError::Disconnected) => return,
                }
            } else {
                match requests.recv() {
                    Ok(request) => Some(request),
                    Err(_) => return,
                }
            };
            match request {
                Some(request) => self.handle(request),
                None => self.search_next_page(),
            }
        }
    }

    fn handle(&mut self, request: Request) {
        match request {
            Request::Authenticate(password, reply) => {
                let result = if self.document.authenticate(&password) {
                    self.password = Some(password);
                    self.info().map(Some)
                } else {
                    Ok(None)
                };
                let _ = reply.send(result);
            }
            Request::Display(page, reply) => {
                let _ = reply.send(self.display(page));
            }
            Request::Links(page, reply) => {
                let _ = reply.send(self.document.links(page));
            }
            Request::Outline(reply) => {
                let _ = reply.send(self.document.outline());
            }
            Request::Markup(page, reply) => {
                let _ = reply.send(self.markup(page));
            }
            Request::AllAnnotations(reply) => {
                let result = self.document.page_count().and_then(|count| {
                    let mut pages = Vec::new();
                    for page in 0..count {
                        let annotations = self.document.annotations(page)?;
                        if !annotations.is_empty() {
                            pages.push((page, annotations));
                        }
                    }
                    Ok(pages)
                });
                let _ = reply.send(result);
            }
            Request::Edit(page, edit, reply) => {
                let _ = reply.send(self.edit(page, *edit));
            }
            Request::Save(write, reply) => {
                let _ = reply.send(self.save(write));
            }
            Request::Pages(edit, reply) => {
                let _ = reply.send(self.page_edit(edit));
            }
            Request::Metadata(reply) => {
                let _ = reply.send(self.document.metadata());
            }
            Request::Lift(page, id, reply) => {
                let _ = reply.send(self.document.lift_annotation(page, &id));
            }
            Request::Draft(page, annotation, fresh, reply) => {
                let _ = reply.send(self.document.draft_annotation(page, &annotation, fresh));
            }
            Request::AnnotationLayer(page, reply) => {
                let _ = reply.send(self.document.annotation_layer(page));
            }
            Request::Extract(pages, reply) => {
                let _ = reply.send(self.document.extract_pages(&pages));
            }
            Request::Export(options, write, reply) => {
                let _ = reply.send(self.export(&options, write));
            }
            Request::Search {
                needle,
                ticket,
                events,
            } => {
                if let Some(previous) = self.search.take() {
                    previous.ticket.cancel();
                }
                let total = self.document.page_count().unwrap_or(0);
                self.search = Some(ActiveSearch {
                    needle,
                    ticket,
                    events,
                    next_page: 0,
                    total,
                });
            }
        }
    }

    fn search_next_page(&mut self) {
        let Some(mut search) = self.search.take() else {
            return;
        };
        if search.ticket.is_cancelled() || search.events.is_closed() {
            return;
        }
        if search.next_page >= search.total {
            let _ = search.events.unbounded_send(SearchEvent::Finished);
            return;
        }
        let page = search.next_page;
        search.next_page += 1;
        let quads = self
            .display(page)
            .and_then(|display| display.search(&search.needle))
            .unwrap_or_default();
        if !quads.is_empty() {
            let _ = search
                .events
                .unbounded_send(SearchEvent::Matches { page, quads });
        }
        let _ = search.events.unbounded_send(SearchEvent::Progress {
            searched: search.next_page,
            total: search.total,
        });
        self.search = Some(search);
    }
}

/// Maps a failed or cancelled future result into the engine error type.
pub fn flatten<T>(result: std::result::Result<Result<T>, Canceled>) -> Result<T> {
    result.unwrap_or_else(|_| Err(Error::Closed))
}
