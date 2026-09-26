//! A scripted scroll and zoom run for comparing renderers, enabled with
//! `PREV_BENCH=<seconds>`. It idles first to measure idle CPU, then drives
//! one step per frame and prints frame, CPU, memory and tile statistics.

use std::time::{Duration, Instant};

pub const IDLE: Duration = Duration::from_secs(3);
const SCROLL_PER_FRAME: f32 = 30.0;
const ZOOM_PER_FRAME: f32 = 1.02;
const ZOOM_SWING_FRAMES: u32 = 40;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Step {
    ScrollBy(f32),
    Zoom(f32),
    Finish,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Phase {
    Idle,
    Scroll,
    Zoom,
    ScrollZoomed,
    Done,
}

pub struct Bench {
    active: Duration,
    phase: Phase,
    phase_started: Instant,
    idle_cpu: Option<f64>,
    cpu_at_phase: f64,
    active_started: Option<Instant>,
    last_frame: Option<Instant>,
    frames: Vec<(Phase, f32)>,
    zoom_frames: u32,
}

/// Seconds of CPU used by this process so far. `/proc` reports clock ticks
/// in USER_HZ, which is 100 on Linux.
fn cpu_seconds() -> f64 {
    let stat = std::fs::read_to_string("/proc/self/stat").unwrap_or_default();
    // Fields after the command name, which is in parentheses.
    let fields: Vec<&str> = stat
        .rsplit_once(')')
        .map_or("", |(_, rest)| rest)
        .split_whitespace()
        .collect();
    let ticks = |index: usize| {
        fields
            .get(index)
            .and_then(|field| field.parse::<f64>().ok())
            .unwrap_or(0.0)
    };
    (ticks(11) + ticks(12)) / 100.0
}

fn status_kib(key: &str) -> u64 {
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    status
        .lines()
        .find_map(|line| line.strip_prefix(key))
        .and_then(|rest| rest.split_whitespace().next()?.parse().ok())
        .unwrap_or(0)
}

fn percentile(sorted: &[f32], share: f64) -> f32 {
    if sorted.is_empty() {
        return 0.0;
    }
    sorted[((sorted.len() - 1) as f64 * share).round() as usize]
}

impl Bench {
    pub fn from_env() -> Option<Self> {
        let seconds: f64 = std::env::var("PREV_BENCH").ok()?.parse().ok()?;
        Some(Self {
            active: Duration::from_secs_f64(seconds.max(1.0)),
            phase: Phase::Idle,
            phase_started: Instant::now(),
            idle_cpu: None,
            cpu_at_phase: cpu_seconds(),
            active_started: None,
            last_frame: None,
            frames: Vec::new(),
            zoom_frames: 0,
        })
    }

    /// Restarts the idle phase once the document is on screen.
    pub fn start_idle(&mut self) {
        self.phase = Phase::Idle;
        self.phase_started = Instant::now();
        self.cpu_at_phase = cpu_seconds();
    }

    /// Ends the idle phase and starts driving frames.
    pub fn idle_finished(&mut self) {
        if self.phase != Phase::Idle {
            return;
        }
        let wall = self.phase_started.elapsed().as_secs_f64();
        self.idle_cpu = Some((cpu_seconds() - self.cpu_at_phase) / wall);
        self.phase = Phase::Scroll;
        self.phase_started = Instant::now();
        self.active_started = Some(self.phase_started);
        self.cpu_at_phase = cpu_seconds();
        self.last_frame = None;
    }

    pub fn is_driving(&self) -> bool {
        !matches!(self.phase, Phase::Idle | Phase::Done)
    }

    /// Records a frame and returns what to do next.
    pub fn frame(&mut self, now: Instant) -> Option<Step> {
        if !self.is_driving() {
            return None;
        }
        if let Some(last) = self.last_frame {
            self.frames
                .push((self.phase, (now - last).as_secs_f32() * 1000.0));
        }
        self.last_frame = Some(now);
        let elapsed = self.active_started?.elapsed();
        let share = elapsed.as_secs_f64() / self.active.as_secs_f64();
        self.phase = match share {
            share if share >= 1.0 => Phase::Done,
            share if share >= 0.7 => Phase::ScrollZoomed,
            share if share >= 0.4 => Phase::Zoom,
            _ => Phase::Scroll,
        };
        Some(match self.phase {
            Phase::Scroll | Phase::ScrollZoomed => Step::ScrollBy(SCROLL_PER_FRAME),
            Phase::Zoom => {
                self.zoom_frames += 1;
                let zooming_in = (self.zoom_frames / ZOOM_SWING_FRAMES).is_multiple_of(2);
                Step::Zoom(if zooming_in {
                    ZOOM_PER_FRAME
                } else {
                    1.0 / ZOOM_PER_FRAME
                })
            }
            Phase::Done | Phase::Idle => Step::Finish,
        })
    }

    pub fn report(&self, tile_latencies: &[Duration]) -> String {
        let active = self
            .active_started
            .map_or(0.0, |started| started.elapsed().as_secs_f64());
        let active_cpu = (cpu_seconds() - self.cpu_at_phase) / active.max(0.001);
        let mut lines = vec![
            format!("backend: {}", backend_name()),
            format!(
                "idle cpu: {:.1}% of one core",
                self.idle_cpu.unwrap_or(0.0) * 100.0
            ),
            format!("active cpu: {:.0}% of one core", active_cpu * 100.0),
            format!(
                "memory: {:.0} MB resident now, {:.0} MB peak",
                status_kib("VmRSS:") as f64 / 1024.0,
                status_kib("VmHWM:") as f64 / 1024.0
            ),
        ];
        for (name, phases) in [
            (
                "all",
                &[Phase::Scroll, Phase::Zoom, Phase::ScrollZoomed][..],
            ),
            ("scroll", &[Phase::Scroll][..]),
            ("zoom", &[Phase::Zoom][..]),
            ("scroll zoomed", &[Phase::ScrollZoomed][..]),
        ] {
            let mut intervals: Vec<f32> = self
                .frames
                .iter()
                .filter(|(phase, _)| phases.contains(phase))
                .map(|(_, ms)| *ms)
                .collect();
            intervals.sort_by(f32::total_cmp);
            let median = percentile(&intervals, 0.5);
            let long = intervals.iter().filter(|ms| **ms > median * 1.5).count();
            lines.push(format!(
                "frames {name}: {} frames, median {:.2} ms, p95 {:.2} ms, p99 {:.2} ms, max {:.1} ms, {} over 1.5x median",
                intervals.len(),
                median,
                percentile(&intervals, 0.95),
                percentile(&intervals, 0.99),
                intervals.last().copied().unwrap_or(0.0),
                long
            ));
        }
        let mut latencies: Vec<f32> = tile_latencies
            .iter()
            .map(|latency| latency.as_secs_f32() * 1000.0)
            .collect();
        latencies.sort_by(f32::total_cmp);
        lines.push(format!(
            "tiles: {} rendered, request to arrival median {:.1} ms, p95 {:.1} ms, max {:.1} ms",
            latencies.len(),
            percentile(&latencies, 0.5),
            percentile(&latencies, 0.95),
            latencies.last().copied().unwrap_or(0.0)
        ));
        lines.join("\n")
    }
}

fn backend_name() -> String {
    match (std::env::var("ICED_BACKEND"), std::env::var("WGPU_BACKEND")) {
        (Ok(iced), _) => iced,
        (_, Ok(wgpu)) => format!("wgpu {wgpu}"),
        _ => "default".to_owned(),
    }
}
