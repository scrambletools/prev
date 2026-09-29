//! Smithay Clipboard
//!
//! Provides access to the Wayland clipboard for gui applications. The user
//! should have surface around.

#![deny(clippy::all, clippy::if_not_else, clippy::enum_glob_use)]
use std::ffi::c_void;
use std::io::Result;
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::JoinHandle;

use sctk::reexports::calloop::channel::{self, Sender};
use sctk::reexports::client::Connection;
use sctk::reexports::client::backend::Backend;

pub mod dnd;
mod mime;
mod state;
mod worker;

/// Access to a Wayland clipboard.
///
/// Every `Clipboard` on one display shares a single worker, which keeps
/// running when they are dropped, until [`shutdown`]. Toolkits make a
/// clipboard per window and drop it with the window; releasing the
/// worker's data devices while the compositor is sending them offers can
/// leave a hole in libwayland's object map, which ends the connection.
pub struct Clipboard {
    shared: Arc<Shared>,
}

struct Shared {
    display: usize,
    request_sender: Sender<worker::Command>,
    request_receiver: Mutex<Receiver<Result<String>>>,
    thread: Mutex<Option<JoinHandle<()>>>,
}

static SHARED: Mutex<Option<Arc<Shared>>> = Mutex::new(None);

impl Clipboard {
    /// Creates a clipboard, starting the worker thread with its own event
    /// queue the first time, and sharing it after that.
    ///
    /// # Safety
    ///
    /// `display` must be a valid `*mut wl_display` pointer, and it must remain
    /// valid until [`shutdown`] is called.
    pub unsafe fn new(display: *mut c_void) -> Self {
        let mut shared = SHARED.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(existing) = shared.as_ref().filter(|s| s.display == display as usize) {
            return Self { shared: Arc::clone(existing) };
        }
        // A different display: the old worker's is gone.
        if let Some(old) = shared.take() {
            stop(&old);
        }

        let backend = unsafe { Backend::from_foreign_display(display.cast()) };
        let connection = Connection::from_backend(backend);

        // Create channel to send data to clipboard thread.
        let (request_sender, rx_chan) = channel::channel();
        dnd::set_commands(request_sender.clone());
        // Create channel to get data from the clipboard thread.
        let (clipboard_reply_sender, request_receiver) = mpsc::channel();

        let name = String::from("smithay-clipboard");
        let thread = worker::spawn(name, connection, rx_chan, clipboard_reply_sender);

        let new = Arc::new(Shared {
            display: display as usize,
            request_sender,
            request_receiver: Mutex::new(request_receiver),
            thread: Mutex::new(thread),
        });
        *shared = Some(Arc::clone(&new));
        Self { shared: new }
    }

    /// Sends a request and waits for its reply, one request at a time so
    /// each reply reaches its caller.
    fn request(&self, command: worker::Command) -> Result<String> {
        let receiver =
            self.shared.request_receiver.lock().unwrap_or_else(PoisonError::into_inner);
        let _ = self.shared.request_sender.send(command);
        // The clipboard thread is dead, however we shouldn't crash downstream, so
        // propogating an error.
        receiver.recv().unwrap_or_else(|_| Err(std::io::Error::other("clipboard is dead.")))
    }

    /// Load clipboard data.
    ///
    /// Loads content from a clipboard on a last observed seat.
    pub fn load(&self) -> Result<String> {
        self.request(worker::Command::Load)
    }

    /// Store to a clipboard.
    ///
    /// Stores to a clipboard on a last observed seat.
    pub fn store<T: Into<String>>(&self, text: T) {
        let request = worker::Command::Store(text.into());
        let _ = self.shared.request_sender.send(request);
    }

    /// Load primary clipboard data.
    ///
    /// Loads content from a  primary clipboard on a last observed seat.
    pub fn load_primary(&self) -> Result<String> {
        self.request(worker::Command::LoadPrimary)
    }

    /// Store to a primary clipboard.
    ///
    /// Stores to a primary clipboard on a last observed seat.
    pub fn store_primary<T: Into<String>>(&self, text: T) {
        let request = worker::Command::StorePrimary(text.into());
        let _ = self.shared.request_sender.send(request);
    }
}

/// Stops the shared worker and waits for it, releasing its devices. Call
/// it before the Wayland display is disconnected, as the application
/// exits; a clipboard made after it starts a new worker.
pub fn shutdown() {
    let old = SHARED.lock().unwrap_or_else(PoisonError::into_inner).take();
    if let Some(old) = old {
        stop(&old);
    }
}

fn stop(shared: &Shared) {
    let _ = shared.request_sender.send(worker::Command::Exit);
    let thread = shared.thread.lock().unwrap_or_else(PoisonError::into_inner).take();
    if let Some(thread) = thread {
        let _ = thread.join();
    }
}
