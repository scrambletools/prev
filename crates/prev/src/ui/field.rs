//! The label of an outlined text field: inside the field while it is empty
//! and unfocused, and in its outline once it has the keyboard focus or
//! text, as in Material 3. Which one shows depends on the text input's
//! focus, which only its own widget state knows, so the choice is made
//! while drawing.

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{Operation, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell};
use iced::mouse::{self, Cursor};
use iced::widget::text_input;
use iced::{Element, Event, Length, Rectangle, Size, Theme};

type Paragraph = <iced::Renderer as iced::advanced::text::Renderer>::Paragraph;

/// A field and its label in its three places. `field` is the text input,
/// or a container around it.
pub struct Labelled<'a, Message> {
    field: Element<'a, Message>,
    /// In the outline, while the field is unfocused but has text.
    raised: Element<'a, Message>,
    /// In the outline, while the field has the focus.
    raised_focused: Element<'a, Message>,
    /// Inside the field, standing in for a placeholder.
    resting: Element<'a, Message>,
    empty: bool,
}

pub fn labelled<'a, Message: 'a>(
    field: impl Into<Element<'a, Message>>,
    raised: impl Into<Element<'a, Message>>,
    raised_focused: impl Into<Element<'a, Message>>,
    resting: impl Into<Element<'a, Message>>,
    empty: bool,
) -> Labelled<'a, Message> {
    Labelled {
        field: field.into(),
        raised: raised.into(),
        raised_focused: raised_focused.into(),
        resting: resting.into(),
        empty,
    }
}

/// Whether the field's text input has the focus. A container passes its
/// content's tree through as its own, so the field's tree is the input's.
fn focused(field: &Tree) -> bool {
    field.tag == tree::Tag::of::<text_input::State<Paragraph>>()
        && field
            .state
            .downcast_ref::<text_input::State<Paragraph>>()
            .is_focused()
}

impl<'a, Message: 'a> Labelled<'a, Message> {
    fn children_mut(&mut self) -> [&mut Element<'a, Message>; 4] {
        [
            &mut self.field,
            &mut self.raised,
            &mut self.raised_focused,
            &mut self.resting,
        ]
    }
}

impl<'a, Message: 'a> Widget<Message, Theme, iced::Renderer> for Labelled<'a, Message> {
    fn children(&self) -> Vec<Tree> {
        vec![
            Tree::new(&self.field),
            Tree::new(&self.raised),
            Tree::new(&self.raised_focused),
            Tree::new(&self.resting),
        ]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[
            &self.field,
            &self.raised,
            &self.raised_focused,
            &self.resting,
        ]);
    }

    fn size(&self) -> Size<Length> {
        self.field.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let field = self
            .field
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits);
        let size = field.size();
        // The labels lie over the field, in its bounds.
        let within = layout::Limits::new(Size::ZERO, size);
        let mut nodes = vec![field];
        for (index, label) in self.children_mut().into_iter().enumerate().skip(1) {
            nodes.push(
                label
                    .as_widget_mut()
                    .layout(&mut tree.children[index], renderer, &within),
            );
        }
        layout::Node::with_children(size, nodes)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let field = layout.children().next().expect("field layout");
        self.field
            .as_widget_mut()
            .operate(&mut tree.children[0], field, renderer, operation);
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
        let before = focused(&tree.children[0]);
        let field = layout.children().next().expect("field layout");
        self.field.as_widget_mut().update(
            &mut tree.children[0],
            event,
            field,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
        // The label moves when the focus does.
        if focused(&tree.children[0]) != before {
            shell.request_redraw();
        }
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
        let mut layouts = layout.children();
        let field = layouts.next().expect("field layout");
        self.field.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            field,
            cursor,
            viewport,
        );
        let focused = focused(&tree.children[0]);
        let (index, label) = if focused {
            (2, &self.raised_focused)
        } else if !self.empty {
            (1, &self.raised)
        } else {
            (3, &self.resting)
        };
        let label_layout = layouts.nth(index - 1).expect("label layout");
        label.as_widget().draw(
            &tree.children[index],
            renderer,
            theme,
            style,
            label_layout,
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
        let field = layout.children().next().expect("field layout");
        self.field.as_widget().mouse_interaction(
            &tree.children[0],
            field,
            cursor,
            viewport,
            renderer,
        )
    }
}

impl<'a, Message: 'a> From<Labelled<'a, Message>> for Element<'a, Message> {
    fn from(labelled: Labelled<'a, Message>) -> Self {
        Element::new(labelled)
    }
}
