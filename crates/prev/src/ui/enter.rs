//! Springs an element into place when it first appears: side sheets
//! slide in from their edge, dialogs grow from slightly smaller.

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{Operation, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay};
use iced::mouse::{self, Cursor};
use iced::{Element, Event, Length, Rectangle, Size, Theme, Transformation, Vector, window};

use super::motion::{Motion, Spring};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum From {
    /// Offset by this fraction of the element's own size.
    Offset(f32, f32),
    /// Scaled by this factor about the centre.
    Scale(f32),
}

pub struct Enter<'a, Message> {
    content: Element<'a, Message>,
    from: From,
    motion: Motion,
}

pub fn enter<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    from: From,
) -> Enter<'a, Message> {
    Enter {
        content: content.into(),
        from,
        motion: Motion::DEFAULT_SPATIAL,
    }
}

/// A side sheet sliding in from the right edge.
pub fn from_right<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    enter(content, From::Offset(1.0, 0.0)).into()
}

/// A side sheet sliding in from the left edge.
pub fn from_left<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    enter(content, From::Offset(-1.0, 0.0)).into()
}

/// A snackbar rising from below.
pub fn from_below<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    enter(content, From::Offset(0.0, 1.5)).into()
}

/// A dialog growing into place.
pub fn grow<'a, Message: 'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    let mut enter = enter(content, From::Scale(0.9));
    enter.motion = Motion::FAST_SPATIAL;
    enter.into()
}

struct State {
    /// 0 at the start position, 1 in place.
    progress: Spring,
}

impl<Message> Enter<'_, Message> {
    fn transformation(&self, state: &State, bounds: Rectangle) -> Transformation {
        let remaining = 1.0 - state.progress.value;
        match self.from {
            From::Offset(x, y) => Transformation::translate(
                x * bounds.width * remaining,
                y * bounds.height * remaining,
            ),
            From::Scale(scale) => {
                let factor = 1.0 - (1.0 - scale) * remaining;
                let centre = bounds.center();
                Transformation::translate(centre.x, centre.y)
                    * Transformation::scale(factor)
                    * Transformation::translate(-centre.x, -centre.y)
            }
        }
    }
}

impl<'a, Message: 'a> Widget<Message, Theme, iced::Renderer> for Enter<'a, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        let mut progress = Spring::new(0.0, self.motion).precision(0.002);
        progress.go_to(1.0);
        tree::State::new(State { progress })
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.content.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        self.content
            .as_widget_mut()
            .operate(&mut tree.children[0], layout, renderer, operation);
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
        let state = tree.state.downcast_mut::<State>();
        if !state.progress.is_settled() {
            if let Event::Window(window::Event::RedrawRequested(now)) = event {
                state.progress.advance(*now);
            }
            shell.request_redraw();
        }
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        use iced::advanced::Renderer as _;
        let state = tree.state.downcast_ref::<State>();
        let draw = |renderer: &mut iced::Renderer| {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                layout,
                cursor,
                viewport,
            );
        };
        if state.progress.is_settled() {
            draw(renderer);
            return;
        }
        let bounds = layout.bounds();
        let transformation = self.transformation(state, bounds);
        // Slides stay inside their own area, as if coming out from under
        // the edge of the window.
        let clip = match self.from {
            From::Offset(..) => bounds,
            From::Scale(_) => *viewport,
        };
        renderer.with_layer(clip, |renderer| {
            renderer.with_transformation(transformation, draw);
        });
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
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
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message: 'a> std::convert::From<Enter<'a, Message>> for Element<'a, Message> {
    fn from(enter: Enter<'a, Message>) -> Self {
        Element::new(enter)
    }
}
