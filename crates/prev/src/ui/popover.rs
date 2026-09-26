//! A menu or panel that opens below its anchor, such as a toolbar button,
//! and closes when the user clicks outside it.

use iced::advanced::layout::{self, Layout};
use iced::advanced::overlay;
use iced::advanced::renderer;
use iced::advanced::widget::{Operation, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell};
use iced::mouse::{self, Cursor};
use iced::{Element, Event, Length, Point, Rectangle, Size, Theme, Vector, keyboard};

use super::{Scheme, elevation, shape};

const GAP: f32 = 4.0;

pub struct Popover<'a, Message> {
    anchor: Element<'a, Message>,
    content: Option<Element<'a, Message>>,
    on_dismiss: Message,
}

/// Shows `content` below `anchor` while it is `Some`.
pub fn popover<'a, Message: Clone + 'a>(
    anchor: impl Into<Element<'a, Message>>,
    content: Option<Element<'a, Message>>,
    on_dismiss: Message,
) -> Popover<'a, Message> {
    Popover {
        anchor: anchor.into(),
        content,
        on_dismiss,
    }
}

/// An M3 menu surface around `content`.
pub fn surface<'a, Message: 'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    iced::widget::container(content)
        .padding([8, 0])
        .style(|theme: &Theme| {
            let scheme = Scheme::of(theme);
            iced::widget::container::Style {
                background: Some(scheme.surface_container.into()),
                border: iced::border::rounded(shape::LARGE),
                shadow: elevation::shadow(&scheme, 2),
                text_color: Some(scheme.on_surface),
                ..Default::default()
            }
        })
        .into()
}

impl<'a, Message: Clone + 'a> Widget<Message, Theme, iced::Renderer> for Popover<'a, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::stateless()
    }

    fn children(&self) -> Vec<Tree> {
        let mut children = vec![Tree::new(&self.anchor)];
        if let Some(content) = &self.content {
            children.push(Tree::new(content));
        }
        children
    }

    fn diff(&self, tree: &mut Tree) {
        match &self.content {
            Some(content) => tree.diff_children(&[&self.anchor, content]),
            None => tree.diff_children(&[&self.anchor]),
        }
    }

    fn size(&self) -> Size<Length> {
        self.anchor.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.anchor
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
        self.anchor
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
        self.anchor.as_widget_mut().update(
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
        self.anchor.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.anchor.as_widget().mouse_interaction(
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
        let (anchor_tree, rest) = tree.children.split_first_mut()?;
        let Some(content) = self.content.as_mut() else {
            return self.anchor.as_widget_mut().overlay(
                anchor_tree,
                layout,
                renderer,
                viewport,
                translation,
            );
        };
        let bounds = layout.bounds();
        Some(overlay::Element::new(Box::new(PopoverOverlay {
            content,
            tree: &mut rest[0],
            anchor: Rectangle {
                x: bounds.x + translation.x,
                y: bounds.y + translation.y,
                ..bounds
            },
            on_dismiss: self.on_dismiss.clone(),
        })))
    }
}

struct PopoverOverlay<'a, 'b, Message> {
    content: &'a mut Element<'b, Message>,
    tree: &'a mut Tree,
    anchor: Rectangle,
    on_dismiss: Message,
}

impl<Message: Clone> overlay::Overlay<Message, Theme, iced::Renderer>
    for PopoverOverlay<'_, '_, Message>
{
    fn layout(&mut self, renderer: &iced::Renderer, bounds: Size) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, bounds);
        let node = self
            .content
            .as_widget_mut()
            .layout(self.tree, renderer, &limits);
        let size = node.size();
        // Below the anchor, kept inside the window.
        let mut position = Point::new(self.anchor.x, self.anchor.y + self.anchor.height + GAP);
        if position.x + size.width > bounds.width {
            position.x = (bounds.width - size.width).max(0.0);
        }
        if position.y + size.height > bounds.height {
            position.y = (self.anchor.y - size.height - GAP).max(0.0);
        }
        node.move_to(position)
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
    ) {
        let bounds = layout.bounds();
        self.content
            .as_widget()
            .draw(self.tree, renderer, theme, style, layout, cursor, &bounds);
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        self.content
            .as_widget_mut()
            .operate(self.tree, layout, renderer, operation);
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let bounds = layout.bounds();
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(_)) if !cursor.is_over(bounds) => {
                // A click on the anchor toggles the popover itself.
                if !cursor.is_over(self.anchor) {
                    shell.publish(self.on_dismiss.clone());
                }
                return;
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Escape),
                ..
            }) => {
                shell.publish(self.on_dismiss.clone());
                shell.capture_event();
                return;
            }
            _ => {}
        }
        self.content.as_widget_mut().update(
            self.tree, event, layout, cursor, renderer, clipboard, shell, &bounds,
        );
        if let Event::Mouse(_) = event
            && cursor.is_over(bounds)
        {
            shell.capture_event();
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            self.tree,
            layout,
            cursor,
            &layout.bounds(),
            renderer,
        )
    }

    /// Menus inside this one, such as those in a toolbar's overflow.
    fn overlay<'c>(
        &'c mut self,
        layout: Layout<'c>,
        renderer: &iced::Renderer,
    ) -> Option<overlay::Element<'c, Message, Theme, iced::Renderer>> {
        let bounds = layout.bounds();
        self.content
            .as_widget_mut()
            .overlay(self.tree, layout, renderer, &bounds, Vector::ZERO)
    }
}

impl<'a, Message: Clone + 'a> From<Popover<'a, Message>> for Element<'a, Message> {
    fn from(popover: Popover<'a, Message>) -> Self {
        Element::new(popover)
    }
}
