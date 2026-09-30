//! Printing on macOS: AppKit's print panel and print operation, with a
//! view that draws each page as MuPDF renders it, so the panel's preview
//! and the printout match what prev shows on screen. Each page is scaled
//! to fit the printable area of the paper chosen in the panel.

#![allow(unsafe_code)]

use std::cell::RefCell;
use std::path::Path;

use objc2::rc::Retained;
use objc2::{
    AllocAnyThread, DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send,
};
use objc2_app_kit::{
    NSBitmapImageRep, NSCompositingOperation, NSDeviceRGBColorSpace, NSPrintOperation,
    NSPrintPanelOptions, NSView,
};
use objc2_foundation::{NSInteger, NSPoint, NSRange, NSRect, NSSize, NSString};
use prev_pdf::engine::{Document, Engine};
use prev_pdf::geometry::PixelRect;
use prev_pdf::mupdf_engine::MupdfEngine;

use crate::i18n::Describe;

/// The most device pixels per inch pages are rendered at, as on Windows.
const MAX_DPI: f64 = 300.0;

pub struct Pages {
    document: Box<dyn Document>,
    count: usize,
    /// The printable area of the paper, in points, set once the print
    /// operation asks for the page range.
    area: RefCell<NSSize>,
}

define_class!(
    #[unsafe(super(NSView))]
    #[thread_kind = MainThreadOnly]
    #[name = "PrevPrintView"]
    #[ivars = Pages]
    struct PrintView;

    impl PrintView {
        #[unsafe(method(isFlipped))]
        fn is_flipped(&self) -> bool {
            true
        }

        /// One page of the view per page of the document, each the size
        /// of the paper's printable area, stacked down the view.
        #[unsafe(method(knowsPageRange:))]
        fn knows_page_range(&self, range: *mut NSRange) -> bool {
            let pages = self.ivars();
            let area = printable_area(self.mtm()).unwrap_or(NSSize::new(540.0, 720.0));
            *pages.area.borrow_mut() = area;
            self.setFrameSize(NSSize::new(area.width, area.height * pages.count as f64));
            // SAFETY: AppKit passes a range to fill in.
            unsafe {
                *range = NSRange::new(1, pages.count);
            }
            true
        }

        #[unsafe(method(rectForPage:))]
        fn rect_for_page(&self, page: NSInteger) -> NSRect {
            self.slot(page)
        }

        #[unsafe(method(drawRect:))]
        fn draw_rect(&self, _dirty: NSRect) {
            let Some(operation) = NSPrintOperation::currentOperation(self.mtm()) else {
                return;
            };
            let page = operation.currentPage();
            let _ = self.draw_page((page.max(1) - 1) as usize, self.slot(page));
        }
    }
);

impl PrintView {
    /// Where page `page`, counted from 1, sits in the view.
    fn slot(&self, page: NSInteger) -> NSRect {
        let area = *self.ivars().area.borrow();
        let index = (page.max(1) - 1) as f64;
        NSRect::new(NSPoint::new(0.0, area.height * index), area)
    }

    fn new(mtm: MainThreadMarker, pages: Pages) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(pages);
        // SAFETY: NSView's designated initializer.
        unsafe { msg_send![super(this), initWithFrame: NSRect::ZERO] }
    }

    /// Renders page `index` and draws it centred in `slot`, as large as
    /// fits, on white.
    fn draw_page(&self, index: usize, slot: NSRect) -> Option<()> {
        let display = self.ivars().document.display(index).ok()?;
        let size = display.size();
        let (width, height) = (f64::from(size.width), f64::from(size.height));
        let fit = (slot.size.width / width).min(slot.size.height / height);
        let target = NSRect::new(
            NSPoint::new(
                slot.origin.x + (slot.size.width - width * fit) / 2.0,
                slot.origin.y + (slot.size.height - height * fit) / 2.0,
            ),
            NSSize::new(width * fit, height * fit),
        );
        // Device pixels: the page's printed size at up to MAX_DPI.
        let scale = (fit * MAX_DPI / 72.0) as f32;
        let bitmap = display
            .render(
                scale,
                PixelRect {
                    x: 0,
                    y: 0,
                    width: (size.width * scale).ceil() as u32,
                    height: (size.height * scale).ceil() as u32,
                },
            )
            .ok()?;
        let image = image_rep(bitmap.width, bitmap.height, &bitmap.pixels)?;
        // SAFETY: no hints are passed; the rectangles are plain values.
        unsafe {
            image.drawInRect_fromRect_operation_fraction_respectFlipped_hints(
                target,
                NSRect::ZERO,
                NSCompositingOperation::Copy,
                1.0,
                true,
                None,
            );
        }
        Some(())
    }
}

/// An RGBA bitmap as an image AppKit can draw, its transparent parts
/// white, as paper is.
fn image_rep(width: u32, height: u32, rgba: &[u8]) -> Option<Retained<NSBitmapImageRep>> {
    let rep = NSBitmapImageRep::alloc();
    // SAFETY: a null plane pointer asks AppKit to allocate the pixels,
    // which are then filled row by row within their size.
    let rep = unsafe {
        NSBitmapImageRep::initWithBitmapDataPlanes_pixelsWide_pixelsHigh_bitsPerSample_samplesPerPixel_hasAlpha_isPlanar_colorSpaceName_bytesPerRow_bitsPerPixel(
            rep,
            std::ptr::null_mut(),
            width as NSInteger,
            height as NSInteger,
            8,
            4,
            true,
            false,
            NSDeviceRGBColorSpace,
            (width * 4) as NSInteger,
            32,
        )
    }?;
    let pixels = rep.bitmapData();
    if pixels.is_null() {
        return None;
    }
    let length = (width * height * 4) as usize;
    // SAFETY: AppKit allocated `length` bytes for the planes above.
    let target = unsafe { std::slice::from_raw_parts_mut(pixels, length) };
    for (out, pixel) in target.chunks_exact_mut(4).zip(rgba.chunks_exact(4)) {
        let alpha = u16::from(pixel[3]);
        for channel in 0..3 {
            out[channel] = ((u16::from(pixel[channel]) * alpha + 255 * (255 - alpha)) / 255) as u8;
        }
        out[3] = 255;
    }
    Some(rep)
}

/// The printable part of the current operation's paper, in points.
fn printable_area(mtm: MainThreadMarker) -> Option<NSSize> {
    let operation = NSPrintOperation::currentOperation(mtm)?;
    let bounds = operation.printInfo().imageablePageBounds();
    (bounds.size.width > 0.0 && bounds.size.height > 0.0).then_some(bounds.size)
}

/// Shows the print panel for the PDF at `path`, named `title` in the
/// print queue, and prints what the user chooses. Runs on the main
/// thread, where AppKit's printing must; blocks until the panel closes.
pub fn print_on_main(mtm: MainThreadMarker, path: &Path, title: &str) -> Result<(), String> {
    let document = MupdfEngine
        .open(path)
        .map_err(|error| crate::fl!("print-failed", error = error.describe()))?;
    let count = document
        .page_count()
        .map_err(|error| crate::fl!("print-failed", error = error.describe()))?;
    if count == 0 {
        return Ok(());
    }
    let view = PrintView::new(
        mtm,
        Pages {
            document,
            count,
            area: RefCell::new(NSSize::ZERO),
        },
    );
    let operation = NSPrintOperation::printOperationWithView(&view);
    operation.setJobTitle(Some(&NSString::from_str(title)));
    operation.setShowsPrintPanel(true);
    operation.setShowsProgressPanel(true);
    let panel = operation.printPanel();
    panel.setOptions(
        NSPrintPanelOptions::ShowsCopies
            | NSPrintPanelOptions::ShowsPageRange
            | NSPrintPanelOptions::ShowsPaperSize
            | NSPrintPanelOptions::ShowsOrientation
            | NSPrintPanelOptions::ShowsPreview,
    );
    // Cancelling is not a failure.
    operation.runOperation();
    Ok(())
}

/// Runs `work` on the main thread, soon, from any thread.
pub fn on_main(work: impl FnOnce(MainThreadMarker) + Send + 'static) {
    use std::ffi::c_void;
    unsafe extern "C" {
        static _dispatch_main_q: c_void;
        fn dispatch_async_f(
            queue: *const c_void,
            context: *mut c_void,
            work: extern "C" fn(*mut c_void),
        );
    }
    type Work = Box<dyn FnOnce(MainThreadMarker) + Send>;
    extern "C" fn run(context: *mut c_void) {
        // SAFETY: `context` is the box made below, handed over once.
        let work = unsafe { Box::from_raw(context.cast::<Work>()) };
        if let Some(mtm) = MainThreadMarker::new() {
            work(mtm);
        }
    }
    let work: Box<Work> = Box::new(Box::new(work));
    // SAFETY: the main queue lives as long as the process; `run` takes
    // ownership of the box.
    unsafe {
        dispatch_async_f(
            (&raw const _dispatch_main_q).cast(),
            Box::into_raw(work).cast(),
            run,
        );
    }
}
