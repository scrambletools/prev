//! M3 Expressive buttons and icon buttons: state layers, keyboard focus,
//! and corners that spring to a new shape when pressed or selected.

use std::time::Instant;

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Quad};
use iced::advanced::widget::operation::Focusable;
use iced::advanced::widget::{Operation, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay};
use iced::keyboard::{self, key};
use iced::mouse::{self, Cursor};
use iced::{
    Background, Border, Color, Element, Event, Length, Padding, Rectangle, Shadow, Theme, Vector,
    border, touch, window,
};

use super::font::Type;
use super::icon::{self, Icon};
use super::motion::{Motion, Spring};
use super::{Scheme, elevation, state_layer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Filled,
    Tonal,
    Outlined,
    Text,
    Elevated,
    /// Icon buttons without a container.
    Standard,
    /// A list row: no container until selected.
    Row,
    /// A tab: the label turns primary when selected.
    Tab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    ExtraSmall,
    Small,
    Medium,
}

impl Size {
    pub fn height(self) -> f32 {
        match self {
            Size::ExtraSmall => 32.0,
            Size::Small => 40.0,
            Size::Medium => 56.0,
        }
    }

    pub fn icon(self) -> f32 {
        match self {
            Size::ExtraSmall | Size::Small => 20.0,
            Size::Medium => 24.0,
        }
    }

    fn label(self) -> Type {
        match self {
            Size::ExtraSmall | Size::Small => Type::LabelLarge,
            Size::Medium => Type::TitleMedium,
        }
    }

    fn padding(self) -> f32 {
        match self {
            Size::ExtraSmall => 12.0,
            Size::Small => 16.0,
            Size::Medium => 24.0,
        }
    }

    /// Corner radius of the square shape.
    fn square(self) -> f32 {
        match self {
            Size::ExtraSmall | Size::Small => 12.0,
            Size::Medium => 16.0,
        }
    }

    /// Corner radius while pressed.
    fn pressed(self) -> f32 {
        match self {
            Size::ExtraSmall | Size::Small => 8.0,
            Size::Medium => 12.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Round,
    Square,
    /// No rounding in any state, for tabs.
    Flat,
}

/// Where a button sits in a connected button group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Alone,
    First,
    Middle,
    Last,
}

/// Side padding of tab labels, kept small so fixed tabs fit narrow panels.
const TAB_PADDING: f32 = 4.0;

/// Inner corner radius of connected buttons.
const INNER_CORNER: f32 = 8.0;
const INNER_CORNER_PRESSED: f32 = 4.0;

enum Content<'a, Message> {
    Icon(Icon),
    Label(Option<Icon>, String),
    Custom(Element<'a, Message>),
}

pub struct Button<'a, Message> {
    content: Content<'a, Message>,
    kind: Kind,
    size: Size,
    shape: Shape,
    position: Position,
    icon_only: bool,
    selected: Option<bool>,
    width: Length,
    height: Option<f32>,
    on_press: Option<Message>,
}

/// A button with a text label.
pub fn button<'a, Message: Clone + 'a>(
    kind: Kind,
    label: impl Into<String>,
) -> Button<'a, Message> {
    Button::new(Content::Label(None, label.into()), kind, Size::Small)
}

/// A button with a leading icon and a label.
pub fn with_icon<'a, Message: Clone + 'a>(
    kind: Kind,
    icon: Icon,
    label: impl Into<String>,
) -> Button<'a, Message> {
    Button::new(Content::Label(Some(icon), label.into()), kind, Size::Small)
}

/// An icon button.
pub fn icon_button<'a, Message: Clone + 'a>(icon: Icon) -> Button<'a, Message> {
    Button::new(Content::Icon(icon), Kind::Standard, Size::Small)
}

/// A button around any content, such as a list row.
pub fn custom<'a, Message: Clone + 'a>(
    kind: Kind,
    content: impl Into<Element<'a, Message>>,
) -> Button<'a, Message> {
    Button::new(Content::Custom(content.into()), kind, Size::Small)
}

impl<'a, Message: Clone + 'a> Button<'a, Message> {
    fn new(content: Content<'a, Message>, kind: Kind, size: Size) -> Self {
        let icon_only = matches!(content, Content::Icon(_));
        Self {
            content,
            kind,
            size,
            shape: Shape::Round,
            position: Position::Alone,
            icon_only,
            selected: None,
            width: Length::Shrink,
            height: None,
            on_press: None,
        }
    }

    fn build(self) -> Built<'a, Message> {
        let icon_size = self.size.icon();
        let content = match self.content {
            Content::Icon(glyph) => {
                if self.selected == Some(true) {
                    icon::filled(glyph, icon_size).into()
                } else {
                    icon::icon(glyph, icon_size).into()
                }
            }
            Content::Label(glyph, label) => {
                let label = super::font::styled(label, self.size.label());
                match glyph {
                    Some(glyph) => crate::row![icon::icon(glyph, icon_size), label]
                        .spacing(8)
                        .align_y(iced::Center)
                        .into(),
                    None => label.into(),
                }
            }
            Content::Custom(element) => element,
        };
        Built {
            content,
            look: Look {
                kind: self.kind,
                size: self.size,
                shape: self.shape,
                position: self.position,
                icon_only: self.icon_only,
                selected: self.selected,
                width: self.width,
                height: self.height.unwrap_or(self.size.height()),
            },
            on_press: self.on_press,
        }
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn kind(mut self, kind: Kind) -> Self {
        self.kind = kind;
        self
    }

    pub fn shape(mut self, shape: Shape) -> Self {
        self.shape = shape;
        self
    }

    pub fn position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    /// Makes this a toggle button, selected or not.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = Some(selected);
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Overrides the height the size gives.
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }
}

/// How a button looks, apart from its content.
#[derive(Clone, Copy)]
struct Look {
    kind: Kind,
    size: Size,
    shape: Shape,
    position: Position,
    icon_only: bool,
    selected: Option<bool>,
    width: Length,
    height: f32,
}

/// The widget a [`Button`] turns into.
struct Built<'a, Message> {
    content: Element<'a, Message>,
    look: Look,
    on_press: Option<Message>,
}

impl<Message> Built<'_, Message> {
    fn enabled(&self) -> bool {
        self.on_press.is_some()
    }
}

impl Look {
    /// Corner radii (top left, top right, bottom right, bottom left) for
    /// the current state.
    fn corners(&self, height: f32, pressed: bool) -> [f32; 4] {
        if self.shape == Shape::Flat {
            return [0.0; 4];
        }
        let full = height / 2.0;
        let selected = self.selected == Some(true);
        let outer = if pressed {
            self.size.pressed()
        } else {
            match (self.shape, selected, self.position) {
                // Toggle buttons trade shapes when selected.
                (Shape::Round, true, Position::Alone) => self.size.square(),
                (_, true, _) | (Shape::Round, false, _) => full,
                (_, false, _) => self.size.square(),
            }
        };
        let inner = if pressed {
            INNER_CORNER_PRESSED
        } else if selected {
            full
        } else {
            INNER_CORNER
        };
        match self.position {
            Position::Alone => [outer; 4],
            Position::First => [outer, inner, inner, outer],
            Position::Middle => [inner; 4],
            Position::Last => [inner, outer, outer, inner],
        }
    }
}

/// Container, content and outline colors, and elevation level.
struct Colors {
    container: Option<Color>,
    content: Color,
    outline: Option<Color>,
    level: u8,
}

fn colors(scheme: &Scheme, kind: Kind, selected: Option<bool>, enabled: bool) -> Colors {
    if !enabled {
        let faded = |alpha: f32| Color {
            a: alpha,
            ..scheme.on_surface
        };
        let contained = matches!(kind, Kind::Filled | Kind::Tonal | Kind::Elevated);
        return Colors {
            container: contained.then(|| faded(0.1)),
            content: faded(0.38),
            outline: (kind == Kind::Outlined).then(|| faded(0.12)),
            level: 0,
        };
    }
    let plain = |container, content| Colors {
        container,
        content,
        outline: None,
        level: 0,
    };
    match (kind, selected) {
        (Kind::Filled, None | Some(true)) => plain(Some(scheme.primary), scheme.on_primary),
        (Kind::Filled, Some(false)) => {
            plain(Some(scheme.surface_container), scheme.on_surface_variant)
        }
        (Kind::Tonal, None | Some(false)) => plain(
            Some(scheme.secondary_container),
            scheme.on_secondary_container,
        ),
        (Kind::Tonal, Some(true)) => plain(Some(scheme.secondary), scheme.on_secondary),
        (Kind::Outlined, None | Some(false)) => Colors {
            container: None,
            content: if selected.is_some() {
                scheme.on_surface_variant
            } else {
                scheme.primary
            },
            outline: Some(scheme.outline_variant),
            level: 0,
        },
        (Kind::Outlined, Some(true)) => {
            plain(Some(scheme.inverse_surface), scheme.inverse_on_surface)
        }
        (Kind::Text, _) => plain(None, scheme.primary),
        (Kind::Elevated, None | Some(false)) => Colors {
            level: 1,
            ..plain(Some(scheme.surface_container_low), scheme.primary)
        },
        (Kind::Elevated, Some(true)) => plain(Some(scheme.primary), scheme.on_primary),
        (Kind::Standard, None | Some(false)) => plain(None, scheme.on_surface_variant),
        // Selected toolbar tools get a tonal container, as in M3 toolbars.
        (Kind::Standard, Some(true)) => plain(
            Some(scheme.secondary_container),
            scheme.on_secondary_container,
        ),
        (Kind::Tab, None | Some(false)) => plain(None, scheme.on_surface_variant),
        (Kind::Tab, Some(true)) => plain(None, scheme.primary),
        (Kind::Row, None | Some(false)) => plain(None, scheme.on_surface_variant),
        (Kind::Row, Some(true)) => plain(
            Some(scheme.secondary_container),
            scheme.on_secondary_container,
        ),
    }
}

struct State {
    pressed: bool,
    focused: bool,
    /// Focus came from the keyboard, so the focus ring shows.
    focus_visible: bool,
    corners: [Spring; 4],
    layer: Spring,
    started: bool,
}

impl State {
    fn new() -> Self {
        Self {
            pressed: false,
            focused: false,
            focus_visible: false,
            corners: [Spring::new(0.0, Motion::FAST_SPATIAL).precision(0.1); 4],
            layer: Spring::new(0.0, Motion::FAST_EFFECTS).precision(0.002),
            started: false,
        }
    }

    fn moving(&self) -> bool {
        self.corners.iter().any(|spring| !spring.is_settled()) || !self.layer.is_settled()
    }
}

impl Focusable for State {
    fn is_focused(&self) -> bool {
        self.focused
    }

    fn focus(&mut self) {
        self.focused = true;
        self.focus_visible = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
        self.focus_visible = false;
    }
}

impl<'a, Message: Clone + 'a> Widget<Message, Theme, iced::Renderer> for Built<'a, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> iced::Size<Length> {
        iced::Size::new(self.look.width, Length::Fixed(self.look.height))
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let height = self.look.height;
        let padding = if self.look.icon_only {
            0.0
        } else if self.look.kind == Kind::Tab {
            TAB_PADDING
        } else {
            self.look.size.padding()
        };
        let content_limits = limits
            .height(height)
            .shrink(Padding::from([0.0, padding]))
            .loose();
        let content =
            self.content
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &content_limits);
        let content_size = content.size();
        let natural = if self.look.icon_only {
            height
        } else {
            content_size.width + 2.0 * padding
        };
        let size = limits.resolve(
            self.look.width,
            Length::Fixed(height),
            iced::Size::new(natural, height),
        );
        let x = if self.look.kind == Kind::Row {
            padding
        } else {
            ((size.width - content_size.width) / 2.0).max(0.0)
        };
        let offset = Vector::new(x, ((size.height - content_size.height) / 2.0).max(0.0));
        layout::Node::with_children(size, vec![content.translate(offset)])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        if self.enabled() {
            let state = tree.state.downcast_mut::<State>();
            operation.focusable(None, layout.bounds(), state);
        }
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
        let bounds = layout.bounds();
        let enabled = self.enabled();
        let state = tree.state.downcast_mut::<State>();
        if !shell.is_event_captured() {
            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    if enabled && cursor.is_over(bounds) {
                        state.pressed = true;
                        state.focus_visible = false;
                        shell.capture_event();
                    }
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerLifted { .. }) => {
                    if state.pressed {
                        state.pressed = false;
                        if cursor.is_over(bounds)
                            && let Some(message) = &self.on_press
                        {
                            shell.publish(message.clone());
                        }
                        shell.capture_event();
                    }
                }
                Event::Touch(touch::Event::FingerLost { .. }) => state.pressed = false,
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Enter | key::Named::Space),
                    ..
                }) if state.focused => {
                    if let Some(message) = &self.on_press {
                        state.pressed = true;
                        shell.publish(message.clone());
                        shell.capture_event();
                    }
                }
                Event::Keyboard(keyboard::Event::KeyReleased {
                    key: keyboard::Key::Named(key::Named::Enter | key::Named::Space),
                    ..
                }) if state.focused && state.pressed => {
                    state.pressed = false;
                    shell.capture_event();
                }
                _ => {}
            }
        }

        // Aim the springs at the current state.
        let hovered = enabled && cursor.is_over(bounds);
        let corners = self.look.corners(bounds.height, state.pressed);
        let layer = if !enabled {
            0.0
        } else if state.pressed {
            state_layer::PRESSED
        } else if state.focused && state.focus_visible {
            state_layer::FOCUSED
        } else if hovered {
            state_layer::HOVERED
        } else {
            0.0
        };
        if !state.started {
            state.started = true;
            for (spring, radius) in state.corners.iter_mut().zip(corners) {
                spring.jump_to(radius);
            }
            state.layer.jump_to(layer);
        } else {
            for (spring, radius) in state.corners.iter_mut().zip(corners) {
                spring.go_to(radius);
            }
            state.layer.go_to(layer);
        }

        match event {
            Event::Window(window::Event::RedrawRequested(now)) => {
                let now: Instant = *now;
                let mut moving = false;
                for spring in &mut state.corners {
                    moving |= spring.advance(now);
                }
                moving |= state.layer.advance(now);
                if moving {
                    shell.request_redraw();
                }
            }
            _ if state.moving() => shell.request_redraw(),
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
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        use iced::advanced::Renderer as _;
        let state = tree.state.downcast_ref::<State>();
        let scheme = Scheme::of(theme);
        let bounds = layout.bounds();
        let colors = colors(&scheme, self.look.kind, self.look.selected, self.enabled());
        let [top_left, top_right, bottom_right, bottom_left] = state
            .corners
            .map(|spring| spring.value.clamp(0.0, bounds.height / 2.0));
        let radius = border::Radius {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        };
        let border = Border {
            color: colors.outline.unwrap_or(Color::TRANSPARENT),
            width: if colors.outline.is_some() { 1.0 } else { 0.0 },
            radius,
        };
        let shadow = elevation::shadow(&scheme, colors.level);
        if colors.container.is_some() || colors.outline.is_some() || colors.level > 0 {
            renderer.fill_quad(
                Quad {
                    bounds,
                    border,
                    shadow,
                    snap: true,
                },
                Background::Color(colors.container.unwrap_or(Color::TRANSPARENT)),
            );
        }
        if state.layer.value > 0.001 {
            renderer.fill_quad(
                Quad {
                    bounds,
                    border: Border {
                        width: 0.0,
                        ..border
                    },
                    shadow: Shadow::default(),
                    snap: true,
                },
                Background::Color(Color {
                    a: state.layer.value,
                    ..colors.content
                }),
            );
        }
        if state.focused && state.focus_visible {
            renderer.fill_quad(
                Quad {
                    bounds: bounds.expand(2.0 + FOCUS_RING / 2.0),
                    border: Border {
                        color: scheme.secondary,
                        width: FOCUS_RING,
                        radius: grow(radius, 2.0 + FOCUS_RING / 2.0),
                    },
                    shadow: Shadow::default(),
                    snap: true,
                },
                Background::Color(Color::TRANSPARENT),
            );
        }
        // Content too wide for the button, such as a long label in a
        // narrow sidebar, is cut off at the button's edge.
        let Some(clip) = bounds.intersection(viewport) else {
            return;
        };
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            &renderer::Style {
                text_color: colors.content,
            },
            layout.children().next().unwrap(),
            cursor,
            &clip,
        );
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        if self.enabled() && cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
            viewport,
            translation,
        )
    }
}

/// M3 focus indicator thickness.
const FOCUS_RING: f32 = 3.0;

fn grow(radius: border::Radius, amount: f32) -> border::Radius {
    border::Radius {
        top_left: radius.top_left + amount,
        top_right: radius.top_right + amount,
        bottom_right: radius.bottom_right + amount,
        bottom_left: radius.bottom_left + amount,
    }
}

impl<'a, Message: Clone + 'a> From<Button<'a, Message>> for Element<'a, Message> {
    fn from(button: Button<'a, Message>) -> Self {
        Element::new(button.build())
    }
}
