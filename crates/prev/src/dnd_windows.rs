//! Drag and drop on Windows, through OLE, in the types the Wayland side
//! uses: a drop target on each window reports drags as `DragEvent`s with
//! the data in the MIME types prev reads, and drags prev starts offer
//! their data in the clipboard formats other apps read.

#![allow(unsafe_code)]

use std::cell::RefCell;
use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};

use windows::Win32::Foundation::{
    DRAGDROP_S_CANCEL, DRAGDROP_S_DROP, DRAGDROP_S_USEDEFAULTCURSORS, DV_E_FORMATETC, E_NOTIMPL,
    HGLOBAL, HWND, OLE_E_ADVISENOTSUPPORTED, POINT, POINTL, S_OK,
};
use windows::Win32::Graphics::Gdi::ScreenToClient;
use windows::Win32::System::Com::{
    DATADIR_GET, DVASPECT_CONTENT, FORMATETC, IAdviseSink, IDataObject, IDataObject_Impl,
    IEnumFORMATETC, IEnumSTATDATA, STGMEDIUM, STGMEDIUM_0, TYMED_HGLOBAL, TYMED_ISTREAM,
};
use windows::Win32::System::DataExchange::RegisterClipboardFormatW;
use windows::Win32::System::Memory::{
    GMEM_MOVEABLE, GlobalAlloc, GlobalLock, GlobalSize, GlobalUnlock,
};
use windows::Win32::System::Ole::{
    CF_DIB, CF_HDROP, CF_UNICODETEXT, DROPEFFECT, DROPEFFECT_COPY, DROPEFFECT_MOVE,
    DROPEFFECT_NONE, DoDragDrop, IDropSource, IDropSource_Impl, IDropTarget, IDropTarget_Impl,
    OleInitialize, RegisterDragDrop, ReleaseStgMedium,
};
use windows::Win32::System::SystemServices::{MK_LBUTTON, MK_SHIFT, MODIFIERKEYS_FLAGS};
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::Shell::{DROPFILES, DragQueryFileW, HDROP, SHCreateStdEnumFmtEtc};
use windows::core::{BOOL, Error, HRESULT, HSTRING, Ref, implement};

use crate::dnd::{Action, Drag, DragEvent, URI_LIST_MIME};

type Handler = Box<dyn Fn(DragEvent) + Send + Sync>;

static HANDLER: RwLock<Option<Handler>> = RwLock::new(None);
static ACCEPTED: RwLock<Vec<String>> = RwLock::new(Vec::new());
static PREFER_MOVE: AtomicBool = AtomicBool::new(false);

const TEXT_MIME: &str = "text/plain;charset=utf-8";
const PAGES_MIME: &str = crate::paste::PAGES_TYPE;

pub fn set_drag_handler(handler: impl Fn(DragEvent) + Send + Sync + 'static) {
    *HANDLER.write().unwrap_or_else(|poison| poison.into_inner()) = Some(Box::new(handler));
}

pub fn set_accepted_mimes(mimes: Vec<String>) {
    *ACCEPTED
        .write()
        .unwrap_or_else(|poison| poison.into_inner()) = mimes;
}

pub fn set_prefer_move(prefer: bool) {
    PREFER_MOVE.store(prefer, Ordering::Relaxed);
}

/// TEMPORARY: logs drag and drop steps next to prev.exe in development
/// builds, while Windows drag and drop is being brought up.
fn trace(message: &str) {
    if prev_store::paths::PRODUCTION {
        return;
    }
    use std::io::Write;
    let Some(folder) = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(std::path::Path::to_path_buf))
    else {
        return;
    };
    if let Ok(mut log) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(folder.join("dnd.log"))
    {
        let _ = writeln!(log, "{message}");
    }
}

fn emit(event: DragEvent) {
    trace(&format!("emit {:?}", short(&event)));
    if let Ok(handler) = HANDLER.read()
        && let Some(handler) = handler.as_ref()
    {
        handler(event);
    }
}

/// The accepted type to take from `offered`, if any; a type ending in `*`
/// takes any type starting with the rest, as on Wayland.
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

fn registered(name: &str) -> u16 {
    // SAFETY: the name is a valid wide string for the call.
    unsafe { RegisterClipboardFormatW(&HSTRING::from(name)) as u16 }
}

fn formatetc(format: u16, index: i32, tymed: u32) -> FORMATETC {
    FORMATETC {
        cfFormat: format,
        ptd: std::ptr::null_mut(),
        dwAspect: DVASPECT_CONTENT.0,
        lindex: index,
        tymed,
    }
}

/// Makes OLE usable on this thread; repeated calls do nothing more.
fn ensure_ole() {
    // SAFETY: no reserved pointer; the UI thread is single threaded.
    let result = unsafe { OleInitialize(None) };
    trace(&format!("OleInitialize {result:?}"));
}

/// A drag event without its data, for the trace.
fn short(event: &DragEvent) -> String {
    match event {
        DragEvent::Dropped {
            mime, data, x, y, ..
        } => {
            format!("Dropped {mime} {} bytes at {x:.0},{y:.0}", data.len())
        }
        other => format!("{other:?}"),
    }
}

/// Lets `hwnd` take drops. Windows drop targets belong to the window's
/// thread, so this runs on the UI thread, once the window exists.
pub fn register(hwnd: usize) {
    ensure_ole();
    let hwnd = HWND(hwnd as *mut _);
    let target: IDropTarget = Target {
        hwnd,
        chosen: RefCell::new(None),
    }
    .into();
    // SAFETY: `hwnd` is a live window of this thread; OLE keeps its own
    // reference to the target.
    let result = unsafe { RegisterDragDrop(hwnd, &target) };
    trace(&format!("RegisterDragDrop {hwnd:?} {result:?}"));
}

// --- Drops onto prev ---------------------------------------------------

/// The Windows formats a drag carries, as prev's MIME types.
fn offered(data: &IDataObject) -> Vec<String> {
    let has = |format: u16, tymed: u32| {
        // SAFETY: the FORMATETC lives through the call.
        unsafe { data.QueryGetData(&formatetc(format, -1, tymed)) == S_OK }
    };
    let hglobal = TYMED_HGLOBAL.0 as u32;
    let mut types = Vec::new();
    if has(registered(PAGES_MIME), hglobal) {
        types.push(PAGES_MIME.to_owned());
    }
    if has(registered("PNG"), hglobal) {
        types.push("image/png".to_owned());
    }
    if has(CF_DIB.0, hglobal) {
        types.push("image/bmp".to_owned());
    }
    // A browser's dragged picture: its name and contents.
    if has(registered("FileGroupDescriptorW"), hglobal)
        && let Some(name) = described_name(data)
    {
        types.push(format!("application/octet-stream;name=\"{name}\""));
    }
    if has(CF_HDROP.0, hglobal) || has(registered("UniformResourceLocatorW"), hglobal) {
        types.push(URI_LIST_MIME.to_owned());
    }
    if has(CF_UNICODETEXT.0, hglobal) {
        types.push(TEXT_MIME.to_owned());
    }
    types
}

/// The bytes of `format` from `data`, if it offers it as memory or a
/// stream.
fn bytes(data: &IDataObject, format: u16, index: i32) -> Option<Vec<u8>> {
    let request = formatetc(format, index, (TYMED_HGLOBAL.0 | TYMED_ISTREAM.0) as u32);
    // SAFETY: the medium is read, then released as OLE requires.
    unsafe {
        let mut medium = data.GetData(&request).ok()?;
        let read = if medium.tymed == TYMED_HGLOBAL.0 as u32 {
            global_bytes(medium.u.hGlobal)
        } else if medium.tymed == TYMED_ISTREAM.0 as u32 {
            (*medium.u.pstm).as_ref().map(|stream| {
                let mut all = Vec::new();
                let mut chunk = vec![0u8; 64 * 1024];
                loop {
                    let mut got = 0u32;
                    let result = stream.Read(
                        chunk.as_mut_ptr().cast(),
                        chunk.len() as u32,
                        Some(&mut got),
                    );
                    if result.is_err() || got == 0 {
                        break;
                    }
                    all.extend_from_slice(&chunk[..got as usize]);
                }
                all
            })
        } else {
            None
        };
        ReleaseStgMedium(&mut medium);
        read
    }
}

/// A copy of the memory `global` holds.
///
/// # Safety
/// `global` must be a live movable memory handle.
unsafe fn global_bytes(global: HGLOBAL) -> Option<Vec<u8>> {
    unsafe {
        let size = GlobalSize(global);
        let pointer = GlobalLock(global);
        if pointer.is_null() {
            return None;
        }
        let bytes = std::slice::from_raw_parts(pointer.cast::<u8>(), size).to_vec();
        let _ = GlobalUnlock(global);
        Some(bytes)
    }
}

/// UTF-16 text up to its first NUL.
fn wide_text(bytes: &[u8]) -> String {
    let units: Vec<u16> = bytes
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u16::from_le_bytes(*pair))
        .take_while(|unit| *unit != 0)
        .collect();
    String::from_utf16_lossy(&units)
}

/// The name of the first file a FileGroupDescriptorW describes.
fn described_name(data: &IDataObject) -> Option<String> {
    let descriptor = bytes(data, registered("FileGroupDescriptorW"), -1)?;
    // cItems, then FILEDESCRIPTORW: its name starts 72 bytes in and is 260
    // UTF-16 units long.
    let name = descriptor.get(4 + 72..4 + 72 + 520)?;
    let name = wide_text(name);
    (!name.is_empty()).then_some(name)
}

/// The files of a CF_HDROP.
fn dropped_files(data: &IDataObject) -> Vec<std::path::PathBuf> {
    let request = formatetc(CF_HDROP.0, -1, TYMED_HGLOBAL.0 as u32);
    // SAFETY: the HDROP is queried while the medium is held, then released.
    unsafe {
        let Ok(mut medium) = data.GetData(&request) else {
            return Vec::new();
        };
        let drop = HDROP(medium.u.hGlobal.0);
        let count = DragQueryFileW(drop, u32::MAX, None);
        let mut files = Vec::new();
        for index in 0..count {
            let length = DragQueryFileW(drop, index, None) as usize;
            let mut name = vec![0u16; length + 1];
            DragQueryFileW(drop, index, Some(&mut name));
            name.truncate(length);
            files.push(std::path::PathBuf::from(String::from_utf16_lossy(&name)));
        }
        ReleaseStgMedium(&mut medium);
        files
    }
}

/// The dropped data as `mime`.
fn read(data: &IDataObject, mime: &str) -> Vec<u8> {
    match mime {
        URI_LIST_MIME => {
            let files = dropped_files(data);
            if files.is_empty() {
                let url = bytes(data, registered("UniformResourceLocatorW"), -1)
                    .map(|bytes| wide_text(&bytes))
                    .unwrap_or_default();
                return format!("{url}\r\n").into_bytes();
            }
            files
                .iter()
                .flat_map(|path| crate::drag::uri_list(path))
                .collect()
        }
        TEXT_MIME => bytes(data, CF_UNICODETEXT.0, -1)
            .map(|bytes| wide_text(&bytes).into_bytes())
            .unwrap_or_default(),
        "image/png" => bytes(data, registered("PNG"), -1).unwrap_or_default(),
        "image/bmp" => bytes(data, CF_DIB.0, -1)
            .and_then(|dib| crate::paste::bmp_file(&dib))
            .unwrap_or_default(),
        PAGES_MIME => bytes(data, registered(PAGES_MIME), -1).unwrap_or_default(),
        named if named.starts_with("application/octet-stream;name=") => {
            bytes(data, registered("FileContents"), 0).unwrap_or_default()
        }
        _ => Vec::new(),
    }
}

#[implement(IDropTarget)]
struct Target {
    hwnd: HWND,
    /// The type the drag over the window would be taken in.
    chosen: RefCell<Option<String>>,
}

impl Target {
    /// Window-relative logical coordinates of a screen point.
    fn local(&self, point: &POINTL) -> (f64, f64) {
        let mut local = POINT {
            x: point.x,
            y: point.y,
        };
        // SAFETY: the point lives through the calls; the window is ours.
        let scale = unsafe {
            let _ = ScreenToClient(self.hwnd, &mut local);
            f64::from(GetDpiForWindow(self.hwnd).max(96)) / 96.0
        };
        (f64::from(local.x) / scale, f64::from(local.y) / scale)
    }

    fn surface(&self) -> usize {
        self.hwnd.0 as usize
    }

    /// What the drop would do: move with Shift when allowed, else copy.
    fn effect(&self, keys: MODIFIERKEYS_FLAGS, allowed: DROPEFFECT) -> DROPEFFECT {
        if self.chosen.borrow().is_none() {
            return DROPEFFECT_NONE;
        }
        let shift = keys.0 & MK_SHIFT.0 != 0 || PREFER_MOVE.load(Ordering::Relaxed);
        if shift && allowed.0 & DROPEFFECT_MOVE.0 != 0 {
            DROPEFFECT_MOVE
        } else if allowed.0 & DROPEFFECT_COPY.0 != 0 {
            DROPEFFECT_COPY
        } else {
            DROPEFFECT(allowed.0 & DROPEFFECT_MOVE.0)
        }
    }
}

impl IDropTarget_Impl for Target_Impl {
    fn DragEnter(
        &self,
        data: Ref<IDataObject>,
        keys: MODIFIERKEYS_FLAGS,
        point: &POINTL,
        effect: *mut DROPEFFECT,
    ) -> windows::core::Result<()> {
        let offered = data.ok().ok().map(offered).unwrap_or_default();
        trace(&format!("DragEnter offered {offered:?}"));
        let chosen = choose_mime(&offered);
        let accepted = chosen.is_some();
        *self.chosen.borrow_mut() = chosen;
        let (x, y) = self.local(point);
        emit(DragEvent::Entered {
            surface: self.surface(),
            x,
            y,
            accepted,
        });
        // SAFETY: OLE passes a valid effect to read and write.
        unsafe { *effect = self.effect(keys, *effect) };
        Ok(())
    }

    fn DragOver(
        &self,
        keys: MODIFIERKEYS_FLAGS,
        point: &POINTL,
        effect: *mut DROPEFFECT,
    ) -> windows::core::Result<()> {
        let (x, y) = self.local(point);
        emit(DragEvent::Moved {
            surface: self.surface(),
            x,
            y,
        });
        // SAFETY: as in DragEnter.
        unsafe { *effect = self.effect(keys, *effect) };
        Ok(())
    }

    fn DragLeave(&self) -> windows::core::Result<()> {
        *self.chosen.borrow_mut() = None;
        emit(DragEvent::Left {
            surface: self.surface(),
        });
        Ok(())
    }

    fn Drop(
        &self,
        data: Ref<IDataObject>,
        keys: MODIFIERKEYS_FLAGS,
        point: &POINTL,
        effect: *mut DROPEFFECT,
    ) -> windows::core::Result<()> {
        // SAFETY: as in DragEnter.
        let done = unsafe { self.effect(keys, *effect) };
        unsafe { *effect = done };
        let Some(mime) = self.chosen.borrow_mut().take() else {
            return Ok(());
        };
        let Ok(data) = data.ok() else {
            return Ok(());
        };
        let (x, y) = self.local(point);
        emit(DragEvent::Dropped {
            surface: self.surface(),
            x,
            y,
            data: read(data, &mime),
            mime,
            action: if done == DROPEFFECT_MOVE {
                Action::Move
            } else {
                Action::Copy
            },
        });
        Ok(())
    }
}

// --- Drags out of prev -------------------------------------------------

/// `data` in Windows formats: files as CF_HDROP, PNG also as a bitmap,
/// text as UTF-16, and prev's pages as its own format.
fn formats(data: &[(String, Vec<u8>)]) -> Vec<(u16, Vec<u8>)> {
    let mut formats: Vec<(u16, Vec<u8>)> = Vec::new();
    let mut add = |format: u16, bytes: Vec<u8>| {
        if !formats.iter().any(|(known, _)| *known == format) {
            formats.push((format, bytes));
        }
    };
    for (mime, bytes) in data {
        match mime.as_str() {
            URI_LIST_MIME => {
                let files: Vec<std::path::PathBuf> =
                    crate::dnd::parse_uri_list(&String::from_utf8_lossy(bytes))
                        .iter()
                        .filter_map(|uri| crate::dialog::file_uri_to_path(uri))
                        .collect();
                if !files.is_empty() {
                    add(CF_HDROP.0, drop_files(&files));
                }
            }
            "image/png" => {
                add(registered("PNG"), bytes.clone());
                if let Some(bitmap) = crate::paste::decode(bytes, prev_image::ImageFormat::Png) {
                    add(CF_DIB.0, crate::paste::dib(&bitmap));
                }
            }
            mime if mime.starts_with("text/plain") => {
                let mut wide: Vec<u8> = String::from_utf8_lossy(bytes)
                    .encode_utf16()
                    .flat_map(u16::to_le_bytes)
                    .collect();
                wide.extend_from_slice(&[0, 0]);
                add(CF_UNICODETEXT.0, wide);
            }
            PAGES_MIME => add(registered(PAGES_MIME), bytes.clone()),
            _ => {}
        }
    }
    formats
}

/// A DROPFILES block: the header, then the wide paths, each ended by a
/// NUL, and one more NUL at the end.
fn drop_files(files: &[std::path::PathBuf]) -> Vec<u8> {
    let header = DROPFILES {
        pFiles: std::mem::size_of::<DROPFILES>() as u32,
        fWide: BOOL(1),
        ..Default::default()
    };
    // SAFETY: DROPFILES is plain data.
    let mut block = unsafe {
        std::slice::from_raw_parts(
            (&raw const header).cast::<u8>(),
            std::mem::size_of::<DROPFILES>(),
        )
    }
    .to_vec();
    for file in files {
        for unit in file.to_string_lossy().encode_utf16().chain([0]) {
            block.extend_from_slice(&unit.to_le_bytes());
        }
    }
    block.extend_from_slice(&[0, 0]);
    block
}

#[implement(IDataObject)]
struct DataObject {
    formats: Vec<(u16, Vec<u8>)>,
}

impl DataObject_Impl {
    fn find(&self, request: *const FORMATETC) -> Option<&[u8]> {
        // SAFETY: OLE passes a valid FORMATETC.
        let request = unsafe { request.as_ref()? };
        if request.tymed & TYMED_HGLOBAL.0 as u32 == 0 {
            return None;
        }
        self.formats
            .iter()
            .find(|(format, _)| *format == request.cfFormat)
            .map(|(_, bytes)| bytes.as_slice())
    }
}

impl IDataObject_Impl for DataObject_Impl {
    fn GetData(&self, request: *const FORMATETC) -> windows::core::Result<STGMEDIUM> {
        let bytes = self
            .find(request)
            .ok_or_else(|| Error::from_hresult(DV_E_FORMATETC))?;
        // SAFETY: the memory is sized for the bytes and handed to the
        // caller, who frees it.
        unsafe {
            let global = GlobalAlloc(GMEM_MOVEABLE, bytes.len().max(1))?;
            let pointer = GlobalLock(global);
            if pointer.is_null() {
                return Err(Error::from_hresult(DV_E_FORMATETC));
            }
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), pointer.cast::<u8>(), bytes.len());
            let _ = GlobalUnlock(global);
            Ok(STGMEDIUM {
                tymed: TYMED_HGLOBAL.0 as u32,
                u: STGMEDIUM_0 { hGlobal: global },
                pUnkForRelease: std::mem::ManuallyDrop::new(None),
            })
        }
    }

    fn GetDataHere(
        &self,
        _request: *const FORMATETC,
        _medium: *mut STGMEDIUM,
    ) -> windows::core::Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }

    fn QueryGetData(&self, request: *const FORMATETC) -> HRESULT {
        if self.find(request).is_some() {
            S_OK
        } else {
            DV_E_FORMATETC
        }
    }

    fn GetCanonicalFormatEtc(
        &self,
        _request: *const FORMATETC,
        _canonical: *mut FORMATETC,
    ) -> HRESULT {
        E_NOTIMPL
    }

    fn SetData(
        &self,
        _format: *const FORMATETC,
        _medium: *const STGMEDIUM,
        _release: BOOL,
    ) -> windows::core::Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }

    fn EnumFormatEtc(&self, direction: u32) -> windows::core::Result<IEnumFORMATETC> {
        if direction != DATADIR_GET.0 as u32 {
            return Err(Error::from_hresult(E_NOTIMPL));
        }
        let formats: Vec<FORMATETC> = self
            .formats
            .iter()
            .map(|(format, _)| formatetc(*format, -1, TYMED_HGLOBAL.0 as u32))
            .collect();
        // SAFETY: the list lives through the call, which copies it.
        unsafe { SHCreateStdEnumFmtEtc(&formats) }
    }

    fn DAdvise(
        &self,
        _format: *const FORMATETC,
        _flags: u32,
        _sink: Ref<IAdviseSink>,
    ) -> windows::core::Result<u32> {
        Err(Error::from_hresult(OLE_E_ADVISENOTSUPPORTED))
    }

    fn DUnadvise(&self, _connection: u32) -> windows::core::Result<()> {
        Err(Error::from_hresult(OLE_E_ADVISENOTSUPPORTED))
    }

    fn EnumDAdvise(&self) -> windows::core::Result<IEnumSTATDATA> {
        Err(Error::from_hresult(OLE_E_ADVISENOTSUPPORTED))
    }
}

#[implement(IDropSource)]
struct Source;

impl IDropSource_Impl for Source_Impl {
    fn QueryContinueDrag(&self, escape: BOOL, keys: MODIFIERKEYS_FLAGS) -> HRESULT {
        if escape.as_bool() {
            DRAGDROP_S_CANCEL
        } else if keys.0 & MK_LBUTTON.0 == 0 {
            DRAGDROP_S_DROP
        } else {
            S_OK
        }
    }

    fn GiveFeedback(&self, _effect: DROPEFFECT) -> HRESULT {
        DRAGDROP_S_USEDEFAULTCURSORS
    }
}

/// Starts `drag` while the pointer button is held, and reports how it
/// ended through the handler. Windows runs the drag until the button is
/// let go; this returns then.
pub fn start_drag(drag: Drag) -> bool {
    ensure_ole();
    let formats = formats(&drag.data);
    if formats.is_empty() {
        return false;
    }
    let data: IDataObject = DataObject { formats }.into();
    let source: IDropSource = Source.into();
    let allowed = if drag.allow_move {
        DROPEFFECT(DROPEFFECT_COPY.0 | DROPEFFECT_MOVE.0)
    } else {
        DROPEFFECT_COPY
    };
    let mut effect = DROPEFFECT_NONE;
    // SAFETY: both objects live through the call, which runs the drag.
    let result = unsafe { DoDragDrop(&data, &source, allowed, &mut effect) };
    let action = (result == DRAGDROP_S_DROP).then(|| {
        if effect == DROPEFFECT_MOVE {
            Action::Move
        } else {
            Action::Copy
        }
    });
    emit(DragEvent::SourceEnded { action });
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn files_become_a_wide_drop_list() {
        let block = drop_files(&[std::path::PathBuf::from(r"C:\a.pdf")]);
        let header = std::mem::size_of::<DROPFILES>();
        let names: Vec<u16> = block[header..]
            .as_chunks::<2>()
            .0
            .iter()
            .map(|pair| u16::from_le_bytes(*pair))
            .collect();
        let expected: Vec<u16> = r"C:\a.pdf".encode_utf16().chain([0, 0]).collect();
        assert_eq!(names, expected);
    }

    #[test]
    fn wide_text_stops_at_nul() {
        let bytes: Vec<u8> = "hi\0there"
            .encode_utf16()
            .flat_map(u16::to_le_bytes)
            .collect();
        assert_eq!(wide_text(&bytes), "hi");
    }
}
