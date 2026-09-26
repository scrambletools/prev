//! A surface to sign on with the mouse, pen or touchpad.

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Quad};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell};
use iced::mouse::{self, Cursor};
use iced::widget::canvas::{Frame, LineCap, LineJoin, Path, Stroke};
use iced::{Background, Border, Color, Element, Event, Length, Rectangle, Size, Theme, touch};

use crate::ui::{Scheme, shape};

pub const LINE_WIDTH: f32 = 3.0;

type OnStroke<'a, Message> = dyn Fn(Vec<(f32, f32)>) -> Message + 'a;

pub struct SignaturePad<'a, Message> {
    strokes: &'a [Vec<(f32, f32)>],
    size: Size,
    on_stroke: Box<OnStroke<'a, Message>>,
}

pub fn signature_pad<'a, Message>(
    strokes: &'a [Vec<(f32, f32)>],
    size: Size,
    on_stroke: impl Fn(Vec<(f32, f32)>) -> Message + 'a,
) -> SignaturePad<'a, Message> {
    SignaturePad {
        strokes,
        size,
        on_stroke: Box::new(on_stroke),
    }
}

#[derive(Default)]
struct State {
    current: Option<Vec<(f32, f32)>>,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for SignaturePad<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(
            Length::Fixed(self.size.width),
            Length::Fixed(self.size.height),
        )
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(limits.resolve(
            Length::Fixed(self.size.width),
            Length::Fixed(self.size.height),
            self.size,
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
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();
        let bounds = layout.bounds();
        let local = |point: iced::Point| {
            (
                (point.x - bounds.x).clamp(0.0, bounds.width),
                (point.y - bounds.y).clamp(0.0, bounds.height),
            )
        };
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if let Some(position) = cursor.position_over(bounds) {
                    state.current = Some(vec![local(position)]);
                    shell.capture_event();
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { position })
            | Event::Touch(touch::Event::FingerMoved { position, .. }) => {
                if let Some(current) = state.current.as_mut() {
                    current.push(local(*position));
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if let Some(stroke) = state.current.take() {
                    shell.publish((self.on_stroke)(stroke));
                    shell.capture_event();
                }
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
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
        viewport: &Rectangle,
    ) {
        use iced::advanced::Renderer as _;
        use iced::advanced::graphics::geometry::Renderer as _;
        let state = tree.state.downcast_ref::<State>();
        let scheme = Scheme::of(theme);
        let bounds = layout.bounds();
        renderer.fill_quad(
            Quad {
                bounds,
                border: Border {
                    color: scheme.outline_variant,
                    width: 1.0,
                    radius: shape::MEDIUM.into(),
                },
                ..Quad::default()
            },
            Background::Color(Color::WHITE),
        );
        let mut frame = Frame::new(renderer, bounds.size());
        // The line to sign on.
        let baseline = bounds.height * 0.72;
        frame.stroke(
            &Path::line(
                iced::Point::new(24.0, baseline),
                iced::Point::new(bounds.width - 24.0, baseline),
            ),
            Stroke::default()
                .with_color(Color::from_rgba(0.0, 0.0, 0.0, 0.25))
                .with_width(1.0),
        );
        let ink = Stroke {
            width: LINE_WIDTH,
            line_cap: LineCap::Round,
            line_join: LineJoin::Round,
            ..Stroke::default().with_color(Color::from_rgb8(0x10, 0x18, 0x40))
        };
        for stroke in self.strokes.iter().chain(state.current.iter()) {
            let path = Path::new(|builder| {
                let mut points = stroke.iter();
                if let Some(first) = points.next() {
                    builder.move_to(iced::Point::new(first.0, first.1));
                    builder.line_to(iced::Point::new(first.0 + 0.1, first.1));
                    for point in points {
                        builder.line_to(iced::Point::new(point.0, point.1));
                    }
                }
            });
            frame.stroke(&path, ink);
        }
        let geometry = frame.into_geometry();
        renderer.with_layer(*viewport, |renderer| {
            renderer.with_translation(iced::Vector::new(bounds.x, bounds.y), |renderer| {
                renderer.draw_geometry(geometry);
            });
        });
    }
}

impl<'a, Message: 'a> From<SignaturePad<'a, Message>> for Element<'a, Message> {
    fn from(pad: SignaturePad<'a, Message>) -> Self {
        Element::new(pad)
    }
}
