//! Geometry primitives, re-exported from [`geo_types`].

use geo::BoundingRect;
use geo_types::Rect;

/// The geometry of a single vector feature, in dataset (world) coordinates.
///
/// This is [`geo_types::Geometry`] with `f64` coordinates: points,
/// (multi)linestrings, (multi)polygons with holes, collections, etc.
pub type Geometry = geo_types::Geometry<f64>;

/// Axis-aligned bounding box of a geometry in world coordinates.
///
/// Returns `None` for empty geometries (e.g. a `LineString` with no
/// coordinates), which by definition occupy no space and can be skipped by
/// culling and zoom-to-extent logic.
#[must_use]
pub fn bounds(geometry: &Geometry) -> Option<Rect<f64>> {
    geometry.bounding_rect()
}
