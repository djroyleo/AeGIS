//! Spatial data IO for `AeGIS`: folder connections and format drivers.
//!
//! The design mirrors GDAL/OGR: every file format is a [`DatasetDriver`]
//! registered in a [`DriverRegistry`]. The Catalog panel enumerates the
//! registry to decide what it can open; adding `GeoJSON`, shapefile, or
//! `GeoPackage` support means implementing one trait and registering it —
//! no GUI code changes.
//!
//! Note the deliberate boundary: drivers speak [`Dataset`] (pure data), not
//! `aegis-project::Layer` — a file on disk has no symbology or visibility.
//! Turning a freshly read dataset into a styled layer is the caller's job.

pub mod catalog;
pub mod driver;

pub use catalog::{CatalogEntry, FolderConnection};
pub use driver::{Dataset, DatasetDriver, DriverRegistry};
