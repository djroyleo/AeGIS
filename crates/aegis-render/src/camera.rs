//! The map camera: world ↔ screen transforms and navigation.
//!
//! World coordinates are `f64` (GIS coordinates do not survive `f32`);
//! screen coordinates are `f64` logical pixels (see [`crate::screen`]).
//! World Y grows upward, screen Y grows downward — the flip happens here
//! and nowhere else.

use geo_types::{Coord, Rect, coord};

use crate::screen::{ScreenPoint, ScreenRect, ScreenVec};

/// The viewer's position over the map.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MapCamera {
    /// The world-coordinate point currently at the center of the viewport.
    pub center: Coord<f64>,
    /// Zoom level: how many screen pixels one world unit occupies.
    pub pixels_per_unit: f64,
}

impl Default for MapCamera {
    fn default() -> Self {
        Self {
            center: coord! { x: 0.0, y: 0.0 },
            pixels_per_unit: 4.0,
        }
    }
}

impl MapCamera {
    pub const MIN_ZOOM: f64 = 0.01;
    pub const MAX_ZOOM: f64 = 1000.0;

    #[must_use]
    pub const fn world_to_screen(&self, world: Coord<f64>, viewport: &ScreenRect) -> ScreenPoint {
        let c = viewport.center();
        ScreenPoint::new(
            (world.x - self.center.x).mul_add(self.pixels_per_unit, c.x),
            (self.center.y - world.y).mul_add(self.pixels_per_unit, c.y),
        )
    }

    #[must_use]
    pub const fn screen_to_world(&self, screen: ScreenPoint, viewport: &ScreenRect) -> Coord<f64> {
        let c = viewport.center();
        coord! {
            x: self.center.x + (screen.x - c.x) / self.pixels_per_unit,
            y: self.center.y - (screen.y - c.y) / self.pixels_per_unit,
        }
    }

    /// The world-space rectangle currently visible in `viewport`.
    #[must_use]
    pub fn visible_world_rect(&self, viewport: &ScreenRect) -> Rect<f64> {
        Rect::new(
            self.screen_to_world(viewport.min, viewport),
            self.screen_to_world(viewport.max, viewport),
        )
    }

    /// Pan so the world slides with a drag of `delta` screen pixels.
    ///
    /// Dragging right moves the camera *left* (the world follows the
    /// cursor), and the Y axis flips once more.
    pub const fn pan_by_screen_delta(&mut self, delta: ScreenVec) {
        self.center.x -= delta.dx / self.pixels_per_unit;
        self.center.y += delta.dy / self.pixels_per_unit;
    }

    /// Multiply the zoom by `factor`, keeping the world point under
    /// `cursor` fixed on screen (the convention in every GIS / web map).
    pub fn zoom_about(&mut self, cursor: ScreenPoint, viewport: &ScreenRect, factor: f64) {
        let before = self.screen_to_world(cursor, viewport);
        self.pixels_per_unit =
            (self.pixels_per_unit * factor).clamp(Self::MIN_ZOOM, Self::MAX_ZOOM);
        let after = self.screen_to_world(cursor, viewport);
        self.center.x += before.x - after.x;
        self.center.y += before.y - after.y;
    }
}
