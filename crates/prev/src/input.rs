//! The language the user types in, which sets the side an empty text
//! field starts on: the keyboard layout in use on Linux, the input
//! language on Windows, the keyboard input source on macOS. Once a field
//! has text it follows the text's own direction instead.

use std::time::Duration;

/// How often the keyboard layout is looked at. Neither X11, Wayland (as
/// winit passes it on) nor Windows sends prev a change it can wait for.
const POLL: Duration = Duration::from_millis(250);

/// Whether the keyboard layout in use types a right to left script, or
/// `None` when the system does not say.
#[cfg(target_os = "linux")]
pub fn keyboard_right_to_left() -> Option<bool> {
    // What the layout types on the letter keys shows its script.
    let letters = winit::platform::keyboard_layout::letters()?;
    crate::ui::dir::text_direction(&letters)
}

#[cfg(windows)]
static INTERFACE_THREAD: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// Notes the thread the interface runs on: Windows keeps an input
/// language for each thread.
#[cfg(windows)]
#[allow(unsafe_code)]
pub fn remember_interface_thread() {
    let thread = unsafe { windows_sys::Win32::System::Threading::GetCurrentThreadId() };
    INTERFACE_THREAD.store(thread, std::sync::atomic::Ordering::Relaxed);
}

#[cfg(windows)]
#[allow(unsafe_code)]
pub fn keyboard_right_to_left() -> Option<bool> {
    use windows_sys::Win32::Globalization::{
        GetLocaleInfoEx, LCIDToLocaleName, LOCALE_IREADINGLAYOUT, LOCALE_RETURN_NUMBER,
    };
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetKeyboardLayout;

    let thread = INTERFACE_THREAD.load(std::sync::atomic::Ordering::Relaxed);
    // The low word of the keyboard layout handle is its language.
    let layout = unsafe { GetKeyboardLayout(thread) } as usize;
    let language = (layout & 0xffff) as u32;
    let mut name = [0u16; 85];
    let length = unsafe { LCIDToLocaleName(language, name.as_mut_ptr(), name.len() as i32, 0) };
    if length == 0 {
        return None;
    }
    // 0 is left to right and 1 right to left; the other values are
    // vertical layouts, which fields show left to right.
    let mut reading: u32 = 0;
    let written = unsafe {
        GetLocaleInfoEx(
            name.as_ptr(),
            LOCALE_IREADINGLAYOUT | LOCALE_RETURN_NUMBER,
            (&mut reading as *mut u32).cast(),
            2,
        )
    };
    (written != 0).then_some(reading == 1)
}

/// The first language of the keyboard input source in use, from the
/// Text Input Sources API, which must be asked on the main thread.
#[cfg(target_os = "macos")]
#[allow(unsafe_code)]
pub fn keyboard_right_to_left() -> Option<bool> {
    use std::ffi::{c_char, c_void};

    #[link(name = "Carbon", kind = "framework")]
    unsafe extern "C" {
        fn TISCopyCurrentKeyboardInputSource() -> *const c_void;
        fn TISGetInputSourceProperty(source: *const c_void, key: *const c_void) -> *const c_void;
        static kTISPropertyInputSourceLanguages: *const c_void;
    }
    #[link(name = "CoreFoundation", kind = "framework")]
    unsafe extern "C" {
        fn CFArrayGetCount(array: *const c_void) -> isize;
        fn CFArrayGetValueAtIndex(array: *const c_void, index: isize) -> *const c_void;
        fn CFStringGetCString(
            string: *const c_void,
            buffer: *mut c_char,
            size: isize,
            encoding: u32,
        ) -> u8;
        fn CFRelease(object: *const c_void);
    }
    unsafe extern "C" {
        static _dispatch_main_q: c_void;
        fn dispatch_sync_f(
            queue: *const c_void,
            context: *mut c_void,
            work: extern "C" fn(*mut c_void),
        );
    }
    const UTF8: u32 = 0x0800_0100;

    extern "C" fn first_language(context: *mut c_void) {
        // SAFETY: `context` is the `Option<String>` below, alive while
        // `dispatch_sync_f` waits; the source is released after use, and
        // the property and its strings belong to it.
        unsafe {
            let result = &mut *context.cast::<Option<String>>();
            let source = TISCopyCurrentKeyboardInputSource();
            if source.is_null() {
                return;
            }
            let languages = TISGetInputSourceProperty(source, kTISPropertyInputSourceLanguages);
            if !languages.is_null() && CFArrayGetCount(languages) > 0 {
                let language = CFArrayGetValueAtIndex(languages, 0);
                let mut buffer = [0 as c_char; 64];
                if CFStringGetCString(language, buffer.as_mut_ptr(), buffer.len() as isize, UTF8)
                    != 0
                {
                    *result = Some(
                        std::ffi::CStr::from_ptr(buffer.as_ptr())
                            .to_string_lossy()
                            .into_owned(),
                    );
                }
            }
            CFRelease(source);
        }
    }

    let mut language: Option<String> = None;
    // SAFETY: the main queue outlives the call, which waits for the work.
    unsafe {
        dispatch_sync_f(
            (&raw const _dispatch_main_q).cast(),
            (&raw mut language).cast(),
            first_language,
        );
    }
    language.map(|tag| crate::i18n::is_right_to_left(&tag))
}

#[cfg(not(any(target_os = "linux", windows, target_os = "macos")))]
pub fn keyboard_right_to_left() -> Option<bool> {
    None
}

/// The keyboard's direction each time it changes, starting with the
/// current one.
pub fn keyboard_changes() -> impl iced::futures::Stream<Item = Option<bool>> {
    let (mut sender, receiver) = iced::futures::channel::mpsc::channel(4);
    std::thread::Builder::new()
        .name("prev keyboard layout".into())
        .spawn(move || {
            let mut last = None;
            loop {
                let now = keyboard_right_to_left();
                if last != Some(now) {
                    match sender.try_send(now) {
                        Ok(()) => last = Some(now),
                        Err(error) if error.is_disconnected() => return,
                        // Full: tried again next time.
                        Err(_) => {}
                    }
                }
                std::thread::sleep(POLL);
            }
        })
        .expect("start the keyboard layout thread");
    receiver
}
