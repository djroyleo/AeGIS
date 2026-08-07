//! The map canvas: translates egui input into camera navigation, then
//! hands the frame to the render layer.
//!
//! All camera math lives in `aegis_render::MapCamera` (f64, toolkit-free);
//! this module only converts egui's f32 input into it (losslessly — f32 →
//! f64 widening) and triggers painting.

use aegis_project::Project;
use aegis_render::{MapCamera, ScreenPoint, ScreenRect, ScreenVec};
use eframe::egui;

use crate::render_egui;

/// Widen an egui position to render-layer screen coordinates (lossless).
fn screen_point(p: egui::Pos2) -> ScreenPoint {
    ScreenPoint::new(f64::from(p.x), f64::from(p.y))
}

/// Widen an egui rect to a render-layer viewport (lossless).
fn screen_rect(r: egui::Rect) -> ScreenRect {
    ScreenRect::new(screen_point(r.min), screen_point(r.max))
}

pub fn show(ui: &mut egui::Ui, project: &Project, camera: &mut MapCamera) {
    // Claim the whole remaining area as our canvas. We get back a
    // Response (input events on that region) and a Painter (clipped
    // drawing surface for that region).
    let (response, painter) =
        ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
    let viewport = screen_rect(response.rect);

    // ---- Panning ----------------------------------------------------
    if response.dragged() {
        let d = response.drag_delta();
        camera.pan_by_screen_delta(ScreenVec::new(f64::from(d.x), f64::from(d.y)));
    }

    // ---- Zooming (anchored on the cursor) ---------------------------
    if let Some(cursor) = response.hover_pos() {
        // zoom_delta() covers pinch gestures and Ctrl+scroll; we also
        // fold the plain scroll wheel in, since that's the convention
        // in every GIS / web map. (When Ctrl is held, egui reports the
        // scroll as zoom_delta and zeroes smooth_scroll_delta, so the
        // two never double-count.)
        let (pinch, scroll_y) = ui.input(|i| (i.zoom_delta(), i.smooth_scroll_delta.y));
        let zoom = f64::from(pinch) * (f64::from(scroll_y) * 0.005).exp();
        // A factor of exactly 1.0 is a no-op inside zoom_about, so no
        // guard is needed here.
        camera.zoom_about(screen_point(cursor), &viewport, zoom);
    }

    // ---- Rendering --------------------------------------------------
    render_egui::paint(&painter, project, camera, &viewport);
}
