//! Non-destructive edits: a list of operations applied in order to the
//! decoded image, with undo and redo.

use image::imageops::{self, FilterType};
use image::{Rgba, RgbaImage};

use crate::decode::Frame;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CropRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// Preview's Adjust Color controls. The default changes nothing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorAdjust {
    /// Stops of exposure, -2 to 2.
    pub exposure: f32,
    /// -1 to 1.
    pub contrast: f32,
    /// Multiplier, 0 (gray) to 2.
    pub saturation: f32,
    /// Cool (-1) to warm (1).
    pub temperature: f32,
    /// Green (-1) to magenta (1).
    pub tint: f32,
    /// 0 to 1.
    pub sepia: f32,
    /// Unsharp masking, 0 to 1.
    pub sharpness: f32,
    /// Input levels: black and white points (0 to 1) and midtone gamma.
    pub black: f32,
    pub white: f32,
    pub gamma: f32,
}

impl Default for ColorAdjust {
    fn default() -> Self {
        Self {
            exposure: 0.0,
            contrast: 0.0,
            saturation: 1.0,
            temperature: 0.0,
            tint: 0.0,
            sepia: 0.0,
            sharpness: 0.0,
            black: 0.0,
            white: 1.0,
            gamma: 1.0,
        }
    }
}

impl ColorAdjust {
    pub fn is_identity(&self) -> bool {
        *self == Self::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    /// Clockwise quarter turns, 1 to 3.
    Rotate(u8),
    FlipHorizontal,
    FlipVertical,
    Crop(CropRect),
    Resize {
        width: u32,
        height: u32,
    },
    Color(ColorAdjust),
}

impl Operation {
    /// The image size after this operation.
    pub fn output_size(&self, (width, height): (u32, u32)) -> (u32, u32) {
        match self {
            Self::Rotate(turns) if turns % 2 == 1 => (height, width),
            Self::Crop(rect) => (rect.width, rect.height),
            Self::Resize { width, height } => (*width, *height),
            _ => (width, height),
        }
    }

    pub fn apply(&self, image: RgbaImage) -> RgbaImage {
        match self {
            Self::Rotate(turns) => match turns % 4 {
                1 => imageops::rotate90(&image),
                2 => imageops::rotate180(&image),
                3 => imageops::rotate270(&image),
                _ => image,
            },
            Self::FlipHorizontal => imageops::flip_horizontal(&image),
            Self::FlipVertical => imageops::flip_vertical(&image),
            Self::Crop(rect) => {
                let x = rect.x.min(image.width().saturating_sub(1));
                let y = rect.y.min(image.height().saturating_sub(1));
                let width = rect.width.clamp(1, image.width() - x);
                let height = rect.height.clamp(1, image.height() - y);
                imageops::crop_imm(&image, x, y, width, height).to_image()
            }
            Self::Resize { width, height } => imageops::resize(
                &image,
                (*width).max(1),
                (*height).max(1),
                FilterType::Lanczos3,
            ),
            Self::Color(adjust) => adjust_color(image, adjust),
        }
    }
}

fn srgb_to_linear(value: f32) -> f32 {
    if value <= 0.04045 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

fn linear_to_srgb(value: f32) -> f32 {
    if value <= 0.003_130_8 {
        value * 12.92
    } else {
        1.055 * value.powf(1.0 / 2.4) - 0.055
    }
}

/// Per channel tone curve (levels, exposure, contrast) as a lookup table.
fn tone_table(adjust: &ColorAdjust) -> [f32; 256] {
    let exposure = 2f32.powf(adjust.exposure);
    let range = (adjust.white - adjust.black).max(1.0 / 255.0);
    std::array::from_fn(|index| {
        let mut value = index as f32 / 255.0;
        value = ((value - adjust.black) / range)
            .clamp(0.0, 1.0)
            .powf(1.0 / adjust.gamma.max(0.01));
        value = linear_to_srgb((srgb_to_linear(value) * exposure).min(1.0));
        ((value - 0.5) * (1.0 + adjust.contrast) + 0.5).clamp(0.0, 1.0)
    })
}

fn adjust_color(mut image: RgbaImage, adjust: &ColorAdjust) -> RgbaImage {
    if adjust.is_identity() {
        return image;
    }
    let table = tone_table(adjust);
    let warm = adjust.temperature * 0.15;
    let magenta = adjust.tint * 0.15;
    let gains = [
        1.0 + warm + magenta * 0.5,
        1.0 - magenta,
        1.0 - warm + magenta * 0.5,
    ];
    for Rgba([red, green, blue, _]) in image.pixels_mut() {
        let mut rgb = [
            table[*red as usize],
            table[*green as usize],
            table[*blue as usize],
        ];
        for (channel, gain) in rgb.iter_mut().zip(gains) {
            *channel *= gain;
        }
        let luma = 0.2126 * rgb[0] + 0.7152 * rgb[1] + 0.0722 * rgb[2];
        for channel in &mut rgb {
            *channel = luma + (*channel - luma) * adjust.saturation;
        }
        if adjust.sepia > 0.0 {
            let [r, g, b] = rgb;
            let toned = [
                0.393 * r + 0.769 * g + 0.189 * b,
                0.349 * r + 0.686 * g + 0.168 * b,
                0.272 * r + 0.534 * g + 0.131 * b,
            ];
            for (channel, toned) in rgb.iter_mut().zip(toned) {
                *channel += (toned - *channel) * adjust.sepia;
            }
        }
        let [r, g, b] = rgb.map(|channel| (channel.clamp(0.0, 1.0) * 255.0 + 0.5) as u8);
        (*red, *green, *blue) = (r, g, b);
    }
    if adjust.sharpness > 0.0 {
        image = imageops::unsharpen(&image, 1.0 + adjust.sharpness * 2.0, 2);
    }
    image
}

/// Edits applied to one image, with undo and redo.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EditStack {
    operations: Vec<Operation>,
    /// Operations beyond this index were undone.
    applied: usize,
}

impl EditStack {
    pub fn operations(&self) -> &[Operation] {
        &self.operations[..self.applied]
    }

    pub fn is_empty(&self) -> bool {
        self.applied == 0
    }

    pub fn can_undo(&self) -> bool {
        self.applied > 0
    }

    pub fn can_redo(&self) -> bool {
        self.applied < self.operations.len()
    }

    pub fn push(&mut self, operation: Operation) {
        self.operations.truncate(self.applied);
        self.operations.push(operation);
        self.applied += 1;
    }

    /// Replaces a trailing color adjustment instead of stacking one per
    /// slider step, so a whole Adjust Color session undoes in one step.
    pub fn set_color(&mut self, adjust: ColorAdjust) {
        if matches!(self.operations().last(), Some(Operation::Color(_))) {
            self.operations.truncate(self.applied);
            self.operations[self.applied - 1] = Operation::Color(adjust);
        } else {
            self.push(Operation::Color(adjust));
        }
    }

    /// The color adjustment in effect at the end, if the last edit is one.
    pub fn current_color(&self) -> ColorAdjust {
        match self.operations().last() {
            Some(Operation::Color(adjust)) => *adjust,
            _ => ColorAdjust::default(),
        }
    }

    pub fn undo(&mut self) -> bool {
        let undone = self.can_undo();
        self.applied -= usize::from(undone);
        undone
    }

    pub fn redo(&mut self) -> bool {
        let redone = self.can_redo();
        self.applied += usize::from(redone);
        redone
    }

    pub fn output_size(&self, size: (u32, u32)) -> (u32, u32) {
        self.operations()
            .iter()
            .fold(size, |size, operation| operation.output_size(size))
    }

    pub fn apply(&self, frame: &Frame) -> Frame {
        let image = RgbaImage::from_raw(frame.width, frame.height, frame.pixels.clone())
            .expect("frame pixels match its size");
        let image = self
            .operations()
            .iter()
            .fold(image, |image, operation| operation.apply(image));
        Frame {
            width: image.width(),
            height: image.height(),
            pixels: image.into_raw(),
            delay: frame.delay,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    /// A 3x2 image whose pixels encode their position in red and green.
    fn sample() -> Frame {
        let image = RgbaImage::from_fn(3, 2, |x, y| Rgba([x as u8 * 10, y as u8 * 10, 99, 255]));
        Frame {
            width: 3,
            height: 2,
            pixels: image.into_raw(),
            delay: Duration::ZERO,
        }
    }

    fn pixel(frame: &Frame, x: u32, y: u32) -> [u8; 4] {
        let offset = ((y * frame.width + x) * 4) as usize;
        frame.pixels[offset..offset + 4].try_into().unwrap()
    }

    fn apply(operations: &[Operation]) -> Frame {
        let mut stack = EditStack::default();
        for operation in operations {
            stack.push(*operation);
        }
        stack.apply(&sample())
    }

    #[test]
    fn rotation_and_flips_move_pixels() {
        let rotated = apply(&[Operation::Rotate(1)]);
        assert_eq!((rotated.width, rotated.height), (2, 3));
        // The bottom-left pixel ends up top-left after a clockwise turn.
        assert_eq!(pixel(&rotated, 0, 0), [0, 10, 99, 255]);
        let flipped = apply(&[Operation::FlipHorizontal]);
        assert_eq!(pixel(&flipped, 0, 0), [20, 0, 99, 255]);
        let flipped = apply(&[Operation::FlipVertical]);
        assert_eq!(pixel(&flipped, 0, 0), [0, 10, 99, 255]);
        let round_trip = apply(&[Operation::Rotate(1), Operation::Rotate(3)]);
        assert_eq!(round_trip, sample());
    }

    #[test]
    fn crop_and_resize_change_size() {
        let cropped = apply(&[Operation::Crop(CropRect {
            x: 1,
            y: 1,
            width: 2,
            height: 1,
        })]);
        assert_eq!((cropped.width, cropped.height), (2, 1));
        assert_eq!(pixel(&cropped, 0, 0), [10, 10, 99, 255]);
        let clamped = apply(&[Operation::Crop(CropRect {
            x: 2,
            y: 0,
            width: 50,
            height: 50,
        })]);
        assert_eq!((clamped.width, clamped.height), (1, 2));
        let resized = apply(&[Operation::Resize {
            width: 6,
            height: 4,
        }]);
        assert_eq!((resized.width, resized.height), (6, 4));
    }

    #[test]
    fn output_size_follows_the_operations() {
        let mut stack = EditStack::default();
        stack.push(Operation::Rotate(1));
        stack.push(Operation::Crop(CropRect {
            x: 0,
            y: 0,
            width: 10,
            height: 20,
        }));
        stack.push(Operation::Resize {
            width: 5,
            height: 10,
        });
        assert_eq!(stack.output_size((100, 50)), (5, 10));
        stack.undo();
        assert_eq!(stack.output_size((100, 50)), (10, 20));
    }

    #[test]
    fn default_color_changes_nothing() {
        assert_eq!(apply(&[Operation::Color(ColorAdjust::default())]), sample());
    }

    #[test]
    fn color_controls_move_in_the_right_direction() {
        let gray = |value: u8| Frame {
            width: 1,
            height: 1,
            pixels: vec![value, value, value, 255],
            delay: Duration::ZERO,
        };
        let run = |adjust: ColorAdjust, frame: &Frame| {
            let mut stack = EditStack::default();
            stack.push(Operation::Color(adjust));
            stack.apply(frame)
        };
        let brighter = run(
            ColorAdjust {
                exposure: 1.0,
                ..Default::default()
            },
            &gray(100),
        );
        assert!(brighter.pixels[0] > 120, "{:?}", brighter.pixels);
        let contrast = run(
            ColorAdjust {
                contrast: 0.5,
                ..Default::default()
            },
            &gray(60),
        );
        assert!(contrast.pixels[0] < 60);
        let levels = run(
            ColorAdjust {
                black: 0.2,
                white: 0.8,
                ..Default::default()
            },
            &gray(204),
        );
        assert_eq!(levels.pixels[0], 255, "the white point maps to white");
        let red = Frame {
            width: 1,
            height: 1,
            pixels: vec![200, 40, 40, 255],
            delay: Duration::ZERO,
        };
        let desaturated = run(
            ColorAdjust {
                saturation: 0.0,
                ..Default::default()
            },
            &red,
        );
        assert_eq!(desaturated.pixels[0], desaturated.pixels[1]);
        assert_eq!(desaturated.pixels[1], desaturated.pixels[2]);
        let warm = run(
            ColorAdjust {
                temperature: 1.0,
                ..Default::default()
            },
            &gray(128),
        );
        assert!(
            warm.pixels[0] > warm.pixels[2],
            "warmer means more red than blue"
        );
        assert_eq!(warm.pixels[3], 255, "alpha is untouched");
    }

    #[test]
    fn undo_redo_and_color_sessions() {
        let mut stack = EditStack::default();
        stack.push(Operation::Rotate(1));
        stack.set_color(ColorAdjust {
            exposure: 0.5,
            ..Default::default()
        });
        stack.set_color(ColorAdjust {
            exposure: 1.0,
            ..Default::default()
        });
        assert_eq!(
            stack.operations().len(),
            2,
            "slider steps merge into one edit"
        );
        assert_eq!(stack.current_color().exposure, 1.0);
        assert!(stack.undo());
        assert_eq!(stack.operations(), &[Operation::Rotate(1)]);
        assert!(stack.redo());
        assert!(!stack.redo());
        stack.undo();
        stack.push(Operation::FlipHorizontal);
        assert!(!stack.can_redo(), "a new edit drops the undone ones");
        assert_eq!(
            stack.operations(),
            &[Operation::Rotate(1), Operation::FlipHorizontal]
        );
    }
}
