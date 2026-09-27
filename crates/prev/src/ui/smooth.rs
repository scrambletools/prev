//! Smooth scrolling for mouse wheels. A wheel notch arrives as a jump of
//! whole lines, which iced's scrollables take in one step; this turns it
//! into a short eased glide of pixel scrolls, as a touchpad sends, so every
//! scrollable under it moves smoothly.

use std::time::Instant;

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{Operation, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay};
use iced::mouse::{self, Cursor, ScrollDelta};
use iced::{Element, Event, Length, Point, Rectangle, Size, Theme, Vector, keyboard, window};

use super::motion;

/// Pixels per wheel line, as iced's scrollables use.
const LINE: f32 = 60.0;
/// Time constant of the glide, in seconds: most of a notch's distance is
/// covered in about three of these.
const TAU: f32 = 0.06;
/// Frames longer than this (a stall) are cut short.
const LONGEST_FRAME: f32 = 0.1;

pub struct Smooth<'a, Message> {
    content: Element<'a, Message>,
}

/// `content`, with mouse wheel scrolling inside it made smooth.
pub fn smooth<'a, Message: 'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    Element::new(Smooth {
        content: content.into(),
    })
}

#[derive(Default)]
struct State {
    /// Distance still to scroll, in pixels, as a pixel scroll delta.
    remaining: Vector,
    /// Where the wheel was turned; the glide scrolls what is there.
    at: Point,
    last: Option<Instant>,
    modifiers: keyboard::Modifiers,
}

/// Adds `add` to a remaining distance on one axis, starting over when the
/// direction changes.
fn gather(remaining: f32, add: f32) -> f32 {
    if add == 0.0 {
        remaining
    } else if remaining * add < 0.0 {
        add
    } else {
        remaining + add
    }
}

impl<'a, Message: 'a> Widget<Message, Theme, iced::Renderer> for Smooth<'a, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
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
        match event {
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                state.modifiers = *modifiers;
            }
            // With Ctrl held the wheel zooms, a step per notch.
            Event::Mouse(mouse::Event::WheelScrolled {
                delta: ScrollDelta::Lines { x, y },
            }) if !state.modifiers.command() && !motion::reduced() => {
                if let Some(position) = cursor.position() {
                    // Shift turns the wheel sideways, as scrollables do
                    // for lines but not for pixels.
                    let (x, y) = if state.modifiers.shift() {
                        (*y, *x)
                    } else {
                        (*x, *y)
                    };
                    state.remaining = Vector::new(
                        gather(state.remaining.x, x * LINE),
                        gather(state.remaining.y, y * LINE),
                    );
                    state.at = position;
                    shell.request_redraw();
                    shell.capture_event();
                    return;
                }
            }
            _ => {}
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

        let state = tree.state.downcast_mut::<State>();
        let Event::Window(window::Event::RedrawRequested(now)) = event else {
            return;
        };
        if state.remaining == Vector::ZERO {
            state.last = None;
            return;
        }
        let elapsed = match state.last.replace(*now) {
            Some(last) => now
                .saturating_duration_since(last)
                .as_secs_f32()
                .min(LONGEST_FRAME),
            None => 1.0 / 60.0,
        };
        let share = 1.0 - (-elapsed / TAU).exp();
        let mut step = state.remaining * share;
        if (state.remaining - step).x.abs() < 0.5 && (state.remaining - step).y.abs() < 0.5 {
            step = state.remaining;
        }
        state.remaining -= step;
        let at = state.at;
        if state.remaining != Vector::ZERO {
            shell.request_redraw();
        }
        let scroll = Event::Mouse(mouse::Event::WheelScrolled {
            delta: ScrollDelta::Pixels {
                x: step.x,
                y: step.y,
            },
        });
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            &scroll,
            layout,
            Cursor::Available(at),
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
        self.content.as_widget().draw(
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

#[cfg(test)]
mod tests {
    use super::gather;

    #[test]
    fn notches_add_up_and_a_turn_back_starts_over() {
        assert_eq!(gather(60.0, 60.0), 120.0);
        assert_eq!(gather(90.0, -60.0), -60.0);
        assert_eq!(gather(-30.0, 0.0), -30.0);
    }
}
