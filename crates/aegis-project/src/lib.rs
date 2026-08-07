//! The "map document" of `AeGIS`: what the user sees in the Contents panel.
//!
//! This crate owns the split between *data* and *presentation*:
//! `aegis-vector`/`aegis-raster` hold pure datasets; a [`Layer`] here pairs
//! one dataset with a name, visibility, and [`Symbology`], and a
//! [`Project`] is the ordered stack of layers.
//!
//! Adding a new kind of spatial data to `AeGIS` means adding a variant to
//! [`LayerSource`] (plus a renderer arm behind the render seam) — panels,
//! input handling, and project logic stay untouched.

pub mod layer;
pub mod project;
pub mod symbology;

pub use layer::{Layer, LayerSource};
pub use project::Project;
pub use symbology::{RasterSymbology, Symbology, VectorSymbology};
