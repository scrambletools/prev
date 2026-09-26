//! A vertical drag handle for resizing a side panel, shown as an M3 drag
//! handle while hovered or dragged.

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Quad};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell};
use iced::mouse::{self, Cursor};
use iced::{Background, Element, Event, Length, Rectangle, Size, Theme, border};

use super::{Scheme, shape};

/// Width of the grab area.
pub const HANDLE_WIDTH: f32 = 8.0;
const PILL: Size = Size::new(4.0, 48.0);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Drag {
    Started,
    /// Horizontal distance from where the drag started.
    Moved(f32),
    Ended,
}

pub struct Handle<'a, Message> {
    on_drag: Box<dyn Fn(Drag) -> Message + 'a>,
}

pub fn handle<'a, Message>(on_drag: impl Fn(Drag) -> Message + 'a) -> Handle<'a, Message> {
    Handle {
        on_drag: Box::new(on_drag),
    }
}

#[derive(Default)]
struct State {
    /// Cursor x where the drag started.
    origin: Option<f32>,
    hovered: bool,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Handle<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(HANDLE_WIDTH), Length::Fill)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(limits.resolve(Length::Fixed(HANDLE_WIDTH), Length::Fill, Size::ZERO))
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
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();
        let hovered = cursor.is_over(layout.bounds());
        if hovered != state.hovered {
            state.hovered = hovered;
            shell.request_redraw();
        }
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) if hovered => {
                if let Some(position) = cursor.position() {
                    state.origin = Some(position.x);
                    shell.publish((self.on_drag)(Drag::Started));
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if let Some(origin) = state.origin {
                    shell.publish((self.on_drag)(Drag::Moved(position.x - origin)));
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                if state.origin.take().is_some() =>
            {
                shell.publish((self.on_drag)(Drag::Ended));
                shell.capture_event();
                shell.request_redraw();
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        use iced::advanced::Renderer as _;
        let state = tree.state.downcast_ref::<State>();
        let dragging = state.origin.is_some();
        if !state.hovered && !dragging {
            return;
        }
        let scheme = Scheme::of(theme);
        let bounds = layout.bounds();
        let color = if dragging {
            scheme.on_surface
        } else {
            scheme.outline
        };
        renderer.fill_quad(
            Quad {
                bounds: Rectangle::new(
                    iced::Point::new(
                        bounds.center_x() - PILL.width / 2.0,
                        bounds.center_y() - PILL.height / 2.0,
                    ),
                    PILL,
                ),
                border: border::rounded(shape::FULL),
                ..Quad::default()
            },
            Background::Color(color),
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();
        if state.origin.is_some() || cursor.is_over(layout.bounds()) {
            mouse::Interaction::ResizingHorizontally
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, Message: 'a> From<Handle<'a, Message>> for Element<'a, Message> {
    fn from(handle: Handle<'a, Message>) -> Self {
        Element::new(handle)
    }
}

/// A panel width being resized between `min` and `max`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Width {
    pub value: f32,
    min: f32,
    max: f32,
    /// Width when the current drag started.
    start: Option<f32>,
}

impl Width {
    pub const fn new(value: f32, min: f32, max: f32) -> Self {
        Self {
            value,
            min,
            max,
            start: None,
        }
    }

    /// Applies a drag of the handle on the panel's right edge.
    pub fn drag(&mut self, drag: Drag) {
        match drag {
            Drag::Started => self.start = Some(self.value),
            Drag::Moved(offset) => {
                if let Some(start) = self.start {
                    self.value = (start + offset).clamp(self.min, self.max);
                }
            }
            Drag::Ended => self.start = None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drags_within_limits() {
        let mut width = Width::new(264.0, 200.0, 480.0);
        width.drag(Drag::Started);
        width.drag(Drag::Moved(40.0));
        assert_eq!(width.value, 304.0);
        width.drag(Drag::Moved(-500.0));
        assert_eq!(width.value, 200.0);
        // Coming back from past the limit follows the cursor again.
        width.drag(Drag::Moved(10.0));
        assert_eq!(width.value, 274.0);
        width.drag(Drag::Moved(900.0));
        assert_eq!(width.value, 480.0);
        width.drag(Drag::Ended);
        width.drag(Drag::Moved(-100.0));
        assert_eq!(width.value, 480.0, "moves outside a drag are ignored");
    }
}
