//! Tone curves for camera RAW: a curve that makes the developed RAW look
//! like the JPEG the camera made of it, found by matching their
//! brightness histograms, as RawTherapee's auto-matched tone curve does.
//! Curves work on sRGB-encoded values, 0 to 1, and apply to each channel,
//! as camera tone curves do.

/// Histogram resolution, and the number of points in a curve's table.
const BINS: usize = 4096;
/// Points the matched curve is smoothed through, between black and white.
const KNOTS: usize = 32;

/// A monotone curve from 0..=1 to 0..=1, as a lookup table.
#[derive(Debug, Clone, PartialEq)]
pub struct ToneCurve {
    table: Vec<f32>,
}

pub fn srgb_encode(linear: f32) -> f32 {
    let linear = linear.clamp(0.0, 1.0);
    if linear <= 0.003_130_8 {
        linear * 12.92
    } else {
        1.055 * linear.powf(1.0 / 2.4) - 0.055
    }
}

pub fn srgb_decode(encoded: f32) -> f32 {
    let encoded = encoded.clamp(0.0, 1.0);
    if encoded <= 0.040_45 {
        encoded / 12.92
    } else {
        ((encoded + 0.055) / 1.055).powf(2.4)
    }
}

/// Relative luminance of linear sRGB.
fn luminance(rgb: [f32; 3]) -> f32 {
    0.2126 * rgb[0] + 0.7152 * rgb[1] + 0.0722 * rgb[2]
}

/// A normalized cumulative histogram of sRGB-encoded brightness values.
fn cdf(values: impl Iterator<Item = f32>) -> Option<Vec<f64>> {
    let mut histogram = vec![0u64; BINS];
    let mut count = 0u64;
    for value in values {
        let bin = ((value.clamp(0.0, 1.0) * (BINS - 1) as f32).round() as usize).min(BINS - 1);
        histogram[bin] += 1;
        count += 1;
    }
    if count == 0 {
        return None;
    }
    let mut total = 0u64;
    Some(
        histogram
            .into_iter()
            .map(|bin| {
                total += bin;
                total as f64 / count as f64
            })
            .collect(),
    )
}

/// The brightness where `cdf` first reaches `fraction`, 0 to 1.
fn quantile(cdf: &[f64], fraction: f64) -> f32 {
    let index = cdf.partition_point(|value| *value < fraction).min(BINS - 1);
    // Between this bin and the one before, in proportion.
    let (before, at) = if index == 0 {
        (0.0, cdf[0])
    } else {
        (cdf[index - 1], cdf[index])
    };
    let within = if at > before {
        ((fraction - before) / (at - before)).clamp(0.0, 1.0)
    } else {
        0.0
    };
    ((index as f64 - 1.0 + within).max(0.0) / (BINS - 1) as f64) as f32
}

impl ToneCurve {
    pub fn identity() -> Self {
        Self::through(&[(0.0, 0.0), (1.0, 1.0)])
    }

    /// A gentle contrast curve for RAW files that carry no camera JPEG to
    /// match: it lifts the midtones a little and rolls off the ends, like a
    /// camera's standard picture style.
    pub fn standard() -> Self {
        Self::through(&[
            (0.0, 0.0),
            (0.05, 0.035),
            (0.25, 0.26),
            (0.5, 0.58),
            (0.75, 0.84),
            (0.95, 0.975),
            (1.0, 1.0),
        ])
    }

    /// The curve that makes the brightness of `raw` (linear RGB) match that
    /// of `jpeg` (sRGB-encoded RGB, 8 bits). `None` when either is empty or
    /// the match comes out unusable.
    pub fn matched(raw: &[[f32; 3]], jpeg: &[[u8; 3]]) -> Option<Self> {
        let raw_cdf = cdf(raw.iter().map(|rgb| srgb_encode(luminance(*rgb))))?;
        let jpeg_cdf = cdf(jpeg.iter().map(|rgb| {
            let linear = rgb.map(|channel| srgb_decode(f32::from(channel) / 255.0));
            srgb_encode(luminance(linear))
        }))?;
        // Equal fractions of both histograms map onto each other. The ends
        // are few and noisy pixels, so the curve is pinned at black and
        // white and follows quantiles in between.
        let mut points = vec![(0.0, 0.0)];
        for knot in 1..KNOTS {
            let fraction = knot as f64 / KNOTS as f64;
            let from = quantile(&raw_cdf, fraction);
            let to = quantile(&jpeg_cdf, fraction);
            if from > points.last().map_or(0.0, |point| point.0) + 1e-3 {
                points.push((from, to.max(points.last().map_or(0.0, |point| point.1))));
            }
        }
        points.push((1.0, 1.0_f32.max(points.last().map_or(0.0, |point| point.1))));
        if points.len() < 4 {
            return None;
        }
        Some(Self::through(&points))
    }

    /// Like `matched`, for one channel: `raw` linear, `jpeg` sRGB-encoded.
    pub fn matched_channel(
        raw: impl Iterator<Item = f32>,
        jpeg: impl Iterator<Item = u8>,
    ) -> Option<Self> {
        let raw_cdf = cdf(raw.map(srgb_encode))?;
        let jpeg_cdf = cdf(jpeg.map(|value| f32::from(value) / 255.0))?;
        let mut points = vec![(0.0, 0.0)];
        for knot in 1..KNOTS {
            let fraction = knot as f64 / KNOTS as f64;
            let from = quantile(&raw_cdf, fraction);
            let to = quantile(&jpeg_cdf, fraction);
            if from > points.last().map_or(0.0, |point| point.0) + 1e-3 {
                points.push((from, to.max(points.last().map_or(0.0, |point| point.1))));
            }
        }
        points.push((1.0, 1.0_f32.max(points.last().map_or(0.0, |point| point.1))));
        (points.len() >= 4).then(|| Self::through(&points))
    }

    /// Applies the curve to brightness only, keeping hue and saturation:
    /// every channel is scaled by the same amount.
    pub fn apply_luminance(&self, linear: [f32; 3]) -> [f32; 3] {
        let luma = luminance(linear).max(1e-6);
        let target = srgb_decode(self.at(srgb_encode(luma)));
        let scale = target / luma;
        linear.map(|channel| srgb_encode(channel * scale))
    }

    /// A smooth curve through increasing `points`, with slopes chosen so it
    /// never falls (Fritsch and Carlson's monotone cubic).
    fn through(points: &[(f32, f32)]) -> Self {
        let count = points.len();
        let secants: Vec<f32> = points
            .windows(2)
            .map(|pair| (pair[1].1 - pair[0].1) / (pair[1].0 - pair[0].0).max(1e-6))
            .collect();
        let mut slopes = vec![0.0f32; count];
        slopes[0] = secants[0];
        slopes[count - 1] = secants[count - 2];
        for index in 1..count - 1 {
            let (before, after) = (secants[index - 1], secants[index]);
            slopes[index] = if before * after <= 0.0 {
                0.0
            } else {
                (before + after) / 2.0
            };
        }
        for (index, secant) in secants.iter().enumerate() {
            if *secant == 0.0 {
                slopes[index] = 0.0;
                slopes[index + 1] = 0.0;
                continue;
            }
            let (a, b) = (slopes[index] / secant, slopes[index + 1] / secant);
            let length = a.hypot(b);
            if length > 3.0 {
                slopes[index] = 3.0 * a / length * secant;
                slopes[index + 1] = 3.0 * b / length * secant;
            }
        }
        let table = (0..BINS)
            .map(|bin| {
                let x = bin as f32 / (BINS - 1) as f32;
                let segment = points
                    .windows(2)
                    .position(|pair| x <= pair[1].0)
                    .unwrap_or(count - 2);
                let (x0, y0) = points[segment];
                let (x1, y1) = points[segment + 1];
                let width = (x1 - x0).max(1e-6);
                let t = ((x - x0) / width).clamp(0.0, 1.0);
                let (t2, t3) = (t * t, t * t * t);
                let y = (2.0 * t3 - 3.0 * t2 + 1.0) * y0
                    + (t3 - 2.0 * t2 + t) * width * slopes[segment]
                    + (-2.0 * t3 + 3.0 * t2) * y1
                    + (t3 - t2) * width * slopes[segment + 1];
                y.clamp(0.0, 1.0)
            })
            .collect();
        Self { table }
    }

    /// The curve at `x`, 0 to 1, interpolating the table.
    pub fn at(&self, x: f32) -> f32 {
        let position = x.clamp(0.0, 1.0) * (BINS - 1) as f32;
        let index = (position as usize).min(BINS - 2);
        let within = position - index as f32;
        self.table[index] + (self.table[index + 1] - self.table[index]) * within
    }

    /// Encodes linear RGB to sRGB and applies the curve to each channel.
    pub fn apply(&self, linear: [f32; 3]) -> [f32; 3] {
        linear.map(|channel| self.at(srgb_encode(channel)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curves_never_fall_and_keep_the_ends() {
        for curve in [ToneCurve::standard(), ToneCurve::identity()] {
            assert!(curve.at(0.0).abs() < 1e-4 && (curve.at(1.0) - 1.0).abs() < 1e-4);
            let mut last = 0.0;
            for step in 0..=1000 {
                let value = curve.at(step as f32 / 1000.0);
                assert!(value + 1e-6 >= last);
                last = value;
            }
        }
        assert!(ToneCurve::standard().at(0.5) > 0.5, "midtones lifted");
    }

    #[test]
    fn matching_recovers_a_known_curve() {
        // A "camera" that brightens with a gamma of 0.7 on the encoded
        // values; the matched curve should do the same.
        let raw: Vec<[f32; 3]> = (0..20_000)
            .map(|index| {
                let value = srgb_decode((index % 1000) as f32 / 999.0);
                [value, value, value]
            })
            .collect();
        let jpeg: Vec<[u8; 3]> = raw
            .iter()
            .map(|rgb| {
                let encoded = srgb_encode(rgb[0]).powf(0.7);
                let byte = (encoded * 255.0).round() as u8;
                [byte, byte, byte]
            })
            .collect();
        let curve = ToneCurve::matched(&raw, &jpeg).unwrap();
        for x in [0.1f32, 0.3, 0.5, 0.7, 0.9] {
            let expected = x.powf(0.7);
            assert!(
                (curve.at(x) - expected).abs() < 0.02,
                "at {x}: {} for {expected}",
                curve.at(x)
            );
        }
    }

    #[test]
    fn empty_images_give_no_curve() {
        assert!(ToneCurve::matched(&[], &[[0, 0, 0]]).is_none());
    }

    #[test]
    fn srgb_round_trips() {
        for value in [0.0f32, 0.002, 0.1, 0.5, 1.0] {
            assert!((srgb_decode(srgb_encode(value)) - value).abs() < 1e-5);
        }
    }
}
