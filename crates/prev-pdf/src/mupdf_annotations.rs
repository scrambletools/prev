//! Reads and writes annotations and form fields with MuPDF.

use mupdf::color::AnnotationColor;
use mupdf::pdf::{
    AnnotationBorderStyle, AnnotationTextAlign, LineEndingStyle, PdfAnnotation, PdfAnnotationType,
    PdfDocument, PdfObject, PdfPage, WidgetType,
};
use mupdf::{Buffer, Colorspace, Image, Pixmap};

use crate::annotation::{
    Align, Annotation, Field, FieldKind, Font, Kind, LineEnd, Removed, Rgb, StampContent, Style,
    TextMarkup,
};
use crate::engine::{Error, Result};
use crate::geometry::{Point, Quad, Rect};

fn engine_error(error: mupdf::Error) -> Error {
    Error::Engine(error.to_string())
}

/// Converts between prev's page space (origin at the page's top-left) and
/// MuPDF's, which is offset by the page bounds' origin.
#[derive(Clone, Copy)]
pub(crate) struct Space {
    pub x0: f32,
    pub y0: f32,
}

impl Space {
    pub fn of(page: &PdfPage) -> Result<Self> {
        let bounds = page.bounds().map_err(engine_error)?;
        Ok(Self {
            x0: bounds.x0,
            y0: bounds.y0,
        })
    }

    fn point(self, point: mupdf::Point) -> Point {
        Point::new(point.x - self.x0, point.y - self.y0)
    }

    fn to_mupdf(self, point: Point) -> mupdf::Point {
        mupdf::Point::new(point.x + self.x0, point.y + self.y0)
    }

    fn rect(self, rect: mupdf::Rect) -> Rect {
        Rect::new(
            rect.x0 - self.x0,
            rect.y0 - self.y0,
            rect.x1 - self.x0,
            rect.y1 - self.y0,
        )
    }

    fn to_mupdf_rect(self, rect: Rect) -> mupdf::Rect {
        mupdf::Rect::new(
            rect.x0 + self.x0,
            rect.y0 + self.y0,
            rect.x1 + self.x0,
            rect.y1 + self.y0,
        )
    }

    fn quad(self, quad: mupdf::Quad) -> Quad {
        Quad {
            ul: self.point(quad.ul),
            ur: self.point(quad.ur),
            ll: self.point(quad.ll),
            lr: self.point(quad.lr),
        }
    }

    fn to_mupdf_quad(self, quad: Quad) -> mupdf::Quad {
        mupdf::Quad::new(
            self.to_mupdf(quad.ul),
            self.to_mupdf(quad.ur),
            self.to_mupdf(quad.ll),
            self.to_mupdf(quad.lr),
        )
    }
}

fn rgb(color: Option<AnnotationColor>) -> Option<Rgb> {
    match color? {
        AnnotationColor::Gray(gray) => Some(Rgb::new(gray, gray, gray)),
        AnnotationColor::Rgb { red, green, blue } => Some(Rgb::new(red, green, blue)),
        AnnotationColor::Cmyk {
            cyan,
            magenta,
            yellow,
            key,
        } => Some(Rgb::new(
            (1.0 - cyan) * (1.0 - key),
            (1.0 - magenta) * (1.0 - key),
            (1.0 - yellow) * (1.0 - key),
        )),
    }
}

fn annotation_color(color: Rgb) -> AnnotationColor {
    AnnotationColor::Rgb {
        red: color.red,
        green: color.green,
        blue: color.blue,
    }
}

fn line_end(style: LineEndingStyle) -> LineEnd {
    match style {
        LineEndingStyle::OpenArrow => LineEnd::OpenArrow,
        LineEndingStyle::ClosedArrow => LineEnd::ClosedArrow,
        LineEndingStyle::Circle => LineEnd::Circle,
        LineEndingStyle::Square => LineEnd::Square,
        LineEndingStyle::Diamond => LineEnd::Diamond,
        LineEndingStyle::Butt => LineEnd::Butt,
        LineEndingStyle::Slash => LineEnd::Slash,
        _ => LineEnd::None,
    }
}

fn line_ending_style(end: LineEnd) -> LineEndingStyle {
    match end {
        LineEnd::None => LineEndingStyle::None,
        LineEnd::OpenArrow => LineEndingStyle::OpenArrow,
        LineEnd::ClosedArrow => LineEndingStyle::ClosedArrow,
        LineEnd::Circle => LineEndingStyle::Circle,
        LineEnd::Square => LineEndingStyle::Square,
        LineEnd::Diamond => LineEndingStyle::Diamond,
        LineEnd::Butt => LineEndingStyle::Butt,
        LineEnd::Slash => LineEndingStyle::Slash,
    }
}

fn string_entry(object: &PdfObject, key: &str) -> Option<String> {
    object
        .get_dict(key)
        .ok()
        .flatten()
        .and_then(|value| value.as_string().ok())
        .filter(|value| !value.is_empty())
}

/// The id prev uses: `/NM` when present, otherwise the object number.
fn annotation_id(annot: &PdfAnnotation) -> String {
    string_entry(&annot.object(), "NM")
        .unwrap_or_else(|| format!("object-{}", annot.xref().unwrap_or(0)))
}

fn hidden_type(kind: PdfAnnotationType) -> bool {
    matches!(
        kind,
        PdfAnnotationType::Popup | PdfAnnotationType::Widget | PdfAnnotationType::Link
    )
}

pub(crate) fn read_annotations(page: &PdfPage) -> Result<Vec<Annotation>> {
    let space = Space::of(page)?;
    let mut annotations = Vec::new();
    for annot in page.annotations() {
        let Ok(annotation_type) = annot.r#type() else {
            continue;
        };
        if hidden_type(annotation_type) {
            continue;
        }
        annotations.push(read_annotation(&annot, annotation_type, space)?);
    }
    Ok(annotations)
}

fn read_annotation(
    annot: &PdfAnnotation,
    annotation_type: PdfAnnotationType,
    space: Space,
) -> Result<Annotation> {
    let markup = |style| -> Result<Kind> {
        let quads = annot.quad_points().map_err(engine_error)?;
        Ok(Kind::Markup {
            style,
            quads: quads.into_iter().map(|quad| space.quad(quad)).collect(),
        })
    };
    let points = |points: Vec<mupdf::Point>| -> Vec<Point> {
        points.into_iter().map(|point| space.point(point)).collect()
    };
    let kind = match annotation_type {
        PdfAnnotationType::Highlight => markup(TextMarkup::Highlight)?,
        PdfAnnotationType::Underline => markup(TextMarkup::Underline)?,
        PdfAnnotationType::StrikeOut => markup(TextMarkup::StrikeOut)?,
        PdfAnnotationType::Squiggly => markup(TextMarkup::Squiggly)?,
        PdfAnnotationType::Ink => Kind::Ink(
            annot
                .ink_list()
                .map_err(engine_error)?
                .into_iter()
                .map(points)
                .collect(),
        ),
        PdfAnnotationType::Square => Kind::Square,
        PdfAnnotationType::Circle => Kind::Circle,
        PdfAnnotationType::Line => {
            let (start, end) = annot.line().map_err(engine_error)?;
            let (first, last) = annot
                .line_ending_styles()
                .unwrap_or((LineEndingStyle::None, LineEndingStyle::None));
            Kind::Line {
                start: space.point(start),
                end: space.point(end),
                endings: (line_end(first), line_end(last)),
            }
        }
        PdfAnnotationType::Polygon => {
            Kind::Polygon(points(annot.vertices().map_err(engine_error)?))
        }
        PdfAnnotationType::PolyLine => {
            Kind::PolyLine(points(annot.vertices().map_err(engine_error)?))
        }
        PdfAnnotationType::FreeText => Kind::FreeText,
        PdfAnnotationType::Text => Kind::Note,
        PdfAnnotationType::Stamp => Kind::Stamp,
        PdfAnnotationType::Redact => Kind::Redact,
        other => Kind::Other(format!("{other:?}")),
    };
    let rect = space.rect(annot.rect().map_err(engine_error)?);
    let mut style = Style {
        color: rgb(annot.color().ok().flatten()),
        fill: rgb(annot.interior_color().ok().flatten()),
        line_width: annot.border_width().unwrap_or(1.0),
        dashed: annot.border_style().ok() == Some(AnnotationBorderStyle::Dashed),
        opacity: annot.opacity().unwrap_or(1.0),
        ..Style::default()
    };
    if kind == Kind::FreeText {
        if let Ok(Some(appearance)) = annot.default_appearance() {
            style.font = Font::from_resource_name(&appearance.font_name);
            style.font_size = appearance.size;
            style.text_color = rgb(appearance.color).unwrap_or(Rgb::BLACK);
        }
        style.align = match annot.quadding() {
            Ok(AnnotationTextAlign::Center) => Align::Center,
            Ok(AnnotationTextAlign::Right) => Align::Right,
            _ => Align::Left,
        };
    }
    let object = annot.object();
    Ok(Annotation {
        id: annotation_id(annot),
        kind,
        rect,
        style,
        contents: annot
            .contents()
            .ok()
            .flatten()
            .unwrap_or_default()
            .to_owned(),
        subject: string_entry(&object, "Subj"),
        author: annot
            .author()
            .ok()
            .flatten()
            .filter(|author| !author.trim().is_empty())
            .map(str::to_owned),
    })
}

fn find(page: &PdfPage, id: &str) -> Result<PdfAnnotation> {
    page.annotations()
        .find(|annot| annotation_id(annot) == id)
        .ok_or_else(|| Error::Engine(format!("no annotation {id}")))
}

pub(crate) fn add_annotation(
    document: &mut PdfDocument,
    page: &mut PdfPage,
    annotation: &Annotation,
    content: Option<&StampContent>,
) -> Result<()> {
    let space = Space::of(page)?;
    let rect = space.to_mupdf_rect(annotation.rect);
    let points = |points: &[Point]| -> Vec<mupdf::Point> {
        points.iter().map(|point| space.to_mupdf(*point)).collect()
    };
    let mut annot = match &annotation.kind {
        Kind::Markup { style, quads } => {
            let quads: Vec<mupdf::Quad> = quads
                .iter()
                .map(|quad| space.to_mupdf_quad(*quad))
                .collect();
            match style {
                TextMarkup::Highlight => page.add_highlight_annotation(quads),
                TextMarkup::Underline => page.add_underline_annotation(quads),
                TextMarkup::StrikeOut => page.add_strikeout_annotation(quads),
                TextMarkup::Squiggly => page.add_squiggly_annotation(quads),
            }
        }
        Kind::Ink(strokes) => page.add_ink_annotation(strokes.iter().map(|stroke| points(stroke))),
        Kind::Square => page.add_square_annotation(rect),
        Kind::Circle => page.add_circle_annotation(rect),
        Kind::Line { start, end, .. } => {
            page.add_line_annotation(space.to_mupdf(*start), space.to_mupdf(*end))
        }
        Kind::Polygon(vertices) => page.add_polygon_annotation(points(vertices)),
        Kind::PolyLine(vertices) => page.add_polyline_annotation(points(vertices)),
        Kind::FreeText => page.add_free_text_annotation(rect, &annotation.contents),
        Kind::Note => page.add_text_annotation(rect, &annotation.contents),
        Kind::Stamp => page.add_stamp_annotation(rect, "Draft"),
        Kind::Redact => page.add_redact_annotation(rect),
        Kind::Other(name) => {
            return Err(Error::Engine(format!("cannot create {name} annotations")));
        }
    }
    .map_err(engine_error)?;
    let mut object = annot.object();
    object
        .dict_put(
            "NM",
            PdfObject::new_string(&annotation.id).map_err(engine_error)?,
        )
        .map_err(engine_error)?;
    if let Kind::Note = annotation.kind {
        annot.set_icon_name("Comment").map_err(engine_error)?;
    }
    write_properties(page, &mut annot, annotation, space)?;
    annot.update().map_err(engine_error)?;
    // Last, so MuPDF's own update does not replace it with a standard stamp.
    if let Some(content) = content {
        set_stamp_appearance(document, page, &mut annot, annotation.rect, content)?;
    }
    Ok(())
}

pub(crate) fn update_annotation(
    document: &mut PdfDocument,
    page: &mut PdfPage,
    annotation: &Annotation,
    content: Option<&StampContent>,
) -> Result<()> {
    let space = Space::of(page)?;
    let mut annot = find(page, &annotation.id)?;
    let points = |points: &[Point]| -> Vec<mupdf::Point> {
        points.iter().map(|point| space.to_mupdf(*point)).collect()
    };
    match &annotation.kind {
        Kind::Markup { quads, .. } => annot
            .set_quad_points(
                quads
                    .iter()
                    .map(|quad| space.to_mupdf_quad(*quad))
                    .collect::<Vec<_>>(),
            )
            .map_err(engine_error)?,
        Kind::Ink(strokes) => annot
            .set_ink_list(strokes.iter().map(|stroke| points(stroke)))
            .map_err(engine_error)?,
        Kind::Line { start, end, .. } => annot
            .set_line(space.to_mupdf(*start), space.to_mupdf(*end))
            .map_err(engine_error)?,
        Kind::Polygon(vertices) | Kind::PolyLine(vertices) => {
            annot.set_vertices(points(vertices)).map_err(engine_error)?
        }
        // A stamp keeps its own appearance, which setting the rect through
        // MuPDF would replace; the appearance scales to /Rect by itself.
        Kind::Stamp => write_rect(page, &annot, annotation.rect, space)?,
        Kind::Square | Kind::Circle | Kind::FreeText | Kind::Note | Kind::Redact => annot
            .set_rect(space.to_mupdf_rect(annotation.rect))
            .map_err(engine_error)?,
        Kind::Other(_) => {}
    }
    if annotation.kind != Kind::Stamp {
        write_properties(page, &mut annot, annotation, space)?;
    } else {
        // Written directly: MuPDF's setters would mark the stamp for a new,
        // standard appearance.
        let mut object = annot.object();
        object
            .dict_put(
                "Contents",
                PdfObject::new_string(&annotation.contents).map_err(engine_error)?,
            )
            .map_err(engine_error)?;
        match &annotation.subject {
            Some(subject) => object
                .dict_put(
                    "Subj",
                    PdfObject::new_string(subject).map_err(engine_error)?,
                )
                .map_err(engine_error)?,
            None => object.dict_delete("Subj").map_err(engine_error)?,
        }
    }
    annot.update().map_err(engine_error)?;
    if let Some(content) = content {
        set_stamp_appearance(document, page, &mut annot, annotation.rect, content)?;
    }
    Ok(())
}

fn write_text_properties(annot: &mut PdfAnnotation, annotation: &Annotation) -> Result<()> {
    let mut object = annot.object();
    annot
        .set_contents(&annotation.contents)
        .map_err(engine_error)?;
    match &annotation.subject {
        Some(subject) => object
            .dict_put(
                "Subj",
                PdfObject::new_string(subject).map_err(engine_error)?,
            )
            .map_err(engine_error)?,
        None => object.dict_delete("Subj").map_err(engine_error)?,
    }
    if let Some(author) = &annotation.author {
        annot.set_author(author).map_err(engine_error)?;
    }
    Ok(())
}

fn write_properties(
    page: &PdfPage,
    annot: &mut PdfAnnotation,
    annotation: &Annotation,
    space: Space,
) -> Result<()> {
    let _ = (page, space);
    write_text_properties(annot, annotation)?;
    let style = &annotation.style;
    let mut object = annot.object();
    match style.color {
        Some(color) => annot
            .set_color(annotation_color(color))
            .map_err(engine_error)?,
        None => object.dict_delete("C").map_err(engine_error)?,
    }
    let fills = matches!(
        annotation.kind,
        Kind::Square | Kind::Circle | Kind::Polygon(_) | Kind::Line { .. }
    );
    if fills {
        match style.fill {
            Some(fill) => annot
                .set_interior_color(annotation_color(fill))
                .map_err(engine_error)?,
            None => object.dict_delete("IC").map_err(engine_error)?,
        }
    }
    let bordered = !matches!(
        annotation.kind,
        Kind::Markup { .. } | Kind::Note | Kind::Stamp | Kind::Redact
    );
    if bordered {
        // Text boxes have a border only when given a border color.
        let width = if annotation.kind == Kind::FreeText && style.color.is_none() {
            0.0
        } else {
            style.line_width
        };
        annot.set_border_width(width).map_err(engine_error)?;
        if style.dashed {
            annot
                .set_border_style(AnnotationBorderStyle::Dashed)
                .map_err(engine_error)?;
            let dash = (style.line_width * 3.0).max(3.0);
            annot
                .set_border_dash_pattern(&[dash, dash])
                .map_err(engine_error)?;
        } else {
            annot
                .set_border_style(AnnotationBorderStyle::Solid)
                .map_err(engine_error)?;
            annot.clear_border_dash_pattern().map_err(engine_error)?;
        }
    }
    annot.set_opacity(style.opacity).map_err(engine_error)?;
    if let Kind::Line { endings, .. } = annotation.kind {
        annot
            .set_line_ending_styles(line_ending_style(endings.0), line_ending_style(endings.1))
            .map_err(engine_error)?;
    }
    if annotation.kind == Kind::FreeText {
        annot
            .set_default_appearance(
                style.font.resource_name(),
                style.font_size,
                Some(annotation_color(style.text_color)),
            )
            .map_err(engine_error)?;
        annot
            .set_quadding(match style.align {
                Align::Left => AnnotationTextAlign::Left,
                Align::Center => AnnotationTextAlign::Center,
                Align::Right => AnnotationTextAlign::Right,
            })
            .map_err(engine_error)?;
    }
    Ok(())
}

/// PDF user space for a rect in prev's page space.
fn pdf_rect(page: &PdfPage, rect: Rect, space: Space) -> Result<mupdf::Rect> {
    let inverse = page
        .ctm()
        .map_err(engine_error)?
        .invert()
        .ok_or_else(|| Error::Engine("page transform cannot be inverted".into()))?;
    Ok(space.to_mupdf_rect(rect).transform(&inverse))
}

fn number_array(document: &PdfDocument, values: &[f32]) -> Result<PdfObject> {
    let mut array = document.new_array().map_err(engine_error)?;
    for value in values {
        array
            .array_push(document.new_real(*value).map_err(engine_error)?)
            .map_err(engine_error)?;
    }
    Ok(array)
}

fn write_rect(page: &PdfPage, annot: &PdfAnnotation, rect: Rect, space: Space) -> Result<()> {
    let rect = pdf_rect(page, rect, space)?;
    let document = annot
        .object()
        .document()
        .ok_or_else(|| Error::Engine("annotation without document".into()))?;
    let array = number_array(&document, &[rect.x0, rect.y0, rect.x1, rect.y1])?;
    annot.object().dict_put("Rect", array).map_err(engine_error)
}

/// A path around the ellipse filling (0, 0)-(width, height), in PDF
/// content stream operators.
fn ellipse_path(x: f32, y: f32, width: f32, height: f32) -> String {
    // Four cubic Béziers; 0.5523 puts the control points on a circle.
    const KAPPA: f32 = 0.552_284_8;
    let (rx, ry) = (width / 2.0, height / 2.0);
    let (cx, cy) = (x + rx, y + ry);
    let (kx, ky) = (rx * KAPPA, ry * KAPPA);
    format!(
        "{} {} m {} {} {} {} {} {} c {} {} {} {} {} {} c {} {} {} {} {} {} c {} {} {} {} {} {} c h ",
        cx + rx,
        cy,
        cx + rx,
        cy + ky,
        cx + kx,
        cy + ry,
        cx,
        cy + ry,
        cx - kx,
        cy + ry,
        cx - rx,
        cy + ky,
        cx - rx,
        cy,
        cx - rx,
        cy - ky,
        cx - kx,
        cy - ry,
        cx,
        cy - ry,
        cx + kx,
        cy - ry,
        cx + rx,
        cy - ky,
        cx + rx,
        cy,
    )
}

fn set_stamp_appearance(
    document: &mut PdfDocument,
    page: &PdfPage,
    annot: &mut PdfAnnotation,
    rect: Rect,
    content: &StampContent,
) -> Result<()> {
    let space = Space::of(page)?;
    let (width, height) = (rect.width().max(1.0), rect.height().max(1.0));
    let mut resources = document.new_dict().map_err(engine_error)?;
    let stream = match content {
        StampContent::Image {
            image,
            round,
            border,
        } => {
            let pixmap = rgba_pixmap(image)?;
            let image = Image::from_pixmap(&pixmap).map_err(engine_error)?;
            let image = document.add_image(&image).map_err(engine_error)?;
            let mut xobjects = document.new_dict().map_err(engine_error)?;
            xobjects.dict_put("Im", image).map_err(engine_error)?;
            resources
                .dict_put("XObject", xobjects)
                .map_err(engine_error)?;
            let mut stream = String::from("q ");
            if *round {
                stream.push_str(&ellipse_path(0.0, 0.0, width, height));
                stream.push_str("W n ");
            }
            stream.push_str(&format!("{width} 0 0 {height} 0 0 cm /Im Do Q "));
            if let Some((color, line_width)) = border {
                let inset = line_width / 2.0;
                let path = if *round {
                    ellipse_path(inset, inset, width - line_width, height - line_width)
                } else {
                    format!(
                        "{inset} {inset} {} {} re ",
                        width - line_width,
                        height - line_width
                    )
                };
                stream.push_str(&format!(
                    "q {} {} {} RG {line_width} w {path}S Q",
                    color.red, color.green, color.blue
                ));
            }
            stream
        }
        StampContent::Mask {
            hole,
            round,
            opacity,
        } => {
            let mut state = document.new_dict().map_err(engine_error)?;
            state
                .dict_put("ca", document.new_real(*opacity).map_err(engine_error)?)
                .map_err(engine_error)?;
            let mut states = document.new_dict().map_err(engine_error)?;
            states.dict_put("Mask", state).map_err(engine_error)?;
            resources
                .dict_put("ExtGState", states)
                .map_err(engine_error)?;
            // Appearance space runs bottom-up, so flip the hole.
            let hole_x = hole.x0 - rect.x0;
            let hole_y = height - (hole.y1 - rect.y0);
            let hole_path = if *round {
                ellipse_path(hole_x, hole_y, hole.width(), hole.height())
            } else {
                format!("{hole_x} {hole_y} {} {} re ", hole.width(), hole.height())
            };
            format!("q /Mask gs 0 g 0 0 {width} {height} re {hole_path}f* Q")
        }
    };
    let mut form = document.new_dict().map_err(engine_error)?;
    form.dict_put("Type", document.new_name("XObject").map_err(engine_error)?)
        .map_err(engine_error)?;
    form.dict_put("Subtype", document.new_name("Form").map_err(engine_error)?)
        .map_err(engine_error)?;
    form.dict_put("BBox", number_array(document, &[0.0, 0.0, width, height])?)
        .map_err(engine_error)?;
    form.dict_put("Resources", resources)
        .map_err(engine_error)?;
    let appearance = document
        .add_stream(
            &Buffer::from_bytes(stream.as_bytes()).map_err(engine_error)?,
            Some(&form),
            false,
        )
        .map_err(engine_error)?;
    let mut ap = document.new_dict().map_err(engine_error)?;
    ap.dict_put("N", appearance).map_err(engine_error)?;
    let mut object = annot.object();
    object.dict_put("AP", ap).map_err(engine_error)?;
    write_rect(page, annot, rect, space)
}

pub(crate) fn rgba_pixmap(bitmap: &crate::engine::Bitmap) -> Result<Pixmap> {
    let mut pixmap = Pixmap::new_with_w_h(
        &Colorspace::device_rgb(),
        bitmap.width as i32,
        bitmap.height as i32,
        true,
    )
    .map_err(engine_error)?;
    let stride = pixmap.stride() as usize;
    let row_bytes = bitmap.width as usize * 4;
    let samples = pixmap.samples_mut();
    for row in 0..bitmap.height as usize {
        let source = &bitmap.pixels[row * row_bytes..(row + 1) * row_bytes];
        let target = &mut samples[row * stride..row * stride + row_bytes];
        // MuPDF pixmaps with alpha are premultiplied.
        for (from, to) in source
            .as_chunks::<4>()
            .0
            .iter()
            .zip(target.as_chunks_mut::<4>().0.iter_mut())
        {
            let alpha = u16::from(from[3]);
            let premultiply = |channel: u8| ((u16::from(channel) * alpha + 127) / 255) as u8;
            to.copy_from_slice(&[
                premultiply(from[0]),
                premultiply(from[1]),
                premultiply(from[2]),
                from[3],
            ]);
        }
    }
    Ok(pixmap)
}

pub(crate) fn remove_annotation(page: &mut PdfPage, id: &str) -> Result<Removed> {
    let annot = find(page, id)?;
    let object = annot.xref().map_err(engine_error)?;
    page.delete_annotation(annot).map_err(engine_error)?;
    Ok(Removed { object })
}

pub(crate) fn restore_annotation(
    document: &PdfDocument,
    page: &mut PdfPage,
    removed: &Removed,
) -> Result<()> {
    let mut page_object = page.object();
    let mut annots = match page_object.get_dict("Annots").map_err(engine_error)? {
        Some(annots) => annots,
        None => {
            let annots = document.new_array().map_err(engine_error)?;
            page_object
                .dict_put("Annots", annots)
                .map_err(engine_error)?;
            page_object
                .get_dict("Annots")
                .map_err(engine_error)?
                .ok_or_else(|| Error::Engine("could not add /Annots".into()))?
        }
    };
    annots
        .array_push(
            document
                .new_indirect(removed.object, 0)
                .map_err(engine_error)?,
        )
        .map_err(engine_error)
}

// Form fields.

fn on_state(widget_object: &PdfObject) -> Option<String> {
    let normal = widget_object.get_dict("AP").ok()??.get_dict("N").ok()??;
    let count = normal.dict_len().ok()?;
    (0..count as i32).find_map(|index| {
        let key = normal.get_dict_key(index).ok()??;
        let name = String::from_utf8(key.as_name().ok()?).ok()?;
        (name != "Off").then_some(name)
    })
}

fn choice_options(widget_object: &PdfObject) -> Vec<String> {
    let Some(options) = widget_object.get_dict_inheritable("Opt").ok().flatten() else {
        return Vec::new();
    };
    let count = options.len().unwrap_or(0);
    (0..count as i32)
        .filter_map(|index| {
            let option = options.get_array(index).ok()??;
            if option.is_array().ok()? {
                // [export value, shown text]: the export value is what is set.
                option.get_array(0).ok()??.as_string().ok()
            } else {
                option.as_string().ok()
            }
        })
        .collect()
}

fn font_size(widget_object: &PdfObject) -> f32 {
    widget_object
        .get_dict_inheritable("DA")
        .ok()
        .flatten()
        .and_then(|da| da.as_string().ok())
        .and_then(|da| {
            let words: Vec<&str> = da.split_whitespace().collect();
            let position = words.iter().position(|word| *word == "Tf")?;
            words.get(position.checked_sub(1)?)?.parse().ok()
        })
        .unwrap_or(0.0)
}

pub(crate) fn read_fields(page: &PdfPage) -> Result<Vec<Field>> {
    let space = Space::of(page)?;
    let mut fields = Vec::new();
    for widget in page.widgets() {
        let Ok(widget_type) = widget.r#type() else {
            continue;
        };
        let flags = widget
            .field_flags()
            .unwrap_or(mupdf::pdf::FieldFlags::empty());
        let object = widget.annotation().object();
        let kind = match widget_type {
            WidgetType::Text => FieldKind::Text {
                multiline: flags.contains(mupdf::pdf::FieldFlags::MULTILINE),
                password: flags.contains(mupdf::pdf::FieldFlags::PASSWORD),
            },
            WidgetType::Checkbox => FieldKind::Checkbox,
            WidgetType::RadioButton => FieldKind::Radio,
            WidgetType::Combobox | WidgetType::Listbox => FieldKind::Choice {
                options: choice_options(&object),
                combo: widget_type == WidgetType::Combobox,
            },
            WidgetType::Signature => FieldKind::Signature,
            WidgetType::Button | WidgetType::Unknown => FieldKind::Button,
        };
        let on_value = matches!(kind, FieldKind::Checkbox | FieldKind::Radio)
            .then(|| on_state(&object))
            .flatten();
        fields.push(Field {
            id: widget.xref().map_err(engine_error)?,
            name: widget.name().ok().flatten().unwrap_or_default(),
            kind,
            rect: space.rect(widget.annotation().rect().map_err(engine_error)?),
            value: widget.value().ok().flatten().unwrap_or_default(),
            on_value,
            read_only: flags.contains(mupdf::pdf::FieldFlags::READ_ONLY),
            font_size: font_size(&object),
        });
    }
    Ok(fields)
}

pub(crate) fn set_field(
    document: &mut PdfDocument,
    page: &PdfPage,
    id: i32,
    value: &str,
) -> Result<()> {
    let mut widget = page
        .load_widget(id)
        .map_err(engine_error)?
        .ok_or_else(|| Error::Engine(format!("no form field {id}")))?;
    let kind = widget.r#type().map_err(engine_error)?;
    if matches!(kind, WidgetType::Checkbox | WidgetType::RadioButton) {
        return set_button(document, &widget.annotation().object(), value);
    }
    widget
        .set_value(document, value, true)
        .map_err(engine_error)?;
    widget.update().map_err(engine_error)?;
    Ok(())
}

/// Turns a checkbox or radio button on (`value` is its on state) or off
/// ("Off"). MuPDF's setter leaves the other buttons of a radio group on and
/// stores the value as a string, so the states are written here: the
/// field's `/V` as a name, and each button's `/AS`.
fn set_button(document: &PdfDocument, widget: &PdfObject, value: &str) -> Result<()> {
    let name = |value: &str| document.new_name(value).map_err(engine_error);
    // The field is the widget itself, or its parent for grouped buttons.
    let own = || widget.try_clone().map_err(engine_error);
    let mut field = match widget.get_dict("Parent").map_err(engine_error)? {
        Some(parent) => parent,
        None => own()?,
    };
    field.dict_put("V", name(value)?).map_err(engine_error)?;
    let buttons: Vec<PdfObject> = match field.get_dict("Kids").map_err(engine_error)? {
        Some(kids) => {
            let count = kids.len().map_err(engine_error)?;
            (0..count as i32)
                .filter_map(|index| kids.get_array(index).ok().flatten())
                .collect()
        }
        None => vec![own()?],
    };
    for mut button in buttons {
        let state = match on_state(&button) {
            Some(on) if on == value => on,
            _ => "Off".to_owned(),
        };
        button.dict_put("AS", name(&state)?).map_err(engine_error)?;
    }
    Ok(())
}
