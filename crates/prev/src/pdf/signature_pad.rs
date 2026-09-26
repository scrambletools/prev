//! A surface to sign on with the mouse, pen or touchpad.

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Quad};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell};
use iced::mouse::{self, Cursor};
use iced::widget::canvas::{Frame, Path, Stroke};
use iced::{Background, Border, Color, Element, Event, Length, Rectangle, Size, Theme, touch};

use crate::ui::{Scheme, shape};

type OnStroke<'a, Message> = dyn Fn(Vec<(f32, f32)>) -> Message + 'a;

pub struct SignaturePad<'a, Message> {
    strokes: &'a [Vec<(f32, f32)>],
    size: Size,
    /// Device pixels per logical pixel, to draw the ink at the screen's
    /// own resolution.
    scale: f32,
    width: f32,
    ink: [u8; 3],
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
        scale: 1.0,
        width: crate::pdf::signature::PEN_WIDTH,
        ink: crate::pdf::signature::INK,
        on_stroke: Box::new(on_stroke),
    }
}

impl<Message> SignaturePad<'_, Message> {
    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale.max(0.5);
        self
    }

    /// The pen: its width in pad pixels and its color.
    pub fn pen(mut self, width: f32, ink: [u8; 3]) -> Self {
        self.width = width;
        self.ink = ink;
        self
    }
}

/// The ink as an image, and what it was drawn from.
struct Ink {
    key: (usize, usize, usize, u32, u32, [u8; 3]),
    handle: iced::widget::image::Handle,
}

#[derive(Default)]
struct State {
    current: Option<Vec<(f32, f32)>>,
    ink: std::cell::RefCell<Option<Ink>>,
}

impl State {
    /// The strokes, and the one being drawn, antialiased by tiny-skia at
    /// device resolution: smoother than the canvas's multisampling.
    fn ink(
        &self,
        strokes: &[Vec<(f32, f32)>],
        size: Size,
        scale: f32,
        pen_width: f32,
        ink: [u8; 3],
    ) -> Option<iced::widget::image::Handle> {
        let points: usize = strokes.iter().map(Vec::len).sum();
        let current = self.current.as_ref().map_or(0, Vec::len);
        let key = (
            strokes.len(),
            points,
            current,
            (scale * 1000.0) as u32,
            (pen_width * 1000.0) as u32,
            ink,
        );
        let mut cached = self.ink.borrow_mut();
        if let Some(cached) = cached.as_ref().filter(|cached| cached.key == key) {
            return Some(cached.handle.clone());
        }
        let width = (size.width * scale).round().max(1.0) as u32;
        let height = (size.height * scale).round().max(1.0) as u32;
        let mut pixmap = tiny_skia::Pixmap::new(width, height)?;
        let all: Vec<Vec<(f32, f32)>> =
            strokes.iter().chain(self.current.iter()).cloned().collect();
        crate::pdf::signature::draw_strokes(&mut pixmap, &all, pen_width * scale, ink, |(x, y)| {
            (x * scale, y * scale)
        });
        // tiny-skia keeps premultiplied pixels; iced wants them straight.
        let mut pixels = pixmap.take();
        for pixel in pixels.as_chunks_mut::<4>().0 {
            let alpha = pixel[3];
            if alpha > 0 && alpha < 255 {
                for channel in &mut pixel[..3] {
                    *channel = ((*channel as u32 * 255 + alpha as u32 / 2) / alpha as u32) as u8;
                }
            }
        }
        let handle = iced::widget::image::Handle::from_rgba(width, height, pixels);
        *cached = Some(Ink {
            key,
            handle: handle.clone(),
        });
        Some(handle)
    }
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
                    // Points closer than this to the last one are the
                    // pointer's jitter; the curve through the rest is
                    // smoother without them.
                    const MIN_STEP: f32 = 1.5;
                    let point = local(*position);
                    let far = current
                        .last()
                        .is_none_or(|last| (point.0 - last.0).hypot(point.1 - last.1) >= MIN_STEP);
                    if far {
                        current.push(point);
                        shell.request_redraw();
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if let Some(mut stroke) = state.current.take() {
                    // End where the pointer let go, even if that was close.
                    if let Some(position) = cursor.position() {
                        let end = local(position);
                        if stroke.last() != Some(&end) {
                            stroke.push(end);
                        }
                    }
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
        let geometry = frame.into_geometry();
        renderer.with_layer(*viewport, |renderer| {
            renderer.with_translation(iced::Vector::new(bounds.x, bounds.y), |renderer| {
                renderer.draw_geometry(geometry);
            });
            if let Some(ink) = state.ink(
                self.strokes,
                bounds.size(),
                self.scale,
                self.width,
                self.ink,
            ) {
                use iced::advanced::image::Renderer as _;
                renderer.draw_image(
                    iced::advanced::image::Image::new(ink)
                        .filter_method(iced::advanced::image::FilterMethod::Linear),
                    bounds,
                    bounds,
                );
            }
        });
    }
}

impl<'a, Message: 'a> From<SignaturePad<'a, Message>> for Element<'a, Message> {
    fn from(pad: SignaturePad<'a, Message>) -> Self {
        Element::new(pad)
    }
}
