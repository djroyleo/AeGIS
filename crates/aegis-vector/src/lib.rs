//! The vector data model of `AeGIS`.
//!
//! Geometry is built on the `GeoRust` ecosystem's [`geo_types`] (`f64`
//! coordinates, polygons with holes, `Multi*` variants), which gives the
//! rest of `AeGIS` free interop with the `geo` algorithm crate and with the
//! common IO crates (`shapefile`, `geojson`, `geozero`, `gdal`, `proj`).
//!
//! This crate holds pure *data*: no layer names, no visibility flags, no
//! symbology, and — like every crate below `aegis-app` — no GUI types.

pub mod dataset;
pub mod feature;
pub mod geometry;

pub use dataset::{FieldDef, FieldKind, Schema, VectorDataset};
pub use feature::{AttributeValue, Feature};
pub use geometry::{Geometry, bounds};
