//! The map-rendering layer of `AeGIS`.
//!
//! This crate is deliberately **toolkit-agnostic**: it may one day depend
//! on `wgpu`, but it must *never* depend on egui (or any other GUI
//! toolkit). That rule is what keeps the fast-canvas path independent of
//! the chrome around it, and the chrome swappable.
//!
//! Today it owns the [`MapCamera`] (world ↔ screen math in `f64`) and the
//! [`MapRenderer`] seam. The placeholder painter lives in `aegis-app`'s
//! `render_egui` module; when datasets outgrow it, the real `wgpu`
//! pipeline is built *here* — per-layer vertex-buffer caches, camera as a
//! uniform matrix, raster tiles as textures, R-tree culling/hit-testing —
//! and `aegis-app` embeds it via `egui_wgpu::Callback` without touching
//! panels or project logic.

pub mod camera;
pub mod screen;

pub use camera::MapCamera;
pub use screen::{ScreenPoint, ScreenRect, ScreenVec};

use aegis_project::Project;

/// A map renderer turns (project, camera, viewport) into pixels.
///
/// The future `wgpu` pipeline implements this trait here. The current
/// egui-`Painter` placeholder in `aegis-app` intentionally does *not*: it
/// needs a per-frame `egui::Painter`, which this crate must not know
/// about. Once the GPU path lands, `render_egui` shrinks to a thin
/// `egui_wgpu::Callback` adapter over an implementation of this trait.
pub trait MapRenderer {
    /// Rebuild whatever caches this renderer keeps (GPU buffers, tiles)
    /// for the current project contents.
    fn prepare(&mut self, project: &Project);

    /// Draw the visible portion of the project into `viewport`.
    fn render(&mut self, project: &Project, camera: &MapCamera, viewport: ScreenRect);
}
