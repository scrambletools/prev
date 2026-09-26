//! Page text in reading order, and text selection between two points.

use crate::geometry::{Point, Quad, Rect};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TextLayout {
    pub lines: Vec<TextLine>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextLine {
    pub bounds: Rect,
    pub chars: Vec<TextChar>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextChar {
    pub character: char,
    pub quad: Quad,
}

/// A position between characters: before `chars[offset]` of `lines[line]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Caret {
    pub line: usize,
    pub offset: usize,
}

/// Selected text as a half-open range of carets in reading order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    pub start: Caret,
    pub end: Caret,
}

impl Selection {
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

impl TextLayout {
    pub fn is_empty(&self) -> bool {
        self.lines.iter().all(|line| line.chars.is_empty())
    }

    /// Whether `point` is over a character, for the text cursor.
    pub fn is_over_text(&self, point: Point) -> bool {
        self.lines
            .iter()
            .filter(|line| line.bounds.contains(point))
            .any(|line| {
                line.chars
                    .iter()
                    .any(|char| char.quad.bounds().contains(point))
            })
    }

    /// The caret nearest to `point`: on the closest line, before the first
    /// character whose center is right of the point.
    pub fn caret_at(&self, point: Point) -> Option<Caret> {
        let line = self.nearest_line(point)?;
        let chars = &self.lines[line].chars;
        let offset = chars
            .iter()
            .position(|char| char.quad.bounds().center().x > point.x)
            .unwrap_or(chars.len());
        Some(Caret { line, offset })
    }

    fn nearest_line(&self, point: Point) -> Option<usize> {
        let distance = |bounds: &Rect| {
            let vertical = if point.y < bounds.y0 {
                bounds.y0 - point.y
            } else if point.y > bounds.y1 {
                point.y - bounds.y1
            } else {
                0.0
            };
            let horizontal = if point.x < bounds.x0 {
                bounds.x0 - point.x
            } else if point.x > bounds.x1 {
                point.x - bounds.x1
            } else {
                0.0
            };
            (vertical, horizontal)
        };
        self.lines
            .iter()
            .enumerate()
            .filter(|(_, line)| !line.chars.is_empty())
            .min_by(|(_, a), (_, b)| {
                distance(&a.bounds)
                    .partial_cmp(&distance(&b.bounds))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(index, _)| index)
    }

    /// Text between the carets nearest to `from` and `to`, in either order.
    pub fn select(&self, from: Point, to: Point) -> Option<Selection> {
        let (a, b) = (self.caret_at(from)?, self.caret_at(to)?);
        Some(Selection {
            start: a.min(b),
            end: a.max(b),
        })
    }

    /// The word under `point`, for double-click.
    pub fn select_word(&self, point: Point) -> Option<Selection> {
        let caret = self.caret_at(point)?;
        let chars = &self.lines[caret.line].chars;
        let is_word = |index: usize| {
            chars
                .get(index)
                .is_some_and(|char| char.character.is_alphanumeric())
        };
        let anchor = if is_word(caret.offset) {
            caret.offset
        } else {
            caret.offset.checked_sub(1)?
        };
        if !is_word(anchor) {
            return None;
        }
        let start = (0..=anchor)
            .rev()
            .take_while(|index| is_word(*index))
            .last()?;
        let end = (anchor..chars.len())
            .take_while(|index| is_word(*index))
            .last()?
            + 1;
        Some(Selection {
            start: Caret {
                line: caret.line,
                offset: start,
            },
            end: Caret {
                line: caret.line,
                offset: end,
            },
        })
    }

    /// The whole line under `point`, for triple-click.
    pub fn select_line(&self, point: Point) -> Option<Selection> {
        let line = self.nearest_line(point)?;
        Some(Selection {
            start: Caret { line, offset: 0 },
            end: Caret {
                line,
                offset: self.lines[line].chars.len(),
            },
        })
    }

    pub fn select_all(&self) -> Option<Selection> {
        let last = self.lines.len().checked_sub(1)?;
        let selection = Selection {
            start: Caret { line: 0, offset: 0 },
            end: Caret {
                line: last,
                offset: self.lines[last].chars.len(),
            },
        };
        (!selection.is_empty()).then_some(selection)
    }

    fn line_ranges(
        &self,
        selection: Selection,
    ) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
        (selection.start.line..=selection.end.line.min(self.lines.len().saturating_sub(1))).map(
            move |line| {
                let len = self.lines[line].chars.len();
                let start = if line == selection.start.line {
                    selection.start.offset.min(len)
                } else {
                    0
                };
                let end = if line == selection.end.line {
                    selection.end.offset.min(len)
                } else {
                    len
                };
                (line, start, end)
            },
        )
    }

    /// One highlight rectangle per selected line.
    pub fn highlight(&self, selection: Selection) -> Vec<Rect> {
        self.line_ranges(selection)
            .filter(|(_, start, end)| start < end)
            .filter_map(|(line, start, end)| {
                self.lines[line].chars[start..end]
                    .iter()
                    .map(|char| char.quad.bounds())
                    .reduce(|a, b| a.union(&b))
            })
            .collect()
    }

    /// The selected text, with a newline between lines.
    pub fn text(&self, selection: Selection) -> String {
        let mut text = String::new();
        for (line, start, end) in self.line_ranges(selection) {
            if line != selection.start.line {
                text.push('\n');
            }
            text.extend(
                self.lines[line].chars[start..end]
                    .iter()
                    .map(|char| char.character),
            );
        }
        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Lines of monospaced text, 10 points per character and 20 per line.
    fn layout(lines: &[&str]) -> TextLayout {
        TextLayout {
            lines: lines
                .iter()
                .enumerate()
                .map(|(row, content)| {
                    let y = row as f32 * 20.0;
                    let chars: Vec<TextChar> = content
                        .chars()
                        .enumerate()
                        .map(|(column, character)| TextChar {
                            character,
                            quad: Rect::new(
                                column as f32 * 10.0,
                                y,
                                column as f32 * 10.0 + 10.0,
                                y + 12.0,
                            )
                            .into(),
                        })
                        .collect();
                    let width = content.chars().count() as f32 * 10.0;
                    TextLine {
                        bounds: Rect::new(0.0, y, width, y + 12.0),
                        chars,
                    }
                })
                .collect(),
        }
    }

    #[test]
    fn selects_within_a_line_in_either_direction() {
        let text = layout(&["hello world"]);
        let forward = text
            .select(Point::new(1.0, 5.0), Point::new(52.0, 5.0))
            .unwrap();
        assert_eq!(text.text(forward), "hello");
        let backward = text
            .select(Point::new(52.0, 5.0), Point::new(1.0, 5.0))
            .unwrap();
        assert_eq!(forward, backward);
        assert_eq!(
            text.highlight(forward),
            vec![Rect::new(0.0, 0.0, 50.0, 12.0)]
        );
    }

    #[test]
    fn selects_across_lines() {
        let text = layout(&["first line", "second line", "third"]);
        let selection = text
            .select(Point::new(62.0, 5.0), Point::new(22.0, 45.0))
            .unwrap();
        assert_eq!(text.text(selection), "line\nsecond line\nth");
        assert_eq!(text.highlight(selection).len(), 3);
    }

    #[test]
    fn points_outside_text_snap_to_the_nearest_line() {
        let text = layout(&["abc", "def"]);
        let caret = text.caret_at(Point::new(500.0, 25.0)).unwrap();
        assert_eq!(caret, Caret { line: 1, offset: 3 });
        let above = text.caret_at(Point::new(-5.0, -50.0)).unwrap();
        assert_eq!(above, Caret { line: 0, offset: 0 });
        assert!(text.is_over_text(Point::new(5.0, 5.0)));
        assert!(!text.is_over_text(Point::new(5.0, 15.0)));
    }

    #[test]
    fn word_and_line_selection() {
        let text = layout(&["one two, three"]);
        let word = text.select_word(Point::new(55.0, 5.0)).unwrap();
        assert_eq!(text.text(word), "two");
        assert_eq!(
            text.select_word(Point::new(75.0, 5.0))
                .map(|word| text.text(word)),
            None
        );
        let line = text.select_line(Point::new(5.0, 5.0)).unwrap();
        assert_eq!(text.text(line), "one two, three");
    }

    #[test]
    fn select_all_and_empty_layouts() {
        let text = layout(&["a", "", "b"]);
        assert_eq!(text.text(text.select_all().unwrap()), "a\n\nb");
        let empty = TextLayout::default();
        assert!(empty.is_empty());
        assert_eq!(
            empty.select(Point::new(0.0, 0.0), Point::new(1.0, 1.0)),
            None
        );
        assert_eq!(empty.select_all(), None);
    }
}
