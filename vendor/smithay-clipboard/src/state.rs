use std::borrow::Cow;
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::mem;
use std::os::unix::io::{AsRawFd, RawFd};
use std::rc::Rc;
use std::sync::mpsc::Sender;

use sctk::data_device_manager::data_device::{DataDevice, DataDeviceData, DataDeviceHandler};
use sctk::data_device_manager::data_offer::{DataOfferError, DataOfferHandler, DragOffer};
use sctk::data_device_manager::data_source::{CopyPasteSource, DataSourceHandler, DragSource};
use sctk::data_device_manager::{DataDeviceManagerState, WritePipe};
use sctk::primary_selection::PrimarySelectionManagerState;
use sctk::primary_selection::device::{PrimarySelectionDevice, PrimarySelectionDeviceHandler};
use sctk::primary_selection::selection::{PrimarySelectionSource, PrimarySelectionSourceHandler};
use sctk::registry::{ProvidesRegistryState, RegistryState};
use sctk::seat::pointer::{PointerData, PointerEvent, PointerEventKind, PointerHandler};
use sctk::seat::{Capability, SeatHandler, SeatState};
use sctk::shm::slot::{Buffer, SlotPool};
use sctk::shm::{Shm, ShmHandler};
use sctk::{
    delegate_data_device, delegate_pointer, delegate_primary_selection, delegate_registry,
    delegate_seat, delegate_shm, registry_handlers,
};

use sctk::reexports::calloop::{LoopHandle, PostAction};
use sctk::reexports::client::globals::GlobalList;
use sctk::reexports::client::protocol::wl_data_device::WlDataDevice;
use sctk::reexports::client::protocol::wl_data_device_manager::DndAction;
use sctk::reexports::client::protocol::wl_data_source::WlDataSource;
use sctk::reexports::client::protocol::wl_compositor::WlCompositor;
use sctk::reexports::client::protocol::wl_keyboard::WlKeyboard;
use sctk::reexports::client::protocol::wl_shm;
use sctk::reexports::client::protocol::wl_pointer::WlPointer;
use sctk::reexports::client::protocol::wl_seat::WlSeat;
use sctk::reexports::client::protocol::wl_surface::WlSurface;
use sctk::reexports::client::{Connection, Dispatch, Proxy, QueueHandle};
use sctk::reexports::protocols::wp::primary_selection::zv1::client::{
    zwp_primary_selection_device_v1::ZwpPrimarySelectionDeviceV1,
    zwp_primary_selection_source_v1::ZwpPrimarySelectionSourceV1,
};
use wayland_backend::client::ObjectId;

use crate::dnd::{self, Action, Drag, DragEvent};
use crate::mime::{ALLOWED_MIME_TYPES, MimeType, normalize_to_lf};

pub struct State {
    pub primary_selection_manager_state: Option<PrimarySelectionManagerState>,
    pub data_device_manager_state: Option<DataDeviceManagerState>,
    pub reply_tx: Sender<Result<String>>,
    pub exit: bool,

    registry_state: RegistryState,
    seat_state: SeatState,

    seats: HashMap<ObjectId, ClipboardSeatState>,
    /// The latest seat which got an event.
    latest_seat: Option<ObjectId>,

    loop_handle: LoopHandle<'static, Self>,
    queue_handle: QueueHandle<Self>,

    primary_sources: Vec<PrimarySelectionSource>,
    primary_selection_content: Rc<[u8]>,

    data_sources: Vec<CopyPasteSource>,
    data_selection_content: Rc<[u8]>,

    /// Surface under the current drag, if any.
    drag_surface: Option<usize>,
    /// The type a drag over the application would be taken in.
    drag_mime: Option<String>,
    /// A drag the application started, while it lasts.
    outgoing: Option<OutgoingDrag>,
    /// The action the target chose for the outgoing drag.
    outgoing_action: DndAction,
    compositor: Option<WlCompositor>,
    shm: Option<Shm>,
}

/// A drag the application started, and what it keeps alive.
struct OutgoingDrag {
    source: DragSource,
    allows_move: bool,
    data: Rc<Vec<(String, Vec<u8>)>>,
    icon: Option<(WlSurface, Buffer, SlotPool)>,
}

impl State {
    #[must_use]
    pub fn new(
        globals: &GlobalList,
        queue_handle: &QueueHandle<Self>,
        loop_handle: LoopHandle<'static, Self>,
        reply_tx: Sender<Result<String>>,
    ) -> Option<Self> {
        // NOTE: while it's mutable, it's not part of the hash compute.
        #[allow(clippy::mutable_key_type)]
        let mut seats = HashMap::new();

        let data_device_manager_state = DataDeviceManagerState::bind(globals, queue_handle).ok();
        let primary_selection_manager_state =
            PrimarySelectionManagerState::bind(globals, queue_handle).ok();

        // When both globals are not available nothing could be done.
        if data_device_manager_state.is_none() && primary_selection_manager_state.is_none() {
            return None;
        }

        let seat_state = SeatState::new(globals, queue_handle);
        for seat in seat_state.seats() {
            seats.insert(seat.id(), Default::default());
        }

        Some(Self {
            registry_state: RegistryState::new(globals),
            primary_selection_content: Rc::from([]),
            data_selection_content: Rc::from([]),
            queue_handle: queue_handle.clone(),
            primary_selection_manager_state,
            primary_sources: Vec::new(),
            data_device_manager_state,
            data_sources: Vec::new(),
            drag_surface: None,
            drag_mime: None,
            outgoing: None,
            outgoing_action: DndAction::empty(),
            // Only for drag icons; drags work without them.
            compositor: globals.bind::<WlCompositor, _, _>(queue_handle, 1..=4, ()).ok(),
            shm: Shm::bind(globals, queue_handle).ok(),
            latest_seat: None,
            loop_handle,
            exit: false,
            seat_state,
            reply_tx,
            seats,
        })
    }

    /// Store selection for the given target.
    ///
    /// Selection source is only created when `Some(())` is returned.
    pub fn store_selection(&mut self, ty: SelectionTarget, contents: String) -> Option<()> {
        let latest = self.latest_seat.as_ref()?;
        let seat = self.seats.get_mut(latest)?;

        if !seat.has_focus {
            return None;
        }

        let contents = Rc::from(contents.into_bytes());

        match ty {
            SelectionTarget::Clipboard => {
                let mgr = self.data_device_manager_state.as_ref()?;
                self.data_selection_content = contents;
                let source =
                    mgr.create_copy_paste_source(&self.queue_handle, ALLOWED_MIME_TYPES.iter());
                source.set_selection(seat.data_device.as_ref().unwrap(), seat.latest_serial);
                self.data_sources.push(source);
            },
            SelectionTarget::Primary => {
                let mgr = self.primary_selection_manager_state.as_ref()?;
                self.primary_selection_content = contents;
                let source =
                    mgr.create_selection_source(&self.queue_handle, ALLOWED_MIME_TYPES.iter());
                source.set_selection(seat.primary_device.as_ref().unwrap(), seat.latest_serial);
                self.primary_sources.push(source);
            },
        }

        Some(())
    }

    /// Load selection for the given target.
    pub fn load_selection(&mut self, ty: SelectionTarget) -> Result<()> {
        let latest = self
            .latest_seat
            .as_ref()
            .ok_or_else(|| Error::other("no events received on any seat"))?;
        let seat = self.seats.get_mut(latest).ok_or_else(|| Error::other("active seat lost"))?;

        if !seat.has_focus {
            return Err(Error::other("client doesn't have focus"));
        }

        let (read_pipe, mime_type) = match ty {
            SelectionTarget::Clipboard => {
                let selection = seat
                    .data_device
                    .as_ref()
                    .and_then(|data| data.data().selection_offer())
                    .ok_or_else(|| Error::other("selection is empty"))?;

                let mime_type =
                    selection.with_mime_types(MimeType::find_allowed).ok_or_else(|| {
                        Error::new(ErrorKind::NotFound, "supported mime-type is not found")
                    })?;

                (
                    selection.receive(mime_type.to_string()).map_err(|err| match err {
                        DataOfferError::InvalidReceive => Error::other("offer is not ready yet"),
                        DataOfferError::Io(err) => err,
                    })?,
                    mime_type,
                )
            },
            SelectionTarget::Primary => {
                let selection = seat
                    .primary_device
                    .as_ref()
                    .and_then(|data| data.data().selection_offer())
                    .ok_or_else(|| Error::other("selection is empty"))?;

                let mime_type =
                    selection.with_mime_types(MimeType::find_allowed).ok_or_else(|| {
                        Error::new(ErrorKind::NotFound, "supported mime-type is not found")
                    })?;

                (selection.receive(mime_type.to_string())?, mime_type)
            },
        };

        // Mark FD as non-blocking so we won't block ourselves.
        set_non_blocking(read_pipe.as_raw_fd())?;

        let mut reader_buffer = [0; 4096];
        let mut content = Vec::new();
        let _ = self.loop_handle.insert_source(read_pipe, move |_, file, state| {
            let file = unsafe { file.get_mut() };
            loop {
                match file.read(&mut reader_buffer) {
                    Ok(0) => {
                        let utf8 = String::from_utf8_lossy(&content);
                        let content = match utf8 {
                            Cow::Borrowed(_) => {
                                // Don't clone the read data.
                                let mut to_send = Vec::new();
                                mem::swap(&mut content, &mut to_send);
                                String::from_utf8(to_send).unwrap()
                            },
                            Cow::Owned(content) => content,
                        };

                        // Post-process the content according to mime type.
                        let content = match mime_type {
                            MimeType::TextPlainUtf8 | MimeType::TextPlain => {
                                normalize_to_lf(content)
                            },
                            MimeType::Utf8String => content,
                        };

                        let _ = state.reply_tx.send(Ok(content));
                        break PostAction::Remove;
                    },
                    Ok(n) => content.extend_from_slice(&reader_buffer[..n]),
                    Err(err) if err.kind() == ErrorKind::WouldBlock => break PostAction::Continue,
                    Err(err) => {
                        let _ = state.reply_tx.send(Err(err));
                        break PostAction::Remove;
                    },
                };
            }
        });

        Ok(())
    }

    /// Starts `drag` from the surface the pointer button is held on.
    pub fn start_drag(&mut self, drag: Drag) -> Option<()> {
        let latest = self.latest_seat.as_ref()?;
        let seat = self.seats.get(latest)?;
        let (origin, serial) = seat.pressed.clone()?;
        let device = seat.data_device.as_ref()?;
        let mgr = self.data_device_manager_state.as_ref()?;
        // A drag already under way is replaced.
        if let Some(previous) = self.outgoing.take() {
            previous.source.inner().destroy();
        }
        let actions =
            if drag.allow_move { DndAction::Copy | DndAction::Move } else { DndAction::Copy };
        let source = mgr.create_drag_and_drop_source(
            &self.queue_handle,
            drag.data.iter().map(|(mime, _)| mime.as_str()),
            actions,
        );
        let icon = drag.icon.as_ref().and_then(|icon| self.icon_surface(icon));
        source.start_drag(device, &origin, icon.as_ref().map(|(surface, ..)| surface), serial);
        if let Some((surface, ..)) = &icon {
            surface.commit();
        }
        self.outgoing_action = DndAction::empty();
        self.outgoing = Some(OutgoingDrag {
            source,
            allows_move: drag.allow_move,
            data: Rc::new(drag.data),
            icon,
        });
        Some(())
    }

    /// A surface showing `icon`, placed so the pointer is at its hotspot.
    fn icon_surface(&self, icon: &dnd::Icon) -> Option<(WlSurface, Buffer, SlotPool)> {
        let (compositor, shm) = (self.compositor.as_ref()?, self.shm.as_ref()?);
        let (width, height) = (icon.width as i32, icon.height as i32);
        if width <= 0 || height <= 0 || icon.rgba.len() != (width * height * 4) as usize {
            return None;
        }
        let mut pool = SlotPool::new((width * height * 4) as usize, shm).ok()?;
        let (buffer, canvas) =
            pool.create_buffer(width, height, width * 4, wl_shm::Format::Argb8888).ok()?;
        canvas.copy_from_slice(&dnd::to_argb8888(&icon.rgba));
        let surface = compositor.create_surface(&self.queue_handle, ());
        surface.attach(Some(buffer.wl_buffer()), -icon.hotspot.0, -icon.hotspot.1);
        surface.damage(0, 0, width, height);
        Some((surface, buffer, pool))
    }

    /// Ends the outgoing drag, reporting how.
    fn end_outgoing(&mut self, dropped: bool) {
        let Some(drag) = self.outgoing.take() else { return };
        drag.source.inner().destroy();
        if let Some((surface, ..)) = drag.icon {
            surface.destroy();
        }
        // As for drops: without a reported action, the modifier keys decide.
        let moved = if self.outgoing_action.is_empty() {
            drag.allows_move && dnd::prefer_move()
        } else {
            self.outgoing_action.contains(DndAction::Move)
        };
        let action = dropped.then_some(if moved { Action::Move } else { Action::Copy });
        dnd::emit(DragEvent::SourceEnded { action });
    }

    /// Writes the outgoing drag's data in `mime` to `write_pipe`.
    fn send_drag_data(&mut self, write_pipe: WritePipe, mime: String) {
        let Some(drag) = self.outgoing.as_ref() else { return };
        let data = drag.data.clone();
        let Some(index) = data.iter().position(|(offered, _)| *offered == mime) else { return };
        if set_non_blocking(write_pipe.as_raw_fd()).is_err() {
            return;
        }
        let mut written = 0;
        let _ = self.loop_handle.insert_source(write_pipe, move |_, file, _| {
            let file = unsafe { file.get_mut() };
            let contents = &data[index].1;
            loop {
                if written == contents.len() {
                    break PostAction::Remove;
                }
                match file.write(&contents[written..]) {
                    Ok(n) => written += n,
                    Err(err) if err.kind() == ErrorKind::WouldBlock => break PostAction::Continue,
                    Err(_) => break PostAction::Remove,
                }
            }
        });
    }

    /// Takes the drag in the accepted type it offers, with the action the
    /// user's modifiers ask for.
    fn accept_drag(&mut self, offer: &DragOffer) {
        let mime = if dnd::has_handler() {
            offer.with_mime_types(|mimes| dnd::choose_mime(mimes))
        } else {
            None
        };
        offer.accept_mime_type(offer.serial, mime.clone());
        match mime {
            Some(_) => {
                let preferred = if dnd::prefer_move() { DndAction::Move } else { DndAction::Copy };
                offer.set_actions(DndAction::Copy | DndAction::Move, preferred);
            },
            None => offer.set_actions(DndAction::empty(), DndAction::empty()),
        }
        self.drag_mime = mime;
    }

    fn send_request(&mut self, ty: SelectionTarget, write_pipe: WritePipe, mime: String) {
        // We can only send strings, so don't do anything with the mime-type.
        if MimeType::find_allowed(&[mime]).is_none() {
            return;
        }

        // Mark FD as non-blocking so we won't block ourselves.
        if set_non_blocking(write_pipe.as_raw_fd()).is_err() {
            return;
        }

        // Don't access the content on the state directly, since it could change during
        // the send.
        let contents = match ty {
            SelectionTarget::Clipboard => self.data_selection_content.clone(),
            SelectionTarget::Primary => self.primary_selection_content.clone(),
        };

        let mut written = 0;
        let _ = self.loop_handle.insert_source(write_pipe, move |_, file, _| {
            let file = unsafe { file.get_mut() };
            loop {
                match file.write(&contents[written..]) {
                    Ok(n) if written + n == contents.len() => {
                        written += n;
                        break PostAction::Remove;
                    },
                    Ok(n) => written += n,
                    Err(err) if err.kind() == ErrorKind::WouldBlock => break PostAction::Continue,
                    Err(_) => break PostAction::Remove,
                }
            }
        });
    }
}

impl SeatHandler for State {
    fn seat_state(&mut self) -> &mut SeatState {
        &mut self.seat_state
    }

    fn new_seat(&mut self, _: &Connection, _: &QueueHandle<Self>, seat: WlSeat) {
        self.seats.insert(seat.id(), Default::default());
    }

    fn new_capability(
        &mut self,
        _: &Connection,
        qh: &QueueHandle<Self>,
        seat: WlSeat,
        capability: Capability,
    ) {
        let seat_state = self.seats.get_mut(&seat.id()).unwrap();

        match capability {
            Capability::Keyboard => {
                seat_state.keyboard = Some(seat.get_keyboard(qh, seat.id()));

                // Selection sources are tied to the keyboard, so add/remove decives
                // when we gain/loss capability.

                if seat_state.data_device.is_none() && self.data_device_manager_state.is_some() {
                    seat_state.data_device = self
                        .data_device_manager_state
                        .as_ref()
                        .map(|mgr| mgr.get_data_device(qh, &seat));
                }

                if seat_state.primary_device.is_none()
                    && self.primary_selection_manager_state.is_some()
                {
                    seat_state.primary_device = self
                        .primary_selection_manager_state
                        .as_ref()
                        .map(|mgr| mgr.get_selection_device(qh, &seat));
                }
            },
            Capability::Pointer => {
                seat_state.pointer = self.seat_state.get_pointer(qh, &seat).ok();
            },
            _ => (),
        }
    }

    fn remove_capability(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        seat: WlSeat,
        capability: Capability,
    ) {
        let seat_state = self.seats.get_mut(&seat.id()).unwrap();
        match capability {
            Capability::Keyboard => {
                seat_state.data_device = None;
                seat_state.primary_device = None;

                if let Some(keyboard) = seat_state.keyboard.take() {
                    if keyboard.version() >= 3 {
                        keyboard.release()
                    }
                }
            },
            Capability::Pointer => {
                if let Some(pointer) = seat_state.pointer.take() {
                    if pointer.version() >= 3 {
                        pointer.release()
                    }
                }
            },
            _ => (),
        }
    }

    fn remove_seat(&mut self, _: &Connection, _: &QueueHandle<Self>, seat: WlSeat) {
        self.seats.remove(&seat.id());
    }
}

impl PointerHandler for State {
    fn pointer_frame(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        pointer: &WlPointer,
        events: &[PointerEvent],
    ) {
        let seat = pointer.data::<PointerData>().unwrap().seat();
        let seat_id = seat.id();
        let seat_state = match self.seats.get_mut(&seat_id) {
            Some(seat_state) => seat_state,
            None => return,
        };

        let mut updated_serial = false;
        for event in events {
            match event.kind {
                PointerEventKind::Press { serial, .. } => {
                    updated_serial = true;
                    seat_state.latest_serial = serial;
                    // Drags start from the held button's press.
                    seat_state.pressed = Some((event.surface.clone(), serial));
                },
                PointerEventKind::Release { serial, .. } => {
                    updated_serial = true;
                    seat_state.latest_serial = serial;
                    seat_state.pressed = None;
                },
                _ => (),
            }
        }

        // Only update the seat we're using when the serial got updated.
        if updated_serial {
            self.latest_seat = Some(seat_id);
        }
    }
}

fn drag_offer(data_device: &WlDataDevice) -> Option<DragOffer> {
    data_device.data::<DataDeviceData>()?.drag_offer()
}

fn surface_key(surface: &WlSurface) -> usize {
    surface.id().as_ptr() as usize
}

impl DataDeviceHandler for State {
    fn enter(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        data_device: &WlDataDevice,
        x: f64,
        y: f64,
        surface: &WlSurface,
    ) {
        let surface = surface_key(surface);
        self.drag_surface = Some(surface);
        let Some(offer) = drag_offer(data_device) else {
            dnd::emit(DragEvent::Entered { surface, x, y, accepted: false });
            return;
        };
        self.accept_drag(&offer);
        let accepted = self.drag_mime.is_some();
        dnd::emit(DragEvent::Entered { surface, x, y, accepted });
    }

    fn leave(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &WlDataDevice) {
        if let Some(surface) = self.drag_surface.take() {
            dnd::emit(DragEvent::Left { surface });
        }
    }

    fn motion(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        data_device: &WlDataDevice,
        x: f64,
        y: f64,
    ) {
        // Modifier keys may have changed copy to move.
        if let Some(offer) = drag_offer(data_device) {
            self.accept_drag(&offer);
        }
        if let Some(surface) = self.drag_surface {
            dnd::emit(DragEvent::Moved { surface, x, y });
        }
    }

    fn drop_performed(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        data_device: &WlDataDevice,
    ) {
        let Some(offer) = drag_offer(data_device) else { return };
        let surface = surface_key(&offer.surface);
        let (x, y) = (offer.x, offer.y);
        self.drag_surface = None;
        dnd::emit(DragEvent::Left { surface });

        let Some(mime) = self.drag_mime.take().filter(|_| dnd::has_handler()) else {
            offer.destroy();
            return;
        };
        // Some compositors (Hyprland) never report the chosen action;
        // then the user's modifier keys decide, as they would have.
        let moved = if offer.selected_action.is_empty() {
            dnd::prefer_move() && offer.source_actions.contains(DndAction::Move)
        } else {
            offer.selected_action.contains(DndAction::Move)
        };
        let action = if moved { Action::Move } else { Action::Copy };
        let read_pipe = match offer.receive(mime.clone()) {
            Ok(pipe) => pipe,
            Err(_) => {
                offer.destroy();
                return;
            },
        };
        if set_non_blocking(read_pipe.as_raw_fd()).is_err() {
            offer.destroy();
            return;
        }

        let mut reader_buffer = [0; 4096];
        let mut content = Vec::new();
        let _ = self.loop_handle.insert_source(read_pipe, move |_, file, _| {
            let file = unsafe { file.get_mut() };
            loop {
                match file.read(&mut reader_buffer) {
                    Ok(0) => {
                        offer.finish();
                        offer.destroy();
                        let data = mem::take(&mut content);
                        let mime = mime.clone();
                        dnd::emit(DragEvent::Dropped { surface, x, y, mime, data, action });
                        break PostAction::Remove;
                    },
                    Ok(n) => content.extend_from_slice(&reader_buffer[..n]),
                    Err(err) if err.kind() == ErrorKind::WouldBlock => break PostAction::Continue,
                    Err(_) => {
                        offer.destroy();
                        break PostAction::Remove;
                    },
                }
            }
        });
    }

    // The selection is finished and ready to be used.
    fn selection(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &WlDataDevice) {}
}

impl DataSourceHandler for State {
    fn send_request(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        source: &WlDataSource,
        mime: String,
        write_pipe: WritePipe,
    ) {
        if self.is_outgoing(source) {
            self.send_drag_data(write_pipe, mime);
        } else {
            self.send_request(SelectionTarget::Clipboard, write_pipe, mime)
        }
    }

    fn cancelled(&mut self, _: &Connection, _: &QueueHandle<Self>, deleted: &WlDataSource) {
        if self.is_outgoing(deleted) {
            self.end_outgoing(false);
            return;
        }
        self.data_sources.retain(|source| source.inner() != deleted)
    }

    fn accept_mime(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &WlDataSource,
        _: Option<String>,
    ) {
    }

    fn dnd_dropped(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &WlDataSource) {}

    fn action(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        source: &WlDataSource,
        action: DndAction,
    ) {
        if self.is_outgoing(source) {
            self.outgoing_action = action;
        }
    }

    fn dnd_finished(&mut self, _: &Connection, _: &QueueHandle<Self>, source: &WlDataSource) {
        if self.is_outgoing(source) {
            self.end_outgoing(true);
        }
    }
}

impl State {
    fn is_outgoing(&self, source: &WlDataSource) -> bool {
        self.outgoing.as_ref().is_some_and(|drag| drag.source.inner() == source)
    }
}

impl ShmHandler for State {
    fn shm_state(&mut self) -> &mut Shm {
        self.shm.as_mut().expect("shm events come only once it is bound")
    }
}

impl Dispatch<WlCompositor, ()> for State {
    fn event(
        _: &mut State,
        _: &WlCompositor,
        _: <WlCompositor as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<State>,
    ) {
    }
}

impl Dispatch<WlSurface, ()> for State {
    fn event(
        _: &mut State,
        _: &WlSurface,
        _: <WlSurface as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<State>,
    ) {
    }
}

impl DataOfferHandler for State {
    fn source_actions(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &mut DragOffer,
        _: DndAction,
    ) {
    }

    fn selected_action(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &mut DragOffer,
        _: DndAction,
    ) {
    }
}

impl ProvidesRegistryState for State {
    registry_handlers![SeatState];

    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
}

impl PrimarySelectionDeviceHandler for State {
    fn selection(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &ZwpPrimarySelectionDeviceV1,
    ) {
    }
}

impl PrimarySelectionSourceHandler for State {
    fn send_request(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &ZwpPrimarySelectionSourceV1,
        mime: String,
        write_pipe: WritePipe,
    ) {
        self.send_request(SelectionTarget::Primary, write_pipe, mime);
    }

    fn cancelled(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        deleted: &ZwpPrimarySelectionSourceV1,
    ) {
        self.primary_sources.retain(|source| source.inner() != deleted)
    }
}

impl Dispatch<WlKeyboard, ObjectId, State> for State {
    fn event(
        state: &mut State,
        _: &WlKeyboard,
        event: <WlKeyboard as sctk::reexports::client::Proxy>::Event,
        data: &ObjectId,
        _: &Connection,
        _: &QueueHandle<State>,
    ) {
        use sctk::reexports::client::protocol::wl_keyboard::Event as WlKeyboardEvent;
        let seat_state = match state.seats.get_mut(data) {
            Some(seat_state) => seat_state,
            None => return,
        };
        match event {
            WlKeyboardEvent::Key { serial, .. } | WlKeyboardEvent::Modifiers { serial, .. } => {
                seat_state.latest_serial = serial;
                state.latest_seat = Some(data.clone());
            },
            // NOTE both selections rely on keyboard focus.
            WlKeyboardEvent::Enter { serial, .. } => {
                seat_state.latest_serial = serial;
                seat_state.has_focus = true;
            },
            WlKeyboardEvent::Leave { .. } => {
                seat_state.latest_serial = 0;
                seat_state.has_focus = false;
            },
            _ => (),
        }
    }
}

delegate_seat!(State);
delegate_shm!(State);
delegate_pointer!(State);
delegate_data_device!(State);
delegate_primary_selection!(State);
delegate_registry!(State);

#[derive(Debug, Clone, Copy)]
pub enum SelectionTarget {
    /// The target is clipboard selection.
    Clipboard,
    /// The target is primary selection.
    Primary,
}

#[derive(Debug, Default)]
struct ClipboardSeatState {
    keyboard: Option<WlKeyboard>,
    pointer: Option<WlPointer>,
    data_device: Option<DataDevice>,
    primary_device: Option<PrimarySelectionDevice>,
    has_focus: bool,

    /// The latest serial used to set the selection content.
    latest_serial: u32,
    /// The surface and serial of the pointer button press held now.
    pressed: Option<(WlSurface, u32)>,
}

impl Drop for ClipboardSeatState {
    fn drop(&mut self) {
        if let Some(keyboard) = self.keyboard.take() {
            if keyboard.version() >= 3 {
                keyboard.release();
            }
        }

        if let Some(pointer) = self.pointer.take() {
            if pointer.version() >= 3 {
                pointer.release();
            }
        }
    }
}

fn set_non_blocking(raw_fd: RawFd) -> std::io::Result<()> {
    let flags = unsafe { libc::fcntl(raw_fd, libc::F_GETFL) };

    if flags < 0 {
        return Err(std::io::Error::last_os_error());
    }

    let result = unsafe { libc::fcntl(raw_fd, libc::F_SETFL, flags | libc::O_NONBLOCK) };
    if result < 0 {
        return Err(std::io::Error::last_os_error());
    }

    Ok(())
}
