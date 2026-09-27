//! Finding text in a Markdown document: marking matches in the spans each
//! paragraph, heading and code line is drawn from, and scrolling to the
//! current match.

use std::any::Any;
use std::borrow::Cow;
use std::cell::Cell;
use std::ops::Range;

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::text::Highlight;
use iced::advanced::widget::operation::{Operation, Outcome, Scrollable};
use iced::advanced::widget::{Tree, Widget};
use iced::advanced::{Clipboard, Shell, overlay};
use iced::mouse::{self, Cursor};
use iced::widget::Id;
use iced::widget::markdown::Uri;
use iced::widget::text::Span;
use iced::{Color, Element, Event, Length, Rectangle, Size, Theme, Vector, border};

pub type Spans = Vec<Span<'static, Uri>>;

/// The search as the document is drawn: what to find, which match is the
/// current one, and a count of the matches drawn so far, which numbers
/// them in document order. The count lives with the window, so it can be
/// read once the document is drawn.
pub struct Finder<'a> {
    query: &'a str,
    current: Option<usize>,
    count: &'a Cell<usize>,
    match_color: Color,
    current_color: Color,
}

impl<'a> Finder<'a> {
    pub fn new(
        query: &'a str,
        current: Option<usize>,
        count: &'a Cell<usize>,
        match_color: Color,
        current_color: Color,
    ) -> Self {
        count.set(0);
        Self {
            query: query.trim(),
            current,
            count,
            match_color,
            current_color,
        }
    }

    /// How many matches were drawn.
    #[cfg(test)]
    pub fn count(&self) -> usize {
        self.count.get()
    }

    /// `spans` with the matches highlighted, and whether the current match
    /// is among them.
    pub fn mark(&self, spans: &[Span<'static, Uri>]) -> (Spans, bool) {
        if self.query.is_empty() {
            return (spans.to_vec(), false);
        }
        let text: String = spans.iter().map(|span| span.text.as_ref()).collect();
        let found = find_all(&text, self.query);
        if found.is_empty() {
            return (spans.to_vec(), false);
        }
        let first = self.count.get();
        self.count.set(first + found.len());
        let current = self
            .current
            .filter(|current| (first..first + found.len()).contains(current))
            .map(|current| current - first);

        let mut marked = Vec::with_capacity(spans.len() + found.len() * 2);
        let mut start = 0;
        for span in spans {
            let end = start + span.text.len();
            // Cut the span where matches start and end inside it.
            let mut cuts = vec![start, end];
            for range in &found {
                for cut in [range.start, range.end] {
                    if cut > start && cut < end {
                        cuts.push(cut);
                    }
                }
            }
            cuts.sort_unstable();
            cuts.dedup();
            for piece in cuts.windows(2) {
                let (from, to) = (piece[0], piece[1]);
                let mut part = span.clone();
                part.text = Cow::Owned(text[from..to].to_owned());
                if let Some(index) = found
                    .iter()
                    .position(|range| range.start <= from && to <= range.end)
                {
                    let color = if Some(index) == current {
                        self.current_color
                    } else {
                        self.match_color
                    };
                    part.highlight = Some(Highlight {
                        background: color.into(),
                        border: border::rounded(2),
                    });
                }
                marked.push(part);
            }
            start = end;
        }
        (marked, current.is_some())
    }
}

/// Where `needle` occurs in `haystack`, ignoring case, without overlaps.
pub fn find_all(haystack: &str, needle: &str) -> Vec<Range<usize>> {
    if needle.is_empty() {
        return Vec::new();
    }
    let lower = haystack.to_lowercase();
    let needle_lower = needle.to_lowercase();
    // Lowercasing keeps byte offsets only when no character changes length;
    // otherwise match the case as typed.
    let (haystack, needle) = if lower.len() == haystack.len() {
        (lower.as_str(), needle_lower.as_str())
    } else {
        (haystack, needle)
    };
    haystack
        .match_indices(needle)
        .map(|(start, found)| start..start + found.len())
        .collect()
}

/// Marks the element with the current match, so `Reveal` can find it.
pub struct Current<'a, Message> {
    content: Element<'a, Message>,
}

/// Tells the current match from other widgets reporting to operations.
struct CurrentMatch;

pub fn current<'a, Message: 'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    Element::new(Current {
        content: content.into(),
    })
}

impl<'a, Message: 'a> Widget<Message, Theme, iced::Renderer> for Current<'a, Message> {
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
        operation.custom(None, layout.bounds(), &mut CurrentMatch);
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

/// Finds how far to scroll the scrollable `id` so the current match sits a
/// third of the way down, when it is not already in view.
pub struct Reveal {
    id: Id,
    scroll: Option<(Rectangle, Rectangle, Vector)>,
    target: Option<Rectangle>,
}

impl Reveal {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            scroll: None,
            target: None,
        }
    }
}

impl Operation<Option<f32>> for Reveal {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<Option<f32>>)) {
        operate(self);
    }

    fn scrollable(
        &mut self,
        id: Option<&Id>,
        bounds: Rectangle,
        content_bounds: Rectangle,
        translation: Vector,
        _state: &mut dyn Scrollable,
    ) {
        if id == Some(&self.id) {
            self.scroll = Some((bounds, content_bounds, translation));
        }
    }

    fn custom(&mut self, _id: Option<&Id>, bounds: Rectangle, state: &mut dyn Any) {
        if state.is::<CurrentMatch>() && self.target.is_none() {
            self.target = Some(bounds);
        }
    }

    fn finish(&self) -> Outcome<Option<f32>> {
        let (Some((bounds, content, translation)), Some(target)) = (self.scroll, self.target)
        else {
            return Outcome::Some(None);
        };
        // The match's place in the content, and the part now in view.
        let top = target.y - content.y;
        let shown = translation.y..translation.y + bounds.height;
        if top >= shown.start && top + target.height <= shown.end {
            return Outcome::Some(None);
        }
        Outcome::Some(Some((top - bounds.height / 3.0).max(0.0)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_every_match_ignoring_case() {
        assert_eq!(find_all("Rust and rust", "RUST"), vec![0..4, 9..13]);
        assert_eq!(find_all("aaaa", "aa"), vec![0..2, 2..4]);
        assert!(find_all("text", "").is_empty());
    }

    #[test]
    fn marks_matches_across_spans() {
        let count = Cell::new(0);
        let finder = Finder::new("ld te", None, &count, Color::BLACK, Color::WHITE);
        let spans: Spans = vec![Span::new("bold"), Span::new(" text")];
        let (marked, current) = finder.mark(&spans);
        let texts: Vec<&str> = marked.iter().map(|span| span.text.as_ref()).collect();
        assert_eq!(texts, ["bo", "ld", " te", "xt"]);
        assert!(marked[1].highlight.is_some() && marked[2].highlight.is_some());
        assert!(marked[0].highlight.is_none() && marked[3].highlight.is_none());
        assert_eq!(finder.count(), 1);
        assert!(!current);
    }

    #[test]
    fn numbers_matches_in_drawing_order() {
        let count = Cell::new(0);
        let finder = Finder::new("a", Some(2), &count, Color::BLACK, Color::WHITE);
        let one: Spans = vec![Span::new("a a")];
        let two: Spans = vec![Span::new("a")];
        assert!(!finder.mark(&one).1);
        assert!(finder.mark(&two).1);
        assert_eq!(finder.count(), 3);
    }
}
