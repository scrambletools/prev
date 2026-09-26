//! Placement of an image in its view: zoom, fit and centring, in logical
//! pixels. Zoom is displayed pixels per image pixel.

pub const MIN_ZOOM: f32 = 0.01;
pub const MAX_ZOOM: f32 = 64.0;
pub const ZOOM_STEP: f32 = 1.25;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Fit {
    /// Fit in the view, never enlarging past actual size.
    Fit,
    Zoom(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}

/// Where the image goes inside the scrollable content.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Placement {
    pub zoom: f32,
    /// Size of the scrollable content: the image, or the view if larger.
    pub content: (f32, f32),
    pub image: Rect,
}

pub fn resolve_zoom(fit: Fit, image: (f32, f32), view: (f32, f32)) -> f32 {
    let zoom = match fit {
        Fit::Fit => {
            let fit = (view.0 / image.0.max(1.0)).min(view.1 / image.1.max(1.0));
            fit.min(1.0)
        }
        Fit::Zoom(zoom) => zoom,
    };
    zoom.clamp(MIN_ZOOM, MAX_ZOOM)
}

pub fn place(image: (f32, f32), zoom: f32, view: (f32, f32)) -> Placement {
    let (width, height) = (image.0 * zoom, image.1 * zoom);
    let content = (width.max(view.0), height.max(view.1));
    Placement {
        zoom,
        content,
        image: Rect {
            x: (content.0 - width) / 2.0,
            y: (content.1 - height) / 2.0,
            width,
            height,
        },
    }
}

/// The point of the image under `(x, y)` in content space, as fractions of
/// the image size, for keeping it in place while zooming.
pub fn image_fraction(placement: &Placement, x: f32, y: f32) -> (f32, f32) {
    let image = placement.image;
    (
        ((x - image.x) / image.width.max(1.0)).clamp(0.0, 1.0),
        ((y - image.y) / image.height.max(1.0)).clamp(0.0, 1.0),
    )
}

pub fn fraction_to_content(placement: &Placement, fraction: (f32, f32)) -> (f32, f32) {
    let image = placement.image;
    (
        image.x + fraction.0 * image.width,
        image.y + fraction.1 * image.height,
    )
}

/// The power of two to divide the image by for display: the largest that
/// still has at least as many pixels as the screen shows, so downscaling
/// happens once with a good filter instead of per frame on the GPU.
pub fn downscale_level(image_width: u32, displayed_device_width: f32) -> u32 {
    let mut level = 1;
    while level < 64 && (image_width / (level * 2)) as f32 >= displayed_device_width {
        level *= 2;
    }
    level
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fit_never_enlarges() {
        assert_eq!(resolve_zoom(Fit::Fit, (100.0, 50.0), (800.0, 600.0)), 1.0);
        assert_eq!(resolve_zoom(Fit::Fit, (1600.0, 600.0), (800.0, 600.0)), 0.5);
        assert_eq!(resolve_zoom(Fit::Fit, (800.0, 1200.0), (800.0, 600.0)), 0.5);
        assert_eq!(
            resolve_zoom(Fit::Zoom(1000.0), (10.0, 10.0), (800.0, 600.0)),
            MAX_ZOOM
        );
    }

    #[test]
    fn small_images_are_centred_large_ones_scroll() {
        let small = place((100.0, 50.0), 1.0, (800.0, 600.0));
        assert_eq!(small.content, (800.0, 600.0));
        assert_eq!(
            small.image,
            Rect {
                x: 350.0,
                y: 275.0,
                width: 100.0,
                height: 50.0
            }
        );
        let large = place((1000.0, 500.0), 2.0, (800.0, 600.0));
        assert_eq!(large.content, (2000.0, 1000.0));
        assert_eq!((large.image.x, large.image.y), (0.0, 0.0));
    }

    #[test]
    fn fractions_round_trip() {
        let placement = place((400.0, 200.0), 2.0, (500.0, 500.0));
        let fraction = image_fraction(&placement, 200.0, 250.0);
        assert_eq!(fraction, (0.25, 0.5));
        let (x, y) = fraction_to_content(&placement, fraction);
        assert!((x - 200.0).abs() < 0.01 && (y - 250.0).abs() < 0.01);
    }

    #[test]
    fn downscale_levels() {
        assert_eq!(downscale_level(6000, 6000.0), 1);
        assert_eq!(downscale_level(6000, 2000.0), 2);
        assert_eq!(downscale_level(6000, 1400.0), 4);
        assert_eq!(downscale_level(6000, 1.0), 64);
    }
}
