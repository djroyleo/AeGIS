//! Coordinate reference system identification.
//!
//! For now this is only a *label* carried alongside datasets so that the
//! rest of the architecture can plumb it through from day one. Actual
//! reprojection (via e.g. the `proj` crate) is future work.

/// Identifies the coordinate reference system of a dataset or map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Crs {
    /// CRS is unknown or irrelevant (e.g. dev sample data in plain XY).
    #[default]
    Unknown,
    /// An EPSG-registered CRS, e.g. `Epsg(4326)` for WGS 84.
    Epsg(u32),
}
