//! M3 Expressive spring motion.

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

/// Stiffness and damping ratio of a spring, from the M3 motion tokens.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Motion {
    pub stiffness: f32,
    pub damping: f32,
}

impl Motion {
    /// Expressive spatial springs overshoot a little; use them for
    /// position, size and shape.
    pub const FAST_SPATIAL: Motion = Motion {
        stiffness: 800.0,
        damping: 0.6,
    };
    pub const DEFAULT_SPATIAL: Motion = Motion {
        stiffness: 380.0,
        damping: 0.8,
    };
    /// Effects springs never overshoot; use them for color and opacity.
    pub const FAST_EFFECTS: Motion = Motion {
        stiffness: 3800.0,
        damping: 1.0,
    };
    pub const DEFAULT_EFFECTS: Motion = Motion {
        stiffness: 1600.0,
        damping: 1.0,
    };
}

static REDUCED: AtomicBool = AtomicBool::new(false);

/// Makes every spring jump straight to its target.
pub fn set_reduced(reduced: bool) {
    REDUCED.store(reduced, Ordering::Relaxed);
}

pub fn reduced() -> bool {
    REDUCED.load(Ordering::Relaxed)
}

/// Largest integration step, for stability with stiff springs.
const STEP: f32 = 1.0 / 480.0;
/// Frames longer than this (a stall or a hidden window) are cut short.
const LONGEST_FRAME: f32 = 0.1;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spring {
    pub value: f32,
    pub velocity: f32,
    pub target: f32,
    motion: Motion,
    /// How close counts as arrived, in the value's units.
    precision: f32,
    last: Option<Instant>,
}

impl Spring {
    pub fn new(value: f32, motion: Motion) -> Self {
        Self {
            value,
            velocity: 0.0,
            target: value,
            motion,
            precision: 0.01,
            last: None,
        }
    }

    pub fn precision(mut self, precision: f32) -> Self {
        self.precision = precision;
        self
    }

    pub fn set_motion(&mut self, motion: Motion) {
        self.motion = motion;
    }

    pub fn go_to(&mut self, target: f32) {
        if target != self.target {
            self.target = target;
            if reduced() {
                self.jump_to(target);
            }
        }
    }

    pub fn jump_to(&mut self, target: f32) {
        self.target = target;
        self.value = target;
        self.velocity = 0.0;
        self.last = None;
    }

    pub fn is_settled(&self) -> bool {
        (self.value - self.target).abs() < self.precision && self.velocity.abs() < self.precision
    }

    /// Advances to `now`. Returns whether the spring still moves.
    pub fn advance(&mut self, now: Instant) -> bool {
        if self.is_settled() {
            self.value = self.target;
            self.velocity = 0.0;
            self.last = None;
            return false;
        }
        let Some(last) = self.last.replace(now) else {
            // The first frame only starts the clock.
            return true;
        };
        let mut remaining = now
            .saturating_duration_since(last)
            .as_secs_f32()
            .min(LONGEST_FRAME);
        let stiffness = self.motion.stiffness;
        let friction = 2.0 * self.motion.damping * stiffness.sqrt();
        while remaining > 0.0 {
            let step = remaining.min(STEP);
            let acceleration = -stiffness * (self.value - self.target) - friction * self.velocity;
            self.velocity += acceleration * step;
            self.value += self.velocity * step;
            remaining -= step;
        }
        if self.is_settled() {
            self.value = self.target;
            self.velocity = 0.0;
            self.last = None;
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn run(spring: &mut Spring, seconds: f32) -> f32 {
        let start = Instant::now();
        let mut peak = spring.value;
        let mut frame = 0;
        spring.advance(start);
        while frame < (seconds * 120.0) as u32 {
            frame += 1;
            let now = start + Duration::from_secs_f32(frame as f32 / 120.0);
            spring.advance(now);
            peak = peak.max(spring.value);
        }
        peak
    }

    #[test]
    fn spatial_springs_overshoot_and_settle() {
        let mut spring = Spring::new(0.0, Motion::FAST_SPATIAL);
        spring.go_to(1.0);
        let peak = run(&mut spring, 1.0);
        assert!(peak > 1.02, "expressive springs bounce, peak {peak}");
        assert!(spring.is_settled());
        assert_eq!(spring.value, 1.0);
    }

    #[test]
    fn effects_springs_do_not_overshoot() {
        let mut spring = Spring::new(0.0, Motion::DEFAULT_EFFECTS).precision(0.001);
        spring.go_to(1.0);
        let peak = run(&mut spring, 1.0);
        assert!(peak <= 1.0 + 1e-3, "peak {peak}");
        assert!(spring.is_settled());
    }

    #[test]
    fn settled_springs_stop_asking_for_frames() {
        let mut spring = Spring::new(4.0, Motion::DEFAULT_SPATIAL);
        assert!(!spring.advance(Instant::now()));
        spring.jump_to(2.0);
        assert!(!spring.advance(Instant::now()));
        assert_eq!(spring.value, 2.0);
    }
}
