//! Screen-space geometry in logical pixels, independent of any toolkit.
//!
//! Kept in `f64` so camera math stays lossless end-to-end; the GUI crate
//! converts to its own `f32` types only at the last moment (painting).

/// A position on screen, in logical pixels. Y grows downward.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ScreenPoint {
    pub x: f64,
    pub y: f64,
}

impl ScreenPoint {
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// A displacement on screen, in logical pixels (e.g. a drag delta).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ScreenVec {
    pub dx: f64,
    pub dy: f64,
}

impl ScreenVec {
    #[must_use]
    pub const fn new(dx: f64, dy: f64) -> Self {
        Self { dx, dy }
    }
}

/// An axis-aligned rectangle on screen (the map viewport), logical pixels.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ScreenRect {
    pub min: ScreenPoint,
    pub max: ScreenPoint,
}

impl ScreenRect {
    #[must_use]
    pub const fn new(min: ScreenPoint, max: ScreenPoint) -> Self {
        Self { min, max }
    }

    #[must_use]
    pub const fn center(&self) -> ScreenPoint {
        ScreenPoint::new(
            f64::midpoint(self.min.x, self.max.x),
            f64::midpoint(self.min.y, self.max.y),
        )
    }

    #[must_use]
    pub const fn width(&self) -> f64 {
        self.max.x - self.min.x
    }

    #[must_use]
    pub const fn height(&self) -> f64 {
        self.max.y - self.min.y
    }
}
