//! Draws the visible pages of a `PdfViewer` and turns mouse input into
//! viewer messages. Sized to the whole document; meant to sit inside a
//! scrollable, and only draws what the scrollable shows.

use iced::advanced::image::{FilterMethod, Image};
use iced::advanced::layout::{self, Layout};
use iced::advanced::mouse::click::{self, Click};
use iced::advanced::renderer::{self, Quad};
use iced::advanced::widget::{self, Tree, Widget};
use iced::advanced::{Clipboard, Shell};
use iced::keyboard::{self, Modifiers};
use iced::mouse::{self, Cursor, ScrollDelta};
use iced::{
    Background, Border, Color, Element, Event, Length, Rectangle, Shadow, Size, Theme, window,
};

use super::layout::{Area, visible_tiles};
use super::markup::{self, Handle, Tool};
use super::overlay::{self, Mapping};
use super::viewer::{PdfMessage, PdfViewer, Zoom};
use prev_pdf::annotation::FieldKind;

const LINE_SCROLL_ZOOM: f32 = 1.1;

pub struct PageCanvas<'a, Message> {
    viewer: &'a PdfViewer,
    on_message: Box<dyn Fn(PdfMessage) -> Message + 'a>,
    /// Page background, black in slideshow.
    backdrop: Option<Color>,
}

impl<'a, Message> PageCanvas<'a, Message> {
    pub fn new(viewer: &'a PdfViewer, on_message: impl Fn(PdfMessage) -> Message + 'a) -> Self {
        Self {
            viewer,
            on_message: Box::new(on_message),
            backdrop: None,
        }
    }

    pub fn backdrop(mut self, color: Color) -> Self {
        self.backdrop = Some(color);
        self
    }
}

#[derive(Default)]
struct State {
    pressed: bool,
    last_click: Option<Click>,
    modifiers: Modifiers,
    reported_view: Option<Area>,
}

/// The part of the canvas shown by the enclosing scrollable, in document space.
fn visible_area(bounds: Rectangle, viewport: &Rectangle) -> Area {
    Area {
        x: viewport.x - bounds.x,
        y: viewport.y - bounds.y,
        width: viewport.width,
        height: viewport.height,
    }
}

impl<Message> PageCanvas<'_, Message> {
    /// The cursor for markup tools, annotations and form fields.
    fn editing_interaction(&self, x: f32, y: f32) -> Option<mouse::Interaction> {
        let viewer = self.viewer;
        if self.backdrop.is_some() {
            return None;
        }
        match viewer.edit.tool {
            Tool::Select => {}
            Tool::Highlight(_) => return Some(mouse::Interaction::Text),
            _ => return Some(mouse::Interaction::Crosshair),
        }
        let per_pixel = 1.0 / super::layout::points_to_pixels(viewer.layout.zoom);
        if let Some((page, annotation)) = viewer.selected_annotation() {
            let point = viewer.layout.to_page(page, x, y)?;
            match markup::hit_handle(annotation, point, markup::HANDLE_PIXELS * per_pixel) {
                Some(Handle::Edge { x: 0, .. }) => {
                    return Some(mouse::Interaction::ResizingVertically);
                }
                Some(Handle::Edge { y: 0, .. }) => {
                    return Some(mouse::Interaction::ResizingHorizontally);
                }
                Some(Handle::Edge { x, y }) if x == y => {
                    return Some(mouse::Interaction::ResizingDiagonallyDown);
                }
                Some(Handle::Edge { .. }) => {
                    return Some(mouse::Interaction::ResizingDiagonallyUp);
                }
                Some(Handle::LineStart | Handle::LineEnd) => {
                    return Some(mouse::Interaction::Crosshair);
                }
                Some(Handle::Body) if markup::movable(annotation) => {
                    return Some(mouse::Interaction::Grab);
                }
                _ => {}
            }
        }
        let (page, point) = viewer.layout.hit(x, y)?;
        let markup = viewer.markup.get(&page)?;
        let slop = markup::HIT_SLOP * per_pixel.max(1.0);
        if markup::hit_annotation(&markup.annotations, point, slop).is_some() {
            return Some(mouse::Interaction::Pointer);
        }
        let field = markup
            .fields
            .iter()
            .find(|field| field.rect.contains(point) && !field.read_only)?;
        Some(match field.kind {
            FieldKind::Text { .. } => mouse::Interaction::Text,
            FieldKind::Button | FieldKind::Signature => return None,
            _ => mouse::Interaction::Pointer,
        })
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for PageCanvas<'_, Message> {
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        let content = self.viewer.layout.content;
        Size::new(Length::Fixed(content.width), Length::Fixed(content.height))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let content = self.viewer.layout.content;
        layout::Node::new(limits.resolve(
            Length::Fixed(content.width),
            Length::Fixed(content.height),
            Size::new(content.width, content.height),
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

        // Report the visible area whenever it changes: scrolling and resizing.
        let view = visible_area(bounds, viewport);
        if state.reported_view != Some(view) && view.width > 0.0 && view.height > 0.0 {
            state.reported_view = Some(view);
            if view != self.viewer.view {
                shell.publish((self.on_message)(PdfMessage::ViewChanged(view)));
            }
        }

        let to_document = |point: iced::Point| (point.x - bounds.x, point.y - bounds.y);
        match event {
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                // Always sent: this widget's state is new whenever the
                // layout around it changes, so it cannot tell what changed.
                if modifiers.shift() != self.viewer.shift() {
                    shell.publish((self.on_message)(PdfMessage::Shift(modifiers.shift())));
                }
                state.modifiers = *modifiers
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let Some(position) = cursor.position_over(*viewport) else {
                    return;
                };
                let click = Click::new(position, mouse::Button::Left, state.last_click);
                state.last_click = Some(click);
                state.pressed = true;
                let clicks = match click.kind() {
                    click::Kind::Single => 1,
                    click::Kind::Double => 2,
                    click::Kind::Triple => 3,
                };
                let (x, y) = to_document(position);
                shell.publish((self.on_message)(PdfMessage::Press { x, y, clicks }));
                shell.capture_event();
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) if state.pressed => {
                // The event has window coordinates; the cursor given to this
                // widget is already moved by the scroll offset.
                let Some(position) = cursor.position() else {
                    return;
                };
                let (x, y) = to_document(position);
                shell.publish((self.on_message)(PdfMessage::Drag { x, y }));
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) if state.pressed => {
                state.pressed = false;
                let (x, y) = cursor.position().map_or((0.0, 0.0), to_document);
                shell.publish((self.on_message)(PdfMessage::Release { x, y }));
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
                    let factor = LINE_SCROLL_ZOOM.powf(lines);
                    let anchor = (position.x - viewport.x, position.y - viewport.y);
                    shell.publish((self.on_message)(PdfMessage::Zoom(Zoom::By {
                        factor,
                        anchor,
                    })));
                }
                shell.capture_event();
            }
            Event::Window(window::Event::RedrawRequested(_)) => {}
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let Some(position) = cursor.position_over(*viewport) else {
            return mouse::Interaction::None;
        };
        let bounds = layout.bounds();
        let (x, y) = (position.x - bounds.x, position.y - bounds.y);
        if let Some(interaction) = self.editing_interaction(x, y) {
            return interaction;
        }
        if self.viewer.link_at(x, y).is_some() {
            mouse::Interaction::Pointer
        } else if self.viewer.is_over_text(x, y) {
            mouse::Interaction::Text
        } else {
            mouse::Interaction::None
        }
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        viewport: &Rectangle,
    ) {
        use iced::advanced::Renderer as _;
        use iced::advanced::image::Renderer as _;

        let bounds = layout.bounds();
        let viewer = self.viewer;
        if let Some(color) = self.backdrop {
            renderer.fill_quad(
                Quad {
                    bounds: *viewport,
                    ..Quad::default()
                },
                Background::Color(color),
            );
        }
        let view = visible_area(bounds, viewport);
        let scale = viewer.render_scale();
        let points_to_pixels = scale / viewer.device_scale;
        let palette = theme.extended_palette();
        let selection_color = Color {
            a: 0.35,
            ..palette.primary.base.color
        };

        let pages: Vec<(usize, Rectangle)> = viewer
            .layout
            .visible_pages(&view)
            .into_iter()
            .filter_map(|page| {
                let area = viewer.layout.page_area(page)?;
                let rect = Rectangle::new(
                    iced::Point::new(bounds.x + area.x, bounds.y + area.y),
                    Size::new(area.width, area.height),
                );
                Some((page, rect))
            })
            .collect();

        // Previews and tiles share the linear filter: iced batches images by
        // filter method, which would otherwise reorder them.
        for &(page, page_rect) in &pages {
            let shadow = if self.backdrop.is_some() {
                Shadow::default()
            } else {
                crate::ui::elevation::shadow(&crate::ui::Scheme::of(theme), 2)
            };
            renderer.fill_quad(
                Quad {
                    bounds: page_rect,
                    border: Border::default(),
                    shadow,
                    snap: true,
                },
                Background::Color(Color::WHITE),
            );
            let area = viewer.layout.page_area(page).unwrap_or_default();
            let tiles: Vec<_> = visible_tiles(&area, &view, viewer.device_scale)
                .into_iter()
                .map(|tile| {
                    let rect = Rectangle::new(
                        iced::Point::new(
                            page_rect.x + tile.x as f32 / viewer.device_scale,
                            page_rect.y + tile.y as f32 / viewer.device_scale,
                        ),
                        Size::new(
                            tile.width as f32 / viewer.device_scale,
                            tile.height as f32 / viewer.device_scale,
                        ),
                    );
                    (rect, viewer.tile(&viewer.tile_key(page, &tile)))
                })
                .collect();
            if tiles.iter().any(|(_, handle)| handle.is_none())
                && let Some(preview) = viewer.previews.get(&page)
            {
                renderer.draw_image(
                    Image::new(preview.clone()).filter_method(FilterMethod::Linear),
                    page_rect,
                    *viewport,
                );
            }
            for (rect, handle) in tiles {
                if let Some(handle) = handle {
                    renderer.draw_image(
                        Image::new(handle.clone()).filter_method(FilterMethod::Linear),
                        rect,
                        *viewport,
                    );
                }
            }
        }

        // Quads draw before images within a layer, so highlights need their own.
        renderer.with_layer(*viewport, |renderer| {
            for &(page, page_rect) in &pages {
                let to_screen = |rect: prev_pdf::geometry::Rect| {
                    Rectangle::new(
                        iced::Point::new(
                            page_rect.x + rect.x0 * points_to_pixels,
                            page_rect.y + rect.y0 * points_to_pixels,
                        ),
                        Size::new(
                            rect.width() * points_to_pixels,
                            rect.height() * points_to_pixels,
                        ),
                    )
                };
                for (rect, current) in viewer.page_matches(page) {
                    let color = if current {
                        Color::from_rgba8(255, 150, 0, 0.55)
                    } else {
                        Color::from_rgba8(255, 230, 0, 0.45)
                    };
                    renderer.fill_quad(
                        Quad {
                            bounds: to_screen(rect),
                            ..Quad::default()
                        },
                        Background::Color(color),
                    );
                }
                // Fillable form fields get Preview's light blue tint.
                if viewer.edit.tool == Tool::Select
                    && self.backdrop.is_none()
                    && let Some(markup) = viewer.markup.get(&page)
                {
                    for field in markup.fields.iter().filter(|field| {
                        !field.read_only
                            && !matches!(field.kind, FieldKind::Button | FieldKind::Signature)
                    }) {
                        let bounds = to_screen(field.rect);
                        // Radio buttons are round, so their tint is too.
                        let radius = if field.kind == FieldKind::Radio {
                            bounds.width.min(bounds.height) / 2.0
                        } else {
                            0.0
                        };
                        renderer.fill_quad(
                            Quad {
                                bounds,
                                border: Border {
                                    radius: radius.into(),
                                    ..Border::default()
                                },
                                ..Quad::default()
                            },
                            Background::Color(Color::from_rgba8(80, 140, 255, 0.12)),
                        );
                    }
                }
                if let (Some(selection), Some(text)) =
                    (viewer.page_selection(page), viewer.texts.get(&page))
                {
                    for rect in text.highlight(selection) {
                        renderer.fill_quad(
                            Quad {
                                bounds: to_screen(rect),
                                ..Quad::default()
                            },
                            Background::Color(selection_color),
                        );
                    }
                }
            }
        });

        if self.backdrop.is_some() {
            return;
        }
        // Annotations being made or moved, and the selection, as vector
        // geometry above everything else.
        use iced::advanced::graphics::geometry::Renderer as _;
        let accent = crate::ui::Scheme::of(theme).primary;
        let mut frame = iced::widget::canvas::Frame::new(renderer, viewport.size());
        let mapping_for = |page: usize| {
            let (_, page_rect) = pages.iter().find(|(visible, _)| *visible == page)?;
            Some(Mapping {
                origin: iced::Point::new(page_rect.x - viewport.x, page_rect.y - viewport.y),
                scale: points_to_pixels,
            })
        };
        let preview = viewer.preview();
        if let Some((page, annotation)) = &preview
            && let Some(mapping) = mapping_for(*page)
        {
            overlay::paint(&mut frame, annotation, &mapping);
        }
        if let Some((page, annotation)) = viewer.selected_annotation()
            && let Some(mapping) = mapping_for(page)
        {
            let shown = match &preview {
                Some((_, moved)) if moved.id == annotation.id => moved,
                _ => annotation,
            };
            if viewer.edit.text.is_none() {
                overlay::selection(&mut frame, shown, &mapping, accent);
            }
        }
        if let Some((page, area)) = viewer.edit.area
            && let Some(mapping) = mapping_for(page)
        {
            overlay::area(&mut frame, area, &mapping, accent);
        }
        let geometry = frame.into_geometry();
        renderer.with_layer(*viewport, |renderer| {
            renderer.with_translation(iced::Vector::new(viewport.x, viewport.y), |renderer| {
                renderer.draw_geometry(geometry);
            });
        });
    }
}

impl<'a, Message: 'a> From<PageCanvas<'a, Message>> for Element<'a, Message> {
    fn from(canvas: PageCanvas<'a, Message>) -> Self {
        Element::new(canvas)
    }
}
