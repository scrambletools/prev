//! Code blocks highlighted in colors that suit the window's light or dark
//! theme. iced's Markdown parser colors code once, in a dark theme, so
//! prev colors the code itself and keeps the result for each block.

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use iced::highlighter::{self, Stream};
use iced::widget::markdown::Uri;
use iced::widget::text::Span;
use iced::{Color, Font};

/// A highlighted piece of a line: its text, color and font.
#[derive(Debug, Clone)]
struct Piece {
    text: String,
    color: Option<Color>,
    font: Option<Font>,
}

type Lines = Rc<Vec<Vec<Piece>>>;

/// Highlighted code blocks, by theme, language and code.
#[derive(Default)]
pub struct Highlights {
    blocks: RefCell<HashMap<u64, Lines>>,
}

impl Highlights {
    pub fn clear(&self) {
        self.blocks.borrow_mut().clear();
    }

    /// The lines of `code` as spans, colored for a dark or light theme.
    pub fn lines(
        &self,
        language: Option<&str>,
        code: &str,
        dark: bool,
    ) -> Vec<Vec<Span<'static, Uri>>> {
        let mut hasher = DefaultHasher::new();
        (dark, language, code).hash(&mut hasher);
        let key = hasher.finish();
        let lines = self
            .blocks
            .borrow_mut()
            .entry(key)
            .or_insert_with(|| Rc::new(highlight(language, code, dark)))
            .clone();
        lines
            .iter()
            .map(|line| {
                line.iter()
                    .map(|piece| {
                        let mut span = Span::new(piece.text.clone());
                        span.color = piece.color;
                        span.font = piece.font;
                        span
                    })
                    .collect()
            })
            .collect()
    }
}

fn highlight(language: Option<&str>, code: &str, dark: bool) -> Vec<Vec<Piece>> {
    let mut stream = Stream::new(&highlighter::Settings {
        theme: if dark {
            highlighter::Theme::Base16Ocean
        } else {
            highlighter::Theme::InspiredGitHub
        },
        token: language.unwrap_or("txt").to_owned(),
    });
    code.lines()
        .map(|line| {
            let pieces = stream
                .highlight_line(line)
                .map(|(range, highlight)| Piece {
                    text: line[range].to_owned(),
                    color: highlight.color(),
                    font: highlight.font(),
                })
                .collect();
            stream.commit();
            pieces
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_every_character_of_each_line() {
        let highlights = Highlights::default();
        let code = "fn main() {\n    println!(\"hi\");\n}";
        for dark in [false, true] {
            let lines = highlights.lines(Some("rust"), code, dark);
            let text: Vec<String> = lines
                .iter()
                .map(|line| line.iter().map(|span| span.text.as_ref()).collect())
                .collect();
            assert_eq!(text, code.lines().collect::<Vec<_>>());
        }
    }

    #[test]
    fn colors_differ_between_themes() {
        let highlights = Highlights::default();
        let color = |dark| highlights.lines(Some("rust"), "fn main() {}", dark)[0][0].color;
        assert_ne!(color(false), color(true));
    }
}
