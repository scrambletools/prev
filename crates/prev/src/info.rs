//! What the inspectors show: facts about a file, readable sizes and
//! dates, and the labeled sections they are drawn in.

use std::path::Path;

use crate::{column, row};
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeZone};
use iced::Element;
use iced::widget::text;

use crate::ui::{self, Type, component, style};

/// A labeled value in an inspector.
pub type Fact = (String, String);

/// Name, folder, size and modification time of `path`.
pub fn file_facts(path: &Path) -> Vec<Fact> {
    let mut facts = Vec::new();
    if let Some(name) = path.file_name() {
        facts.push((
            crate::fl!("app-fact-name"),
            name.to_string_lossy().into_owned(),
        ));
    }
    if let Some(folder) = path.parent() {
        facts.push((
            crate::fl!("app-fact-folder"),
            prev_store::paths::abbreviate_home(folder)
                .to_string_lossy()
                .into_owned(),
        ));
    }
    if let Ok(metadata) = std::fs::metadata(path) {
        facts.push((crate::fl!("app-fact-size"), human_size(metadata.len())));
        if let Ok(modified) = metadata.modified() {
            let modified: DateTime<Local> = modified.into();
            facts.push((crate::fl!("app-fact-modified"), readable(&modified)));
        }
    }
    facts
}

/// Bytes as a short size, in the units file managers use.
pub fn human_size(bytes: u64) -> String {
    if bytes < 1000 {
        return crate::fl!("app-size-bytes", count = bytes);
    }
    let mut value = bytes as f64;
    let mut unit = 0;
    for next in 0..4 {
        value /= 1000.0;
        unit = next;
        if value < 1000.0 {
            break;
        }
    }
    let size = if value < 10.0 {
        format!("{value:.1}")
    } else {
        format!("{value:.0}")
    };
    match unit {
        0 => crate::fl!("app-size-kb", size = size),
        1 => crate::fl!("app-size-mb", size = size),
        2 => crate::fl!("app-size-gb", size = size),
        _ => crate::fl!("app-size-tb", size = size),
    }
}

fn readable<Tz: TimeZone>(time: &DateTime<Tz>) -> String
where
    Tz::Offset: std::fmt::Display,
{
    time.with_timezone(&Local)
        .format("%-d %B %Y, %H:%M")
        .to_string()
}

/// A PDF date, `D:YYYYMMDDHHmmSSOHH'mm'` with everything after the year
/// optional, as a readable local time. Other text is shown as it is.
pub fn pdf_date(raw: &str) -> String {
    let text = raw.trim().trim_start_matches("D:");
    let digits: String = text.chars().take_while(char::is_ascii_digit).collect();
    let part = |range: std::ops::Range<usize>, default: u32| {
        digits
            .get(range)
            .and_then(|part| part.parse().ok())
            .unwrap_or(default)
    };
    let Some(year) = digits.get(0..4).and_then(|year| year.parse::<i32>().ok()) else {
        return raw.to_owned();
    };
    let Some(date) = NaiveDate::from_ymd_opt(year, part(4..6, 1), part(6..8, 1)) else {
        return raw.to_owned();
    };
    let Some(naive) = date.and_hms_opt(part(8..10, 0), part(10..12, 0), part(12..14, 0)) else {
        return raw.to_owned();
    };
    // The zone: Z, or +HH'mm' / -HH'mm'; local time when absent.
    let zone = &text[digits.len()..];
    let offset = match zone.chars().next() {
        Some('Z') => Some(0),
        Some(sign @ ('+' | '-')) => {
            let numbers: String = zone[1..].chars().filter(char::is_ascii_digit).collect();
            let hours: i32 = numbers.get(0..2).and_then(|h| h.parse().ok()).unwrap_or(0);
            let minutes: i32 = numbers.get(2..4).and_then(|m| m.parse().ok()).unwrap_or(0);
            let seconds = hours * 3600 + minutes * 60;
            Some(if sign == '-' { -seconds } else { seconds })
        }
        _ => None,
    };
    let when = |naive: NaiveDateTime| match offset
        .and_then(chrono::FixedOffset::east_opt)
        .and_then(|zone| zone.from_local_datetime(&naive).single())
    {
        Some(time) => Some(readable(&time)),
        None => Local
            .from_local_datetime(&naive)
            .single()
            .map(|time| readable(&time)),
    };
    when(naive).unwrap_or_else(|| raw.to_owned())
}

/// Sections of facts, as the inspectors show them: a heading, then each
/// label beside its value. Empty values are left out.
pub fn sections_view<'a, Message: Clone + 'a>(
    sections: Vec<(impl text::IntoFragment<'a>, Vec<Fact>)>,
) -> Element<'a, Message> {
    let mut content = column![].spacing(6);
    for (heading, facts) in sections {
        let facts: Vec<Fact> = facts
            .into_iter()
            .filter(|(_, value)| !value.trim().is_empty())
            .collect();
        if facts.is_empty() {
            continue;
        }
        content = content.push(component::section(heading));
        for (label, value) in facts {
            content = content.push(
                row![
                    ui::aligned_to(
                        ui::styled(label, Type::BodySmall).style(style::on_surface_variant),
                        104,
                    ),
                    ui::aligned(
                        ui::styled(value, Type::BodyMedium).wrapping(text::Wrapping::WordOrGlyph)
                    ),
                ]
                .spacing(8),
            );
        }
    }
    content.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_read_like_a_file_manager() {
        assert_eq!(human_size(512), "512 bytes");
        assert_eq!(human_size(1_500), "1.5 KB");
        assert_eq!(human_size(73_332), "73 KB");
        assert_eq!(human_size(25_965_009), "26 MB");
    }

    #[test]
    fn pdf_dates_become_readable() {
        let utc = pdf_date("D:20240315143000Z");
        let expected = readable(
            &chrono::Utc
                .with_ymd_and_hms(2024, 3, 15, 14, 30, 0)
                .unwrap(),
        );
        assert_eq!(utc, expected);
        let zoned = pdf_date("D:20240315143000+02'00'");
        let expected = readable(
            &chrono::Utc
                .with_ymd_and_hms(2024, 3, 15, 12, 30, 0)
                .unwrap(),
        );
        assert_eq!(zoned, expected);
        assert!(pdf_date("D:2024").contains("2024"));
        assert_eq!(pdf_date("sometime"), "sometime");
    }
}
