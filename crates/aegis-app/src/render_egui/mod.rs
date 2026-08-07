//! The **placeholder** map renderer: draws layers with egui's `Painter`.
//!
//! This is the CPU path — fine for development-sized data, not for
//! millions of features. When datasets outgrow it, the real pipeline is
//! built in `aegis-render` on wgpu (GPU-cached geometry, camera as a
//! uniform), and this module shrinks to an `egui_wgpu::Callback` adapter.
//! Nothing outside this module knows how layers become pixels — that is
//! the seam that makes the swap invisible to panels and project logic.

mod raster;
mod vector;

use aegis_project::{LayerSource, Project, Symbology};
use aegis_render::{MapCamera, ScreenPoint, ScreenRect};
use eframe::egui;
use geo_types::Rect;

/// Narrow a render-layer screen point to egui's f32 for painting.
///
/// This is the single place world-derived coordinates drop to f32: values
/// here are already screen-relative (small), so the truncation is safe.
#[allow(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    reason = "screen-space magnitudes fit f32; this is the designated f64->f32 boundary"
)]
const fn to_pos2(p: ScreenPoint) -> egui::Pos2 {
    egui::pos2(p.x as f32, p.y as f32)
}

/// Convert a renderer-agnostic color to egui's.
fn to_color32(c: aegis_core::Color) -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)
}

/// Convert a renderer-agnostic line style to an egui stroke.
fn to_stroke(s: aegis_core::LineStyle) -> egui::Stroke {
    egui::Stroke::new(s.width, to_color32(s.color))
}

/// Axis-aligned overlap test for culling (world coordinates).
fn rects_overlap(a: &Rect<f64>, b: &Rect<f64>) -> bool {
    a.min().x <= b.max().x
        && b.min().x <= a.max().x
        && a.min().y <= b.max().y
        && b.min().y <= a.max().y
}

/// Paint every visible layer of the project, bottom of the stack first.
pub fn paint(
    painter: &egui::Painter,
    project: &Project,
    camera: &MapCamera,
    viewport: &ScreenRect,
) {
    for layer in &project.layers {
        if !layer.visible {
            continue;
        }
        match (&layer.source, &layer.symbology) {
            (LayerSource::Vector(dataset), Symbology::Vector(symbology)) => {
                vector::paint_layer(painter, camera, viewport, dataset, symbology);
            }
            (LayerSource::Raster(dataset), Symbology::Raster(symbology)) => {
                raster::paint_layer(painter, camera, viewport, dataset, *symbology);
            }
            // A layer whose symbology kind does not match its data kind
            // cannot be drawn; constructors in aegis-project keep the two
            // in sync, so this arm is unreachable in practice.
            _ => {}
        }
    }
}
