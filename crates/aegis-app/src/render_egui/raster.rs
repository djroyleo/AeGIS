//! Raster layer painting (stub).
//!
//! Arrives together with the raster subsystem: bands become an egui
//! texture here (CPU path), and GPU tile textures once the wgpu pipeline
//! in `aegis-render` lands. The seam already exists — `mod.rs` dispatches
//! raster layers to this function — so building it will not touch panels,
//! input handling, or project logic.

use aegis_project::RasterSymbology;
use aegis_raster::RasterDataset;
use aegis_render::{MapCamera, ScreenRect};
use eframe::egui;

#[allow(
    clippy::missing_const_for_fn,
    reason = "stub: only const because it is empty; the real body will not be"
)]
pub fn paint_layer(
    _painter: &egui::Painter,
    _camera: &MapCamera,
    _viewport: &ScreenRect,
    _dataset: &RasterDataset,
    _symbology: RasterSymbology,
) {
    // Intentionally empty until raster rendering is implemented.
}
