//! Draws one image inside a scrollable, zooms with Ctrl+scroll, pans by
//! dragging, and wakes up for the next animation frame.

use std::time::Instant;

use iced::advanced::image::{FilterMethod, Image};
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Tree, Widget};
use iced::advanced::{Clipboard, Shell};
use iced::keyboard::{self, Modifiers};
use iced::mouse::{self, Cursor, ScrollDelta};
use iced::widget::image::Handle;
use iced::{Element, Event, Length, Rectangle, Size, Theme, window};

use super::view::Placement;

const LINE_SCROLL_ZOOM: f32 = 1.1;
/// Zoom at or above which pixels are shown sharp instead of smoothed.
const PIXEL_ZOOM: f32 = 3.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CanvasEvent {
    /// The visible part of the content: offset and size in logical pixels.
    ViewChanged {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    ZoomBy {
        factor: f32,
        anchor: (f32, f32),
    },
    Pan {
        dx: f32,
        dy: f32,
    },
    Tick(Instant),
}

pub struct ImageCanvas<'a, Message> {
    handle: Option<&'a Handle>,
    placement: Placement,
    /// When the next animation frame is due.
    next_frame: Option<Instant>,
    on_event: Box<dyn Fn(CanvasEvent) -> Message + 'a>,
}

impl<'a, Message> ImageCanvas<'a, Message> {
    pub fn new(
        handle: Option<&'a Handle>,
        placement: Placement,
        next_frame: Option<Instant>,
        on_event: impl Fn(CanvasEvent) -> Message + 'a,
    ) -> Self {
        Self {
            handle,
            placement,
            next_frame,
            on_event: Box::new(on_event),
        }
    }
}

#[derive(Default)]
struct State {
    drag_from: Option<iced::Point>,
    modifiers: Modifiers,
    reported: Option<(f32, f32, f32, f32)>,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for ImageCanvas<'_, Message> {
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        let (width, height) = self.placement.content;
        Size::new(Length::Fixed(width), Length::Fixed(height))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let (width, height) = self.placement.content;
        layout::Node::new(limits.resolve(
            Length::Fixed(width),
            Length::Fixed(height),
            Size::new(width, height),
        ))
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();
        let bounds = layout.bounds();

        let view = (
            viewport.x - bounds.x,
            viewport.y - bounds.y,
            viewport.width,
            viewport.height,
        );
        if state.reported != Some(view) && view.2 > 0.0 && view.3 > 0.0 {
            state.reported = Some(view);
            let (x, y, width, height) = view;
            shell.publish((self.on_event)(CanvasEvent::ViewChanged {
                x,
                y,
                width,
                height,
            }));
        }

        if let Some(due) = self.next_frame {
            if let Event::Window(window::Event::RedrawRequested(now)) = event
                && *now >= due
            {
                shell.publish((self.on_event)(CanvasEvent::Tick(*now)));
            } else {
                shell.request_redraw_at(window::RedrawRequest::At(due));
            }
        }

        match event {
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                state.modifiers = *modifiers
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(position) = cursor.position_over(*viewport) {
                    state.drag_from = Some(position);
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if let Some(from) = state.drag_from {
                    // The content moves under the cursor, so pan against it.
                    let (dx, dy) = (from.x - position.x, from.y - position.y);
                    state.drag_from = Some(*position);
                    if dx != 0.0 || dy != 0.0 {
                        shell.publish((self.on_event)(CanvasEvent::Pan { dx, dy }));
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                state.drag_from = None
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta }) if state.modifiers.command() => {
                let Some(position) = cursor.position_over(*viewport) else {
                    return;
                };
                let lines = match delta {
                    ScrollDelta::Lines { y, .. } => *y,
                    ScrollDelta::Pixels { y, .. } => y / 40.0,
                };
                if lines != 0.0 {
                    let anchor = (position.x - viewport.x, position.y - viewport.y);
                    let factor = LINE_SCROLL_ZOOM.powf(lines);
                    shell.publish((self.on_event)(CanvasEvent::ZoomBy { factor, anchor }));
                }
                shell.capture_event();
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        _layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();
        let (content_width, content_height) = self.placement.content;
        let scrolls =
            content_width > viewport.width + 0.5 || content_height > viewport.height + 0.5;
        match (state.drag_from, cursor.is_over(*viewport) && scrolls) {
            (Some(_), _) => mouse::Interaction::Grabbing,
            (None, true) => mouse::Interaction::Grab,
            (None, false) => mouse::Interaction::None,
        }
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut iced::Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        viewport: &Rectangle,
    ) {
        use iced::advanced::image::Renderer as _;
        let Some(handle) = self.handle else { return };
        let bounds = layout.bounds();
        let image = self.placement.image;
        let rect = Rectangle::new(
            iced::Point::new(bounds.x + image.x, bounds.y + image.y),
            Size::new(image.width, image.height),
        );
        let filter = if self.placement.zoom >= PIXEL_ZOOM {
            FilterMethod::Nearest
        } else {
            FilterMethod::Linear
        };
        renderer.draw_image(
            Image::new(handle.clone()).filter_method(filter),
            rect,
            *viewport,
        );
    }
}

impl<'a, Message: 'a> From<ImageCanvas<'a, Message>> for Element<'a, Message> {
    fn from(canvas: ImageCanvas<'a, Message>) -> Self {
        Element::new(canvas)
    }
}
