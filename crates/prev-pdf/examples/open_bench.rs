//! Times opening a PDF and drawing its first page, as the viewer does.
//! Usage: cargo run --release -p prev-pdf --example open_bench -- <file.pdf>

use std::sync::Arc;
use std::time::Instant;

use futures::executor::block_on;
use prev_pdf::engine::{page_pixels, zoom_to_scale};
use prev_pdf::geometry::PixelRect;
use prev_pdf::mupdf_engine::MupdfEngine;
use prev_pdf::worker::{DocumentHandle, Opened, RenderPool, Ticket, flatten};

fn main() {
    let path = std::env::args_os()
        .nth(1)
        .expect("usage: open_bench <file.pdf>");
    let pool = RenderPool::new(RenderPool::default_threads());
    let start = Instant::now();
    let (handle, opened) = DocumentHandle::open(Arc::new(MupdfEngine), path.into());
    let Opened::Ready(info) = flatten(block_on(opened)).expect("open") else {
        panic!("encrypted")
    };
    let opened_at = start.elapsed();

    let display = flatten(block_on(handle.display(0))).expect("page");
    let parsed_at = start.elapsed();
    // A 900 pixel wide window at scale 1.25, fitted to width.
    let size = display.size();
    let scale = zoom_to_scale((900.0 - 32.0) / zoom_to_scale(1.0) / size.width) * 1.25;
    let (width, height) = page_pixels(size, scale);
    let bitmap = flatten(block_on(pool.render(
        display,
        scale,
        PixelRect {
            x: 0,
            y: 0,
            width,
            height: height.min(1024),
        },
        0,
        Ticket::new(),
    )))
    .expect("render");
    let rendered_at = start.elapsed();

    println!("pages: {}", info.page_sizes.len());
    println!(
        "open + page sizes: {:.1} ms",
        opened_at.as_secs_f64() * 1000.0
    );
    println!(
        "first page parsed: {:.1} ms",
        parsed_at.as_secs_f64() * 1000.0
    );
    println!(
        "first page drawn ({}x{}): {:.1} ms",
        bitmap.width,
        bitmap.height,
        rendered_at.as_secs_f64() * 1000.0
    );
}
