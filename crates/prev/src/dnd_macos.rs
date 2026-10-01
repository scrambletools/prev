//! Drag and drop on macOS, in the types the Wayland side uses: drags over
//! a window reach prev through the vendored winit's hook and are reported
//! as `DragEvent`s with the data in the MIME types prev reads, and drags
//! prev starts put their data on the drag pasteboard in the types other
//! Mac apps read.

#![allow(unsafe_code)]

use std::cell::RefCell;
use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};

use objc2::rc::Retained;
use objc2::runtime::{AnyObject, NSObject, NSObjectProtocol, ProtocolObject};
use objc2::{
    AllocAnyThread, DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send,
};
use objc2_app_kit::{
    NSApplication, NSBitmapImageRep, NSDeviceRGBColorSpace, NSDragOperation, NSDraggingContext,
    NSDraggingInfo, NSDraggingItem, NSDraggingSession, NSDraggingSource, NSEvent,
    NSEventModifierFlags, NSEventType, NSImage, NSPasteboard, NSPasteboardItem, NSView, NSWindow,
};
use objc2_foundation::{NSArray, NSData, NSInteger, NSPoint, NSRect, NSSize, NSString, NSURL};
use winit::platform::drag_drop::{self, Phase};

use crate::dnd::{Action, Drag, DragEvent, URI_LIST_MIME};

type Handler = Box<dyn Fn(DragEvent) + Send + Sync>;

static HANDLER: RwLock<Option<Handler>> = RwLock::new(None);
static ACCEPTED: RwLock<Vec<String>> = RwLock::new(Vec::new());
static PREFER_MOVE: AtomicBool = AtomicBool::new(false);

const TEXT_MIME: &str = "text/plain;charset=utf-8";
const PAGES_MIME: &str = crate::paste::PAGES_TYPE;

/// Pasteboard types and the MIME types prev knows them by, most wanted
/// first. Files and web addresses both become a URI list.
const TYPES: &[(&str, &str)] = &[
    (PAGES_MIME, "io.github.scrambletools.prev.pages"),
    ("image/png", "public.png"),
    ("image/jpeg", "public.jpeg"),
    ("image/webp", "org.webmproject.webp"),
    ("image/avif", "public.avif"),
    ("image/tiff", "public.tiff"),
    ("image/bmp", "com.microsoft.bmp"),
    ("image/gif", "com.compuserve.gif"),
    (URI_LIST_MIME, FILE_URL),
    (URI_LIST_MIME, WEB_URL),
    (TEXT_MIME, "public.utf8-plain-text"),
];
const FILE_URL: &str = "public.file-url";
const WEB_URL: &str = "public.url";

pub fn set_drag_handler(handler: impl Fn(DragEvent) + Send + Sync + 'static) {
    *HANDLER.write().unwrap_or_else(|poison| poison.into_inner()) = Some(Box::new(handler));
    drag_drop::set_hook(|phase, window, info| {
        let Some(mtm) = MainThreadMarker::new() else {
            return 0;
        };
        // SAFETY: winit passes its live NSWindow and, but for some exits,
        // the NSDraggingInfo AppKit gave it, for the length of this call.
        let window = unsafe { &*window.cast::<NSWindow>() };
        let info = unsafe { info.cast::<ProtocolObject<dyn NSDraggingInfo>>().as_ref() };
        over_window(mtm, phase, window, info)
    });
}

pub fn set_accepted_mimes(mimes: Vec<String>) {
    *ACCEPTED
        .write()
        .unwrap_or_else(|poison| poison.into_inner()) = mimes;
}

pub fn set_prefer_move(prefer: bool) {
    PREFER_MOVE.store(prefer, Ordering::Relaxed);
}

fn emit(event: DragEvent) {
    if let Ok(handler) = HANDLER.read()
        && let Some(handler) = handler.as_ref()
    {
        handler(event);
    }
}

/// Lets the window whose content view is `view` take the types prev
/// reads. On the main thread, once the window exists.
pub fn register(view: usize) {
    // SAFETY: `view` is the content view winit made for a live window.
    let view = unsafe { &*(view as *const NSView) };
    let Some(window) = view.window() else {
        return;
    };
    let mut names: Vec<&str> = TYPES.iter().map(|(_, name)| *name).collect();
    names.dedup();
    let names: Vec<Retained<NSString>> = names.into_iter().map(NSString::from_str).collect();
    window.registerForDraggedTypes(&NSArray::from_retained_slice(&names));
}

// --- Drops onto prev ---------------------------------------------------

thread_local! {
    /// The type the drag over a window would be taken in.
    static CHOSEN: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// The MIME types the pasteboard offers, from its pasteboard types.
fn offered(pasteboard: &NSPasteboard) -> Vec<String> {
    let names: Vec<String> = pasteboard
        .types()
        .map(|types| types.iter().map(|name| name.to_string()).collect())
        .unwrap_or_default();
    let mut mimes: Vec<String> = Vec::new();
    for (mime, name) in TYPES {
        if names.iter().any(|offered| offered == name) && !mimes.iter().any(|m| m == mime) {
            mimes.push((*mime).to_owned());
        }
    }
    mimes
}

/// The accepted type to take from `offered`, if any; a type ending in `*`
/// takes any type starting with the rest.
fn choose_mime(offered: &[String]) -> Option<String> {
    let accepted = ACCEPTED.read().unwrap_or_else(|poison| poison.into_inner());
    accepted
        .iter()
        .find_map(|wanted| match wanted.strip_suffix('*') {
            Some(prefix) => offered
                .iter()
                .find(|mime| mime.starts_with(prefix))
                .cloned(),
            None => offered.contains(wanted).then(|| wanted.clone()),
        })
}

/// The drag's data as `mime`.
fn read(pasteboard: &NSPasteboard, mime: &str) -> Vec<u8> {
    let string = |name: &str| {
        pasteboard
            .stringForType(&NSString::from_str(name))
            .map(|text| text.to_string())
    };
    match mime {
        URI_LIST_MIME => {
            let files: Vec<String> = pasteboard
                .pasteboardItems()
                .map(|items| {
                    items
                        .iter()
                        .filter_map(|item| item.stringForType(&NSString::from_str(FILE_URL)))
                        .map(|url| file_uri(&url.to_string()))
                        .collect()
                })
                .unwrap_or_default();
            let uris = if files.is_empty() {
                string(WEB_URL).into_iter().collect()
            } else {
                files
            };
            uris.iter()
                .map(|uri| format!("{uri}\r\n"))
                .collect::<String>()
                .into_bytes()
        }
        TEXT_MIME => string("public.utf8-plain-text")
            .unwrap_or_default()
            .into_bytes(),
        _ => TYPES
            .iter()
            .find(|(candidate, _)| *candidate == mime)
            .and_then(|(_, name)| pasteboard.dataForType(&NSString::from_str(name)))
            .map(|data| data.to_vec())
            .unwrap_or_default(),
    }
}

/// `url` as a plain `file://` URI. Finder puts file reference URLs on the
/// drag pasteboard, as `file:///.file/id=…`, which name the file by its
/// id; they are resolved to its path.
fn file_uri(url: &str) -> String {
    if !url.contains("/.file/id=") {
        return url.to_owned();
    }
    let Some(path) = NSURL::URLWithString(&NSString::from_str(url))
        .and_then(|url| url.filePathURL())
        .and_then(|url| url.path())
    else {
        return url.to_owned();
    };
    let uri = crate::drag::uri_list(std::path::Path::new(&path.to_string()));
    String::from_utf8_lossy(&uri).trim_end().to_owned()
}

/// Where the drag is in `window`'s content view, in logical pixels from
/// its top left, as iced counts.
fn local(window: &NSWindow, info: &ProtocolObject<dyn NSDraggingInfo>) -> (f64, f64) {
    let point = info.draggingLocation();
    let Some(view) = window.contentView() else {
        return (point.x, point.y);
    };
    let point = view.convertPoint_fromView(point, None);
    if view.isFlipped() {
        (point.x, point.y)
    } else {
        (point.x, view.bounds().size.height - point.y)
    }
}

fn surface(window: &NSWindow) -> usize {
    window
        .contentView()
        .map_or(0, |view| Retained::as_ptr(&view) as usize)
}

/// Whether Shift or Command, the keys that move, is held: by the event
/// AppKit is handling, which carries the keys of remote and synthetic
/// input too, or by the keyboard.
fn move_held() -> bool {
    let moves = |flags: NSEventModifierFlags| {
        flags.intersects(NSEventModifierFlags::Shift | NSEventModifierFlags::Command)
    };
    let current = MainThreadMarker::new()
        .and_then(|mtm| NSApplication::sharedApplication(mtm).currentEvent())
        .is_some_and(|event| moves(event.modifierFlags()));
    current || moves(NSEvent::modifierFlags_class())
}

/// Whether Control, which sends a drop to the sidebar, is held. Asked of
/// the keyboard: while another app's drag is under way, prev is not the
/// active app and hears no key changes.
pub fn control_held() -> bool {
    NSEvent::modifierFlags_class().contains(NSEventModifierFlags::Control)
}

/// What the drop would do: move with Shift or Command when the source
/// allows it, else copy. AppKit narrows what other apps offer while a key
/// is held (Control to a link, Command to a generic operation); prev
/// takes those as copies, since it reads what is dropped either way.
fn operation(info: &ProtocolObject<dyn NSDraggingInfo>) -> NSDragOperation {
    let allowed = info.draggingSourceOperationMask();
    let moving = move_held() || PREFER_MOVE.load(Ordering::Relaxed);
    [
        (moving, NSDragOperation::Move),
        (true, NSDragOperation::Copy),
        (true, NSDragOperation::Generic),
        (true, NSDragOperation::Link),
        (true, NSDragOperation::Move),
    ]
    .into_iter()
    .find(|(wanted, operation)| *wanted && allowed.contains(*operation))
    .map_or(NSDragOperation::None, |(_, operation)| operation)
}

fn over_window(
    _mtm: MainThreadMarker,
    phase: Phase,
    window: &NSWindow,
    info: Option<&ProtocolObject<dyn NSDraggingInfo>>,
) -> usize {
    let surface = surface(window);
    match (phase, info) {
        (Phase::Entered, Some(info)) => {
            let pasteboard = info.draggingPasteboard();
            let chosen = choose_mime(&offered(&pasteboard));
            let accepted = chosen.is_some();
            CHOSEN.with(|slot| *slot.borrow_mut() = chosen);
            let (x, y) = local(window, info);
            emit(DragEvent::Entered {
                surface,
                x,
                y,
                accepted,
            });
            if accepted { operation(info).0 } else { 0 }
        }
        (Phase::Updated, Some(info)) => {
            let (x, y) = local(window, info);
            emit(DragEvent::Moved { surface, x, y });
            if CHOSEN.with(|slot| slot.borrow().is_some()) {
                operation(info).0
            } else {
                0
            }
        }
        (Phase::Perform, Some(info)) => {
            let Some(mime) = CHOSEN.with(|slot| slot.borrow_mut().take()) else {
                return 0;
            };
            let done = operation(info);
            let (x, y) = local(window, info);
            emit(DragEvent::Dropped {
                surface,
                x,
                y,
                data: read(&info.draggingPasteboard(), &mime),
                mime,
                action: if done == NSDragOperation::Move {
                    Action::Move
                } else {
                    Action::Copy
                },
            });
            1
        }
        (Phase::Exited, _) => {
            CHOSEN.with(|slot| *slot.borrow_mut() = None);
            emit(DragEvent::Left { surface });
            0
        }
        _ => 0,
    }
}

// --- Drags out of prev -------------------------------------------------

define_class!(
    /// Reports how a drag prev started ended.
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "PrevDragSource"]
    #[ivars = bool]
    struct DragSource;

    unsafe impl NSObjectProtocol for DragSource {}

    unsafe impl NSDraggingSource for DragSource {
        /// Copy, or move while Shift or Command is held when the drag
        /// allows it; Finder would otherwise move files dropped on the
        /// same disk.
        #[unsafe(method(draggingSession:sourceOperationMaskForDraggingContext:))]
        fn mask(
            &self,
            _session: &NSDraggingSession,
            _context: NSDraggingContext,
        ) -> NSDragOperation {
            if *self.ivars() && move_held() {
                // Only move: offered both, Finder copies what an app drags.
                NSDragOperation::Move
            } else {
                NSDragOperation::Copy
            }
        }

        /// What prev offers decides alone: AppKit would otherwise narrow
        /// it while Command is held, leaving no move.
        #[unsafe(method(ignoreModifierKeysForDraggingSession:))]
        fn ignore_modifier_keys(&self, _session: &NSDraggingSession) -> bool {
            true
        }

        #[unsafe(method(draggingSession:endedAtPoint:operation:))]
        fn ended(&self, _session: &NSDraggingSession, _point: NSPoint, operation: NSDragOperation) {
            let action = if operation.is_empty() {
                None
            } else if operation.contains(NSDragOperation::Move) {
                Some(Action::Move)
            } else {
                Some(Action::Copy)
            };
            emit(DragEvent::SourceEnded { action });
        }
    }
);

impl DragSource {
    fn new(mtm: MainThreadMarker, allow_move: bool) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(allow_move);
        // SAFETY: NSObject's designated initializer.
        unsafe { msg_send![super(this), init] }
    }
}

thread_local! {
    /// The source of the drag under way; AppKit does not keep it.
    static SOURCE: RefCell<Option<Retained<DragSource>>> = const { RefCell::new(None) };
}

/// The pasteboard items for `data`: one per file, the first also carrying
/// the other types, or one item when there are no files.
fn items(data: &[(String, Vec<u8>)]) -> Vec<Retained<NSPasteboardItem>> {
    let files: Vec<String> = data
        .iter()
        .filter(|(mime, _)| mime == URI_LIST_MIME)
        .flat_map(|(_, bytes)| crate::dnd::parse_uri_list(&String::from_utf8_lossy(bytes)))
        .filter(|uri| uri.starts_with("file://"))
        .collect();
    let first = NSPasteboardItem::new();
    for (mime, bytes) in data {
        match mime.as_str() {
            URI_LIST_MIME => {}
            "image/png" => {
                first.setData_forType(
                    &NSData::with_bytes(bytes),
                    &NSString::from_str("public.png"),
                );
            }
            PAGES_MIME => {
                first.setData_forType(
                    &NSData::with_bytes(bytes),
                    &NSString::from_str("io.github.scrambletools.prev.pages"),
                );
            }
            "application/pdf" => {
                first.setData_forType(
                    &NSData::with_bytes(bytes),
                    &NSString::from_str("com.adobe.pdf"),
                );
            }
            text if text.starts_with("text/plain") => {
                let text = NSString::from_str(&String::from_utf8_lossy(bytes));
                first.setString_forType(&text, &NSString::from_str("public.utf8-plain-text"));
            }
            _ => {}
        }
    }
    let mut items = vec![first];
    for (index, file) in files.iter().enumerate() {
        let item = if index == 0 {
            items[0].clone()
        } else {
            let item = NSPasteboardItem::new();
            items.push(item.clone());
            item
        };
        item.setString_forType(&NSString::from_str(file), &NSString::from_str(FILE_URL));
    }
    items
}

/// `icon` as an image `scale` screen pixels to the point.
fn image(icon: &crate::dnd::Icon, scale: f64) -> Option<Retained<NSImage>> {
    let rep = NSBitmapImageRep::alloc();
    // SAFETY: a null plane pointer asks AppKit to allocate the pixels,
    // which are filled row by row within their size below.
    let rep = unsafe {
        NSBitmapImageRep::initWithBitmapDataPlanes_pixelsWide_pixelsHigh_bitsPerSample_samplesPerPixel_hasAlpha_isPlanar_colorSpaceName_bytesPerRow_bitsPerPixel(
            rep,
            std::ptr::null_mut(),
            icon.width as NSInteger,
            icon.height as NSInteger,
            8,
            4,
            true,
            false,
            NSDeviceRGBColorSpace,
            (icon.width * 4) as NSInteger,
            32,
        )
    }?;
    let pixels = rep.bitmapData();
    if pixels.is_null() {
        return None;
    }
    let length = (icon.width * icon.height * 4) as usize;
    // SAFETY: AppKit allocated `length` bytes for the plane above.
    let target = unsafe { std::slice::from_raw_parts_mut(pixels, length) };
    // AppKit's bitmaps hold premultiplied alpha.
    for (out, pixel) in target
        .as_chunks_mut::<4>()
        .0
        .iter_mut()
        .zip(icon.rgba.as_chunks::<4>().0)
    {
        let alpha = u16::from(pixel[3]);
        for channel in 0..3 {
            out[channel] = ((u16::from(pixel[channel]) * alpha) / 255) as u8;
        }
        out[3] = pixel[3];
    }
    let size = NSSize::new(
        f64::from(icon.width) / scale,
        f64::from(icon.height) / scale,
    );
    let image = NSImage::initWithSize(NSImage::alloc(), size);
    image.addRepresentation(&rep);
    Some(image)
}

/// The mouse event the drag starts from: the one AppKit is handling, when
/// it is the button being held, else one made at the pointer.
fn start_event(mtm: MainThreadMarker) -> Option<(Retained<NSEvent>, Retained<NSWindow>)> {
    let app = NSApplication::sharedApplication(mtm);
    if let Some(event) = app.currentEvent()
        && matches!(
            event.r#type(),
            NSEventType::LeftMouseDown | NSEventType::LeftMouseDragged
        )
        && let Some(window) = event.window(mtm)
    {
        return Some((event, window));
    }
    let window = app.keyWindow().or_else(|| app.mainWindow())?;
    let event = NSEvent::mouseEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_clickCount_pressure(
        NSEventType::LeftMouseDragged,
        window.mouseLocationOutsideOfEventStream(),
        NSEvent::modifierFlags_class(),
        0.0,
        window.windowNumber(),
        None,
        0,
        1,
        1.0,
    )?;
    Some((event, window))
}

/// Starts `drag` from the window the pointer button is held on. AppKit
/// runs it after this returns and reports its end to the source.
pub fn start_drag(drag: Drag) -> bool {
    let Some(mtm) = MainThreadMarker::new() else {
        return false;
    };
    let Some((event, window)) = start_event(mtm) else {
        return false;
    };
    let Some(view) = window.contentView() else {
        return false;
    };
    let items = items(&drag.data);
    let scale = window.backingScaleFactor().max(1.0);
    let point = view.convertPoint_fromView(event.locationInWindow(), None);
    let picture = drag.icon.as_ref().and_then(|icon| image(icon, scale));
    let dragging: Vec<Retained<NSDraggingItem>> = items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let dragging = NSDraggingItem::initWithPasteboardWriter(
                NSDraggingItem::alloc(),
                ProtocolObject::from_ref(&**item),
            );
            let (frame, contents) = match (&drag.icon, &picture) {
                (Some(icon), Some(picture)) if index == 0 => {
                    let size = picture.size();
                    let (hot_x, hot_y) = (
                        f64::from(icon.hotspot.0) / scale,
                        f64::from(icon.hotspot.1) / scale,
                    );
                    let y = if view.isFlipped() {
                        point.y - hot_y
                    } else {
                        point.y - (size.height - hot_y)
                    };
                    (
                        NSRect::new(NSPoint::new(point.x - hot_x, y), size),
                        Some(&**picture as &AnyObject),
                    )
                }
                _ => (NSRect::new(point, NSSize::new(1.0, 1.0)), None),
            };
            // SAFETY: the contents are an NSImage or nothing.
            unsafe { dragging.setDraggingFrame_contents(frame, contents) };
            dragging
        })
        .collect();
    let source = DragSource::new(mtm, drag.allow_move);
    view.beginDraggingSessionWithItems_event_source(
        &NSArray::from_retained_slice(&dragging),
        &event,
        ProtocolObject::from_ref(&*source),
    );
    SOURCE.with(|slot| *slot.borrow_mut() = Some(source));
    true
}
